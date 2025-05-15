use std::{
    ffi::{CStr, CString},
    fs::{File, OpenOptions},
    io::{self, Read, Write, BufRead, BufReader, Stdin, Stdout, Stderr},
    os::unix::io::{AsRawFd, RawFd},
    path::Path,
    process,
    str::FromStr,
    sync::atomic::{AtomicBool, Ordering},
    time::{SystemTime, UNIX_EPOCH},
    mem,
    fmt,
    error::Error,
    num::ParseIntError,
    convert::TryFrom,
    ffi::OsStr,
    os::unix::ffi::OsStrExt,
    ptr,
    slice,
    sync::Mutex,
    thread,
};

use libc::{self, c_int, c_void, size_t, off_t, SEEK_SET, SEEK_CUR, SEEK_END, O_RDONLY, O_WRONLY, O_RDWR, O_CREAT, O_TRUNC, O_APPEND, O_EXCL, O_NONBLOCK, O_SYNC, O_DSYNC, O_RSYNC, O_LARGEFILE, O_NOCTTY};
use nix::{
    errno::Errno,
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::{close, lseek, read, write},
};
use lazy_static::lazy_static;

const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;
const MODE_RW: Mode = Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IWGRP | Mode::S_IROTH;

lazy_static! {
    static ref DBGLEV: Mutex<i32> = Mutex::new(0);
    static ref DBGOUT: Mutex<Option<File>> = Mutex::new(None);
    static ref INPUT_BUF: Mutex<Vec<u8>> = Mutex::new(Vec::new());
    static ref RECORD_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
    static ref DEVICE_FD: Mutex<Option<RawFd>> = Mutex::new(None);
}

macro_rules! debug {
    ($level:expr, $($arg:tt)*) => {
        {
            let dbglev = DBGLEV.lock().unwrap();
            let mut dbgout = DBGOUT.lock().unwrap();
            if *dbglev >= $level {
                if let Some(file) = dbgout.as_mut() {
                    writeln!(file, $($arg)*).unwrap();
                }
            }
        }
    };
}

fn trimnl(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
    }
}

fn rmt_read() -> Option<String> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();
    match stdin.read_line(&mut buf) {
        Ok(0) => {
            debug!(10, "reached EOF");
            None
        }
        Ok(_) => {
            debug!(10, "C: {}", buf);
            trimnl(&mut buf);
            Some(buf)
        }
        Err(e) => {
            debug!(1, "read error: {}", e);
            None
        }
    }
}

fn rmt_write(fmt: &str, args: std::fmt::Arguments) {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    write!(stdout, "{}", args).unwrap();
    stdout.flush().unwrap();
    debug!(10, "S: {}", args);
}

fn rmt_reply(code: u64) {
    rmt_write!("A{}\n", code);
}

fn rmt_error_message(code: i32, msg: &str) {
    debug!(10, "S: E{}\n", code);
    debug!(10, "S: {}\n", msg);
    debug!(1, "error: {}\n", msg);
    rmt_write!("E{}\n{}\n", code, msg);
}

fn rmt_error(code: i32) {
    rmt_error_message(code, &std::io::Error::from_raw_os_error(code).to_string());
}

fn prepare_record_buffer(size: usize) {
    let mut buf = RECORD_BUFFER.lock().unwrap();
    if buf.len() < size {
        buf.resize(size, 0);
    }
}

struct RmtKw {
    name: &'static str,
    len: usize,
    value: i32,
}

impl RmtKw {
    const fn new(name: &'static str, value: i32) -> Self {
        Self {
            name,
            len: name.len(),
            value,
        }
    }
}

fn xlat_kw(s: &str, pfx: Option<&str>, kw: &[RmtKw], valp: &mut i32, endp: &mut &str) -> bool {
    let mut s = s;
    if let Some(pfx) = pfx {
        if s.starts_with(pfx) {
            s = &s[pfx.len()..];
        }
    }

    for k in kw {
        if s.len() >= k.len && s.starts_with(k.name) {
            let next_char = s.chars().nth(k.len);
            if next_char.map_or(true, |c| !c.is_alphanumeric()) {
                *valp = k.value;
                *endp = &s[k.len..];
                return true;
            }
        }
    }
    false
}

fn skip_ws(s: &str) -> &str {
    s.trim_start()
}

