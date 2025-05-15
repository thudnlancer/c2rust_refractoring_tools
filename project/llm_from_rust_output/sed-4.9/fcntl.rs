use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::fs::File;
use std::io;

const F_DUPFD: libc::c_int = 0;
const F_DUPFD_CLOEXEC: libc::c_int = 1030;
const F_GETFD: libc::c_int = 1;
const F_SETFD: libc::c_int = 2;
const FD_CLOEXEC: libc::c_int = 1;
const EINVAL: libc::c_int = 22;

#[derive(Debug)]
enum FcntlError {
    IoError(io::Error),
    UnsupportedAction,
}

impl From<io::Error> for FcntlError {
    fn from(err: io::Error) -> Self {
        FcntlError::IoError(err)
    }
}

struct FcntlState {
    have_dupfd_cloexec: Option<bool>,
}

impl FcntlState {
    fn new() -> Self {
        FcntlState {
            have_dupfd_cloexec: None,
        }
    }

    fn rpl_fcntl(&mut self, fd: RawFd, action: libc::c_int, arg: libc::c_int) -> Result<RawFd, FcntlError> {
        match action {
            F_DUPFD => self.rpl_fcntl_dupfd(fd, arg),
            F_DUPFD_CLOEXEC => self.rpl_fcntl_dupfd_cloexec(fd, arg),
            _ => Err(FcntlError::UnsupportedAction),
        }
    }

    fn rpl_fcntl_dupfd(&self, fd: RawFd, target: libc::c_int) -> Result<RawFd, FcntlError> {
        unsafe {
            let result = libc::fcntl(fd, F_DUPFD, target);
            if result == -1 {
                Err(io::Error::last_os_error().into())
            } else {
                Ok(result)
            }
        }
    }

    fn rpl_fcntl_dupfd_cloexec(&mut self, fd: RawFd, target: libc::c_int) -> Result<RawFd, FcntlError> {
        if self.have_dupfd_cloexec.unwrap_or(true) {
            unsafe {
                let result = libc::fcntl(fd, F_DUPFD_CLOEXEC, target);
                if result != -1 {
                    self.have_dupfd_cloexec = Some(true);
                    return Ok(result);
                }

                if io::Error::last_os_error().raw_os_error().unwrap() != EINVAL {
                    return Err(io::Error::last_os_error().into());
                }
            }
        }

        let result = self.rpl_fcntl_dupfd(fd, target)?;
        
        unsafe {
            let flags = libc::fcntl(result, F_GETFD);
            if flags == -1 {
                let saved_errno = io::Error::last_os_error();
                let _ = File::from_raw_fd(result);
                return Err(saved_errno.into());
            }

            if libc::fcntl(result, F_SETFD, flags | FD_CLOEXEC) == -1 {
                let saved_errno = io::Error::last_os_error();
                let _ = File::from_raw_fd(result);
                return Err(saved_errno.into());
            }
        }

        self.have_dupfd_cloexec = Some(false);
        Ok(result)
    }
}

pub fn rpl_fcntl(fd: RawFd, action: libc::c_int, arg: libc::c_int) -> Result<RawFd, FcntlError> {
    let mut state = FcntlState::new();
    state.rpl_fcntl(fd, action, arg)
}