use std::ffi::CStr;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::ptr;
use libc::{self, DIR, O_CLOEXEC};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};
use nix::errno::errno;
use nix::sys::stat::fstat;
use std::fs::File;

pub fn opendir_safer(name: &CStr) -> Option<*mut DIR> {
    unsafe {
        let dp = libc::opendir(name.as_ptr());
        if dp.is_null() {
            return None;
        }

        let fd = libc::dirfd(dp);
        if (0..=2).contains(&fd) {
            let flags = match fcntl(fd, FcntlArg::F_GETFD) {
                Ok(flags) => flags,
                Err(_) => {
                    let e = errno();
                    libc::closedir(dp);
                    errno = e;
                    return None;
                }
            };

            let new_fd = match fcntl(fd, FcntlArg::F_DUPFD_CLOEXEC(3)) {
                Ok(new_fd) => new_fd,
                Err(_) => {
                    let e = errno();
                    libc::closedir(dp);
                    errno = e;
                    return None;
                }
            };

            let new_dp = libc::fdopendir(new_fd);
            if new_dp.is_null() {
                let e = errno();
                libc::close(new_fd);
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