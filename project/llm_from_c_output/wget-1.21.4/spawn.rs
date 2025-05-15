//! POSIX spawn interface implementation in Rust

use libc::{pid_t, mode_t};
use std::os::unix::io::RawFd;
use std::ffi::{CString, OsStr};
use std::path::Path;
use std::ptr;
use std::mem;

/// Data structure to contain attributes for thread creation
#[repr(C)]
#[derive(Debug, Default)]
pub struct PosixSpawnattr {
    flags: i16,
    pgrp: pid_t,
    sd: libc::sigset_t,
    ss: libc::sigset_t,
    sp: libc::sched_param,
    policy: i32,
    pad: [i32; 16],
}

/// Data structure to contain file actions for spawn
#[repr(C)]
#[derive(Debug, Default)]
pub struct PosixSpawnFileActions {
    allocated: i32,
    used: i32,
    actions: *mut SpawnAction,
    pad: [i32; 16],
}

#[repr(C)]
#[derive(Debug)]
enum SpawnAction {
    Open {
        fd: RawFd,
        path: *const libc::c_char,
        oflag: i32,
        mode: mode_t,
    },
    Close {
        fd: RawFd,
    },
    Dup2 {
        fd: RawFd,
        newfd: RawFd,
    },
}

/// Flags for spawn attributes
pub const POSIX_SPAWN_RESETIDS: i16 = 0x01;
pub const POSIX_SPAWN_SETPGROUP: i16 = 0x02;
pub const POSIX_SPAWN_SETSIGDEF: i16 = 0x04;
pub const POSIX_SPAWN_SETSIGMASK: i16 = 0x08;
pub const POSIX_SPAWN_SETSCHEDPARAM: i16 = 0x10;
pub const POSIX_SPAWN_SETSCHEDULER: i16 = 0x20;
pub const POSIX_SPAWN_USEVFORK: i16 = 0x40;

/// Initialize spawn attributes with default values
pub fn posix_spawnattr_init(attr: &mut PosixSpawnattr) -> Result<(), i32> {
    *attr = PosixSpawnattr::default();
    Ok(())
}

/// Destroy spawn attributes
pub fn posix_spawnattr_destroy(attr: &mut PosixSpawnattr) -> Result<(), i32> {
    *attr = PosixSpawnattr::default();
    Ok(())
}

/// Set spawn attribute flags
pub fn posix_spawnattr_setflags(attr: &mut PosixSpawnattr, flags: i16) -> Result<(), i32> {
    attr.flags = flags;
    Ok(())
}

/// Get spawn attribute flags
pub fn posix_spawnattr_getflags(attr: &PosixSpawnattr) -> Result<i16, i32> {
    Ok(attr.flags)
}

/// Set process group for spawn
pub fn posix_spawnattr_setpgroup(attr: &mut PosixSpawnattr, pgroup: pid_t) -> Result<(), i32> {
    attr.pgrp = pgroup;
    Ok(())
}

/// Get process group from spawn attributes
pub fn posix_spawnattr_getpgroup(attr: &PosixSpawnattr) -> Result<pid_t, i32> {
    Ok(attr.pgrp)
}

/// Set signal mask for spawn
pub fn posix_spawnattr_setsigmask(
    attr: &mut PosixSpawnattr,
    sigmask: &libc::sigset_t,
) -> Result<(), i32> {
    attr.ss = *sigmask;
    Ok(())
}

/// Get signal mask from spawn attributes
pub fn posix_spawnattr_getsigmask(
    attr: &PosixSpawnattr,
    sigmask: &mut libc::sigset_t,
) -> Result<(), i32> {
    *sigmask = attr.ss;
    Ok(())
}

/// Initialize file actions with default values
pub fn posix_spawn_file_actions_init(
    file_actions: &mut PosixSpawnFileActions,
) -> Result<(), i32> {
    *file_actions = PosixSpawnFileActions::default();
    Ok(())
}

/// Destroy file actions
pub fn posix_spawn_file_actions_destroy(
    file_actions: &mut PosixSpawnFileActions,
) -> Result<(), i32> {
    // Free any allocated actions
    unsafe {
        if !file_actions.actions.is_null() {
            libc::free(file_actions.actions as *mut libc::c_void);
        }
    }
    *file_actions = PosixSpawnFileActions::default();
    Ok(())
}

/// Add open action to file actions
pub fn posix_spawn_file_actions_addopen(
    file_actions: &mut PosixSpawnFileActions,
    fd: RawFd,
    path: &Path,
    oflag: i32,
    mode: mode_t,
) -> Result<(), i32> {
    let path_c = CString::new(path.as_os_str().as_bytes()).map_err(|_| libc::EINVAL)?;
    
    // Allocate new action
    let action = Box::new(SpawnAction::Open {
        fd,
        path: path_c.into_raw(),
        oflag,
        mode,
    });
    
    // Add to actions list
    unsafe {
        let new_ptr = libc::realloc(
            file_actions.actions as *mut libc::c_void,
            (file_actions.used + 1) as usize * mem::size_of::<SpawnAction>(),
        ) as *mut SpawnAction;
        
        if new_ptr.is_null() {
            return Err(libc::ENOMEM);
        }
        
        file_actions.actions = new_ptr;
        ptr::write(file_actions.actions.add(file_actions.used as usize), *Box::into_raw(action));
        file_actions.used += 1;
    }
    
    Ok(())
}

