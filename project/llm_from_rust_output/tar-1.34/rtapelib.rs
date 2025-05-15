use std::ffi::{CString, CStr};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{RawFd, AsRawFd};
use std::process::{Command, Stdio};
use std::io::{self, Read, Write};
use std::ptr;
use std::mem;
use libc::{self, c_int, c_char, c_long, c_ulong, c_void, pid_t, uid_t, gid_t, off_t, size_t, ssize_t};
use nix::unistd::{fork, ForkResult, dup2, close, pipe};
use nix::sys::wait::waitpid;
use nix::errno::Errno;

const RMT_COMMAND: &str = "/usr/local/libexec/rmt";
const MAX_PIPES: usize = 4;

struct RemoteConnection {
    from_remote: [RawFd; 2],
    to_remote: [RawFd; 2],
}

impl RemoteConnection {
    fn new() -> io::Result<Self> {
        let from_remote = pipe()?;
        let to_remote = pipe()?;
        Ok(Self {
            from_remote: [from_remote.0, from_remote.1],
            to_remote: [to_remote.0, to_remote.1],
        })
    }

    fn shutdown(&mut self, errno: i32) {
        let _ = close(self.from_remote[0]);
        let _ = close(self.to_remote[1]);
        self.from_remote[0] = -1;
        self.to_remote[1] = -1;
        unsafe { *libc::__errno_location() = errno };
    }
}

struct RemoteConnections {
    connections: [Option<RemoteConnection>; MAX_PIPES],
}

impl RemoteConnections {
    fn new() -> Self {
        Self {
            connections: [None, None, None, None],
        }
    }

    fn get_available(&mut self) -> Option<(usize, &mut RemoteConnection)> {
        for (i, conn) in self.connections.iter_mut().enumerate() {
            if conn.is_none() {
                *conn = RemoteConnection::new().ok();
                return conn.as_mut().map(|c| (i, c));
            }
        }
        None
    }

    fn get(&mut self, handle: usize) -> Option<&mut RemoteConnection> {
        self.connections.get_mut(handle).and_then(|c| c.as_mut())
    }
}

static mut CONNECTIONS: RemoteConnections = RemoteConnections::new();

fn sys_reset_uid_gid() -> io::Result<()> {
    let uid = unsafe { libc::getuid() };
    let gid = unsafe { libc::getgid() };
    
    let pw = unsafe { libc::getpwuid(uid) };
    if pw.is_null() {
        return Err(io::Error::last_os_error());
    }

    let pw_name = unsafe { CStr::from_ptr((*pw).pw_name) };
    if unsafe { libc::initgroups(pw_name.as_ptr(), libc::getgid()) } != 0 {
        return Err(io::Error::last_os_error());
    }

    if gid != unsafe { libc::getegid() } {
        if unsafe { libc::setgid(gid) } != 0 {
            let err = io::Error::last_os_error();
            if err.raw_os_error() != Some(1) {
                return Err(err);
            }
        }
    }

    if uid != unsafe { libc::geteuid() } {
        if unsafe { libc::setuid(uid) } != 0 {
            let err = io::Error::last_os_error();
            if err.raw_os_error() != Some(1) {
                return Err(err);
            }
        }
    }

    Ok(())
}

fn do_command(conn: &mut RemoteConnection, buffer: &[u8]) -> io::Result<()> {
    let mut pipe_handler = unsafe {
        libc::signal(
            libc::SIGPIPE,
            libc::SIG_IGN as libc::sighandler_t,
        )
    };

    let written = unsafe {
        libc::write(
            conn.to_remote[1],
            buffer.as_ptr() as *const c_void,
            buffer.len(),
        )
    };

    unsafe {
        libc::signal(libc::SIGPIPE, pipe_handler);
    }

    if written == buffer.len() as isize {
        Ok(())
    } else {
        conn.shutdown(libc::EIO);
        Err(io::Error::last_os_error())
    }
}

fn get_status_string(conn: &mut RemoteConnection) -> io::Result<String> {
    let mut buffer = [0u8; 64];
    let mut cursor = 0;

    while cursor < 64 {
        let read = unsafe {
            libc::read(
                conn.from_remote[0],
                buffer[cursor..].as_mut_ptr() as *mut c_void,
                1,
            )
        };

        if read != 1 {
            conn.shutdown(libc::EIO);
            return Err(io::Error::last_os_error());
        }

        if buffer[cursor] == b'\n' {
            buffer[cursor] = 0;
            break;
        }

        cursor += 1;
    }

    if cursor == 64 {
        conn.shutdown(libc::EIO);
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Status too long"));
    }

    let status = unsafe { CStr::from_ptr(buffer.as_ptr() as *const c_char) };
    let status_str = status.to_string_lossy().trim().to_string();

    if status_str.starts_with('E') || status_str.starts_with('F') {
        let mut ch = 0u8;
        loop {
            let read = unsafe {
                libc::read(
                    conn.from_remote[0],
                    &mut ch as *mut u8 as *mut c_void,
                    1,
                )
            };

            if read != 1 || ch == b'\n' {
                break;
            }
        }

        let errno = status_str[1..].parse().unwrap_or(libc::EIO);
        if status_str.starts_with('F') {
            conn.shutdown(errno);
        }
        unsafe { *libc::__errno_location() = errno };
        return Err(io::Error::from_raw_os_error(errno));
    }

    if !status_str.starts_with('A') {
        conn.shutdown(libc::EIO);
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid status"));
    }

    Ok(status_str[1..].to_string())
}

