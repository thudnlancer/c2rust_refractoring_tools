use nix::fcntl::{dup2, FcntlArg, FdFlag};
use std::os::unix::io::RawFd;

pub fn dup_safer_flag(fd: RawFd, flag: i32) -> nix::Result<RawFd> {
    let new_fd = if flag & 0o2000000 != 0 {
        dup2(fd, 3)?
    } else {
        dup2(fd, 0)?
    };
    Ok(new_fd)
}