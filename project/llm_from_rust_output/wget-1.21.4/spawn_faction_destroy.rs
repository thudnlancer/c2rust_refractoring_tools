use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum SpawnActionTag {
    Close = 0,
    Dup2 = 1,
    Open = 2,
    Chdir = 3,
    Fchdir = 4,
}

#[derive(Clone, Debug)]
pub enum SpawnAction {
    Close { fd: i32 },
    Dup2 { fd: i32, newfd: i32 },
    Open { fd: i32, path: CString, oflag: i32, mode: u32 },
    Chdir { path: CString },
    Fchdir { fd: i32 },
}

#[derive(Debug)]
pub struct PosixSpawnFileActions {
    actions: Vec<SpawnAction>,
}

impl PosixSpawnFileActions {
    pub fn destroy(self) -> i32 {
        // The Vec will automatically clean up its contents when dropped
        0
    }
}

// Helper function to convert Path to CString
fn path_to_cstring(path: &Path) -> Result<CString, std::ffi::NulError> {
    CString::new(path.as_os_str().as_bytes())
}