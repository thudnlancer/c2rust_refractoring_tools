use std::{
    ffi::OsStr,
    fs::{File, OpenOptions},
    io::{self, Write, stderr, stdout},
    path::Path,
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
    sync::Mutex,
    time::SystemTime,
};

const DEFAULT_LOGFILE: &str = "wget-log";
const SAVED_LOG_LINES: usize = 24;
const STATIC_LENGTH: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LogOptions {
    Verbose,
    NotQuiet,
    NonVerbose,
    Always,
    Progress,
}

struct LogLine {
    static_line: [u8; STATIC_LENGTH + 1],
    malloced_line: Option<Vec<u8>>,
    content: Option<Vec<u8>>,
}

impl Default for LogLine {
    fn default() -> Self {
        Self {
            static_line: [0; STATIC_LENGTH + 1],
            malloced_line: None,
            content: None,
        }
    }
}

struct Logger {
    logfp: Option<File>,
    stdlogfp: Option<File>,
    filelogfp: Option<File>,
    logfile: Option<String>,
    warclogfp: Option<File>,
    inhibit_logging: AtomicBool,
    save_context_p: AtomicBool,
    flush_log_p: AtomicBool,
    needs_flushing: AtomicBool,
    log_lines: Mutex<[LogLine; SAVED_LOG_LINES]>,
    log_line_current: AtomicI32,
    trailing_line: AtomicBool,
    redirect_request_signal_name: Mutex<Option<String>>,
}

impl Logger {
    fn new() -> Self {
        Self {
            logfp: None,
            stdlogfp: None,
            filelogfp: None,
            logfile: None,
            warclogfp: None,
            inhibit_logging: AtomicBool::new(false),
            save_context_p: AtomicBool::new(false),
            flush_log_p: AtomicBool::new(true),
            needs_flushing: AtomicBool::new(false),
            log_lines: Mutex::new([LogLine::default(); SAVED_LOG_LINES]),
            log_line_current: AtomicI32::new(-1),
            trailing_line: AtomicBool::new(false),
            redirect_request_signal_name: Mutex::new(None),
        }
    }

    fn log_set_warc_log_fp(&mut self, fp: Option<File>) {
        self.warclogfp = fp;
    }

    fn logputs(&self, o: LogOptions, s: &str) {
        if self.inhibit_logging.load(Ordering::Relaxed) {
            return;
        }

        self.check_redirect_output();

        let fp = if o == LogOptions::Progress {
            self.get_progress_fp()
        } else {
            self.get_log_fp()
        };

        if fp.is_none() {
            return;
        }

        let warcfp = self.get_warc_log_fp();

        match o {
            LogOptions::Progress if !opt.show_progress => return,
            LogOptions::NotQuiet if opt.quiet => return,
            LogOptions::NonVerbose if opt.verbose || opt.quiet => return,
            LogOptions::Verbose if !opt.verbose => return,
            _ => (),
        }

        if let Some(mut fp) = fp {
            let _ = write!(fp, "{}", s);
        }

        if let Some(mut warcfp) = warcfp {
            let _ = write!(warcfp, "{}", s);
        }

        if self.save_context_p.load(Ordering::Relaxed) {
            self.saved_append(s);
        }

        if self.flush_log_p.load(Ordering::Relaxed) {
            self.logflush();
        } else {
            self.needs_flushing.store(true, Ordering::Relaxed);
        }
    }

    fn logflush(&self) {
        if let Some(mut fp) = self.get_log_fp() {
            let _ = fp.flush();
        }

        if let Some(mut warcfp) = self.get_warc_log_fp() {
            let _ = warcfp.flush();
        }

        self.needs_flushing.store(false, Ordering::Relaxed);
    }

    fn log_set_flush(&self, flush: bool) {
        if flush == self.flush_log_p.load(Ordering::Relaxed) {
            return;
        }

        self.flush_log_p.store(flush, Ordering::Relaxed);

        if flush && self.needs_flushing.load(Ordering::Relaxed) {
            self.logflush();
        }
    }

