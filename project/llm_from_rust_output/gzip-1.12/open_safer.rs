use std::ffi::CString;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::fs::{File, OpenOptions};

pub fn open_safer(file: &str, flags: i32, mode: Option<u32>) -> std::io::Result<File> {
    let c_file = CString::new(file)?;
    let raw_fd = unsafe {
        let mode = mode.unwrap_or(0);
        libc::open(c_file.as_ptr(), flags, mode)
    };
    
    if raw_fd == -1 {
        return Err(std::io::Error::last_os_error());
    }

    let safer_fd = unsafe { libc::fd_safer(raw_fd) };
    if safer_fd == -1 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(unsafe { File::from_raw_fd(safer_fd) })
}