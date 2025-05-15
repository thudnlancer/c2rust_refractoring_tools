use std::os::unix::io::FromRawFd;
use std::os::unix::io::IntoRawFd;

pub fn dup_safer(fd: libc::c_int) -> libc::c_int {
    let file = unsafe { std::fs::File::from_raw_fd(fd) };
    let new_fd = file.into_raw_fd();
    unsafe { std::mem::forget(file) };
    new_fd
}