use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::prelude::*;
use std::path::{Path, PathBuf};

const PROC_SELF_FD_DIR_SIZE_BOUND: usize = 27;
const PATH_MAX: usize = 4096;

pub fn openat_proc_name(fd: RawFd, file: &str) -> io::Result<PathBuf> {
    if file.is_empty() {
        return Ok(PathBuf::new());
    }

    static PROC_STATUS: std::sync::OnceLock<io::Result<()>> = std::sync::OnceLock::new();
    let status = PROC_STATUS.get_or_init(|| {
        let proc_self_fd = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_PATH)
            .open("/proc/self/fd")?;
        
        let dotdot_path = format!("/proc/self/fd/{}/../fd", proc_self_fd.as_raw_fd());
        if !Path::new(&dotdot_path).exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "proc path not accessible"));
        }
        Ok(())
    });

    if status.is_err() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "proc filesystem not available",
        ));
    }

    let bufsize = PROC_SELF_FD_DIR_SIZE_BOUND + file.len();
    if PATH_MAX - 64 < bufsize {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "path too long",
        ));
    }

    let mut path = PathBuf::new();
    path.push(format!("/proc/self/fd/{}/", fd));
    path.push(file);

    Ok(path)
}