use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::fs::File;
use std::io;

#[derive(Debug, Clone, Copy)]
pub enum FcntlCommand {
    DupFd(RawFd),
    DupFdCloexec(RawFd),
    GetFd,
    SetFd,
    GetFl,
    SetFl,
    GetOwn,
    SetOwn,
    GetSig,
    SetSig,
    GetLease,
    SetLease,
    Notify,
    Other(i32),
}

impl FcntlCommand {
    fn from_raw(cmd: i32) -> Self {
        match cmd {
            0 => FcntlCommand::DupFd(0),
            1030 => FcntlCommand::DupFdCloexec(0),
            1 => FcntlCommand::GetFd,
            2 => FcntlCommand::SetFd,
            3 => FcntlCommand::GetFl,
            4 => FcntlCommand::SetFl,
            8 => FcntlCommand::GetOwn,
            9 => FcntlCommand::SetOwn,
            10 => FcntlCommand::GetSig,
            11 => FcntlCommand::SetSig,
            1024 => FcntlCommand::GetLease,
            1025 => FcntlCommand::SetLease,
            1026 => FcntlCommand::Notify,
            _ => FcntlCommand::Other(cmd),
        }
    }
}

pub fn rpl_fcntl(fd: RawFd, command: FcntlCommand) -> io::Result<RawFd> {
    match command {
        FcntlCommand::DupFd(target) => dup_fd(fd, target),
        FcntlCommand::DupFdCloexec(target) => dup_fd_cloexec(fd, target),
        FcntlCommand::GetFd => get_fd(fd),
        FcntlCommand::SetFd => set_fd(fd),
        FcntlCommand::GetFl => get_fl(fd),
        FcntlCommand::SetFl => set_fl(fd),
        FcntlCommand::GetOwn => get_own(fd),
        FcntlCommand::SetOwn => set_own(fd),
        FcntlCommand::GetSig => get_sig(fd),
        FcntlCommand::SetSig => set_sig(fd),
        FcntlCommand::GetLease => get_lease(fd),
        FcntlCommand::SetLease => set_lease(fd),
        FcntlCommand::Notify => notify(fd),
        FcntlCommand::Other(cmd) => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported fcntl command")),
    }
}

fn dup_fd(fd: RawFd, target: RawFd) -> io::Result<RawFd> {
    let new_fd = unsafe { libc::fcntl(fd, libc::F_DUPFD, target) };
    if new_fd == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(new_fd)
    }
}

fn dup_fd_cloexec(fd: RawFd, target: RawFd) -> io::Result<RawFd> {
    let new_fd = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, target) };
    if new_fd == -1 {
        if io::Error::last_os_error().raw_os_error() == Some(libc::EINVAL) {
            let new_fd = dup_fd(fd, target)?;
            set_cloexec(new_fd)?;
            Ok(new_fd)
        } else {
            Err(io::Error::last_os_error())
        }
    } else {
        Ok(new_fd)
    }
}

fn set_cloexec(fd: RawFd) -> io::Result<()> {
    let flags = get_fl(fd)?;
    let new_flags = flags | libc::FD_CLOEXEC;
    unsafe {
        if libc::fcntl(fd, libc::F_SETFD, new_flags) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn get_fd(fd: RawFd) -> io::Result<RawFd> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if flags == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(flags)
    }
}

fn set_fd(fd: RawFd) -> io::Result<()> {
    let flags = get_fd(fd)?;
    unsafe {
        if libc::fcntl(fd, libc::F_SETFD, flags) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn get_fl(fd: RawFd) -> io::Result<i32> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if flags == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(flags)
    }
}

fn set_fl(fd: RawFd) -> io::Result<()> {
    let flags = get_fl(fd)?;
    unsafe {
        if libc::fcntl(fd, libc::F_SETFL, flags) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn get_own(fd: RawFd) -> io::Result<RawFd> {
    let pid = unsafe { libc::fcntl(fd, libc::F_GETOWN) };
    if pid == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(pid)
    }
}

fn set_own(fd: RawFd) -> io::Result<()> {
    let pid = get_own(fd)?;
    unsafe {
        if libc::fcntl(fd, libc::F_SETOWN, pid) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn get_sig(fd: RawFd) -> io::Result<RawFd> {
    let sig = unsafe { libc::fcntl(fd, libc::F_GETSIG) };
    if sig == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(sig)
    }
}

fn set_sig(fd: RawFd) -> io::Result<()> {
    let sig = get_sig(fd)?;
    unsafe {
        if libc::fcntl(fd, libc::F_SETSIG, sig) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn get_lease(fd: RawFd) -> io::Result<RawFd> {
    let lease = unsafe { libc::fcntl(fd, libc::F_GETLEASE) };
    if lease == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(lease)
    }
}

fn set_lease(fd: RawFd) -> io::Result<()> {
    let lease = get_lease(fd)?;
    unsafe {
        if libc::fcntl(fd, libc::F_SETLEASE, lease) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn notify(fd: RawFd) -> io::Result<()> {
    unsafe {
        if libc::fcntl(fd, libc::F_NOTIFY, 0) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}