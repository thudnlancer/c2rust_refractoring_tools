use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::prelude::OwnedFd;
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
    fn to_raw(self) -> (i32, Option<i32>) {
        match self {
            FcntlCommand::DupFd(fd) => (libc::F_DUPFD, Some(fd)),
            FcntlCommand::DupFdCloexec(fd) => (libc::F_DUPFD_CLOEXEC, Some(fd)),
            FcntlCommand::GetFd => (libc::F_GETFD, None),
            FcntlCommand::SetFd(arg) => (libc::F_SETFD, Some(arg)),
            FcntlCommand::GetFl => (libc::F_GETFL, None),
            FcntlCommand::SetFl(arg) => (libc::F_SETFL, Some(arg)),
            FcntlCommand::GetOwn => (libc::F_GETOWN, None),
            FcntlCommand::SetOwn(arg) => (libc::F_SETOWN, Some(arg)),
            FcntlCommand::GetSig => (libc::F_GETSIG, None),
            FcntlCommand::SetSig(arg) => (libc::F_SETSIG, Some(arg)),
            FcntlCommand::GetLease => (libc::F_GETLEASE, None),
            FcntlCommand::SetLease(arg) => (libc::F_SETLEASE, Some(arg)),
            FcntlCommand::Other(cmd, arg) => (cmd, arg),
        }
    }
}

pub fn fcntl(fd: RawFd, cmd: FcntlCommand) -> io::Result<OwnedFd> {
    let (cmd, arg) = cmd.to_raw();
    
    unsafe {
        let res = match arg {
            Some(arg) => libc::fcntl(fd, cmd, arg),
            None => libc::fcntl(fd, cmd),
        };
        
        if res == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(OwnedFd::from_raw_fd(res))
        }
    }
}

pub fn dupfd_cloexec(fd: RawFd, target: RawFd) -> io::Result<OwnedFd> {
    match fcntl(fd, FcntlCommand::DupFdCloexec(target)) {
        Ok(new_fd) => Ok(new_fd),
        Err(e) if e.raw_os_error() == Some(libc::EINVAL) => {
            // Fallback to dup + set cloexec manually
            let new_fd = fcntl(fd, FcntlCommand::DupFd(target))?;
            
            let flags = fcntl(new_fd.as_raw_fd(), FcntlCommand::GetFd)?;
            fcntl(new_fd.as_raw_fd(), FcntlCommand::SetFd(flags | libc::FD_CLOEXEC))?;
            
            Ok(new_fd)
        }
        Err(e) => Err(e),
    }
}

pub fn dupfd(fd: RawFd, target: RawFd) -> io::Result<OwnedFd> {
    fcntl(fd, FcntlCommand::DupFd(target))
}