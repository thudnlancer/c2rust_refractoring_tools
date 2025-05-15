use std::{
    ffi::{CStr, CString},
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write, Stdin, Stdout},
    os::unix::io::{AsRawFd, FromRawFd, RawFd},
    path::Path,
    ptr,
    slice,
    str::FromStr,
    sync::atomic::{AtomicI32, Ordering},
};

const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

static DBG_LEV: AtomicI32 = AtomicI32::new(0);
static mut DBG_OUT: Option<File> = None;

macro_rules! debug {
    ($lev:expr, $msg:expr) => {
        unsafe {
            if DBG_OUT.is_some() && $lev <= DBG_LEV.load(Ordering::Relaxed) {
                write!(DBG_OUT.as_mut().unwrap(), "{}", $msg).unwrap();
            }
        }
    };
    ($lev:expr, $fmt:expr, $x:expr) => {
        unsafe {
            if DBG_OUT.is_some() && $lev <= DBG_LEV.load(Ordering::Relaxed) {
                write!(DBG_OUT.as_mut().unwrap(), $fmt, $x).unwrap();
            }
        }
    };
    ($lev:expr, $fmt:expr, $x1:expr, $x2:expr) => {
        unsafe {
            if DBG_OUT.is_some() && $lev <= DBG_LEV.load(Ordering::Relaxed) {
                write!(DBG_OUT.as_mut().unwrap(), $fmt, $x1, $x2).unwrap();
            }
        }
    };
}

fn trimnl(str: &mut String) {
    if str.ends_with('\n') {
        str.pop();
    }
}

struct Rmt {
    stdin: Stdin,
    stdout: Stdout,
    input_buf: String,
    record_buf: Vec<u8>,
    device_fd: Option<File>,
}

impl Rmt {
    fn new() -> Self {
        Rmt {
            stdin: io::stdin(),
            stdout: io::stdout(),
            input_buf: String::new(),
            record_buf: Vec::new(),
            device_fd: None,
        }
    }

    fn read(&mut self) -> io::Result<Option<&str>> {
        self.input_buf.clear();
        let bytes_read = self.stdin.lock().read_line(&mut self.input_buf)?;
        if bytes_read > 0 {
            debug!(10, "C: {}", self.input_buf);
            trimnl(&mut self.input_buf);
            Ok(Some(&self.input_buf))
        } else {
            debug!(10, "reached EOF");
            Ok(None)
        }
    }

    fn write(&mut self, fmt: &str, args: std::fmt::Arguments) -> io::Result<()> {
        self.stdout.write_fmt(args)?;
        self.stdout.flush()?;
        debug!(10, "S: {}", fmt);
        Ok(())
    }

    fn reply(&mut self, code: u64) -> io::Result<()> {
        self.write("A{}\n", format_args!("{}", code))
    }

    fn error_message(&mut self, code: i32, msg: &str) -> io::Result<()> {
        debug!(10, "S: E{}\n", code);
        debug!(10, "S: {}\n", msg);
        debug!(1, "error: {}\n", msg);
        writeln!(self.stdout, "E{}", code)?;
        writeln!(self.stdout, "{}", msg)?;
        self.stdout.flush()
    }

    fn error(&mut self, code: i32) -> io::Result<()> {
        let msg = io::Error::from_raw_os_error(code).to_string();
        self.error_message(code, &msg)
    }

    fn prepare_record_buffer(&mut self, size: usize) {
        if size > self.record_buf.len() {
            self.record_buf.resize(size, 0);
        }
    }

