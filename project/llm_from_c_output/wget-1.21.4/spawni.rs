use std::ffi::{CString, CStr};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::os::unix::process::CommandExt;
use nix::unistd::{fork, ForkResult, execve, close, dup2, chdir, fchdir};
use nix::sys::signal::{sigprocmask, SigSet, Signal};
use nix::sys::wait::WaitStatus;
use nix::sched::{sched_setparam, sched_setscheduler, SchedParam, SchedPolicy};
use nix::unistd::{getuid, getgid, setpgid, seteuid, setegid};
use libc::{mode_t, O_LARGEFILE};
use std::ptr;
use std::mem;
use std::env;

const SPAWN_ERROR: i32 = 127;

pub fn spawni(
    pid: &mut i32,
    file: &str,
    file_actions: Option<&posix_spawn_file_actions_t>,
    attrp: Option<&posix_spawnattr_t>,
    argv: &[&str],
    envp: Option<&[&str]>,
    use_path: bool,
) -> Result<(), i32> {
    let flags = attrp.map_or(0, |a| a._flags);

    match unsafe { fork() }? {
        ForkResult::Parent { child } => {
            *pid = child.as_raw();
            Ok(())
        }
        ForkResult::Child => {
            // Set signal mask
            if flags & POSIX_SPAWN_SETSIGMASK != 0 {
                let ss = &attrp.unwrap()._ss;
                sigprocmask(SigSet::from_raw(ss), None).map_err(|_| SPAWN_ERROR)?;
            }

            // Set signal default actions
            if flags & POSIX_SPAWN_SETSIGDEF != 0 {
                let sd = &attrp.unwrap()._sd;
                for sig in 1..=Signal::SIGRTMAX as i32 {
                    if sd.contains(sig) {
                        nix::sys::signal::signal(sig, nix::sys::signal::SigHandler::SigDfl)
                            .map_err(|_| SPAWN_ERROR)?;
                    }
                }
            }

            // Set scheduling parameters
            #[cfg(any(target_os = "linux", target_os = "android"))]
            {
                if flags & (POSIX_SPAWN_SETSCHEDPARAM | POSIX_SPAWN_SETSCHEDULER) != 0 {
                    let sp = &attrp.unwrap()._sp;
                    if flags & POSIX_SPAWN_SETSCHEDULER != 0 {
                        sched_setscheduler(
                            0,
                            SchedPolicy::from(attrp.unwrap()._policy),
                            sp,
                        )
                        .map_err(|_| SPAWN_ERROR)?;
                    } else {
                        sched_setparam(0, sp).map_err(|_| SPAWN_ERROR)?;
                    }
                }
            }

            // Set process group
            if flags & POSIX_SPAWN_SETPGROUP != 0 {
                setpgid(0, attrp.unwrap()._pgrp).map_err(|_| SPAWN_ERROR)?;
            }

            // Reset IDs
            if flags & POSIX_SPAWN_RESETIDS != 0 {
                seteuid(getuid()).map_err(|_| SPAWN_ERROR)?;
                setegid(getgid()).map_err(|_| SPAWN_ERROR)?;
            }

            // Execute file actions
            if let Some(actions) = file_actions {
                for action in &actions._actions[..actions._used] {
                    match action.tag {
                        spawn_do_close => {
                            close(action.action.close_action.fd).map_err(|_| SPAWN_ERROR)?;
                        }
                        spawn_do_open => {
                            let fd = open(
                                action.action.open_action.path,
                                action.action.open_action.oflag | O_LARGEFILE,
                                action.action.open_action.mode,
                            )
                            .map_err(|_| SPAWN_ERROR)?;
                            
                            if fd != action.action.open_action.fd {
                                dup2(fd, action.action.open_action.fd)
                                    .map_err(|_| SPAWN_ERROR)?;
                                close(fd).map_err(|_| SPAWN_ERROR)?;
                            }
                        }
                        spawn_do_dup2 => {
                            dup2(
                                action.action.dup2_action.fd,
                                action.action.dup2_action.newfd,
                            )
                            .map_err(|_| SPAWN_ERROR)?;
                        }
                        spawn_do_chdir => {
                            chdir(action.action.chdir_action.path).map_err(|_| SPAWN_ERROR)?;
                        }
                        spawn_do_fchdir => {
                            fchdir(action.action.fchdir_action.fd).map_err(|_| SPAWN_ERROR)?;
                        }
                    }
                }
            }

            // Prepare argv and envp
            let c_argv: Vec<CString> = argv
                .iter()
                .map(|s| CString::new(*s).unwrap())
                .collect();
            let c_argv_ptr: Vec<*const libc::c_char> = c_argv
                .iter()
                .map(|s| s.as_ptr())
                .chain(Some(ptr::null()))
                .collect();

            let c_envp_ptr = if let Some(env) = envp {
                let c_env: Vec<CString> = env
                    .iter()
                    .map(|s| CString::new(*s).unwrap())
                    .collect();
                let ptrs: Vec<*const libc::c_char> = c_env
                    .iter()
                    .map(|s| s.as_ptr())
                    .chain(Some(ptr::null()))
                    .collect();
                ptrs.as_ptr()
            } else {
                ptr::null()
            };

            if !use_path || file.contains('/') {
                execve(
                    CString::new(file).unwrap().as_ptr(),
                    c_argv_ptr.as_ptr(),
                    c_envp_ptr,
                );
            } else {
                // Search PATH
                let path = env::var_os("PATH")
                    .unwrap_or_else(|| {
                        #[cfg(any(target_os = "linux", target_os = "macos"))]
                        {
                            let len = unsafe { libc::confstr(libc::_CS_PATH, ptr::null_mut(), 0) };
                            let mut buf = vec![0; len];
                            unsafe {
                                libc::confstr(libc::_CS_PATH, buf.as_mut_ptr(), len);
                            }
                            std::ffi::OsString::from_vec(buf)
                        }
                        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
                        {
                            std::ffi::OsString::from("")
                        }
                    });

                for dir in env::split_paths(&path) {
                    let mut path = dir.join(file);
                    if let Ok(c_path) = CString::new(path.as_os_str().as_bytes()) {
                        execve(c_path.as_ptr(), c_argv_ptr.as_ptr(), c_envp_ptr);
                        match nix::errno::errno() {
                            nix::errno::Errno::EACCES
                            | nix::errno::Errno::ENOENT
                            | nix::errno::Errno::ESTALE
                            | nix::errno::Errno::ENOTDIR => continue,
                            _ => break,
                        }
                    }
                }
            }

            unsafe { libc::_exit(SPAWN_ERROR) };
        }
    }
}

