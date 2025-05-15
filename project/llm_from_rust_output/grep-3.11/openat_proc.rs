use std::ffi::{CString, CStr};
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};

const PROC_SELF_FD_DIR_SIZE_BOUND: usize = 27;

fn openat_proc_name(fd: i32, file: &CStr) -> io::Result<CString> {
    if file.to_bytes().is_empty() {
        return Ok(CString::new("").unwrap());
    }

    static PROC_STATUS: std::sync::OnceLock<io::Result<()>> = std::sync::OnceLock::new();
    let status = PROC_STATUS.get_or_init(|| {
        let proc_self_fd = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_PATH | libc::O_DIRECTORY)
            .open("/proc/self/fd")?;
        
        let dotdot_path = format!("/proc/self/fd/{}/../fd", proc_self_fd.as_raw_fd());
        let exists = Path::new(&dotdot_path).exists();
        
        if !exists {
            Err(io::Error::new(io::ErrorKind::NotFound, "proc path not accessible"))
        } else {
            Ok(())
        }
    });

    if status.is_err() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "proc filesystem not available",
        ));
    }

    let file_len = file.to_bytes().len();
    let bufsize = PROC_SELF_FD_DIR_SIZE_BOUND + file_len;
    let mut buf = Vec::with_capacity(bufsize);

    let dir_part = format!("/proc/self/fd/{}/", fd);
    buf.extend_from_slice(dir_part.as_bytes());
    buf.extend_from_slice(file.to_bytes());

    CString::new(buf).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "null byte in path"))
}