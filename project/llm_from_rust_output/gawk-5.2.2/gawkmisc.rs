use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::fs::{File, Metadata, OpenOptions};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::io::{self, Error, ErrorKind};
use libc::{c_int, c_char, c_void, c_uint, c_long, c_ulong, c_double, stat, timespec, fcntl, F_GETFD, F_SETFD, FD_CLOEXEC};
use std::ptr;
use std::mem;
use std::env;
use std::str;

static QUOTE: char = '\'';
static DEFPATH: &str = ".:/usr/local/share/awk";
static DEFLIBPATH: &str = "/usr/local/lib/gawk";
static ENVSEP: char = ':';

pub fn gawk_name(filespec: &str) -> &str {
    Path::new(filespec)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(filespec)
}

pub fn os_arg_fixup(_argc: &mut i32, _argv: &mut Vec<String>) {}

pub fn os_devopen(_name: &str, _flag: i32) -> io::Result<i32> {
    Err(Error::new(ErrorKind::Other, "Not implemented"))
}

pub fn optimal_bufsize(fd: i32) -> io::Result<usize> {
    let mut st = unsafe { mem::zeroed() };
    if unsafe { fstat(fd, &mut st) } == -1 {
        return Err(Error::last_os_error());
    }

    if let Ok(val) = env::var("AWKBUFSIZE") {
        if val == "exact" {
            return Ok(st.st_size as usize);
        } else if let Ok(num) = val.parse::<usize>() {
            return Ok(num);
        }
    }

    if st.st_mode & 0o170000 == 0o100000 && st.st_size > 0 {
        if st.st_size < st.st_blksize.max(8192) {
            return Ok(st.st_size as usize);
        }
    }

    Ok(st.st_blksize.max(8192) as usize)
}

pub fn ispath(file: &str) -> bool {
    file.contains('/')
}

pub fn isdirpunct(c: char) -> bool {
    c == '/'
}

pub fn os_close_on_exec(fd: i32, name: &str, what: &str, dir: &str) -> io::Result<()> {
    if fd <= 2 {
        return Ok(());
    }

    let flags = unsafe { fcntl(fd, F_GETFD) };
    if flags == -1 {
        return Err(Error::last_os_error());
    }

    if unsafe { fcntl(fd, F_SETFD, flags | FD_CLOEXEC) } == -1 {
        return Err(Error::last_os_error());
    }

    Ok(())
}

pub fn os_isdir(fd: i32) -> io::Result<bool> {
    let mut st = unsafe { mem::zeroed() };
    if unsafe { fstat(fd, &mut st) } == -1 {
        return Err(Error::last_os_error());
    }
    Ok(st.st_mode & 0o170000 == 0o40000)
}

pub fn os_isreadable(iobuf: &awk_input_buf_t) -> (bool, bool) {
    if iobuf.fd == -1 {
        return (false, false);
    }

    match iobuf.sbuf.st_mode & 0o170000 {
        0o100000 | 0o20000 | 0o140000 | 0o10000 => (true, false),
        0o40000 => (false, true),
        _ => (false, false),
    }
}

pub fn os_is_setuid() -> bool {
    unsafe { getuid() != geteuid() && geteuid() == 0 }
}

pub fn os_setbinmode(_fd: i32, _mode: i32) -> io::Result<()> {
    Ok(())
}

pub fn os_restore_mode(_fd: i32) {}

pub fn os_isatty(fd: i32) -> bool {
    unsafe { isatty(fd) != 0 }
}

pub fn files_are_same(path: &str, src: &SRCFILE) -> io::Result<bool> {
    let st = std::fs::metadata(path)?;
    Ok(st.dev() == src.sbuf.st_dev && st.ino() == src.sbuf.st_ino)
}

pub fn init_sockets() {}

pub fn os_maybe_set_errno() {}

// Helper types to match C structs
#[repr(C)]
pub struct awk_input_buf_t {
    pub fd: c_int,
    pub sbuf: stat,
    // Other fields omitted for brevity
}

#[repr(C)]
pub struct SRCFILE {
    pub sbuf: stat,
    // Other fields omitted for brevity
}

extern "C" {
    fn getuid() -> c_uint;
    fn geteuid() -> c_uint;
    fn isatty(fd: c_int) -> c_int;
    fn fstat(fd: c_int, buf: *mut stat) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
}