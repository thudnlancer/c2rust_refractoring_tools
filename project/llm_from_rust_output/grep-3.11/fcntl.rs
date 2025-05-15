use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::fs::File;
use std::io;

#[derive(Debug, Copy, Clone)]
pub enum FcntlCommand {
    DupFd(RawFd),
    DupFdCloexec(RawFd),
    GetFd,
    SetFd(i32),
    GetFl,
    SetFl(i32),
    GetOwn,
    SetOwn(i32),
    GetSig,
    SetSig(i32),
    GetLease,
    SetLease(i32),
    Other(i32, Option<i32>),
}

impl FcntlCommand {
    pub fn execute(&self, fd: RawFd) -> io::Result<RawFd> {
        unsafe {
            let res = match *self {
                FcntlCommand::DupFd(target) => libc::fcntl(fd, libc::F_DUPFD, target),
                FcntlCommand::DupFdCloexec(target) => Self::dupfd_cloexec(fd, target)?,
                FcntlCommand::GetFd => libc::fcntl(fd, libc::F_GETFD),
                FcntlCommand::SetFd(arg) => libc::fcntl(fd, libc::F_SETFD, arg),
                FcntlCommand::GetFl => libc::fcntl(fd, libc::F_GETFL),
                FcntlCommand::SetFl(arg) => libc::fcntl(fd, libc::F_SETFL, arg),
                FcntlCommand::GetOwn => libc::fcntl(fd, libc::F_GETOWN),
                FcntlCommand::SetOwn(arg) => libc::fcntl(fd, libc::F_SETOWN, arg),
                FcntlCommand::GetSig => libc::fcntl(fd, libc::F_GETSIG),
                FcntlCommand::SetSig(arg) => libc::fcntl(fd, libc::F_SETSIG, arg),
                FcntlCommand::GetLease => libc::fcntl(fd, libc::F_GETLEASE),
                FcntlCommand::SetLease(arg) => libc::fcntl(fd, libc::F_SETLEASE, arg),
                FcntlCommand::Other(cmd, Some(arg)) => libc::fcntl(fd, cmd, arg),
                FcntlCommand::Other(cmd, None) => libc::fcntl(fd, cmd),
            };

            if res == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(res)
            }
        }
    }

    unsafe fn dupfd_cloexec(fd: RawFd, target: RawFd) -> io::Result<RawFd> {
        static mut HAVE_DUPFD_CLOEXEC: i32 = 0;

        if HAVE_DUPFD_CLOEXEC >= 0 {
            let res = libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, target);
            if res >= 0 || io::Error::last_os_error().raw_os_error() != Some(libc::EINVAL) {
                if res >= 0 {
                    HAVE_DUPFD_CLOEXEC = 1;
                }
                return if res == -1 {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(res)
                };
            } else {
                HAVE_DUPFD_CLOEXEC = -1;
            }
        }

        let res = libc::fcntl(fd, libc::F_DUPFD, target);
        if res == -1 {
            return Err(io::Error::last_os_error());
        }

        if HAVE_DUPFD_CLOEXEC == -1 {
            let flags = libc::fcntl(res, libc::F_GETFD);
            if flags == -1 
                || libc::fcntl(res, libc::F_SETFD, flags | libc::FD_CLOEXEC) == -1 
            {
                let saved_errno = io::Error::last_os_error();
                libc::close(res);
                return Err(saved_errno);
            }
        }

        Ok(res)
    }
}

pub fn fcntl<F: AsRawFd>(fd: &F, cmd: FcntlCommand) -> io::Result<File> {
    let raw_fd = cmd.execute(fd.as_raw_fd())?;
    Ok(unsafe { File::from_raw_fd(raw_fd) })
}