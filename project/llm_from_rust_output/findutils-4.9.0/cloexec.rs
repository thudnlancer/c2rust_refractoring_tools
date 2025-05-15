use std::os::unix::io::{AsRawFd, RawFd};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};

pub fn set_cloexec_flag(desc: RawFd, value: bool) -> nix::Result<()> {
    let flags = fcntl(desc, FcntlArg::F_GETFD)?;
    let mut fd_flags = FdFlag::from_bits_truncate(flags);
    fd_flags.set(FdFlag::FD_CLOEXEC, value);
    fcntl(desc, FcntlArg::F_SETFD(fd_flags))?;
    Ok(())
}

pub fn dup_cloexec(fd: RawFd) -> nix::Result<RawFd> {
    fcntl(fd, FcntlArg::F_DUPFD_CLOEXEC(0))
}