/// Add close action to file actions
pub fn posix_spawn_file_actions_addclose(
    file_actions: &mut PosixSpawnFileActions,
    fd: RawFd,
) -> Result<(), i32> {
    let action = Box::new(SpawnAction::Close { fd });
    
    unsafe {
        let new_ptr = libc::realloc(
            file_actions.actions as *mut libc::c_void,
            (file_actions.used + 1) as usize * mem::size_of::<SpawnAction>(),
        ) as *mut SpawnAction;
        
        if new_ptr.is_null() {
            return Err(libc::ENOMEM);
        }
        
        file_actions.actions = new_ptr;
        ptr::write(file_actions.actions.add(file_actions.used as usize), *Box::into_raw(action));
        file_actions.used += 1;
    }
    
    Ok(())
}

/// Add dup2 action to file actions
pub fn posix_spawn_file_actions_adddup2(
    file_actions: &mut PosixSpawnFileActions,
    fd: RawFd,
    newfd: RawFd,
) -> Result<(), i32> {
    let action = Box::new(SpawnAction::Dup2 { fd, newfd });
    
    unsafe {
        let new_ptr = libc::realloc(
            file_actions.actions as *mut libc::c_void,
            (file_actions.used + 1) as usize * mem::size_of::<SpawnAction>(),
        ) as *mut SpawnAction;
        
        if new_ptr.is_null() {
            return Err(libc::ENOMEM);
        }
        
        file_actions.actions = new_ptr;
        ptr::write(file_actions.actions.add(file_actions.used as usize), *Box::into_raw(action));
        file_actions.used += 1;
    }
    
    Ok(())
}

/// Spawn a new process
pub fn posix_spawn(
    pid: &mut pid_t,
    path: &Path,
    file_actions: Option<&PosixSpawnFileActions>,
    attrp: Option<&PosixSpawnattr>,
    argv: &[&OsStr],
    envp: &[&OsStr],
) -> Result<(), i32> {
    // Convert arguments to C strings
    let path_c = CString::new(path.as_os_str().as_bytes()).map_err(|_| libc::EINVAL)?;
    
    let argv_c: Vec<CString> = argv
        .iter()
        .map(|s| CString::new(s.as_bytes()).map_err(|_| libc::EINVAL))
        .collect::<Result<_, _>>()?;
    
    let envp_c: Vec<CString> = envp
        .iter()
        .map(|s| CString::new(s.as_bytes()).map_err(|_| libc::EINVAL))
        .collect::<Result<_, _>>()?;
    
    // Prepare pointers
    let mut argv_ptrs: Vec<*const libc::c_char> = argv_c.iter().map(|s| s.as_ptr()).collect();
    argv_ptrs.push(ptr::null());
    
    let mut envp_ptrs: Vec<*const libc::c_char> = envp_c.iter().map(|s| s.as_ptr()).collect();
    envp_ptrs.push(ptr::null());
    
    // Call the actual spawn implementation
    let res = unsafe {
        libc::fork()
    };
    
    match res {
        -1 => Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL)),
        0 => {
            // Child process
            // Execute file actions
            if let Some(actions) = file_actions {
                for i in 0..actions.used {
                    unsafe {
                        let action = &*actions.actions.add(i as usize);
                        match action {
                            SpawnAction::Open { fd, path, oflag, mode } => {
                                let new_fd = libc::open(*path, *oflag, *mode);
                                if new_fd == -1 {
                                    libc::_exit(127);
                                }
                                if new_fd != *fd {
                                    if libc::dup2(new_fd, *fd) == -1 {
                                        libc::_exit(127);
                                    }
                                    libc::close(new_fd);
                                }
                            }
                            SpawnAction::Close { fd } => {
                                if libc::close(*fd) == -1 {
                                    libc::_exit(127);
                                }
                            }
                            SpawnAction::Dup2 { fd, newfd } => {
                                if libc::dup2(*fd, *newfd) == -1 {
                                    libc::_exit(127);
                                }
                            }
                        }
                    }
                }
            }
            
            // Apply attributes
            if let Some(attr) = attrp {
                if attr.flags & POSIX_SPAWN_RESETIDS != 0 {
                    if libc::setegid(libc::getgid()) == -1 || libc::seteuid(libc::getuid()) == -1 {
                        libc::_exit(127);
                    }
                }
                
                if attr.flags & POSIX_SPAWN_SETPGROUP != 0 {
                    if libc::setpgid(0, attr.pgrp) == -1 {
                        libc::_exit(127);
                    }
                }
                
                if attr.flags & POSIX_SPAWN_SETSIGMASK != 0 {
                    if libc::sigprocmask(libc::SIG_SETMASK, &attr.ss, ptr::null_mut()) == -1 {
                        libc::_exit(127);
                    }
                }
                
                if attr.flags & POSIX_SPAWN_SETSCHEDULER != 0 {
                    if libc::sched_setscheduler(0, attr.policy, &attr.sp) == -1 {
                        libc::_exit(127);
                    }
                } else if attr.flags & POSIX_SPAWN_SETSCHEDPARAM != 0 {
                    if libc::sched_setparam(0, &attr.sp) == -1 {
                        libc::_exit(127);
                    }
                }
            }
            
            // Execute the program
            libc::execve(
                path_c.as_ptr(),
                argv_ptrs.as_ptr() as *mut *mut libc::c_char,
                envp_ptrs.as_ptr() as *mut *mut libc::c_char,
            );
            
            // If we get here, execve failed
            libc::_exit(127);
        }
        child_pid => {
            // Parent process
            *pid = child_pid;
            Ok(())
        }
    }
}