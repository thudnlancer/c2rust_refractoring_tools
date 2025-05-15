use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};

pub fn dup_safer_flag(fd: RawFd, flag: i32) -> Result<RawFd, nix::Error> {
    let cmd = if flag & 0o2000000 != 0 {
        FcntlArg::F_DUPFD_CLOEXEC(3)
    } else {
        FcntlArg::F_DUPFD(3)
    };
    
    fcntl(fd, cmd)
}