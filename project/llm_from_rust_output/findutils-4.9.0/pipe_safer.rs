use std::os::unix::io::{FromRawFd, RawFd};
use std::process;
use nix::unistd::{pipe, close};
use nix::errno::Errno;

pub fn pipe_safer() -> Result<(RawFd, RawFd), Errno> {
    let (fd0, fd1) = pipe()?;
    
    let fd0 = match fd_safer(fd0) {
        Ok(fd) => fd,
        Err(e) => {
            let _ = close(fd1);
            return Err(e);
        }
    };
    
    let fd1 = match fd_safer(fd1) {
        Ok(fd) => fd,
        Err(e) => {
            let _ = close(fd0);
            return Err(e);
        }
    };
    
    Ok((fd0, fd1))
}

fn fd_safer(fd: RawFd) -> Result<RawFd, Errno> {
    if fd < 0 {
        return Err(Errno::EBADF);
    }
    
    // In a real implementation, this would contain the actual fd_safer logic
    // that ensures the fd is safe to use (e.g. setting FD_CLOEXEC)
    Ok(fd)
}