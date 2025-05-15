use std::ffi::CStr;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::ptr;
use libc::{self, c_char, c_int, DIR};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};
use nix::errno::errno;
use nix::sys::stat::fstat;
use std::fs::File;

pub fn opendir_safer(name: *const c_char) -> Option<*mut DIR> {
    unsafe {
        let dp = libc::opendir(name);
        if dp.is_null() {
            return None;
        }

        let fd = libc::dirfd(dp);
        if (0..=2).contains(&fd) {
            let new_fd = match fcntl(fd, FcntlArg::F_DUPFD_CLOEXEC(3)) {
                Ok(new_fd) => new_fd,
                Err(e) => {
                    libc::closedir(dp);
                    errno = e as i32;
                    return None;
                }
            };

            let new_dp = libc::fdopendir(new_fd);
            if new_dp.is_null() {
                libc::close(new_fd);
                let e = errno;
                libc::closedir(dp);
                errno = e;
                return None;
            }

            libc::closedir(dp);
            Some(new_dp)
        } else {
            Some(dp)
        }
    }
}