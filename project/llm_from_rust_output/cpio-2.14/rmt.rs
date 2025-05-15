use std::ffi::{CString, CStr};
use std::os::unix::io::{RawFd, AsRawFd};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::Path;
use std::ptr;
use std::mem;
use std::process;
use std::env;
use nix::errno::Errno;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, lseek, Whence};
use libc::{c_int, c_char, c_void, c_ulong, c_long, size_t, off_t};

const DEBUG_FILE_OPTION: u32 = 256;

struct RmtKw {
    name: &'static str,
    value: i32,
}

static OPEN_FLAG_KW: &[RmtKw] = &[
    RmtKw { name: "APPEND", value: 0o2000 },
    RmtKw { name: "CREAT", value: 0o100 },
    RmtKw { name: "DSYNC", value: 0o10000 },
    RmtKw { name: "EXCL", value: 0o200 },
    RmtKw { name: "LARGEFILE", value: 0 },
    RmtKw { name: "NOCTTY", value: 0o400 },
    RmtKw { name: "NONBLOCK", value: 0o4000 },
    RmtKw { name: "RDONLY", value: 0 },
    RmtKw { name: "RDWR", value: 0o2 },
    RmtKw { name: "RSYNC", value: 0o4010000 },
    RmtKw { name: "SYNC", value: 0o4010000 },
    RmtKw { name: "TRUNC", value: 0o1000 },
    RmtKw { name: "WRONLY", value: 0o1 },
];

static SEEK_WHENCE_KW: &[RmtKw] = &[
    RmtKw { name: "SET", value: 0 },
    RmtKw { name: "CUR", value: 1 },
    RmtKw { name: "END", value: 2 },
];

struct RmtState {
    dbglev: i32,
    dbgout: Option<File>,
    device_fd: Option<RawFd>,
    input_buf: Vec<u8>,
    record_buf: Vec<u8>,
}

impl RmtState {
    fn new() -> Self {
        RmtState {
            dbglev: 0,
            dbgout: None,
            device_fd: None,
            input_buf: Vec::new(),
            record_buf: Vec::new(),
        }
    }

