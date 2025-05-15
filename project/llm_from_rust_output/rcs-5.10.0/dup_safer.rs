use std::os::unix::io::{FromRawFd, RawFd};
use std::fs::File;

pub fn dup_safer(fd: RawFd) -> Result<RawFd, std::io::Error> {
    let file = unsafe { File::from_raw_fd(fd) };
    let new_fd = file.try_clone()?;
    let raw_fd = new_fd.into_raw_fd();
    Ok(raw_fd)
}