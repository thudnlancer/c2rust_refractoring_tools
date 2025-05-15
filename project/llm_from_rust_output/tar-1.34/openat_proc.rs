use std::ffi::{CString, CStr};
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};

const PROC_SELF_FD_DIR_SIZE_BOUND: usize = 27;

#[derive(Debug)]
enum ProcStatus {
    Unknown,
    Available,
    Unavailable,
}

static mut PROC_STATUS: ProcStatus = ProcStatus::Unknown;

fn check_proc_status() -> io::Result<ProcStatus> {
    unsafe {
        match PROC_STATUS {
            ProcStatus::Unknown => {
                let proc_self_fd = OpenOptions::new()
                    .read(true)
                    .open("/proc/self/fd")?;

                let fd = proc_self_fd.as_raw_fd();
                let dotdot_path = format!("/proc/self/fd/{}/../fd", fd);
                
                let status = if Path::new(&dotdot_path).exists() {
                    ProcStatus::Available
                } else {
                    ProcStatus::Unavailable
                };

                PROC_STATUS = status;
                Ok(status)
            }
            status => Ok(status),
        }
    }
}

pub fn openat_proc_name(fd: i32, file: &CStr) -> io::Result<CString> {
    if file.to_bytes().is_empty() {
        return Ok(CString::new("").unwrap());
    }

    let status = check_proc_status()?;
    if let ProcStatus::Unavailable = status {
        return Err(io::Error::new(io::ErrorKind::NotFound, "/proc/self/fd not available"));
    }

    let file_str = file.to_str().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let required_size = PROC_SELF_FD_DIR_SIZE_BOUND + file.to_bytes().len();

    let mut path_buf = PathBuf::with_capacity(required_size);
    path_buf.push(format!("/proc/self/fd/{}/", fd));
    path_buf.push(file_str);

    let path = path_buf.as_os_str().as_bytes();
    CString::new(path).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}