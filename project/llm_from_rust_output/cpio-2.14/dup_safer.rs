use std::os::unix::io::{FromRawFd, RawFd};
use std::fs::File;

pub fn dup_safer(fd: RawFd) -> Result<RawFd, std::io::Error> {
    let file = unsafe { File::from_raw_fd(fd) };
    let new_fd = file.try_clone()?.into_raw_fd();
    std::mem::forget(file); // Prevent closing the original fd
    Ok(new_fd)
}