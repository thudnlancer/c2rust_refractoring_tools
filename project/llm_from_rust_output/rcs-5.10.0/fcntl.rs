use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::fs::File;
use std::io;

const F_DUPFD: libc::c_int = 0;
const F_DUPFD_CLOEXEC: libc::c_int = 1030;
const F_GETFD: libc::c_int = 1;
const F_SETFD: libc::c_int = 2;
const FD_CLOEXEC: libc::c_int = 1;

#[derive(Debug, Clone, Copy)]
pub enum FcntlCommand {
    DupFd,
    DupFdCloexec,
    GetFd,
    SetFd,
    Other(i32),
}

impl From<libc::c_int> for FcntlCommand {
    fn from(cmd: libc::c_int) -> Self {
        match cmd {
            F_DUPFD => FcntlCommand::DupFd,
            F_DUPFD_CLOEXEC => FcntlCommand::DupFdCloexec,
            F_GETFD => FcntlCommand::GetFd,
            F_SETFD => FcntlCommand::SetFd,
            _ => FcntlCommand::Other(cmd),
        }
    }
}

pub fn rpl_fcntl(fd: RawFd, command: FcntlCommand, arg: Option<i32>) -> io::Result<RawFd> {
    match command {
        FcntlCommand::DupFd => {
            let target = arg.ok_or_else(|| io::Error::from_raw_os_error(libc::EINVAL))?;
            rpl_fcntl_dupfd(fd, target)
        }
        FcntlCommand::DupFdCloexec => {
            let target = arg.ok_or_else(|| io::Error::from_raw_os_error(libc::EINVAL))?;
            rpl_fcntl_dupfd_cloexec(fd, target)
        }
        FcntlCommand::GetFd => {
            let flags = unsafe { libc::fcntl(fd, F_GETFD) };
            if flags == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(flags)
            }
        }
        FcntlCommand::SetFd => {
            let flags = arg.ok_or_else(|| io::Error::from_raw_os_error(libc::EINVAL))?;
            let result = unsafe { libc::fcntl(fd, F_SETFD, flags) };
            if result == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(result)
            }
        }
        FcntlCommand::Other(cmd) => {
            let result = match arg {
                Some(arg) => unsafe { libc::fcntl(fd, cmd, arg) },
                None => unsafe { libc::fcntl(fd, cmd) },
            };
            if result == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(result)
            }
        }
    }
}

fn rpl_fcntl_dupfd(fd: RawFd, target: RawFd) -> io::Result<RawFd> {
    let result = unsafe { libc::fcntl(fd, F_DUPFD, target) };
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}

fn rpl_fcntl_dupfd_cloexec(fd: RawFd, target: RawFd) -> io::Result<RawFd> {
    static HAVE_DUPFD_CLOEXEC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

    let have_dupfd_cloexec = HAVE_DUPFD_CLOEXEC.load(std::sync::atomic::Ordering::Relaxed);
    
    let result = if have_dupfd_cloexec >= 0 {
        let res = unsafe { libc::fcntl(fd, F_DUPFD_CLOEXEC, target) };
        if res >= 0 || io::Error::last_os_error().raw_os_error() != Some(libc::EINVAL) {
            HAVE_DUPFD_CLOEXEC.store(1, std::sync::atomic::Ordering::Relaxed);
            res
        } else {
            let res = rpl_fcntl_dupfd(fd, target)?;
            if res >= 0 {
                HAVE_DUPFD_CLOEXEC.store(-1, std::sync::atomic::Ordering::Relaxed);
            }
            res
        }
    } else {
        rpl_fcntl_dupfd(fd, target)?
    };

    if result >= 0 && HAVE_DUPFD_CLOEXEC.load(std::sync::atomic::Ordering::Relaxed) == -1 {
        let flags = unsafe { libc::fcntl(result, F_GETFD) };
        if flags == -1 {
            let saved_errno = io::Error::last_os_error();
            unsafe { libc::close(result) };
            return Err(saved_errno);
        }

        if unsafe { libc::fcntl(result, F_SETFD, flags | FD_CLOEXEC) } == -1 {
            let saved_errno = io::Error::last_os_error();
            unsafe { libc::close(result) };
            return Err(saved_errno);
        }
    }

    Ok(result)
}