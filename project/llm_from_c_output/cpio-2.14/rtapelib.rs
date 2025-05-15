use std::{
    ffi::{CStr, CString},
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    mem,
    net::{TcpStream, ToSocketAddrs},
    os::unix::{
        io::{AsRawFd, FromRawFd, RawFd},
        net::UnixStream,
    },
    path::{Path, PathBuf},
    process::{Command, Stdio},
    str,
    sync::Mutex,
    time::Duration,
};

use libc::{self, c_int, c_void, off_t, size_t};
use nix::{
    sys::signal::{self, Signal},
    unistd::{self, Pid},
};

const MAXUNIT: usize = 4;
const COMMAND_BUFFER_SIZE: usize = 64;
const EXIT_ON_EXEC_ERROR: i32 = 128;

const PREAD: usize = 0;
const PWRITE: usize = 1;

static mut FROM_REMOTE: [[c_int; 2]; MAXUNIT] = [[-1; 2]; MAXUNIT];
static mut TO_REMOTE: [[c_int; 2]; MAXUNIT] = [[-1; 2]; MAXUNIT];

static RMT_COMMAND: &str = "/etc/rmt";
static mut RMT_DEV_NAME: Option<CString> = None;
static mut FORCE_LOCAL_OPTION: bool = false;

struct RemoteTape {
    read_fd: RawFd,
    write_fd: RawFd,
}

impl RemoteTape {
    fn new(read_fd: RawFd, write_fd: RawFd) -> Self {
        Self { read_fd, write_fd }
    }

    fn shutdown(&mut self, errno: i32) -> io::Result<()> {
        unsafe {
            libc::close(self.read_fd);
            libc::close(self.write_fd);
        }
        self.read_fd = -1;
        self.write_fd = -1;
        Err(io::Error::from_raw_os_error(errno))
    }

    fn do_command(&self, buffer: &str) -> io::Result<()> {
        let length = buffer.len();
        let pipe_handler = signal::sigaction(
            Signal::SIGPIPE,
            &signal::SigAction::new(
                signal::SigHandler::SigIgn,
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )?;

        let written = unsafe {
            libc::write(
                self.write_fd,
                buffer.as_ptr() as *const c_void,
                length,
            )
        };

        signal::sigaction(Signal::SIGPIPE, &pipe_handler)?;

        if written == length as isize {
            Ok(())
        } else {
            self.shutdown(libc::EIO)
        }
    }

    fn get_status_string(&self, command_buffer: &mut [u8]) -> io::Result<&str> {
        let mut cursor = 0;
        for i in 0..COMMAND_BUFFER_SIZE {
            let read = unsafe {
                libc::read(
                    self.read_fd,
                    command_buffer[i..].as_mut_ptr() as *mut c_void,
                    1,
                )
            };
            if read != 1 {
                return self.shutdown(libc::EIO);
            }
            if command_buffer[i] == b'\n' {
                command_buffer[i] = 0;
                cursor = i;
                break;
            }
        }

        if cursor == COMMAND_BUFFER_SIZE {
            return self.shutdown(libc::EIO);
        }

        let status_str = unsafe {
            CStr::from_ptr(command_buffer.as_ptr() as *const i8)
                .to_str()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))?
        };

        if status_str.starts_with('E') || status_str.starts_with('F') {
            let mut character = 0u8;
            loop {
                let read = unsafe {
                    libc::read(
                        self.read_fd,
                        &mut character as *mut _ as *mut c_void,
                        1,
                    )
                };
                if read != 1 || character == b'\n' {
                    break;
                }
            }

            let errno = status_str[1..].parse().unwrap_or(libc::EIO);
            if status_str.starts_with('F') {
                return self.shutdown(errno);
            }
            return Err(io::Error::from_raw_os_error(errno));
        }

        if !status_str.starts_with('A') {
            return self.shutdown(libc::EIO);
        }

