use std::fs::File;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::path::Path;
use std::io::{Result, Error, ErrorKind};

pub fn fopen_safer(file: &Path, mode: &str) -> Result<File> {
    let file = File::open(file)?;
    let fd = file.into_raw_fd();

    if fd >= 0 && fd <= 2 {
        let new_fd = unsafe { libc::dup(fd) };
        if new_fd < 0 {
            let e = std::io::Error::last_os_error();
            return Err(e);
        }

        let new_file = unsafe { File::from_raw_fd(new_fd) };
        Ok(new_file)
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}