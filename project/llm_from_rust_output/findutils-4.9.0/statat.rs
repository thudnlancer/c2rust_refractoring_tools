use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::mem::MaybeUninit;
use libc::{c_int, stat, timespec};

#[derive(Debug)]
pub enum StatError {
    InvalidPath,
    SystemError(i32),
}

pub fn lstatat(fd: c_int, name: &Path, st: &mut stat) -> Result<(), StatError> {
    fstatat(fd, name, st, libc::AT_SYMLINK_NOFOLLOW)
}

pub fn statat(fd: c_int, name: &Path, st: &mut stat) -> Result<(), StatError> {
    fstatat(fd, name, st, 0)
}

fn fstatat(fd: c_int, name: &Path, st: &mut stat, flag: c_int) -> Result<(), StatError> {
    let c_str = CString::new(name.as_os_str().as_bytes()).map_err(|_| StatError::InvalidPath)?;
    
    let result = unsafe {
        let mut stat_buf = MaybeUninit::<stat>::uninit();
        let ret = libc::fstatat(
            fd,
            c_str.as_ptr(),
            stat_buf.as_mut_ptr(),
            flag,
        );
        
        if ret == 0 {
            *st = stat_buf.assume_init();
            Ok(())
        } else {
            Err(StatError::SystemError(std::io::Error::last_os_error().raw_os_error().unwrap_or(0)))
        }
    };
    
    result
}