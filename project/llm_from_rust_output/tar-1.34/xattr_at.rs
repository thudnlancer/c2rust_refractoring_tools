use std::ffi::{CString, CStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::io;
use libc::{c_int, c_char, c_void, size_t, ssize_t};

const AT_FDCWD: c_int = -100;
const EBADF: c_int = 9;

#[derive(Debug)]
struct SavedCwd {
    desc: c_int,
    name: Option<CString>,
}

impl SavedCwd {
    fn save() -> io::Result<Self> {
        // Placeholder for actual implementation
        Ok(Self {
            desc: 0,
            name: None,
        })
    }

    fn restore(&self) -> io::Result<()> {
        // Placeholder for actual implementation
        Ok(())
    }
}

impl Drop for SavedCwd {
    fn drop(&mut self) {
        // Placeholder for cleanup
    }
}

fn is_absolute_path(file: &CStr) -> bool {
    file.to_bytes().first() == Some(&b'/')
}

fn handle_proc_path(fd: c_int, file: &CStr) -> io::Result<Option<CString>> {
    // Placeholder for actual /proc path handling
    Ok(None)
}

macro_rules! xattr_operation {
    ($func:ident, $fd:expr, $file:expr, $($arg:expr),*) => {{
        let file_cstr = CStr::from_ptr($file);
        
        if $fd == AT_FDCWD || is_absolute_path(file_cstr) {
            return unsafe { $func(file_cstr.as_ptr(), $($arg),*) };
        }

        if let Some(proc_path) = handle_proc_path($fd, file_cstr)? {
            let result = unsafe { $func(proc_path.as_ptr(), $($arg),*) };
            if result != -1 {
                return Ok(result);
            }
            let err = io::Error::last_os_error();
            if ![libc::ENOTDIR, libc::ENOENT, libc::EPERM, libc::EACCES, libc::ENOSYS, libc::EOPNOTSUPP]
                .contains(&err.raw_os_error().unwrap_or(0))
            {
                return Err(err);
            }
        }

        let saved_cwd = SavedCwd::save()?;
        
        if $fd >= 0 && $fd == saved_cwd.desc {
            return Err(io::Error::from_raw_os_error(EBADF));
        }

        if unsafe { libc::fchdir($fd) } != 0 {
            return Err(io::Error::last_os_error());
        }

        let result = unsafe { $func(file_cstr.as_ptr(), $($arg),*) };
        let saved_err = if result == -1 {
            Some(io::Error::last_os_error())
        } else {
            None
        };

        if let Err(e) = saved_cwd.restore() {
            eprintln!("Failed to restore working directory: {}", e);
        }

        if let Some(err) = saved_err {
            return Err(err);
        }

        Ok(result)
    }};
}

pub fn llistxattrat(fd: c_int, file: *const c_char, list: *mut c_char, size: size_t) -> io::Result<ssize_t> {
    xattr_operation!(libc::llistxattr, fd, file, list, size)
}

pub fn setxattrat(fd: c_int, file: *const c_char, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> io::Result<c_int> {
    xattr_operation!(libc::setxattr, fd, file, name, value, size, flags)
}

pub fn lgetxattrat(fd: c_int, file: *const c_char, name: *const c_char, value: *mut c_void, size: size_t) -> io::Result<ssize_t> {
    xattr_operation!(libc::lgetxattr, fd, file, name, value, size)
}

pub fn getxattrat(fd: c_int, file: *const c_char, name: *const c_char, value: *mut c_void, size: size_t) -> io::Result<ssize_t> {
    xattr_operation!(libc::getxattr, fd, file, name, value, size)
}

pub fn listxattrat(fd: c_int, file: *const c_char, list: *mut c_char, size: size_t) -> io::Result<ssize_t> {
    xattr_operation!(libc::listxattr, fd, file, list, size)
}

pub fn lsetxattrat(fd: c_int, file: *const c_char, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> io::Result<c_int> {
    xattr_operation!(libc::lsetxattr, fd, file, name, value, size, flags)
}