use std::ffi::CString;
use std::os::unix::prelude::*;

pub fn openat_safer(
    fd: i32,
    file: &str,
    flags: i32,
    mode: Option<u32>,
) -> std::io::Result<i32> {
    let c_file = CString::new(file)?;
    
    let raw_fd = unsafe {
        if let Some(m) = mode {
            libc::openat(fd, c_file.as_ptr(), flags, m)
        } else {
            libc::openat(fd, c_file.as_ptr(), flags)
        }
    };

    if raw_fd == -1 {
        return Err(std::io::Error::last_os_error());
    }

    let safer_fd = unsafe { fd_safer(raw_fd) };
    if safer_fd == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(safer_fd)
    }
}

extern "C" {
    fn fd_safer(fd: libc::c_int) -> libc::c_int;
}