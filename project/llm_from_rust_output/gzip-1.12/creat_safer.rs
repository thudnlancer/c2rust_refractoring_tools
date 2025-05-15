use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{FromRawFd, IntoRawFd};

pub type ModeT = u32;

pub fn creat_safer(file: &str, mode: ModeT) -> std::io::Result<File> {
    let c_file = CString::new(file)?;
    let fd = unsafe { libc::creat(c_file.as_ptr(), mode as libc::mode_t) };
    if fd == -1 {
        return Err(std::io::Error::last_os_error());
    }
    
    let safer_fd = unsafe { libc::fd_safer(fd) };
    if safer_fd == -1 {
        return Err(std::io::Error::last_os_error());
    }
    
    Ok(unsafe { File::from_raw_fd(safer_fd) })
}