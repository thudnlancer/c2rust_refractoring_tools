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
    GetPipesize,
    SetPipesize(i32),
    Other(i32, Option<i32>),
}

impl FcntlCommand {
    pub fn from_raw(command: i32, arg: Option<i32>) -> Self {
        match command {
            0 => FcntlCommand::DupFd(arg.unwrap()),
            1030 => FcntlCommand::DupFdCloexec(arg.unwrap()),
            1 => FcntlCommand::GetFd,
            2 => FcntlCommand::SetFd(arg.unwrap()),
            3 => FcntlCommand::GetFl,
            4 => FcntlCommand::SetFl(arg.unwrap()),
            5 => FcntlCommand::GetOwn,
            6 => FcntlCommand::SetOwn(arg.unwrap()),
            7 => FcntlCommand::GetSig,
            8 => FcntlCommand::SetSig(arg.unwrap()),
            9 => FcntlCommand::GetLease,
            10 => FcntlCommand::SetLease(arg.unwrap()),
            11 => FcntlCommand::GetPipesize,
            12 => FcntlCommand::SetPipesize(arg.unwrap()),
            _ => FcntlCommand::Other(command, arg),
        }
    }
}

pub fn rpl_fcntl(fd: RawFd, command: FcntlCommand) -> io::Result<RawFd> {
    match command {
        FcntlCommand::DupFd(target) => dup_fd(fd, target),
        FcntlCommand::DupFdCloexec(target) => dup_fd_cloexec(fd, target),
        FcntlCommand::GetFd => get_fd(fd),
        FcntlCommand::SetFd(arg) => set_fd(fd, arg).map(|_| 0),
        FcntlCommand::GetFl => get_fl(fd),
        FcntlCommand::SetFl(arg) => set_fl(fd, arg).map(|_| 0),
        FcntlCommand::GetOwn => get_own(fd),
        FcntlCommand::SetOwn(arg) => set_own(fd, arg).map(|_| 0),
        FcntlCommand::GetSig => get_sig(fd),
        FcntlCommand::SetSig(arg) => set_sig(fd, arg).map(|_| 0),
        FcntlCommand::GetLease => get_lease(fd),
        FcntlCommand::SetLease(arg) => set_lease(fd, arg).map(|_| 0),
        FcntlCommand::GetPipesize => get_pipesize(fd),
        FcntlCommand::SetPipesize(arg) => set_pipesize(fd, arg).map(|_| 0),
        FcntlCommand::Other(cmd, arg) => {
            let res = unsafe {
                match arg {
                    Some(a) => libc::fcntl(fd, cmd, a),
                    None => libc::fcntl(fd, cmd),
                }
            };
            if res == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(res)
            }
        }
    }
}

fn dup_fd(fd: RawFd, target: RawFd) -> io::Result<RawFd> {
    let res = unsafe { libc::fcntl(fd, libc::F_DUPFD, target) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

static HAVE_DUPFD_CLOEXEC: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

fn dup_fd_cloexec(fd: RawFd, target: RawFd) -> io::Result<RawFd> {
    let have_dupfd_cloexec = HAVE_DUPFD_CLOEXEC.load(std::sync::atomic::Ordering::Relaxed);
    
    if have_dupfd_cloexec >= 0 {
        let res = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, target) };
        if res >= 0 {
            HAVE_DUPFD_CLOEXEC.store(1, std::sync::atomic::Ordering::Relaxed);
            return Ok(res);
        }
        
        if io::Error::last_os_error().raw_os_error() != Some(libc::EINVAL) {
            return Err(io::Error::last_os_error());
        }
    }

    let result = dup_fd(fd, target)?;
    
    if have_dupfd_cloexec >= 0 {
        HAVE_DUPFD_CLOEXEC.store(-1, std::sync::atomic::Ordering::Relaxed);
    }

    let flags = get_fd(result)?;
    set_fd(result, flags | libc::FD_CLOEXEC)?;
    Ok(result)
}

fn get_fd(fd: RawFd) -> io::Result<RawFd> {
    let res = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

fn set_fd(fd: RawFd, arg: i32) -> io::Result<()> {
    let res = unsafe { libc::fcntl(fd, libc::F_SETFD, arg) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn get_fl(fd: RawFd) -> io::Result<RawFd> {
    let res = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

fn set_fl(fd: RawFd, arg: i32) -> io::Result<()> {
    let res = unsafe { libc::fcntl(fd, libc::F_SETFL, arg) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn get_own(fd: RawFd) -> io::Result<RawFd> {
    let res = unsafe { libc::fcntl(fd, libc::F_GETOWN) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

fn set_own(fd: RawFd, arg: i32) -> io::Result<()> {
    let res = unsafe { libc::fcntl(fd, libc::F_SETOWN, arg) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn get_sig(fd: RawFd) -> io::Result<RawFd> {
    let res = unsafe { libc::fcntl(fd, libc::F_GETSIG) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

fn set_sig(fd: RawFd, arg: i32) -> io::Result<()> {
    let res = unsafe { libc::fcntl(fd, libc::F_SETSIG, arg) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn get_lease(fd: RawFd) -> io::Result<RawFd> {
    let res = unsafe { libc::fcntl(fd, libc::F_GETLEASE) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

fn set_lease(fd: RawFd, arg: i32) -> io::Result<()> {
    let res = unsafe { libc::fcntl(fd, libc::F_SETLEASE, arg) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn get_pipesize(fd: RawFd) -> io::Result<RawFd> {
    let res = unsafe { libc::fcntl(fd, libc::F_GETPIPE_SZ) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(res)
    }
}

fn set_pipesize(fd: RawFd, arg: i32) -> io::Result<()> {
    let res = unsafe { libc::fcntl(fd, libc::F_SETPIPE_SZ, arg) };
    if res == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}