        Ok(&status_str[1..])
    }

    fn get_status(&self) -> io::Result<i64> {
        let mut command_buffer = [0u8; COMMAND_BUFFER_SIZE];
        let status = self.get_status_string(&mut command_buffer)?;
        let result = status.parse().unwrap_or(-1);
        if result >= 0 {
            Ok(result)
        } else {
            Err(io::Error::from_raw_os_error(libc::EIO))
        }
    }

    fn get_status_off(&self) -> io::Result<off_t> {
        let mut command_buffer = [0u8; COMMAND_BUFFER_SIZE];
        let status = self.get_status_string(&mut command_buffer)?;

        let mut count: off_t = 0;
        let mut negative = false;
        let mut chars = status.chars();

        while let Some(c) = chars.next() {
            if !c.is_whitespace() {
                negative = c == '-';
                if c == '+' || c == '-' {
                    continue;
                }
                break;
            }
        }

        for c in chars {
            if let Some(digit) = c.to_digit(10) {
                let c10 = 10 * count;
                let nc = if negative {
                    c10 - digit as off_t
                } else {
                    c10 + digit as off_t
                };

                if c10 / 10 != count || (negative && c10 < nc) || (!negative && nc < c10) {
                    return Err(io::Error::from_raw_os_error(libc::EOVERFLOW));
                }
                count = nc;
            } else {
                break;
            }
        }

        Ok(count)
    }
}

fn encode_oflag(oflag: i32) -> String {
    let mut buf = format!("{} ", oflag);

    match oflag & libc::O_ACCMODE {
        libc::O_RDONLY => buf.push_str("O_RDONLY"),
        libc::O_RDWR => buf.push_str("O_RDWR"),
        libc::O_WRONLY => buf.push_str("O_WRONLY"),
        _ => panic!("Invalid open mode"),
    }

    if oflag & libc::O_APPEND != 0 {
        buf.push_str("|O_APPEND");
    }
    if oflag & libc::O_CREAT != 0 {
        buf.push_str("|O_CREAT");
    }
    if oflag & libc::O_DSYNC != 0 {
        buf.push_str("|O_DSYNC");
    }
    if oflag & libc::O_EXCL != 0 {
        buf.push_str("|O_EXCL");
    }
    if oflag & libc::O_LARGEFILE != 0 {
        buf.push_str("|O_LARGEFILE");
    }
    if oflag & libc::O_NOCTTY != 0 {
        buf.push_str("|O_NOCTTY");
    }
    if oflag & libc::O_NONBLOCK != 0 {
        buf.push_str("|O_NONBLOCK");
    }
    if oflag & libc::O_RSYNC != 0 {
        buf.push_str("|O_RSYNC");
    }
    if oflag & libc::O_SYNC != 0 {
        buf.push_str("|O_SYNC");
    }
    if oflag & libc::O_TRUNC != 0 {
        buf.push_str("|O_TRUNC");
    }

    buf
}

fn sys_reset_uid_gid() -> io::Result<()> {
    let uid = unistd::getuid();
    let gid = unistd::getgid();
    let pw = unsafe { libc::getpwuid(uid.as_raw()) };
    if pw.is_null() {
        return Err(io::Error::last_os_error());
    }

    let name = unsafe { CStr::from_ptr((*pw).pw_name) }.to_str()?;
    unistd::initgroups(name, gid)?;

    if gid != unistd::getegid() {
        unistd::setgid(gid).map_err(|e| io::Error::from_raw_os_error(e as i32))?;
    }

    if uid != unistd::geteuid() {
        unistd::setuid(uid).map_err(|e| io::Error::from_raw_os_error(e as i32))?;
    }

    Ok(())
}

pub fn rmt_open(file_name: &str, open_mode: i32, bias: i32, remote_shell: Option<&str>) -> io::Result<i32> {
    let mut remote_pipe_number = 0;
    unsafe {
        while remote_pipe_number < MAXUNIT {
            if FROM_REMOTE[remote_pipe_number][PREAD] == -1 && FROM_REMOTE[remote_pipe_number][PWRITE] == -1 {
                break;
            }
            remote_pipe_number += 1;
        }
    }

    if remote_pipe_number == MAXUNIT {
        return Err(io::Error::from_raw_os_error(libc::EMFILE));
    }

    let (remote_host, remote_user, remote_file) = parse_file_name(file_name)?;

    // Validate host
    let _ = format!("{}:0", remote_host)
        .to_socket_addrs()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Cannot resolve host"))?;

    let remote_shell = remote_shell.unwrap_or("ssh");

    let mut child = Command::new(remote_shell)
        .arg(remote_host)
        .arg(RMT_COMMAND)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    unsafe {
        TO_REMOTE[remote_pipe_number][PWRITE] = stdin.as_raw_fd();
        FROM_REMOTE[remote_pipe_number][PREAD] = stdout.as_raw_fd();
    }

    mem::forget(stdin);
    mem::forget(stdout);

    let command = format!("O{}\n{}\n", remote_file, encode_oflag(open_mode));
    let mut tape = RemoteTape::new(
        unsafe { FROM_REMOTE[remote_pipe_number][PREAD] },
        unsafe { TO_REMOTE[remote_pipe_number][PWRITE] },
    );

    tape.do_command(&command)?;
    tape.get_status()?;

    Ok(remote_pipe_number as i32 + bias)
}

