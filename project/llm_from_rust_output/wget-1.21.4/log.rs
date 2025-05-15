use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, Write};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

static mut LOGFP: *mut File = ptr::null_mut();
static mut STDTLOGFP: *mut File = ptr::null_mut();
static mut FILELOGFP: *mut File = ptr::null_mut();
static mut LOGFILE: *mut c_char = ptr::null_mut();
static mut SHELL_IS_INTERACTIVE: c_int = 0;
static mut WARCLOGFP: *mut File = ptr::null_mut();
static mut INHIBIT_LOGGING: bool = false;
static mut SAVE_CONTEXT_P: bool = false;
static mut FLUSH_LOG_P: bool = true;
static mut NEEDS_FLUSHING: bool = false;

struct LogLn {
    static_line: [c_char; 129],
    malloced_line: *mut c_char,
    content: *mut c_char,
}

static mut LOG_LINES: [LogLn; 24] = [LogLn {
    static_line: [0; 129],
    malloced_line: ptr::null_mut(),
    content: ptr::null_mut(),
}; 24];

static mut LOG_LINE_CURRENT: c_int = -1;
static mut TRAILING_LINE: bool = false;

unsafe fn free_log_line(num: c_int) {
    let ln = &mut LOG_LINES[num as usize];
    if !ln.malloced_line.is_null() {
        libc::free(ln.malloced_line as *mut c_void);
        ln.malloced_line = ptr::null_mut();
    }
    ln.content = ptr::null_mut();
}

unsafe fn saved_append_1(start: *const c_char, end: *const c_char) {
    let len = end as usize - start as usize;
    if len == 0 {
        return;
    }

    if !TRAILING_LINE {
        let ln = if LOG_LINE_CURRENT == -1 {
            LOG_LINE_CURRENT = 0;
            &mut LOG_LINES[0]
        } else {
            free_log_line(LOG_LINE_CURRENT);
            &mut LOG_LINES[LOG_LINE_CURRENT as usize]
        };

        if len > 128 {
            ln.malloced_line = strdupdelim(start, end);
            ln.content = ln.malloced_line;
        } else {
            ptr::copy_nonoverlapping(start, ln.static_line.as_mut_ptr(), len);
            ln.static_line[len] = 0;
            ln.content = ln.static_line.as_mut_ptr();
        }
    } else {
        let ln = &mut LOG_LINES[LOG_LINE_CURRENT as usize];
        if !ln.malloced_line.is_null() {
            let old_len = libc::strlen(ln.malloced_line);
            ln.malloced_line = libc::realloc(
                ln.malloced_line as *mut c_void,
                old_len + len + 1,
            ) as *mut c_char;
            ptr::copy_nonoverlapping(
                start,
                ln.malloced_line.offset(old_len as isize),
                len,
            );
            *ln.malloced_line.offset((old_len + len) as isize) = 0;
            ln.content = ln.malloced_line;
        } else {
            let old_len = libc::strlen(ln.static_line.as_ptr());
            if old_len + len > 128 {
                ln.malloced_line = libc::malloc(old_len + len + 1) as *mut c_char;
                ptr::copy_nonoverlapping(
                    ln.static_line.as_ptr(),
                    ln.malloced_line,
                    old_len,
                );
                ptr::copy_nonoverlapping(
                    start,
                    ln.malloced_line.offset(old_len as isize),
                    len,
                );
                *ln.malloced_line.offset((old_len + len) as isize) = 0;
                ln.content = ln.malloced_line;
            } else {
                ptr::copy_nonoverlapping(
                    start,
                    ln.static_line.as_mut_ptr().offset(old_len as isize),
                    len,
                );
                ln.static_line[old_len + len] = 0;
                ln.content = ln.static_line.as_mut_ptr();
            }
        }
    }

    TRAILING_LINE = *end.offset(-1) != '\n' as c_char;
    if !TRAILING_LINE {
        LOG_LINE_CURRENT += 1;
        if LOG_LINE_CURRENT >= 24 {
            LOG_LINE_CURRENT = 0;
        }
    }
}

unsafe fn saved_append(s: *const c_char) {
    let mut start = s;
    while *start != 0 {
        let mut end = libc::strchr(start, '\n' as c_int);
        if end.is_null() {
            end = start.offset(libc::strlen(start) as isize);
        } else {
            end = end.offset(1);
        }
        saved_append_1(start, end);
        start = end;
    }
}

unsafe fn get_log_fp() -> *mut File {
    if INHIBIT_LOGGING {
        return ptr::null_mut();
    }
    if !LOGFP.is_null() {
        return LOGFP;
    }
    stderr()
}

