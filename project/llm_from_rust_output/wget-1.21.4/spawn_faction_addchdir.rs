use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr;

type ModeT = c_uint;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
enum SpawnActionTag {
    Close = 0,
    Dup2 = 1,
    Open = 2,
    Chdir = 3,
    Fchdir = 4,
}

#[derive(Debug, Clone)]
enum SpawnAction {
    Close { fd: c_int },
    Dup2 { fd: c_int, newfd: c_int },
    Open { fd: c_int, path: CString, oflag: c_int, mode: ModeT },
    Chdir { path: CString },
    Fchdir { fd: c_int },
}

#[derive(Debug)]
struct PosixSpawnFileActions {
    actions: Vec<SpawnAction>,
}

impl PosixSpawnFileActions {
    fn new() -> Self {
        Self { actions: Vec::new() }
    }

    fn add_chdir(&mut self, path: &str) -> Result<(), i32> {
        let path_copy = match CString::new(path) {
            Ok(p) => p,
            Err(_) => return Err(12), // ENOMEM
        };

        self.actions.push(SpawnAction::Chdir { path: path_copy });
        Ok(())
    }

    fn ensure_capacity(&mut self) -> Result<(), i32> {
        // In Rust, Vec handles capacity automatically
        Ok(())
    }
}

fn posix_spawn_file_actions_addchdir(
    file_actions: &mut PosixSpawnFileActions,
    path: &str,
) -> c_int {
    match file_actions.add_chdir(path) {
        Ok(_) => 0,
        Err(e) => e,
    }
}