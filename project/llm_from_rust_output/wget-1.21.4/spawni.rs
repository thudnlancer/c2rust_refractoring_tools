use std::env;
use std::ffi::{CString, CStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use nix::{
    sched::{sched_setparam, sched_setscheduler, SchedParam, Scheduler},
    sys::signal::{sigprocmask, SigSet, SigmaskHow, Signal},
    unistd::{fork, ForkResult, execve, setpgid, getuid, getgid, setegid, seteuid, chdir, fchdir, dup2, close},
    fcntl::{open, OFlag},
    sys::stat::Mode,
    errno::Errno,
};

#[derive(Debug, Clone)]
pub struct SpawnFileActions {
    actions: Vec<SpawnAction>,
}

#[derive(Debug, Clone)]
enum SpawnAction {
    Close { fd: i32 },
    Dup2 { fd: i32, newfd: i32 },
    Open { fd: i32, path: CString, oflag: OFlag, mode: Mode },
    Chdir { path: CString },
    Fchdir { fd: i32 },
}

#[derive(Debug, Clone)]
pub struct SpawnAttr {
    flags: SpawnFlags,
    pgrp: i32,
    sd: SigSet,
    ss: SigSet,
    sp: SchedParam,
    policy: Scheduler,
}

bitflags! {
    pub struct SpawnFlags: i16 {
        const RESETIDS = 0x01;
        const SETPGROUP = 0x02;
        const SETSIGDEF = 0x04;
        const SETSIGMASK = 0x08;
        const SETSCHEDPARAM = 0x10;
        const SETSCHEDULER = 0x20;
        const NOVFORK = 0x40;
    }
}

pub fn posix_spawn_internal(
    pid: Option<&mut i32>,
    file: &Path,
    file_actions: Option<&SpawnFileActions>,
    attrp: Option<&SpawnAttr>,
    argv: &[CString],
    envp: &[CString],
    use_path: bool,
) -> Result<(), Errno> {
    let flags = attrp.map(|a| a.flags).unwrap_or(SpawnFlags::empty());
    
    let new_pid = if flags.contains(SpawnFlags::NOVFORK) || 
        (flags & (SpawnFlags::SETSIGMASK | SpawnFlags::SETSIGDEF | 
                  SpawnFlags::SETSCHEDPARAM | SpawnFlags::SETSCHEDULER | 
                  SpawnFlags::SETPGROUP | SpawnFlags::RESETIDS)).is_empty() && 
        file_actions.is_none() 
    {
        unsafe { libc::vfork() }
    } else {
        match fork()? {
            ForkResult::Parent { child } => child.as_raw(),
            ForkResult::Child => {
                child_process(file, file_actions, attrp, argv, envp, use_path)?;
                unsafe { libc::_exit(127) };
            }
        }
    };

    if let Some(p) = pid {
        *p = new_pid;
    }
    Ok(())
}

fn child_process(
    file: &Path,
    file_actions: Option<&SpawnFileActions>,
    attrp: Option<&SpawnAttr>,
    argv: &[CString],
    envp: &[CString],
    use_path: bool,
) -> Result<(), Errno> {
    let flags = attrp.map(|a| a.flags).unwrap_or(SpawnFlags::empty());
    
    if flags.contains(SpawnFlags::SETSIGMASK) {
        sigprocmask(SigmaskHow::SIG_SETMASK, Some(&attrp.unwrap().ss), None)?;
    }

    if flags.contains(SpawnFlags::SETSIGDEF) {
        let sa = Signal::empty();
        for sig in Signal::iterator() {
            if attrp.unwrap().sd.contains(sig) {
                unsafe {
                    libc::signal(sig as i32, libc::SIG_DFL);
                }
            }
        }
    }

    if flags.contains(SpawnFlags::SETSCHEDPARAM) && !flags.contains(SpawnFlags::SETSCHEDULER) {
        sched_setparam(0, &attrp.unwrap().sp)?;
    } else if flags.contains(SpawnFlags::SETSCHEDULER) {
        sched_setscheduler(
            0,
            attrp.unwrap().policy,
            if flags.contains(SpawnFlags::SETSCHEDPARAM) {
                &attrp.unwrap().sp
            } else {
                &SchedParam::new(0)
            },
        )?;
    }

    if flags.contains(SpawnFlags::SETPGROUP) {
        setpgid(0, attrp.unwrap().pgrp)?;
    }

    if flags.contains(SpawnFlags::RESETIDS) {
        seteuid(getuid())?;
        setegid(getgid())?;
    }

    if let Some(actions) = file_actions {
        for action in &actions.actions {
            match action {
                SpawnAction::Close { fd } => close(*fd)?,
                SpawnAction::Dup2 { fd, newfd } => dup2(*fd, *newfd)?,
                SpawnAction::Open { fd, path, oflag, mode } => {
                    let new_fd = open(path, *oflag, *mode)?;
                    if new_fd != *fd {
                        dup2(new_fd, *fd)?;
                        close(new_fd)?;
                    }
                },
                SpawnAction::Chdir { path } => chdir(path)?,
                SpawnAction::Fchdir { fd } => fchdir(*fd)?,
            }
        }
    }

    if !use_path || file.as_os_str().as_bytes().contains(&b'/') {
        execve(file, argv, envp)?;
        return Err(Errno::last());
    }

    let path = env::var_os("PATH")
        .map(|p| p.into_string().unwrap())
        .unwrap_or_else(|| {
            let len = unsafe { libc::confstr(libc::_CS_PATH, std::ptr::null_mut(), 0) };
            let mut buf = vec![0; len];
            unsafe { libc::confstr(libc::_CS_PATH, buf.as_mut_ptr() as *mut libc::c_char, len) };
            CStr::from_bytes_with_nul(&buf).unwrap().to_str().unwrap().to_string()
        });

    for dir in path.split(':') {
        let full_path = Path::new(dir).join(file);
        match execve(&full_path, argv, envp) {
            Ok(_) => break,
            Err(e) => match e {
                Errno::EACCES | Errno::ENOENT | Errno::ENOTDIR | Errno::ELOOP => continue,
                _ => return Err(e),
            },
        }
    }

    Err(Errno::last())
}