// Helper types to mirror the C structures
struct posix_spawn_file_actions_t {
    _actions: Vec<__spawn_action>,
    _used: usize,
}

struct __spawn_action {
    tag: SpawnActionTag,
    action: SpawnAction,
}

enum SpawnActionTag {
    spawn_do_close,
    spawn_do_open,
    spawn_do_dup2,
    spawn_do_chdir,
    spawn_do_fchdir,
}

union SpawnAction {
    close_action: CloseAction,
    open_action: OpenAction,
    dup2_action: Dup2Action,
    chdir_action: ChdirAction,
    fchdir_action: FchdirAction,
}

struct CloseAction {
    fd: i32,
}

struct OpenAction {
    path: *const libc::c_char,
    oflag: i32,
    mode: mode_t,
    fd: i32,
}

struct Dup2Action {
    fd: i32,
    newfd: i32,
}

struct ChdirAction {
    path: *const libc::c_char,
}

struct FchdirAction {
    fd: i32,
}

struct posix_spawnattr_t {
    _flags: i16,
    _pgrp: i32,
    _sd: SigSet,
    _ss: SigSet,
    _sp: SchedParam,
    _policy: i32,
}

const POSIX_SPAWN_RESETIDS: i16 = 1 << 0;
const POSIX_SPAWN_SETPGROUP: i16 = 1 << 1;
const POSIX_SPAWN_SETSIGDEF: i16 = 1 << 2;
const POSIX_SPAWN_SETSIGMASK: i16 = 1 << 3;
const POSIX_SPAWN_SETSCHEDPARAM: i16 = 1 << 4;
const POSIX_SPAWN_SETSCHEDULER: i16 = 1 << 5;
const POSIX_SPAWN_USEVFORK: i16 = 1 << 6;