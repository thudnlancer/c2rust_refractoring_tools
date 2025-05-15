use std::ffi::CStr;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::ptr;
use libc::{c_char, c_int, DIR};
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::sys::stat::Mode;
use nix::errno::errno;

pub fn opendir_safer(name: *const c_char) -> Option<*mut DIR> {
    let dp = unsafe { libc::opendir(name) };
    if dp.is_null() {
        return None;
    }

    let fd = unsafe { libc::dirfd(dp) };
    if (0..=2).contains(&fd) {
        let new_fd = match fcntl(fd, FcntlArg::F_DUPFD_CLOEXEC(3)) {
            Ok(new_fd) => new_fd,
            Err(e) => {
                unsafe { libc::closedir(dp) };
                errno = e as i32;
                return None;
            }
        };

        let new_dp = unsafe { libc::fdopendir(new_fd) };
        if new_dp.is_null() {
            unsafe { libc::close(new_fd) };
            let e = errno;
            unsafe { libc::closedir(dp) };
            errno = e;
            return None;
        }

        unsafe { libc::closedir(dp) };
        Some(new_dp)
    } else {
        Some(dp)
    }
}