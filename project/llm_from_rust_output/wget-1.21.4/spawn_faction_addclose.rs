use std::os::unix::io::RawFd;
use std::ptr;
use std::ffi::CString;

#[derive(Debug, Clone, Copy)]
pub enum SpawnAction {
    Close { fd: RawFd },
    Dup2 { fd: RawFd, newfd: RawFd },
    Open { fd: RawFd, path: CString, oflag: i32, mode: libc::mode_t },
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

    pub fn add_close(&mut self, fd: RawFd) -> Result<(), i32> {
        let maxfd = unsafe { libc::getdtablesize() };
        if fd < 0 || fd >= maxfd {
            return Err(libc::EBADF);
        }

        self.actions.push(SpawnAction::Close { fd });
        Ok(())
    }

    // Other action methods would be implemented similarly
}

// Helper functions for FFI compatibility would go here