    fn trimnl(&mut self, s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
        }
    }

    fn read_line(&mut self) -> io::Result<String> {
        self.input_buf.clear();
        io::stdin().read_until(b'\n', &mut self.input_buf)?;
        let mut s = String::from_utf8_lossy(&self.input_buf).into_owned();
        self.trimnl(&mut s);
        Ok(s)
    }

    fn write_response(&mut self, s: &str) -> io::Result<()> {
        print!("{}", s);
        io::stdout().flush()?;
        if let Some(ref mut f) = self.dbgout {
            write!(f, "S: {}", s)?;
            f.flush()?;
        }
        Ok(())
    }

    fn reply(&mut self, code: u64) -> io::Result<()> {
        self.write_response(&format!("A{}\n", code))
    }

    fn error_message(&mut self, code: i32, msg: &str) -> io::Result<()> {
        if let Some(ref mut f) = self.dbgout {
            writeln!(f, "S: E{}", code)?;
            writeln!(f, "S: {}", msg)?;
            if self.dbglev >= 1 {
                writeln!(f, "error: {}", msg)?;
            }
        }
        println!("E{}\n{}", code, msg);
        io::stdout().flush()?;
        Ok(())
    }

    fn error(&mut self, err: Errno) -> io::Result<()> {
        self.error_message(err as i32, &err.desc())
    }

    fn prepare_record_buffer(&mut self, size: usize) {
        if size > self.record_buf.len() {
            self.record_buf.resize(size, 0);
        }
    }

    fn skip_ws(s: &str) -> &str {
        s.trim_start()
    }

    fn xlat_kw(s: &str, kw_list: &[RmtKw]) -> Option<(i32, &str)> {
        for kw in kw_list {
            if let Some(rest) = s.strip_prefix(kw.name) {
                if rest.is_empty() || !rest.chars().next().unwrap().is_alphanumeric() {
                    return Some((kw.value, rest));
                }
            }
        }
        None
    }

    fn decode_open_flag(&mut self, mstr: &str) -> Result<OFlag, String> {
        let mut mode = OFlag::empty();
        let mut parts = mstr.split('|').map(str::trim);

        for part in parts {
            if let Some((val, _)) = Self::xlat_kw(part, OPEN_FLAG_KW) {
                mode |= OFlag::from_bits_truncate(val as i32);
            } else if let Ok(num) = part.parse::<i32>() {
                mode |= OFlag::from_bits_truncate(num);
            } else {
                return Err("invalid open mode".to_string());
            }
        }

        Ok(mode)
    }

    fn open_device(&mut self, device: &str) -> io::Result<()> {
        let flag_str = self.read_line()?;
        let flags = match self.decode_open_flag(&flag_str) {
            Ok(f) => f,
            Err(e) => {
                self.error_message(22, &e)?;
                return Ok(());
            }
        };

        if let Some(fd) = self.device_fd {
            close(fd).map_err(|e| io::Error::from_raw_os_error(e as i32))?;
        }

        let mode = Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IROTH;
        match open(device, flags, mode) {
            Ok(fd) => {
                self.device_fd = Some(fd);
                self.reply(0)
            }
            Err(e) => {
                self.error(e)
            }
        }
    }

    fn close_device(&mut self) -> io::Result<()> {
        if let Some(fd) = self.device_fd.take() {
            close(fd).map_err(|e| io::Error::from_raw_os_error(e as i32))?;
        }
        self.reply(0)
    }

    fn lseek_device(&mut self, whence_str: &str) -> io::Result<()> {
        let whence = match whence_str {
            "0" => Whence::SeekSet,
            "1" => Whence::SeekCur,
            "2" => Whence::SeekEnd,
            _ => {
                if let Some((val, _)) = Self::xlat_kw(whence_str, SEEK_WHENCE_KW) {
                    match val {
                        0 => Whence::SeekSet,
                        1 => Whence::SeekCur,
                        2 => Whence::SeekEnd,
                        _ => {
                            self.error_message(22, "Invalid seek direction")?;
                            return Ok(());
                        }
                    }
                } else {
                    self.error_message(22, "Invalid seek direction")?;
                    return Ok(());
                }
            }
        };

        let offset_str = self.read_line()?;
        let offset: off_t = match offset_str.parse() {
            Ok(n) => n,
            Err(_) => {
                self.error_message(22, "Invalid seek offset")?;
                return Ok(());
            }
        };

        if let Some(fd) = self.device_fd {
            match lseek(fd, offset, whence) {
                Ok(pos) => self.reply(pos as u64),
                Err(e) => self.error(e),
            }
        } else {
            self.error_message(22, "No device open")?;
            Ok(())
        }
    }

    fn read_device(&mut self, size_str: &str) -> io::Result<()> {
        let size: usize = match size_str.parse() {
            Ok(n) => n,
            Err(_) => {
                self.error_message(22, "Invalid byte count")?;
                return Ok(());
            }
        };

        if let Some(fd) = self.device_fd {
            self.prepare_record_buffer(size);
            let n = nix::unistd::read(fd, &mut self.record_buf[..size])?;
            self.reply(n as u64)?;
            io::stdout().write_all(&self.record_buf[..n])?;
            io::stdout().flush()?;
            Ok(())
        } else {
            self.error_message(22, "No device open")?;
            Ok(())
        }
    }

    fn write_device(&mut self, size_str: &str) -> io::Result<()> {
        let size: usize = match size_str.parse() {
            Ok(n) => n,
            Err(_) => {
                self.error_message(22, "Invalid byte count")?;
                return Ok(());
            }
        };

        if let Some(fd) = self.device_fd {
            self.prepare_record_buffer(size);
            io::stdin().read_exact(&mut self.record_buf[..size])?;
            let n = nix::unistd::write(fd, &self.record_buf[..size])?;
            if n != size {
                self.error_message(22, "Short write")?;
            } else {
                self.reply(n as u64)?;
            }
            Ok(())
        } else {
            self.error_message(22, "No device open")?;
            Ok(())
        }
    }

    fn process_command(&mut self, cmd: &str) -> io::Result<bool> {
        if cmd.is_empty() {
            return Ok(false);
        }

        let (op, arg) = cmd.split_at(1);
        match op {
            "C" => {
                self.close_device()?;
                Ok(true)
            }
            "I" => {
                // iocop not implemented
                Ok(false)
            }
            "L" => {
                self.lseek_device(arg)?;
                Ok(false)
            }
            "O" => {
                self.open_device(arg)?;
                Ok(false)
            }
            "R" => {
                self.read_device(arg)?;
                Ok(false)
            }
            "S" => {
                // status not implemented
                Ok(false)
            }
            "W" => {
                self.write_device(arg)?;
                Ok(false)
            }
            _ => {
                self.error_message(22, "Garbage command")?;
                Ok(false)
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut state = RmtState::new();
    let args: Vec<String> = env::args().collect();

    // Parse arguments (simplified)
    for arg in args.iter().skip(1) {
        if arg == "-d" || arg == "--debug" {
            state.dbglev = 1;
        } else if arg.starts_with("--debug-file=") {
            let path = arg.trim_start_matches("--debug-file=");
            state.dbgout = Some(File::create(path)?);
        } else {
            state.dbgout = Some(File::create(arg)?);
            state.dbglev = 1;
        }
    }

    let mut stop = false;
    while !stop {
        let line = state.read_line()?;
        stop = state.process_command(&line)?;
    }

    state.close_device()?;
    Ok(())
}