const OPEN_FLAG_KW: &[RmtKw] = &[
    RmtKw::new("APPEND", O_APPEND),
    RmtKw::new("CREAT", O_CREAT),
    RmtKw::new("DSYNC", O_DSYNC),
    RmtKw::new("EXCL", O_EXCL),
    RmtKw::new("LARGEFILE", O_LARGEFILE),
    RmtKw::new("NOCTTY", O_NOCTTY),
    RmtKw::new("NONBLOCK", O_NONBLOCK),
    RmtKw::new("RDONLY", O_RDONLY),
    RmtKw::new("RDWR", O_RDWR),
    RmtKw::new("RSYNC", O_RSYNC),
    RmtKw::new("SYNC", O_SYNC),
    RmtKw::new("TRUNC", O_TRUNC),
    RmtKw::new("WRONLY", O_WRONLY),
];

fn decode_open_flag(mstr: &str, pmode: &mut i32) -> Result<(), String> {
    let mstr = skip_ws(mstr);
    let mut numeric_mode = 0;
    let mut mode = 0;
    let mut p = mstr;

    if let Some(c) = mstr.chars().next() {
        if c.is_digit(10) {
            numeric_mode = i32::from_str(mstr).map_err(|e| e.to_string())?;
            p = skip_ws(&mstr[numeric_mode.to_string().len()..]);
        }
    }

    if !p.is_empty() {
        let mut mstr = p;
        while !mstr.is_empty() {
            mstr = skip_ws(mstr);
            if mstr.is_empty() {
                break;
            }

            let v = if let Some(c) = mstr.chars().next() {
                if c.is_digit(10) {
                    let num = i32::from_str(mstr).map_err(|e| e.to_string())?;
                    p = skip_ws(&mstr[num.to_string().len()..]);
                    num
                } else {
                    let mut val = 0;
                    let mut end = "";
                    if !xlat_kw(mstr, Some("O_"), OPEN_FLAG_KW, &mut val, &mut end) {
                        return Err("invalid open mode".to_string());
                    }
                    p = end;
                    val
                }
            } else {
                break;
            };

            mode |= v;

            p = skip_ws(p);
            if p.is_empty() {
                break;
            } else if p.starts_with('|') {
                mstr = &p[1..];
            } else {
                return Err("invalid open mode".to_string());
            }
        }
    } else {
        mode = numeric_mode;
    }

    *pmode = mode;
    Ok(())
}

fn open_device(device: &str) {
    let flag_str = match rmt_read() {
        Some(s) => s,
        None => {
            debug!(1, "unexpected EOF");
            process::exit(EXIT_FAILURE);
        }
    };

    let mut flag = 0;
    if let Err(e) = decode_open_flag(&flag_str, &mut flag) {
        rmt_error_message(libc::EINVAL, &e);
        return;
    }

    let mut device_fd = DEVICE_FD.lock().unwrap();
    if let Some(fd) = device_fd.take() {
        close(fd).unwrap();
    }

    match open(Path::new(device), OFlag::from_bits_truncate(flag), Mode::from_bits_truncate(MODE_RW.bits())) {
        Ok(fd) => {
            *device_fd = Some(fd);
            rmt_reply(0);
        }
        Err(e) => {
            rmt_error(e as i32);
        }
    }
}

fn close_device() {
    let mut device_fd = DEVICE_FD.lock().unwrap();
    if let Some(fd) = device_fd.take() {
        match close(fd) {
            Ok(_) => rmt_reply(0),
            Err(e) => rmt_error(e as i32),
        }
    } else {
        rmt_reply(0);
    }
}

const SEEK_WHENCE_KW: &[RmtKw] = &[
    RmtKw::new("SET", SEEK_SET),
    RmtKw::new("CUR", SEEK_CUR),
    RmtKw::new("END", SEEK_END),
];

fn lseek_device(str: &str) {
    let whence = if str.len() == 1 {
        match str.chars().next().unwrap() {
            '0' => SEEK_SET,
            '1' => SEEK_CUR,
            '2' => SEEK_END,
            _ => {
                rmt_error_message(libc::EINVAL, "Seek direction out of range");
                return;
            }
        }
    } else {
        let mut val = 0;
        let mut end = "";
        if !xlat_kw(str, Some("SEEK_"), SEEK_WHENCE_KW, &mut val, &mut end) {
            rmt_error_message(libc::EINVAL, "Invalid seek direction");
            return;
        }
        val
    };

    let offset_str = match rmt_read() {
        Some(s) => s,
        None => {
            rmt_error_message(libc::EINVAL, "Invalid seek offset");
            return;
        }
    };

    let offset = match offset_str.parse::<i64>() {
        Ok(n) => n,
        Err(_) => {
            rmt_error_message(libc::EINVAL, "Invalid seek offset");
            return;
        }
    };

    let device_fd = DEVICE_FD.lock().unwrap();
    if let Some(fd) = *device_fd {
        match lseek(fd, offset, whence) {
            Ok(off) => rmt_reply(off as u64),
            Err(e) => rmt_error(e as i32),
        }
    } else {
        rmt_error_message(libc::EBADF, "No device open");
    }
}

