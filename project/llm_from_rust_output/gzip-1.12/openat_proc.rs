use std::ffi::{CString, CStr};
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};

const PROC_SELF_FD_DIR_SIZE_BOUND: usize = 27;

static PROC_STATUS: std::sync::OnceLock<io::Result<()>> = std::sync::OnceLock::new();

fn check_proc_status() -> io::Result<()> {
    PROC_STATUS.get_or_init(|| {
        let proc_self_fd = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_PATH | libc::O_CLOEXEC)
            .open("/proc/self/fd")?;

        let fd = proc_self_fd.as_raw_fd();
        let dotdot_path = format!("/proc/self/fd/{}/../fd", fd);
        
        if !Path::new(&dotdot_path).exists() {
            Err(io::Error::new(io::ErrorKind::NotFound, "proc fd path not accessible"))
        } else {
            Ok(())
        }
    }).clone()
}

pub fn openat_proc_name(fd: i32, file: &CStr) -> io::Result<CString> {
    if file.to_bytes().is_empty() {
        return Ok(CString::new("").unwrap());
    }

    check_proc_status()?;

    let file_len = file.to_bytes().len();
    let required_size = PROC_SELF_FD_DIR_SIZE_BOUND + file_len;
    let mut path_buf = PathBuf::with_capacity(required_size);

    path_buf.push(format!("/proc/self/fd/{}/", fd));
    path_buf.push(Path::new(OsStrExt::from_bytes(file.to_bytes())));

    Ok(CString::new(path_buf.into_os_string().into_vec()).unwrap())
}