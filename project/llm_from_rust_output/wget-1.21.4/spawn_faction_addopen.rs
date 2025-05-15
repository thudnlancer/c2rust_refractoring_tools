use std::ffi::{CString, CStr};
use std::os::unix::io::RawFd;
use std::ptr;
use libc::{mode_t, c_int};

#[derive(Debug, Clone, Copy)]
pub enum SpawnAction {
    Close { fd: RawFd },
    Dup2 { fd: RawFd, newfd: RawFd },
    Open { fd: RawFd, path: CString, oflag: c_int, mode: mode_t },
    Chdir { path: CString },
    Fchdir { fd: RawFd },
}

#[derive(Debug)]
pub struct PosixSpawnFileActions {
    actions: Vec<SpawnAction>,
}

impl PosixSpawnFileActions {
    pub fn new() -> Self {
        PosixSpawnFileActions {
            actions: Vec::new(),
        }
    }

    pub fn add_open(
        &mut self,
        fd: RawFd,
        path: &CStr,
        oflag: c_int,
        mode: mode_t,
    ) -> Result<(), i32> {
        let maxfd = unsafe { libc::getdtablesize() };
        if fd < 0 || fd >= maxfd {
            return Err(libc::EBADF);
        }

        let path_copy = match path.to_owned().into_string() {
            Ok(s) => CString::new(s).unwrap(),
            Err(_) => return Err(libc::ENOMEM),
        };

        self.actions.push(SpawnAction::Open {
            fd,
            path: path_copy,
            oflag,
            mode,
        });

        Ok(())
    }

    // Other action methods would follow similar patterns
}

#[no_mangle]
pub extern "C" fn rpl_posix_spawn_file_actions_addopen(
    file_actions: *mut PosixSpawnFileActions,
    fd: c_int,
    path: *const libc::c_char,
    oflag: c_int,
    mode: mode_t,
) -> c_int {
    if file_actions.is_null() || path.is_null() {
        return libc::EINVAL;
    }

    let path_cstr = unsafe { CStr::from_ptr(path) };
    let file_actions = unsafe { &mut *file_actions };

    match file_actions.add_open(fd, path_cstr, oflag, mode) {
        Ok(_) => 0,
        Err(e) => e,
    }
}