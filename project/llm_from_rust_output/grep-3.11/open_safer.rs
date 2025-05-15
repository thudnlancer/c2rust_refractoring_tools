use std::ffi::CString;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::fs::{File, OpenOptions};

pub type Mode = u32;

pub fn open_safer(file: &str, flags: i32, mode: Option<Mode>) -> std::io::Result<File> {
    let c_file = CString::new(file)?;
    
    let file = unsafe {
        let fd = libc::open(c_file.as_ptr(), flags, mode.unwrap_or(0));
        if fd == -1 {
            return Err(std::io::Error::last_os_error());
        }
        
        let safer_fd = libc::fd_safer(fd);
        if safer_fd == -1 {
            libc::close(fd);
            return Err(std::io::Error::last_os_error());
        }
        
        File::from_raw_fd(safer_fd)
    };
    
    Ok(file)
}