pub fn rmt_close(handle: i32) -> io::Result<()> {
    let mut tape = RemoteTape::new(
        unsafe { FROM_REMOTE[handle as usize][PREAD] },
        unsafe { TO_REMOTE[handle as usize][PWRITE] },
    );
    tape.do_command("C\n")?;
    tape.get_status()?;
    tape.shutdown(0)
}

pub fn rmt_read(handle: i32, buffer: &mut [u8], length: usize) -> io::Result<usize> {
    let mut tape = RemoteTape::new(
        unsafe { FROM_REMOTE[handle as usize][PREAD] },
        unsafe { TO_REMOTE[handle as usize][PWRITE] },
    );

    let command = format!("R{}\n", length);
    tape.do_command(&command)?;
    let status = tape.get_status()? as usize;

    if status > length {
        return Err(io::Error::from_raw_os_error(libc::EIO));
    }

    let mut total = 0;
    while total < status {
        let read = unsafe {
            libc::read(
                tape.read_fd,
                buffer[total..].as_mut_ptr() as *mut c_void,
                status - total,
            )
        };
        if read <= 0 {
            return tape.shutdown(libc::EIO);
        }
        total += read as usize;
    }

    Ok(total)
}

pub fn rmt_write(handle: i32, buffer: &[u8], length: usize) -> io::Result<usize> {
    let mut tape = RemoteTape::new(
        unsafe { FROM_REMOTE[handle as usize][PREAD] },
        unsafe { TO_REMOTE[handle as usize][PWRITE] },
    );

    let command = format!("W{}\n", length);
    tape.do_command(&command)?;

    let pipe_handler = signal::sigaction(
        Signal::SIGPIPE,
        &signal::SigAction::new(
            signal::SigHandler::SigIgn,
            signal::SaFlags::empty(),
            signal::SigSet::empty(),
        ),
    )?;

    let written = unsafe {
        libc::write(
            tape.write_fd,
            buffer.as_ptr() as *const c_void,
            length,
        )
    };

    signal::sigaction(Signal::SIGPIPE, &pipe_handler)?;

    if written == length as isize {
        let r = tape.get_status()?;
        if r == length as i64 {
            Ok(length)
        } else {
            Ok(r as usize)
        }
    } else {
        tape.shutdown(libc::EIO).map(|_| 0)
    }
}

pub fn rmt_lseek(handle: i32, offset: off_t, whence: i32) -> io::Result<off_t> {
    let mut tape = RemoteTape::new(
        unsafe { FROM_REMOTE[handle as usize][PREAD] },
        unsafe { TO_REMOTE[handle as usize][PWRITE] },
    );

    let whence = match whence {
        libc::SEEK_SET => 0,
        libc::SEEK_CUR => 1,
        libc::SEEK_END => 2,
        _ => panic!("Invalid whence"),
    };

    let command = format!("L{}\n{}\n", offset, whence);
    tape.do_command(&command)?;
    tape.get_status_off()
}

fn parse_file_name(file_name: &str) -> io::Result<(&str, Option<&str>, &str)> {
    let mut remote_host = file_name;
    let mut remote_user = None;
    let mut remote_file = None;

    for (i, c) in file_name.chars().enumerate() {
        match c {
            '\n' => return Err(io::Error::from_raw_os_error(libc::ENOENT)),
            '@' if remote_user.is_none() => {
                remote_user = Some(&file_name[..i]);
                remote_host = &file_name[i + 1..];
            }
            ':' if remote_file.is_none() => {
                remote_file = Some(&file_name[i + 1..]);
                break;
            }
            _ => {}
        }
    }

    Ok((
        remote_host,
        remote_user,
        remote_file.ok_or_else(|| io::Error::from_raw_os_error(libc::ENOENT))?,
    ))
}