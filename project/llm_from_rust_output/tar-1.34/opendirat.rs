use std::ffi::CStr;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::ptr;
use libc::{c_int, O_RDONLY, O_DIRECTORY, O_NOCTTY, O_NONBLOCK, O_CLOEXEC};

pub type DIR = libc::DIR;

pub fn opendirat(
    dir_fd: RawFd,
    dir: &CStr,
    extra_flags: c_int,
    pnew_fd: &mut RawFd,
) -> Option<*mut DIR> {
    let open_flags = O_RDONLY | O_DIRECTORY | O_NOCTTY | O_NONBLOCK | O_CLOEXEC | extra_flags;
    
    let new_fd = unsafe { libc::openat(dir_fd, dir.as_ptr(), open_flags) };
    if new_fd < 0 {
        return None;
    }

    let dirp = unsafe { libc::fdopendir(new_fd) };
    if !dirp.is_null() {
        *pnew_fd = new_fd;
        Some(dirp)
    } else {
        let fdopendir_errno = unsafe { *libc::__errno_location() };
        unsafe { libc::close(new_fd) };
        unsafe { *libc::__errno_location() = fdopendir_errno };
        None
    }
}