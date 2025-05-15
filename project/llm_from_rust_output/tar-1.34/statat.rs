use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use libc::{self, stat, timespec};

#[derive(Debug)]
pub enum StatError {
    InvalidPath,
    SystemError(i32),
}

pub fn lstatat<P: AsRef<Path>>(fd: i32, path: P) -> Result<stat, StatError> {
    fstatat(fd, path, libc::AT_SYMLINK_NOFOLLOW)
}

pub fn statat<P: AsRef<Path>>(fd: i32, path: P) -> Result<stat, StatError> {
    fstatat(fd, path, 0)
}

fn fstatat<P: AsRef<Path>>(fd: i32, path: P, flag: i32) -> Result<stat, StatError> {
    let path_cstring = CString::new(path.as_ref().as_os_str().as_bytes())
        .map_err(|_| StatError::InvalidPath)?;

    let mut stat_buf = unsafe { std::mem::zeroed() };
    
    let res = unsafe {
        libc::fstatat(
            fd,
            path_cstring.as_ptr(),
            &mut stat_buf,
            flag,
        )
    };

    if res == 0 {
        Ok(stat_buf)
    } else {
        Err(StatError::SystemError(std::io::Error::last_os_error().raw_os_error().unwrap_or_default()))
    }
}