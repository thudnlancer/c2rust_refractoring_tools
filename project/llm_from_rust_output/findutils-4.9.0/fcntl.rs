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
            match *self {
                FcntlCommand::DupFd(target) => {
                    let res = libc::fcntl(fd, libc::F_DUPFD, target);
                    if res == -1 {
                        Err(io::Error::last_os_error())
                    } else {
                        Ok(res)
                    }
                }
                FcntlCommand::DupFdCloexec(target) => {
                    static mut HAVE_DUPFD_CLOEXEC: Option<bool> = None;
                    
                    let result = if let Some(true) = unsafe { HAVE_DUPFD_CLOEXEC } {
                        libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, target)
                    } else {
                        let res = libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, target);
                        if res != -1 {
                            unsafe { HAVE_DUPFD_CLOEXEC = Some(true) };
                            res
                        } else {
                            let err = io::Error::last_os_error();
                            if err.raw_os_error() == Some(libc::EINVAL) {
                                let res = libc::fcntl(fd, libc::F_DUPFD, target);
                                if res != -1 {
                                    unsafe { HAVE_DUPFD_CLOEXEC = Some(false) };
                                    res
                                } else {
                                    return Err(io::Error::last_os_error());
                                }
                            } else {
                                return Err(err);
                            }
                        }
                    };

                    if result != -1 && unsafe { HAVE_DUPFD_CLOEXEC == Some(false) } {
                        let flags = libc::fcntl(result, libc::F_GETFD);
                        if flags == -1 {
                            let saved_errno = io::Error::last_os_error();
                            let _ = libc::close(result);
                            return Err(saved_errno);
                        }
                        
                        if libc::fcntl(result, libc::F_SETFD, flags | libc::FD_CLOEXEC) == -1 {
                            let saved_errno = io::Error::last_os_error();
                            let _ = libc::close(result);
                            return Err(saved_errno);
                        }
                    }
                    Ok(result)
                }
                FcntlCommand::GetFd => {
                    let res = libc::fcntl(fd, libc::F_GETFD);
                    if res == -1 {
                        Err(io::Error::last_os_error())
                    } else {
                        Ok(res)
                    }
                }
                FcntlCommand::SetFd(arg) => {
                    let res = libc::fcntl(fd, libc::F_SETFD, arg);
                    if res == -1 {
                        Err(io::Error::last_os_error())
                    } else {
                        Ok(res)
                    }
                }
                FcntlCommand::GetFl => {
                    let res = libc::fcntl(fd, libc::F_GETFL);
                    if res == -1 {
                        Err(io::Error::last_os_error())
                    } else {
                        Ok(res)
                    }
                }
                FcntlCommand::SetFl(arg) => {
                    let res = libc::fcntl(fd, libc::F_SETFL, arg);
                    if res == -1 {
                        Err(io::Error::last_os_error())
                    } else {
                        Ok(res)
                    }
                }
                FcntlCommand::Other(cmd, Some(arg)) => {
                    let res = libc::fcntl(fd, cmd, arg);
                    if res == -1 {
                        Err(io::Error::last_os_error())
                    } else {
                        Ok(res)
                    }
                }
                FcntlCommand::Other(cmd, None) => {
                    let res = libc::fcntl(fd, cmd);
                    if res == -1 {
                        Err(io::Error::last_os_error())
                    } else {
                        Ok(res)
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
}

pub fn fcntl<F: AsRawFd>(fd: &F, cmd: FcntlCommand) -> io::Result<RawFd> {
    cmd.execute(fd.as_raw_fd())
}

pub fn dup<F: AsRawFd>(fd: &F) -> io::Result<File> {
    let new_fd = fcntl(fd, FcntlCommand::DupFd(0))?;
    Ok(unsafe { File::from_raw_fd(new_fd) })
}

pub fn dup_cloexec<F: AsRawFd>(fd: &F) -> io::Result<File> {
    let new_fd = fcntl(fd, FcntlCommand::DupFdCloexec(0))?;
    Ok(unsafe { File::from_raw_fd(new_fd) })
}