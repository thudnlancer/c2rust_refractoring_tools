use std::os::unix::io::RawFd;
use std::ptr::NonNull;

pub type Mode = libc::mode_t;

#[derive(Debug, Clone, Copy)]
pub enum SpawnAction {
    Close { fd: RawFd },
    Dup2 { fd: RawFd, newfd: RawFd },
    Open { fd: RawFd, path: String, oflag: libc::c_int, mode: Mode },
    Chdir { path: String },
    Fchdir { fd: RawFd },
}

#[derive(Debug)]
pub struct PosixSpawnFileActions {
    allocated: usize,
    used: usize,
    actions: Vec<SpawnAction>,
}

impl PosixSpawnFileActions {
    pub fn new() -> Self {
        Self {
            allocated: 0,
            used: 0,
            actions: Vec::new(),
        }
    }

    fn ensure_capacity(&mut self) -> Result<(), libc::c_int> {
        if self.used == self.allocated {
            self.actions.reserve(1);
            self.allocated = self.actions.capacity();
            if self.actions.capacity() == self.used {
                return Err(libc::ENOMEM);
            }
        }
        Ok(())
    }

    pub fn add_dup2(&mut self, fd: RawFd, newfd: RawFd) -> Result<(), libc::c_int> {
        let maxfd = unsafe { libc::getdtablesize() };
        if fd < 0 || newfd < 0 || fd >= maxfd || newfd >= maxfd {
            return Err(libc::EBADF);
        }

        self.ensure_capacity()?;

        self.actions.push(SpawnAction::Dup2 { fd, newfd });
        self.used += 1;

        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn rpl_posix_spawn_file_actions_adddup2(
    file_actions: *mut PosixSpawnFileActions,
    fd: RawFd,
    newfd: RawFd,
) -> libc::c_int {
    if file_actions.is_null() {
        return libc::EINVAL;
    }

    unsafe {
        match (*file_actions).add_dup2(fd, newfd) {
            Ok(_) => 0,
            Err(e) => e,
        }
    }
}