use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::fs::File;
use std::io;

#[derive(Debug, Clone, Copy)]
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
                FcntlCommand::DupFdCloexec(target) => {
                    static mut HAVE_DUPFD_CLOEXEC: Option<bool> = None;
                    
                    let result = if HAVE_DUPFD_CLOEXEC != Some(false) {
                        let res = libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, target);
                        if res >= 0 || io::Error::last_os_error().raw_os_error() != Some(libc::EINVAL) {
                            res
                        } else {
                            HAVE_DUPFD_CLOEXEC = Some(false);
                            libc::fcntl(fd, libc::F_DUPFD, target)
                        }
                    } else {
                        libc::fcntl(fd, libc::F_DUPFD, target)
                    };

                    if result >= 0 && HAVE_DUPFD_CLOEXEC == Some(false) {
                        let flags = libc::fcntl(result, libc::F_GETFD);
                        if flags < 0 || libc::fcntl(result, libc::F_SETFD, flags | libc::FD_CLOEXEC) < 0 {
                            let saved_errno = io::Error::last_os_error();
                            let _ = libc::close(result);
                            return Err(saved_errno);
                        }
                    }
                    result
                }
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

            if res < 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(res)
            }
        }
    }
}

pub fn rpl_fcntl(fd: RawFd, command: FcntlCommand) -> io::Result<File> {
    let new_fd = command.execute(fd)?;
    Ok(unsafe { File::from_raw_fd(new_fd) })
}