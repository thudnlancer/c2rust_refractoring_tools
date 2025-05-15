use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write, Error, ErrorKind};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use std::ptr;
use std::mem;
use std::cmp::Ordering;
use libc::{c_int, c_uint, c_char, c_void, size_t, ssize_t, pid_t, mode_t, time_t};
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::collections::HashMap;

static mut MK_STATE: c_uint = 0;

pub fn make_seed(seed: c_uint) {
    unsafe {
        MK_STATE = seed;
    }
}

pub fn make_rand() -> c_uint {
    unsafe {
        if MK_STATE == 0 {
            MK_STATE = (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as c_uint)
                .wrapping_add(1);
        }
        MK_STATE ^= MK_STATE << 13;
        MK_STATE ^= MK_STATE >> 17;
        MK_STATE ^= MK_STATE << 5;
        MK_STATE
    }
}

pub fn make_pid() -> pid_t {
    unsafe { libc::getpid() }
}

pub fn xmalloc(size: size_t) -> *mut c_void {
    let size = if size == 0 { 1 } else { size };
    unsafe {
        let ptr = libc::malloc(size);
        if ptr.is_null() {
            panic!("Out of memory");
        }
        ptr
    }
}

pub fn xcalloc(size: size_t) -> *mut c_void {
    let size = if size == 0 { 1 } else { size };
    unsafe {
        let ptr = libc::calloc(size, 1);
        if ptr.is_null() {
            panic!("Out of memory");
        }
        ptr
    }
}

pub fn xrealloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    let size = if size == 0 { 1 } else { size };
    unsafe {
        let ptr = if ptr.is_null() {
            libc::malloc(size)
        } else {
            libc::realloc(ptr, size)
        };
        if ptr.is_null() {
            panic!("Out of memory");
        }
        ptr
    }
}

pub fn xstrdup(s: &CStr) -> *mut c_char {
    unsafe {
        let ptr = libc::strdup(s.as_ptr());
        if ptr.is_null() {
            panic!("Out of memory");
        }
        ptr
    }
}

pub fn xstrndup(s: &CStr, len: size_t) -> *mut c_char {
    unsafe {
        let ptr = libc::strndup(s.as_ptr(), len);
        if ptr.is_null() {
            panic!("Out of memory");
        }
        ptr
    }
}

pub fn writebuf(fd: RawFd, buf: &[u8]) -> Result<usize, Error> {
    let mut written = 0;
    while written < buf.len() {
        let res = unsafe { libc::write(fd, buf[written..].as_ptr() as *const c_void, buf[written..].len()) };
        if res == -1 {
            let err = Error::last_os_error();
            if err.kind() != ErrorKind::Interrupted {
                return Err(err);
            }
        } else {
            written += res as usize;
        }
    }
    Ok(written)
}

pub fn readbuf(fd: RawFd, buf: &mut [u8]) -> Result<usize, Error> {
    let mut read = 0;
    while read < buf.len() {
        let res = unsafe { libc::read(fd, buf[read..].as_mut_ptr() as *mut c_void, buf[read..].len()) };
        if res == -1 {
            let err = Error::last_os_error();
            if err.kind() != ErrorKind::Interrupted {
                return Err(err);
            }
        } else if res == 0 {
            break;
        } else {
            read += res as usize;
        }
    }
    Ok(read)
}

pub fn get_tmpdir() -> PathBuf {
    let env_vars = ["MAKE_TMPDIR", "TMPDIR"];
    for var in &env_vars {
        if let Ok(dir) = env::var(var) {
            let path = Path::new(&dir);
            if path.is_dir() {
                return path.to_path_buf();
            }
        }
    }
    Path::new("/tmp").to_path_buf()
}

pub fn get_tmpfd() -> Result<(RawFd, PathBuf), Error> {
    let tmpdir = get_tmpdir();
    let mut template = tmpdir.join("GmXXXXXX");
    
    unsafe {
        let template_c = CString::new(template.as_os_str().as_bytes()).unwrap();
        let mut template_vec = template_c.into_bytes_with_nul();
        let fd = libc::mkstemp(template_vec.as_mut_ptr() as *mut c_char);
        if fd == -1 {
            return Err(Error::last_os_error());
        }
        
        template = PathBuf::from(OsString::from_vec(template_vec[..template_vec.len()-1].to_vec()));
        Ok((fd, template))
    }
}

pub fn get_tmpfile() -> Result<(File, PathBuf), Error> {
    let (fd, path) = get_tmpfd()?;
    unsafe {
        let file = File::from_raw_fd(fd);
        Ok((file, path))
    }
}

pub fn alpha_compare(s1: &CStr, s2: &CStr) -> Ordering {
    let s1_bytes = s1.to_bytes();
    let s2_bytes = s2.to_bytes();
    
    if s1_bytes[0] != s2_bytes[0] {
        s1_bytes[0].cmp(&s2_bytes[0])
    } else {
        s1.cmp(s2)
    }
}

pub fn collapse_continuations(line: &mut CString) {
    // Implementation would need to handle backslash continuation lines
    // This is a simplified placeholder
    let bytes = line.to_bytes_with_nul();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i+1 < bytes.len() && bytes[i+1] == b'\n' {
            out.push(b' ');
            i += 2;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    *line = CString::new(out).unwrap();
}

pub fn print_spaces(n: usize) {
    print!("{}", " ".repeat(n));
}

pub fn concat(strings: &[&CStr]) -> CString {
    let total_len = strings.iter().map(|s| s.to_bytes().len()).sum();
    let mut buf = Vec::with_capacity(total_len);
    for s in strings {
        buf.extend_from_slice(s.to_bytes());
    }
    unsafe { CString::from_vec_unchecked(buf) }
}

pub fn make_toui(s: &CStr) -> Result<c_uint, &'static str> {
    let s = s.to_str().map_err(|_| "Invalid UTF-8")?;
    s.parse().map_err(|_| "Invalid number")
}

pub fn make_lltoa(val: i64) -> CString {
    CString::new(format!("{}", val)).unwrap()
}

pub fn make_ulltoa(val: u64) -> CString {
    CString::new(format!("{}", val)).unwrap()
}