use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::{self, Read, Write, BufRead, BufReader, Seek, SeekFrom};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::process;
use std::ptr;
use std::str;
use libc::{c_int, c_long, c_ulong, c_char, c_void, size_t, off_t, uintmax_t};
use nix::errno::Errno;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, lseek, read, write};

struct RmtState {
    device_fd: Option<RawFd>,
    record_buffer: Vec<u8>,
    input_buf: String,
    debug_level: i32,
    debug_out: Option<File>,
}

impl RmtState {
    fn new() -> Self {
        RmtState {
            device_fd: None,
            record_buffer: Vec::new(),
            input_buf: String::new(),
            debug_level: 0,
            debug_out: None,
        }
    }

    fn debug_print(&mut self, level: i32, msg: &str) {
        if self.debug_level >= level {
            if let Some(file) = &mut self.debug_out {
                writeln!(file, "{}", msg).ok();
            } else {
                eprintln!("{}", msg);
            }
        }
    }

    fn read_line(&mut self) -> io::Result<Option<String>> {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        self.input_buf.clear();
        let bytes_read = handle.read_line(&mut self.input_buf)?;
        if bytes_read == 0 {
            Ok(None)
        } else {
            let line = self.input_buf.trim_end().to_string();
            self.debug_print(10, &format!("C: {}", line));
            Ok(Some(line))
        }
    }

    fn write_response(&mut self, msg: &str) -> io::Result<()> {
        print!("{}", msg);
        io::stdout().flush()?;
        self.debug_print(10, &format!("S: {}", msg));
        Ok(())
    }

    fn reply(&mut self, code: u64) -> io::Result<()> {
        self.write_response(&format!("A{}\n", code))
    }

    fn error_message(&mut self, code: i32, msg: &str) -> io::Result<()> {
        self.debug_print(10, &format!("S: E{}\n", code));
        self.debug_print(10, &format!("S: {}\n", msg));
        self.debug_print(1, &format!("error: {}\n", msg));
        self.write_response(&format!("E{}\n{}\n", code, msg))
    }

    fn error(&mut self, err: Errno) -> io::Result<()> {
        self.error_message(err as i32, &err.desc())
    }

    fn prepare_record_buffer(&mut self, size: usize) {
        if size > self.record_buffer.len() {
            self.record_buffer.resize(size, 0);
        }
    }

    fn open_device(&mut self, device: &str, flags: i32) -> io::Result<()> {
        let mode = Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IWGRP | Mode::S_IROTH;
        let oflag = OFlag::from_bits_truncate(flags);
        
        if let Some(fd) = self.device_fd {
            close(fd)?;
        }
        
        let fd = open(Path::new(device), oflag, mode)?;
        self.device_fd = Some(fd);
        self.reply(0)
    }

    fn close_device(&mut self) -> io::Result<()> {
        if let Some(fd) = self.device_fd.take() {
            close(fd)?;
        }
        self.reply(0)
    }

    fn seek_device(&mut self, whence: SeekFrom, offset: off_t) -> io::Result<()> {
        let fd = self.device_fd.ok_or(Errno::EBADF)?;
        let new_offset = lseek(fd, offset, whence)?;
        self.reply(new_offset as u64)
    }

    fn read_device(&mut self, size: usize) -> io::Result<()> {
        let fd = self.device_fd.ok_or(Errno::EBADF)?;
        self.prepare_record_buffer(size);
        
        let bytes_read = read(fd, &mut self.record_buffer[..size])?;
        self.reply(bytes_read as u64)?;
        io::stdout().write_all(&self.record_buffer[..bytes_read])?;
        io::stdout().flush()
    }

    fn write_device(&mut self, size: usize) -> io::Result<()> {
        let fd = self.device_fd.ok_or(Errno::EBADF)?;
        self.prepare_record_buffer(size);
        
        io::stdin().read_exact(&mut self.record_buffer[..size])?;
        let bytes_written = write(fd, &self.record_buffer[..size])?;
        
        if bytes_written != size {
            Err(Errno::EIO.into())
        } else {
            self.reply(bytes_written as u64)
        }
    }

    fn handle_command(&mut self, cmd: &str) -> io::Result<bool> {
        if cmd.is_empty() {
            return Ok(false);
        }

        let (cmd_char, args) = cmd.split_at(1);
        match cmd_char {
            "C" => {
                self.close_device()?;
                Ok(true)
            }
            "O" => {
                let flag_line = self.read_line()?.ok_or(Errno::EIO)?;
                let flags = parse_open_flags(&flag_line)?;
                self.open_device(args, flags)?;
                Ok(false)
            }
            "L" => {
                let (whence, offset) = parse_seek_args(args)?;
                self.seek_device(whence, offset)?;
                Ok(false)
            }
            "R" => {
                let size = parse_size(args)?;
                self.read_device(size)?;
                Ok(false)
            }
            "W" => {
                let size = parse_size(args)?;
                self.write_device(size)?;
                Ok(false)
            }
            _ => {
                self.error_message(22, "Garbage command")?;
                Err(Errno::EINVAL.into())
            }
        }
    }
}

fn parse_open_flags(flags_str: &str) -> io::Result<i32> {
    // Simplified flag parsing - actual implementation should handle all cases
    let mut flags = 0;
    for part in flags_str.split('|') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        
        flags |= match part {
            "O_RDONLY" => libc::O_RDONLY,
            "O_WRONLY" => libc::O_WRONLY,
            "O_RDWR" => libc::O_RDWR,
            "O_CREAT" => libc::O_CREAT,
            "O_EXCL" => libc::O_EXCL,
            "O_TRUNC" => libc::O_TRUNC,
            "O_APPEND" => libc::O_APPEND,
            _ => {
                if let Ok(num) = part.parse::<i32>() {
                    num
                } else {
                    return Err(Errno::EINVAL.into());
                }
            }
        };
    }
    Ok(flags)
}

fn parse_seek_args(args: &str) -> io::Result<(SeekFrom, off_t)> {
    let (whence_str, offset_str) = args.split_once(' ').unwrap_or((args, "0"));
    let offset = offset_str.parse::<off_t>().map_err(|_| Errno::EINVAL)?;
    
    let whence = match whence_str {
        "0" | "SEEK_SET" => SeekFrom::Start(offset as u64),
        "1" | "SEEK_CUR" => SeekFrom::Current(offset),
        "2" | "SEEK_END" => SeekFrom::End(offset),
        _ => return Err(Errno::EINVAL.into()),
    };
    
    Ok((whence, offset))
}

fn parse_size(args: &str) -> io::Result<usize> {
    args.parse::<usize>().map_err(|_| Errno::EINVAL.into())
}

fn main() -> io::Result<()> {
    let mut state = RmtState::new();
    
    // TODO: Parse command line arguments
    
    while let Some(cmd) = state.read_line()? {
        if state.handle_command(&cmd)? {
            break;
        }
    }
    
    state.close_device()?;
    Ok(())
}