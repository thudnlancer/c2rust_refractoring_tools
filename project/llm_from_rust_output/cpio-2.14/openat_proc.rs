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
                
                PROC_STATUS = if Path::new(&dotdot_path).exists() {
                    ProcStatus::Available
                } else {
                    ProcStatus::Unavailable
                };
                
                Ok(PROC_STATUS)
            }
            status => Ok(status),
        }
    }
}

pub fn openat_proc_name(fd: i32, file: &CStr) -> io::Result<CString> {
    if file.to_bytes().is_empty() {
        return Ok(CString::new("").unwrap());
    }

    let proc_status = check_proc_status()?;
    if let ProcStatus::Unavailable = proc_status {
        return Err(io::Error::new(io::ErrorKind::NotFound, "/proc/self/fd not available"));
    }

    let file_path = file.to_str().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let required_size = PROC_SELF_FD_DIR_SIZE_BOUND + file_path.len();
    
    let mut buf = Vec::with_capacity(required_size);
    let dir_part = format!("/proc/self/fd/{}/", fd);
    buf.extend_from_slice(dir_part.as_bytes());
    buf.extend_from_slice(file_path.as_bytes());
    
    CString::new(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}