unsafe fn get_progress_fp() -> *mut File {
    if opt.show_progress == 1 {
        return stderr();
    }
    get_log_fp()
}

unsafe fn get_warc_log_fp() -> *mut File {
    if INHIBIT_LOGGING {
        return ptr::null_mut();
    }
    if !WARCLOGFP.is_null() {
        return WARCLOGFP;
    }
    if !LOGFP.is_null() {
        return ptr::null_mut();
    }
    stderr()
}

pub unsafe fn log_set_warc_log_fp(fp: *mut File) {
    WARCLOGFP = fp;
}

pub unsafe fn logputs(o: LogOptions, s: *const c_char) {
    let fp = if o == LogOptions::Progress {
        get_progress_fp()
    } else {
        get_log_fp()
    };

    if fp.is_null() {
        return;
    }

    let warcfp = get_warc_log_fp();
    match o {
        LogOptions::Progress if opt.show_progress == 0 => return,
        LogOptions::NotQuiet if opt.quiet => return,
        LogOptions::NonVerbose if opt.verbose != 0 || opt.quiet => return,
        LogOptions::Verbose if opt.verbose == 0 => return,
        _ => {}
    }

    libc::fputs(s, fp);
    if !warcfp.is_null() {
        libc::fputs(s, warcfp);
    }

    if SAVE_CONTEXT_P {
        saved_append(s);
    }

    if FLUSH_LOG_P {
        logflush();
    } else {
        NEEDS_FLUSHING = true;
    }
}

pub unsafe fn logflush() {
    let fp = get_log_fp();
    let warcfp = get_warc_log_fp();
    
    if !fp.is_null() {
        libc::fflush(fp);
    }
    if !warcfp.is_null() {
        libc::fflush(warcfp);
    }
    NEEDS_FLUSHING = false;
}

pub unsafe fn log_set_flush(flush: bool) {
    if flush == FLUSH_LOG_P {
        return;
    }
    if !flush {
        FLUSH_LOG_P = false;
    } else {
        if NEEDS_FLUSHING {
            logflush();
        }
        FLUSH_LOG_P = true;
    }
}

pub unsafe fn log_set_save_context(savep: bool) -> bool {
    let old = SAVE_CONTEXT_P;
    SAVE_CONTEXT_P = savep;
    old
}

pub unsafe fn log_init(file: *const c_char, appendp: bool) {
    if !file.is_null() {
        if *file == '-' as c_char && *file.offset(1) == 0 {
            STDTLOGFP = stdout();
            LOGFP = STDTLOGFP;
        } else {
            let mode = if appendp {
                "a"
            } else {
                "w"
            };
            FILELOGFP = libc::fopen(file, mode.as_ptr());
            if FILELOGFP.is_null() {
                eprintln!(
                    "{}: {}: {}",
                    CStr::from_ptr(exec_name).to_string_lossy(),
                    CStr::from_ptr(file).to_string_lossy(),
                    io::Error::last_os_error()
                );
                std::process::exit(1);
            }
            LOGFP = FILELOGFP;
        }
    } else {
        STDTLOGFP = stderr();
        LOGFP = STDTLOGFP;
        if isatty(libc::fileno(LOGFP)) != 0 {
            SAVE_CONTEXT_P = true;
        }
    }
    SHELL_IS_INTERACTIVE = isatty(0);
}

pub unsafe fn log_close() {
    if !LOGFP.is_null() && LOGFP != stderr() && LOGFP != stdout() {
        if LOGFP == STDTLOGFP {
            STDTLOGFP = ptr::null_mut();
        }
        if LOGFP == FILELOGFP {
            FILELOGFP = ptr::null_mut();
        }
        libc::fclose(LOGFP);
    }
    LOGFP = ptr::null_mut();
    INHIBIT_LOGGING = true;
    SAVE_CONTEXT_P = false;

    for i in 0..24 {
        free_log_line(i);
    }
    LOG_LINE_CURRENT = -1;
    TRAILING_LINE = false;
}

#[repr(u32)]
enum LogOptions {
    Verbose = 0,
    NotQuiet = 1,
    NonVerbose = 2,
    Always = 3,
    Progress = 4,
}

fn stdout() -> *mut File {
    unsafe { libc::stdout as *mut File }
}

fn stderr() -> *mut File {
    unsafe { libc::stderr as *mut File }
}

unsafe fn isatty(fd: c_int) -> c_int {
    libc::isatty(fd)
}

unsafe fn strdupdelim(start: *const c_char, end: *const c_char) -> *mut c_char {
    let len = end as usize - start as usize;
    let s = libc::malloc(len + 1) as *mut c_char;
    ptr::copy_nonoverlapping(start, s, len);
    *s.offset(len as isize) = 0;
    s
}