use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::path::Path;
use std::io;
use libc::{STDERR_FILENO, DIR};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};
use nix::sys::stat::fstat;
use std::ptr;

// Like opendir, but do not clobber stdin, stdout, or stderr.
pub fn opendir_safer<P: AsRef<Path>>(name: P) -> Result<*mut DIR, io::Error> {
    unsafe {
        let dp = libc::opendir(name.as_ref().as_ptr() as *const _);
        
        if !dp.is_null() {
            let fd = libc::dirfd(dp);
            
            if fd >= 0 && fd <= STDERR_FILENO {
                let newdp = match fdopendir_safe(fd) {
                    Ok(dir) => dir,
                    Err(e) => {
                        libc::closedir(dp);
                        return Err(e);
                    }
                };
                
                libc::closedir(dp);
                return Ok(newdp);
            }
        }
        
        if dp.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(dp)
        }
    }
}

fn fdopendir_safe(fd: RawFd) -> Result<*mut DIR, io::Error> {
    unsafe {
        let new_fd = fcntl(fd, FcntlArg::F_DUPFD_CLOEXEC(STDERR_FILENO + 1))
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;
        
        let newdp = libc::fdopendir(new_fd);
        if newdp.is_null() {
            libc::close(new_fd);
            Err(io::Error::last_os_error())
        } else {
            Ok(newdp)
        }
    }
}

// Helper to close DIR pointer when no longer needed
pub fn closedir_safe(dir: *mut DIR) {
    if !dir.is_null() {
        unsafe {
            libc::closedir(dir);
        }
    }
}