    fn log_set_save_context(&self, savep: bool) -> bool {
        let old = self.save_context_p.load(Ordering::Relaxed);
        self.save_context_p.store(savep, Ordering::Relaxed);
        old
    }

    fn logprintf(&self, o: LogOptions, fmt: &str, args: std::fmt::Arguments) {
        if self.inhibit_logging.load(Ordering::Relaxed) {
            return;
        }

        self.check_redirect_output();

        match o {
            LogOptions::Progress if !opt.show_progress => return,
            LogOptions::NotQuiet if opt.quiet => return,
            LogOptions::NonVerbose if opt.verbose || opt.quiet => return,
            LogOptions::Verbose if !opt.verbose => return,
            _ => (),
        }

        let mut state = LogVPrintfState {
            bigmsg: None,
            expected_size: 0,
            allocated: 0,
        };

        loop {
            if self.log_vprintf_internal(&mut state, fmt, args) {
                break;
            }
        }
    }

    fn log_init(&mut self, file: Option<&str>, appendp: bool) -> io::Result<()> {
        if let Some(file) = file {
            if file == "-" {
                self.stdlogfp = Some(stdout());
                self.logfp = self.stdlogfp.clone();
            } else {
                let filelogfp = OpenOptions::new()
                    .write(true)
                    .append(appendp)
                    .create(true)
                    .open(file)?;
                self.filelogfp = Some(filelogfp);
                self.logfp = self.filelogfp.clone();
                self.logfile = Some(file.to_string());
            }
        } else {
            self.stdlogfp = Some(stderr());
            self.logfp = self.stdlogfp.clone();

            if atty::is(atty::Stream::Stderr) {
                self.save_context_p.store(true, Ordering::Relaxed);
            }
        }

        Ok(())
    }

    fn log_close(&mut self) {
        if let Some(fp) = self.logfp.take() {
            if !fp.as_raw_fd() == stderr().as_raw_fd() 
                && !fp.as_raw_fd() == stdout().as_raw_fd() 
            {
                if self.stdlogfp.as_ref() == Some(&fp) {
                    self.stdlogfp = None;
                }
                if self.filelogfp.as_ref() == Some(&fp) {
                    self.filelogfp = None;
                }
                let _ = fp.sync_all();
            }
        }

        self.inhibit_logging.store(true, Ordering::Relaxed);
        self.save_context_p.store(false, Ordering::Relaxed);

        let mut log_lines = self.log_lines.lock().unwrap();
        for line in log_lines.iter_mut() {
            *line = LogLine::default();
        }
        self.log_line_current.store(-1, Ordering::Relaxed);
        self.trailing_line.store(false, Ordering::Relaxed);
    }

    // ... (其他方法的实现)
}

struct LogVPrintfState {
    bigmsg: Option<Vec<u8>>,
    expected_size: usize,
    allocated: usize,
}

// 全局logger实例
lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

// 公共接口函数
pub fn log_set_warc_log_fp(fp: Option<File>) {
    LOGGER.lock().unwrap().log_set_warc_log_fp(fp);
}

pub fn logputs(o: LogOptions, s: &str) {
    LOGGER.lock().unwrap().logputs(o, s);
}

pub fn logflush() {
    LOGGER.lock().unwrap().logflush();
}

pub fn log_set_flush(flush: bool) {
    LOGGER.lock().unwrap().log_set_flush(flush);
}

pub fn log_set_save_context(savep: bool) -> bool {
    LOGGER.lock().unwrap().log_set_save_context(savep)
}

pub fn logprintf(o: LogOptions, fmt: &str, args: std::fmt::Arguments) {
    LOGGER.lock().unwrap().logprintf(o, fmt, args)
}

pub fn log_init(file: Option<&str>, appendp: bool) -> io::Result<()> {
    LOGGER.lock().unwrap().log_init(file, appendp)
}

pub fn log_close() {
    LOGGER.lock().unwrap().log_close()
}