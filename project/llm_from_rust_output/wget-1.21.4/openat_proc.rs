use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::prelude::*;
use std::path::Path;

const PROC_SELF_FD_DIR_SIZE_BOUND: usize = 27;

pub fn openat_proc_name(fd: RawFd, file: &str) -> io::Result<CString> {
    if file.is_empty() {
        return Ok(CString::new("").unwrap());
    }

    static PROC_STATUS: std::sync::OnceLock<io::Result<bool>> = std::sync::OnceLock::new();
    let proc_status = PROC_STATUS.get_or_init(|| {
        let proc_self_fd = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_PATH | libc::O_CLOEXEC)
            .open("/proc/self/fd")?;

        let dotdot_path = format!("/proc/self/fd/{}/../fd", proc_self_fd.as_raw_fd());
        let accessible = Path::new(&dotdot_path).exists();
        Ok(accessible)
    });

    if proc_status.as_ref().map_err(|e| io::Error::new(e.kind(), e.to_string()))? == &false {
        return Err(io::Error::new(io::ErrorKind::NotFound, "/proc/self/fd not accessible"));
    }

    let path = format!("/proc/self/fd/{}/{}", fd, file);
    CString::new(path).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}