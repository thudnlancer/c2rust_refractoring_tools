use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};

pub fn set_cloexec_flag(desc: RawFd, value: bool) -> Result<(), nix::Error> {
    let flags = FdFlag::from_bits_truncate(fcntl(desc, FcntlArg::F_GETFD)?);
    let new_flags = if value {
        flags | FdFlag::FD_CLOEXEC
    } else {
        flags & !FdFlag::FD_CLOEXEC
    };
    
    if flags != new_flags {
        fcntl(desc, FcntlArg::F_SETFD(new_flags))?;
    }
    Ok(())
}

pub fn dup_cloexec(fd: RawFd) -> Result<RawFd, nix::Error> {
    fcntl(fd, FcntlArg::F_DUPFD_CLOEXEC(0))
}