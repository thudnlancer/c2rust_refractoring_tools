use std::os::unix::io::{AsRawFd, RawFd};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};

pub fn set_cloexec_flag(desc: RawFd, value: bool) -> Result<(), nix::Error> {
    let flags = fcntl(desc, FcntlArg::F_GETFD)?;
    let mut fd_flags = FdFlag::from_bits_truncate(flags);
    
    if value {
        fd_flags.insert(FdFlag::FD_CLOEXEC);
    } else {
        fd_flags.remove(FdFlag::FD_CLOEXEC);
    }
    
    fcntl(desc, FcntlArg::F_SETFD(fd_flags))?;
    Ok(())
}

pub fn dup_cloexec(fd: RawFd) -> Result<RawFd, nix::Error> {
    fcntl(fd, FcntlArg::F_DUPFD_CLOEXEC(0))
}