    fn open_device(&mut self, device: &str) -> io::Result<()> {
        let flag_str = match self.read()? {
            Some(s) => s,
            None => {
                debug!(1, "unexpected EOF");
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected EOF"));
            }
        };

        let flag = match decode_open_flag(flag_str) {
            Ok(f) => f,
            Err(e) => {
                self.error_message(libc::EINVAL, &e.to_string())?;
                return Ok(());
            }
        };

        if let Some(fd) = self.device_fd.take() {
            drop(fd);
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(flag)
            .open(device)?;

        self.device_fd = Some(file);
        self.reply(0)
    }

    fn close_device(&mut self) -> io::Result<()> {
        if let Some(fd) = self.device_fd.take() {
            fd.sync_all()?;
        }
        self.reply(0)
    }

    fn lseek_device(&mut self, whence_str: &str) -> io::Result<()> {
        let whence = match decode_seek_whence(whence_str) {
            Ok(w) => w,
            Err(e) => {
                self.error_message(libc::EINVAL, &e.to_string())?;
                return Ok(());
            }
        };

        let offset_str = match self.read()? {
            Some(s) => s,
            None => {
                debug!(1, "unexpected EOF");
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected EOF"));
            }
        };

        let offset = match offset_str.parse::<i64>() {
            Ok(o) => o,
            Err(_) => {
                self.error_message(libc::EINVAL, "Invalid seek offset")?;
                return Ok(());
            }
        };

        let fd = match &self.device_fd {
            Some(f) => f.as_raw_fd(),
            None => {
                self.error_message(libc::EBADF, "No device open")?;
                return Ok(());
            }
        };

        let new_offset = unsafe { libc::lseek(fd, offset, whence) };
        if new_offset < 0 {
            self.error(io::Error::last_os_error().raw_os_error().unwrap())?;
        } else {
            self.reply(new_offset as u64)?;
        }
        Ok(())
    }

    fn read_device(&mut self, count_str: &str) -> io::Result<()> {
        let count = match count_str.parse::<usize>() {
            Ok(c) => c,
            Err(_) => {
                self.error_message(libc::EINVAL, "Invalid byte count")?;
                return Ok(());
            }
        };

        let fd = match &self.device_fd {
            Some(f) => f,
            None => {
                self.error_message(libc::EBADF, "No device open")?;
                return Ok(());
            }
        };

        self.prepare_record_buffer(count);
        let bytes_read = fd.read(&mut self.record_buf[..count])?;
        self.reply(bytes_read as u64)?;
        self.stdout.write_all(&self.record_buf[..bytes_read])?;
        self.stdout.flush()
    }

    fn write_device(&mut self, count_str: &str) -> io::Result<()> {
        let count = match count_str.parse::<usize>() {
            Ok(c) => c,
            Err(_) => {
                self.error_message(libc::EINVAL, "Invalid byte count")?;
                return Ok(());
            }
        };

        let fd = match &mut self.device_fd {
            Some(f) => f,
            None => {
                self.error_message(libc::EBADF, "No device open")?;
                return Ok(());
            }
        };

        self.prepare_record_buffer(count);
        self.stdin.lock().read_exact(&mut self.record_buf[..count])?;
        let bytes_written = fd.write(&self.record_buf[..count])?;
        if bytes_written != count {
            self.error_message(libc::EIO, "Short write")?;
        } else {
            self.reply(bytes_written as u64)?;
        }
        Ok(())
    }
}

fn decode_open_flag(s: &str) -> Result<i32, io::Error> {
    // Simplified implementation - actual implementation would parse flags
    Ok(libc::O_RDWR)
}

fn decode_seek_whence(s: &str) -> Result<i32, io::Error> {
    match s {
        "0" | "SET" | "SEEK_SET" => Ok(libc::SEEK_SET),
        "1" | "CUR" | "SEEK_CUR" => Ok(libc::SEEK_CUR),
        "2" | "END" | "SEEK_END" => Ok(libc::SEEK_END),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid seek direction",
        )),
    }
}

fn main() -> io::Result<()> {
    let mut rmt = Rmt::new();
    let mut stop = false;

    while !stop {
        if let Some(buf) = rmt.read()? {
            match buf.chars().next() {
                Some('C') => {
                    rmt.close_device()?;
                    stop = true;
                }
                Some('L') => rmt.lseek_device(&buf[1..])?,
                Some('O') => rmt.open_device(&buf[1..])?,
                Some('R') => rmt.read_device(&buf[1..])?,
                Some('W') => rmt.write_device(&buf[1..])?,
                _ => {
                    debug!(1, "garbage input {}\n", buf);
                    rmt.error_message(libc::EINVAL, "Garbage command")?;
                    return Ok(());
                }
            }
        } else {
            break;
        }
    }

    Ok(())
}