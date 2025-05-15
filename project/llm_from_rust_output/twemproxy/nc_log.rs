use std::{
    ffi::{CStr, CString},
    fs::{File, OpenOptions},
    io::{self, Write},
    os::unix::io::{AsRawFd, FromRawFd, IntoRawFd},
    ptr,
    time::{SystemTime, UNIX_EPOCH},
};

const LOG_LEVEL_MAX: i32 = 11;

struct Logger {
    name: Option<CString>,
    level: i32,
    file: Option<File>,
    nerror: i32,
}

impl Logger {
    fn new() -> Self {
        Logger {
            name: None,
            level: 0,
            file: None,
            nerror: 0,
        }
    }

    fn init(&mut self, level: i32, name: Option<&CStr>) -> io::Result<()> {
        self.level = level.clamp(0, LOG_LEVEL_MAX);
        self.name = name.map(|n| n.to_owned());

        let file = if let Some(name) = &self.name {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(name.to_str().unwrap())
                .map_err(|e| {
                    self.log_stderr(&format!(
                        "opening log file '{}' failed: {}",
                        name.to_str().unwrap(),
                        e
                    ));
                    e
                })?
        } else {
            unsafe { File::from_raw_fd(2) }
        };

        self.file = Some(file);
        Ok(())
    }

    fn deinit(&mut self) {
        if let Some(file) = self.file.take() {
            if file.as_raw_fd() != 2 {
                let _ = file.into_raw_fd(); // Close on drop
            }
        }
    }

    fn reopen(&mut self) -> io::Result<()> {
        if let Some(name) = &self.name {
            if let Some(file) = &mut self.file {
                if file.as_raw_fd() != 2 {
                    let new_file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(name.to_str().unwrap())?;
                    *file = new_file;
                }
            }
        }
        Ok(())
    }

    fn loggable(&self, level: i32) -> bool {
        level <= self.level
    }

    fn log(&mut self, file: &str, line: i32, panic: bool, msg: &str) {
        if self.file.is_none() {
            return;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        let secs = now.as_secs();
        let millis = now.subsec_millis();

        let timestamp = chrono::NaiveDateTime::from_timestamp(secs as i64, 0);
        let formatted_time = timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

        let log_msg = format!(
            "[{}] {}:{} {}\n",
            formatted_time, file, line, msg
        );

        if let Some(file) = &mut self.file {
            if let Err(e) = file.write_all(log_msg.as_bytes()) {
                self.nerror += 1;
                eprintln!("Failed to write log: {}", e);
            }
        }

        if panic {
            std::process::abort();
        }
    }

    fn log_stderr(&mut self, msg: &str) {
        eprintln!("{}", msg);
    }

    fn log_hexdump(&mut self, file: &str, line: i32, data: &[u8]) {
        if self.file.is_none() {
            return;
        }

        let mut output = String::new();
        for (i, chunk) in data.chunks(16).enumerate() {
            let offset = i * 16;
            let hex: Vec<String> = chunk
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect();
            let ascii: String = chunk
                .iter()
                .map(|&b| {
                    if b.is_ascii_graphic() || b == b' ' {
                        b as char
                    } else {
                        '.'
                    }
                })
                .collect();

            output.push_str(&format!(
                "{:08x}  {: <39}  |{: <16}|\n",
                offset,
                hex.chunks(2)
                    .map(|pair| pair.join(" "))
                    .collect::<Vec<_>>()
                    .join(" "),
                ascii
            ));
        }

        if let Some(file) = &mut self.file {
            if let Err(e) = file.write_all(output.as_bytes()) {
                self.nerror += 1;
                eprintln!("Failed to write hexdump: {}", e);
            }
        }
    }
}

static mut LOGGER: Logger = Logger::new();

pub fn log_init(level: i32, name: Option<&CStr>) -> io::Result<()> {
    unsafe { LOGGER.init(level, name) }
}

pub fn log_deinit() {
    unsafe { LOGGER.deinit() }
}

pub fn log_reopen() -> io::Result<()> {
    unsafe { LOGGER.reopen() }
}

pub fn log_level_up() {
    unsafe {
        if LOGGER.level < LOG_LEVEL_MAX {
            LOGGER.level += 1;
            LOGGER.log("", 0, false, &format!("up log level to {}", LOGGER.level));
        }
    }
}

pub fn log_level_down() {
    unsafe {
        if LOGGER.level > 0 {
            LOGGER.level -= 1;
            LOGGER.log("", 0, false, &format!("down log level to {}", LOGGER.level));
        }
    }
}

pub fn log_level_set(level: i32) {
    unsafe {
        LOGGER.level = level.clamp(0, LOG_LEVEL_MAX);
        LOGGER.log(
            "nc_log.c",
            105,
            false,
            &format!("set log level to {}", LOGGER.level),
        );
    }
}

pub fn log_loggable(level: i32) -> bool {
    unsafe { LOGGER.loggable(level) }
}

pub fn log(file: &str, line: i32, panic: bool, msg: &str) {
    unsafe { LOGGER.log(file, line, panic, msg) }
}

pub fn log_stderr(msg: &str) {
    unsafe { LOGGER.log_stderr(msg) }
}

pub fn log_hexdump(file: &str, line: i32, data: &[u8]) {
    unsafe { LOGGER.log_hexdump(file, line, data) }
}