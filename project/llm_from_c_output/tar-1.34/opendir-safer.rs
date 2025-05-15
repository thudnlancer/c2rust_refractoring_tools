use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::io;
use libc::{STDERR_FILENO, DIR};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};
use nix::errno::Errno;

/// Like opendir, but do not clobber stdin, stdout, or stderr.
pub fn opendir_safer<P: AsRef<Path>>(name: P) -> io::Result<*mut DIR> {
    let dp = unsafe { libc::opendir(name.as_ref().as_ptr() as *const libc::c_char) };

    if !dp.is_null() {
        let fd = unsafe { libc::dirfd(dp) };

        if fd >= 0 && fd <= STDERR_FILENO {
            // If fdopendir is native (as on Linux), then it is safe to
            // assume dirfd(fdopendir(n))==n. If we are using the
            // gnulib module fdopendir, then this guarantee is not met,
            // but fdopendir recursively calls opendir_safer up to 3
            // times to at least get a safe fd. If fdopendir is not
            // present but dirfd is accurate (as on cygwin 1.5.x), then
            // we recurse up to 3 times ourselves. Finally, if dirfd
            // always fails (as on mingw), then we are already safe.
            let newdp;
            let e;

            #[cfg(any(target_os = "linux", target_os = "android"))]
            {
                match fcntl(fd, FcntlArg::F_DUPFD_CLOEXEC(STDERR_FILENO + 1)) {
                    Ok(new_fd) => {
                        newdp = unsafe { libc::fdopendir(new_fd) };
                        e = Errno::last();
                        if newdp.is_null() {
                            unsafe { libc::close(new_fd) };
                        }
                    }
                    Err(err) => {
                        newdp = std::ptr::null_mut();
                        e = err;
                    }
                }
            }

            #[cfg(not(any(target_os = "linux", target_os = "android")))]
            {
                newdp = opendir_safer(name);
                e = Errno::last();
            }

            unsafe { libc::closedir(dp) };
            Errno::set_errno(e);
            return if newdp.is_null() {
                Err(io::Error::from_raw_os_error(e as i32))
            } else {
                Ok(newdp)
            };
        }
    }

    if dp.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(dp)
    }
}