fn get_status(conn: &mut RemoteConnection) -> io::Result<c_long> {
    let status = get_status_string(conn)?;
    let result = status.parse().unwrap_or(-1);
    if result >= 0 {
        Ok(result)
    } else {
        unsafe { *libc::__errno_location() = libc::EIO };
        Err(io::Error::from_raw_os_error(libc::EIO))
    }
}

fn get_status_off(conn: &mut RemoteConnection) -> io::Result<off_t> {
    let status = get_status_string(conn)?;
    let mut chars = status.chars().peekable();
    let mut negative = false;
    let mut count: off_t = 0;

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => { chars.next(); }
            '-' => {
                negative = true;
                chars.next();
            }
            '+' => { chars.next(); }
            _ => break,
        }
    }

    for c in chars {
        if !c.is_digit(10) {
            return Err(io::Error::from_raw_os_error(libc::EINVAL));
        }

        let digit = c.to_digit(10).unwrap() as off_t;
        let c10 = 10 * count;
        let nc = if negative { c10 - digit } else { c10 + digit };

        if c10 / 10 != count || (negative && c10 < nc) || (!negative && nc < c10) {
            return Err(io::Error::from_raw_os_error(libc::EOVERFLOW));
        }

        count = nc;
    }

    Ok(count)
}

pub fn rmt_open(file_name: &str, open_mode: c_int, bias: c_int, remote_shell: Option<&str>) -> io::Result<c_int> {
    unsafe {
        let (handle, conn) = match CONNECTIONS.get_available() {
            Some((h, c)) => (h, c),
            None => return Err(io::Error::from_raw_os_error(libc::EMFILE)),
        };

        // Parse remote host, user and file
        // ... (implementation omitted for brevity)

        let remote_shell = remote_shell.unwrap_or("/usr/bin/rsh");
        let remote_shell_basename = remote_shell.rsplit('/').next().unwrap();

        match fork() {
            Ok(ForkResult::Parent { child }) => {
                close(conn.to_remote[0])?;
                close(conn.from_remote[1])?;

                // Send open command
                let mut command = format!("O{}\n", remote_file);
                // ... (append open_mode flags)
                command.push_str("\n");

                do_command(conn, command.as_bytes())?;
                get_status(conn)?;

                Ok(handle as c_int + bias)
            }
            Ok(ForkResult::Child) => {
                dup2(conn.to_remote[0], 0)?;
                dup2(conn.from_remote[1], 1)?;
                
                close(conn.to_remote[1])?;
                close(conn.from_remote[0])?;

                sys_reset_uid_gid()?;

                let mut cmd = Command::new(remote_shell);
                if let Some(user) = remote_user {
                    cmd.args(&[remote_host, "-l", user, RMT_COMMAND]);
                } else {
                    cmd.args(&[remote_host, RMT_COMMAND]);
                }

                let status = cmd.status()?;
                std::process::exit(status.code().unwrap_or(1));
            }
            Err(e) => Err(e.into()),
        }
    }
}

pub fn rmt_close(handle: c_int) -> io::Result<c_int> {
    unsafe {
        if let Some(conn) = CONNECTIONS.get(handle as usize) {
            do_command(conn, b"C\n")?;
            let status = get_status(conn)?;
            conn.shutdown(0);
            Ok(status as c_int)
        } else {
            Err(io::Error::from_raw_os_error(libc::EBADF))
        }
    }
}

pub fn rmt_read(handle: c_int, buffer: &mut [u8], length: size_t) -> io::Result<size_t> {
    unsafe {
        if let Some(conn) = CONNECTIONS.get(handle as usize) {
            let cmd = format!("R{}\n", length);
            do_command(conn, cmd.as_bytes())?;
            
            let status = get_status(conn)? as size_t;
            if status > length {
                return Err(io::Error::from_raw_os_error(libc::EIO));
            }

            let mut total = 0;
            while total < status {
                let read = libc::read(
                    conn.from_remote[0],
                    buffer[total..].as_mut_ptr() as *mut c_void,
                    status - total,
                );

                if read <= 0 {
                    conn.shutdown(libc::EIO);
                    return Err(io::Error::last_os_error());
                }

                total += read as size_t;
            }

            Ok(status)
        } else {
            Err(io::Error::from_raw_os_error(libc::EBADF))
        }
    }
}

// Similar implementations for rmt_write, rmt_lseek, rmt_ioctl
// ... (omitted for brevity)