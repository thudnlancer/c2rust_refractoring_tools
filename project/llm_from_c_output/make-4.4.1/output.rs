use std::fs::{File, OpenOptions};
use std::io::{self, Write, Seek, SeekFrom};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::ffi::CString;
use std::ptr;
use libc::{O_APPEND, fcntl, F_GETFL, F_SETFL};

const OUTPUT_NONE: RawFd = -1;

struct Output {
    out: RawFd,
    err: RawFd,
    syncout: bool,
}

impl Output {
    fn new() -> Self {
        Output {
            out: OUTPUT_NONE,
            err: OUTPUT_NONE,
            syncout: false,
        }
    }

    fn is_set(&self) -> bool {
        self.out >= 0 || self.err >= 0
    }
}

static OUTPUT_CONTEXT: Mutex<Option<Arc<Output>>> = Mutex::new(None);
static STDIO_TRACED: AtomicBool = AtomicBool::new(false);

fn output_set(new: Arc<Output>) {
    let mut ctx = OUTPUT_CONTEXT.lock().unwrap();
    *ctx = if new.syncout { Some(new) } else { None };
}

fn output_unset() {
    let mut ctx = OUTPUT_CONTEXT.lock().unwrap();
    *ctx = None;
}

fn output_traced() {
    STDIO_TRACED.store(true, Ordering::SeqCst);
}

fn output_is_traced() -> bool {
    STDIO_TRACED.load(Ordering::SeqCst)
}

fn output_write(fd: RawFd, buffer: &[u8]) -> io::Result<()> {
    let mut file = unsafe { File::from_raw_fd(fd) };
    file.write_all(buffer)?;
    file.flush()?;
    Ok(())
}

fn output_init(out: Option<&mut Output>) {
    if let Some(out) = out {
        out.out = OUTPUT_NONE;
        out.err = OUTPUT_NONE;
        out.syncout = output_sync_enabled();
        return;
    }

    set_append_mode(std::io::stdout().as_raw_fd());
    set_append_mode(std::io::stderr().as_raw_fd());
}

fn set_append_mode(fd: RawFd) {
    unsafe {
        let flags = fcntl(fd, F_GETFL);
        if flags != -1 {
            fcntl(fd, F_SETFL, flags | O_APPEND);
        }
    }
}

fn output_close(out: Option<&mut Output>) {
    if out.is_none() {
        if output_is_traced() {
            log_working_directory(false);
        }
        return;
    }

    let out = out.unwrap();
    if out.out >= 0 {
        unsafe { libc::close(out.out); }
    }
    if out.err >= 0 && out.err != out.out {
        unsafe { libc::close(out.err); }
    }

    output_init(Some(out));
}

fn output_start() {
    if let Some(ctx) = OUTPUT_CONTEXT.lock().unwrap().as_ref() {
        if ctx.syncout && !ctx.is_set() {
            setup_tmpfile(ctx);
        }
    }

    if !output_is_traced() && should_print_dir() {
        STDIO_TRACED.store(log_working_directory(true), Ordering::SeqCst);
    }
}

fn outputs(is_err: bool, msg: &str) {
    if msg.is_empty() {
        return;
    }

    output_start();

    let ctx = OUTPUT_CONTEXT.lock().unwrap();
    _outputs(ctx.as_ref(), is_err, msg);
}

fn _outputs(out: Option<&Arc<Output>>, is_err: bool, msg: &str) {
    if let Some(out) = out {
        if out.syncout {
            let fd = if is_err { out.err } else { out.out };
            if fd != OUTPUT_NONE {
                if let Err(e) = output_write(fd, msg.as_bytes()) {
                    eprintln!("write error: {}", e);
                }
                return;
            }
        }
    }

    let mut stream = if is_err { std::io::stderr() } else { std::io::stdout() };
    writeln!(stream, "{}", msg).unwrap();
}

fn log_working_directory(entering: bool) -> bool {
    // Implementation omitted for brevity
    false
}

fn setup_tmpfile(out: &Arc<Output>) {
    // Implementation omitted for brevity
}

fn output_sync_enabled() -> bool {
    // Implementation omitted for brevity
    false
}

fn should_print_dir() -> bool {
    // Implementation omitted for brevity
    false
}

fn message(prefix: bool, len: usize, fmt: &str, args: std::fmt::Arguments) {
    // Implementation omitted for brevity
}

fn error(flocp: Option<&Floc>, len: usize, fmt: &str, args: std::fmt::Arguments) {
    // Implementation omitted for brevity
}

fn fatal(flocp: Option<&Floc>, len: usize, fmt: &str, args: std::fmt::Arguments) {
    // Implementation omitted for brevity
}

struct Floc {
    filenm: Option<String>,
    lineno: u32,
    offset: u32,
}

fn perror_with_name(str: &str, name: &str) {
    // Implementation omitted for brevity
}

fn pfatal_with_name(name: &str) {
    // Implementation omitted for brevity
}

fn out_of_memory() {
    // Implementation omitted for brevity
    std::process::exit(1);
}