fn read_device(str: &str) {
    let size = match str.parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            rmt_error_message(libc::EINVAL, "Invalid byte count");
            return;
        }
    };

    prepare_record_buffer(size);
    let mut buf = RECORD_BUFFER.lock().unwrap();

    let device_fd = DEVICE_FD.lock().unwrap();
    if let Some(fd) = *device_fd {
        match read(fd, &mut buf[..size]) {
            Ok(status) => {
                rmt_reply(status as u64);
                io::stdout().write_all(&buf[..status]).unwrap();
            }
            Err(e) => rmt_error(e as i32),
        }
    } else {
        rmt_error_message(libc::EBADF, "No device open");
    }
}

fn write_device(str: &str) {
    let size = match str.parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            rmt_error_message(libc::EINVAL, "Invalid byte count");
            return;
        }
    };

    prepare_record_buffer(size);
    let mut buf = RECORD_BUFFER.lock().unwrap();

    if io::stdin().read_exact(&mut buf[..size]).is_err() {
        rmt_error_message(libc::EIO, "Premature eof");
        return;
    }

    let device_fd = DEVICE_FD.lock().unwrap();
    if let Some(fd) = *device_fd {
        match write(fd, &buf[..size]) {
            Ok(status) => {
                if status != size {
                    rmt_error(libc::EIO);
                } else {
                    rmt_reply(status as u64);
                }
            }
            Err(e) => rmt_error(e as i32),
        }
    } else {
        rmt_error_message(libc::EBADF, "No device open");
    }
}

fn iocop_device(str: &str) {
    let opcode = match str.parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            rmt_error_message(libc::EINVAL, "Invalid operation code");
            return;
        }
    };

    let count_str = match rmt_read() {
        Some(s) => s,
        None => {
            rmt_error_message(libc::EINVAL, "Invalid byte count");
            return;
        }
    };

    let count = match count_str.parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            rmt_error_message(libc::EINVAL, "Byte count out of range");
            return;
        }
    };

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        use libc::{mtop, MTIOCTOP};

        let device_fd = DEVICE_FD.lock().unwrap();
        if let Some(fd) = *device_fd {
            let mtop = mtop {
                mt_op: opcode,
                mt_count: count,
            };

            unsafe {
                if libc::ioctl(fd, MTIOCTOP, &mtop) < 0 {
                    rmt_error(Errno::last().into());
                } else {
                    rmt_reply(0);
                }
            }
        } else {
            rmt_error_message(libc::EBADF, "No device open");
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        rmt_error_message(libc::ENOSYS, "Operation not supported");
    }
}

fn status_device(str: &str) {
    if !str.is_empty() {
        rmt_error_message(libc::EINVAL, "Unexpected arguments");
        return;
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        use libc::{mtget, MTIOCGET};

        let device_fd = DEVICE_FD.lock().unwrap();
        if let Some(fd) = *device_fd {
            let mut mtget = mtget::default();

            unsafe {
                if libc::ioctl(fd, MTIOCGET, &mut mtget) < 0 {
                    rmt_error(Errno::last().into());
                } else {
                    rmt_reply(mem::size_of::<mtget>() as u64);
                    io::stdout()
                        .write_all(slice::from_raw_parts(
                            &mtget as *const _ as *const u8,
                            mem::size_of::<mtget>(),
                        ))
                        .unwrap();
                }
            }
        } else {
            rmt_error_message(libc::EBADF, "No device open");
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        rmt_error_message(libc::ENOSYS, "Operation not supported");
    }
}

fn main() {
    let mut stop = false;

    while !stop {
        match rmt_read() {
            Some(buf) => {
                match buf.chars().next() {
                    Some('C') => {
                        close_device();
                        stop = true;
                    }
                    Some('I') => iocop_device(&buf[1..]),
                    Some('L') => lseek_device(&buf[1..]),
                    Some('O') => open_device(&buf[1..]),
                    Some('R') => read_device(&buf[1..]),
                    Some('S') => status_device(&buf[1..]),
                    Some('W') => write_device(&buf[1..]),
                    _ => {
                        debug!(1, "garbage input {}", buf);
                        rmt_error_message(libc::EINVAL, "Garbage command");
                        process::exit(EXIT_FAILURE);
                    }
                }
            }
            None => break,
        }
    }

    close_device();
}