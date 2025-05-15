use std::{
    ffi::CStr,
    fs::{File, OpenOptions},
    io::{self, Write},
    os::unix::io::{AsRawFd, FromRawFd, RawFd},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
    sync::atomic::{AtomicI32, Ordering},
    fmt::{self, Write as FmtWrite},
};

const LOG_EMERG: i32 = 0;
const LOG_ALERT: i32 = 1;
const LOG_CRIT: i32 = 2;
const LOG_ERR: i32 = 3;
const LOG_WARN: i32 = 4;
const LOG_NOTICE: i32 = 5;
const LOG_INFO: i32 = 6;
const LOG_DEBUG: i32 = 7;
const LOG_VERB: i32 = 8;
const LOG_VVERB: i32 = 9;
const LOG_VVVERB: i32 = 10;
const LOG_PVERB: i32 = 11;

const LOG_MAX_LEN: usize = 256;

struct Logger {
    name: Option<String>,
    level: AtomicI32,
    fd: Option<File>,
    nerror: AtomicI32,
}

impl Logger {
    fn new() -> Self {
        Logger {
            name: None,
            level: AtomicI32::new(LOG_NOTICE),
            fd: None,
            nerror: AtomicI32::new(0),
        }
    }

    fn init(&mut self, level: i32, filename: Option<&str>) -> io::Result<()> {
        let level = level.clamp(LOG_EMERG, LOG_PVERB);
        self.level.store(level, Ordering::Relaxed);

        if let Some(name) = filename {
            self.name = Some(name.to_string());
            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(name)?;
            self.fd = Some(file);
        } else {
            self.fd = Some(unsafe { File::from_raw_fd(2) });
        }

        Ok(())
    }

    fn deinit(&mut self) {
        if let Some(fd) = self.fd.take() {
            if fd.as_raw_fd() != 2 {
                let _ = fd.sync_all();
            }
        }
    }

    fn reopen(&mut self) -> io::Result<()> {
        if let Some(name) = &self.name {
            if let Some(fd) = self.fd.take() {
                if fd.as_raw_fd() != 2 {
                    let _ = fd.sync_all();
                }
            }
            self.fd = Some(OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(name)?);
        }
        Ok(())
    }

    fn level_up(&self) {
        let mut level = self.level.load(Ordering::Relaxed);
        if level < LOG_PVERB {
            level += 1;
            self.level.store(level, Ordering::Relaxed);
            log_safe(&format!("up log level to {}", level));
        }
    }

    fn level_down(&self) {
        let mut level = self.level.load(Ordering::Relaxed);
        if level > LOG_EMERG {
            level -= 1;
            self.level.store(level, Ordering::Relaxed);
            log_safe(&format!("down log level to {}", level));
        }
    }

    fn set_level(&self, level: i32) {
        let level = level.clamp(LOG_EMERG, LOG_PVERB);
        self.level.store(level, Ordering::Relaxed);
        loga(&format!("set log level to {}", level));
    }

    fn loggable(&self, level: i32) -> bool {
        level <= self.level.load(Ordering::Relaxed)
    }

    fn write_log(&self, buf: &[u8]) -> io::Result<()> {
        if let Some(fd) = &self.fd {
            fd.write_all(buf)?;
        }
        Ok(())
    }

    fn write_stderr(&self, buf: &[u8]) -> io::Result<()> {
        io::stderr().write_all(buf)
    }
}

lazy_static! {
    static ref LOGGER: std::sync::Mutex<Logger> = std::sync::Mutex::new(Logger::new());
}

pub fn log_init(level: i32, filename: Option<&str>) -> io::Result<()> {
    LOGGER.lock().unwrap().init(level, filename)
}

pub fn log_deinit() {
    LOGGER.lock().unwrap().deinit();
}

pub fn log_reopen() -> io::Result<()> {
    LOGGER.lock().unwrap().reopen()
}

pub fn log_level_up() {
    LOGGER.lock().unwrap().level_up();
}

pub fn log_level_down() {
    LOGGER.lock().unwrap().level_down();
}

pub fn log_level_set(level: i32) {
    LOGGER.lock().unwrap().set_level(level);
}

pub fn log_loggable(level: i32) -> bool {
    LOGGER.lock().unwrap().loggable(level)
}

pub fn log_stderr(msg: &str) {
    let mut buf = Vec::with_capacity(LOG_MAX_LEN);
    writeln!(buf, "{}", msg).unwrap();
    let _ = LOGGER.lock().unwrap().write_stderr(&buf);
}

pub fn log_safe(msg: &str) {
    let mut buf = Vec::with_capacity(LOG_MAX_LEN);
    writeln!(buf, "[.......................] {}", msg).unwrap();
    let _ = LOGGER.lock().unwrap().write_log(&buf);
}

pub fn log_stderr_safe(msg: &str) {
    let mut buf = Vec::with_capacity(LOG_MAX_LEN);
    writeln!(buf, "[.......................] {}", msg).unwrap();
    let _ = LOGGER.lock().unwrap().write_stderr(&buf);
}

pub fn loga(msg: &str) {
    let mut buf = Vec::with_capacity(LOG_MAX_LEN);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    let tm = time::now();
    write!(
        buf,
        "[{}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}] {}",
        tm.tm_year + 1900,
        tm.tm_mon + 1,
        tm.tm_mday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec,
        now.subsec_millis(),
        msg
    ).unwrap();
    let _ = LOGGER.lock().unwrap().write_log(&buf);
}

pub fn log_error(msg: &str) {
    if log_loggable(LOG_ALERT) {
        loga(msg);
    }
}

pub fn log_warn(msg: &str) {
    if log_loggable(LOG_WARN) {
        loga(msg);
    }
}

pub fn log_panic(msg: &str) {
    if log_loggable(LOG_EMERG) {
        loga(msg);
        panic!("{}", msg);
    }
}

#[cfg(feature = "debug")]
pub fn log_debug(level: i32, msg: &str) {
    if log_loggable(level) {
        loga(msg);
    }
}

#[cfg(feature = "debug")]
pub fn log_hexdump(level: i32, data: &[u8]) {
    if log_loggable(level) {
        let mut buf = Vec::with_capacity(8 * LOG_MAX_LEN);
        let mut offset = 0;
        for chunk in data.chunks(16) {
            write!(buf, "{:08x}  ", offset).unwrap();
            for (i, &byte) in chunk.iter().enumerate() {
                write!(buf, "{:02x}{}", byte, if i == 7 { "  " } else { " " }).unwrap();
            }
            for _ in chunk.len()..16 {
                write!(buf, "   ").unwrap();
            }
            write!(buf, " |").unwrap();
            for &byte in chunk {
                write!(
                    buf,
                    "{}",
                    if byte.is_ascii_graphic() {
                        byte as char
                    } else {
                        '.'
                    }
                ).unwrap();
            }
            writeln!(buf, "|").unwrap();
            offset += 16;
        }
        let _ = LOGGER.lock().unwrap().write_log(&buf);
    }
}

#[cfg(not(feature = "debug"))]
pub fn log_debug(_level: i32, _msg: &str) {}

#[cfg(not(feature = "debug"))]
pub fn log_hexdump(_level: i32, _data: &[u8]) {}

pub fn log_stacktrace() {
    // Implementation would use backtrace crate
}