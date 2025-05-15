use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::Path;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use libc::{self, c_int, c_char, c_void, size_t, off_t};

static OUTPUT_SYNC: AtomicBool = AtomicBool::new(false);
static MAKELEVEL: AtomicU32 = AtomicU32::new(0);
static PROGRAM: &str = "make";

struct Output {
    out: Option<File>,
    err: Option<File>,
    sync: bool,
}

impl Output {
    fn new() -> Self {
        Self {
            out: None,
            err: None,
            sync: false,
        }
    }

    fn init(&mut self) {
        self.out = None;
        self.err = None;
        self.sync = OUTPUT_SYNC.load(Ordering::Relaxed);
    }

    fn close(&mut self) {
        if self.sync {
            self.dump();
        }
        self.out = None;
        self.err = None;
    }

    fn dump(&mut self) {
        let out_empty = self.out.as_ref().map_or(true, |f| f.metadata().map_or(true, |m| m.len() == 0));
        let err_empty = self.err.as_ref().map_or(true, |f| f.metadata().map_or(true, |m| m.len() == 0));

        if out_empty && err_empty {
            return;
        }

        if OUTPUT_SYNC.load(Ordering::Relaxed) {
            // Handle output synchronization
        }

        if let Some(out) = &mut self.out {
            pump_from_tmp(out, io::stdout());
            out.seek(SeekFrom::Start(0)).unwrap();
            out.set_len(0).unwrap();
        }

        if let Some(err) = &mut self.err {
            if Some(err) != self.out.as_ref() {
                pump_from_tmp(err, io::stderr());
                err.seek(SeekFrom::Start(0)).unwrap();
                err.set_len(0).unwrap();
            }
        }
    }
}

fn pump_from_tmp(from: &mut File, mut to: impl Write) {
    let mut buf = [0; 8192];
    from.seek(SeekFrom::Start(0)).unwrap();

    loop {
        let len = from.read(&mut buf).unwrap();
        if len == 0 {
            break;
        }
        to.write_all(&buf[..len]).unwrap();
        to.flush().unwrap();
    }
}

fn output_tmpfd() -> io::Result<File> {
    let temp_file = tempfile::tempfile()?;
    Ok(temp_file)
}

fn setup_tmpfile(output: &mut Output) {
    if OUTPUT_SYNC.load(Ordering::Relaxed) {
        if output.out.is_none() {
            if let Ok(fd) = output_tmpfd() {
                output.out = Some(fd);
            }
        }

        if output.err.is_none() && output.out.is_some() {
            output.err = output.out.clone();
        } else if output.err.is_none() {
            if let Ok(fd) = output_tmpfd() {
                output.err = Some(fd);
            }
        }
    }
}

fn log_working_directory(entering: bool) -> bool {
    // Implementation of directory logging
    true
}

fn outputs(is_err: bool, msg: &str) {
    let mut output = Output::new();
    output.init();

    if OUTPUT_SYNC.load(Ordering::Relaxed) {
        setup_tmpfile(&mut output);
    }

    if is_err {
        if let Some(err) = &mut output.err {
            writeln!(err, "{}", msg).unwrap();
        } else {
            eprintln!("{}", msg);
        }
    } else {
        if let Some(out) = &mut output.out {
            writeln!(out, "{}", msg).unwrap();
        } else {
            println!("{}", msg);
        }
    }
}

fn message(prefix: bool, fmt: &str, args: std::fmt::Arguments) {
    let msg = if prefix {
        if MAKELEVEL.load(Ordering::Relaxed) == 0 {
            format!("{}: {}", PROGRAM, fmt)
        } else {
            format!("{}[{}]: {}", PROGRAM, MAKELEVEL.load(Ordering::Relaxed), fmt)
        }
    } else {
        fmt.to_string()
    };

    outputs(false, &msg);
}

fn error(loc: Option<(&str, u64)>, fmt: &str, args: std::fmt::Arguments) {
    let msg = match loc {
        Some((file, line)) => format!("{}:{}: {}", file, line, fmt),
        None if MAKELEVEL.load(Ordering::Relaxed) == 0 => format!("{}: {}", PROGRAM, fmt),
        None => format!("{}[{}]: {}", PROGRAM, MAKELEVEL.load(Ordering::Relaxed), fmt),
    };

    outputs(true, &msg);
}

fn fatal(loc: Option<(&str, u64)>, fmt: &str, args: std::fmt::Arguments) -> ! {
    error(loc, fmt, args);
    std::process::exit(2);
}

fn perror_with_name(str: &str, name: &str) {
    let err = io::Error::last_os_error();
    error(None, &format!("{}{}: {}", str, name, err), std::fmt::Arguments::new());
}

fn pfatal_with_name(name: &str) -> ! {
    let err = io::Error::last_os_error();
    fatal(None, &format!("{}: {}", name, err), std::fmt::Arguments::new());
}

fn out_of_memory() -> ! {
    eprintln!("{}: *** virtual memory exhausted", PROGRAM);
    std::process::exit(2);
}