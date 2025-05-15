use std::mem;
use std::ptr;
use std::ffi::CString;

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum SpawnActionType {
    Close = 0,
    Dup2 = 1,
    Open = 2,
    Chdir = 3,
    Fchdir = 4,
}

#[derive(Copy, Clone)]
pub union SpawnAction {
    close_action: CloseAction,
    dup2_action: Dup2Action,
    open_action: OpenAction,
    chdir_action: ChdirAction,
    fchdir_action: FchdirAction,
}

#[derive(Copy, Clone)]
pub struct CloseAction {
    pub fd: i32,
}

#[derive(Clone)]
pub struct ChdirAction {
    pub path: CString,
}

#[derive(Clone)]
pub struct OpenAction {
    pub fd: i32,
    pub path: CString,
    pub oflag: i32,
    pub mode: u32,
}

#[derive(Copy, Clone)]
pub struct Dup2Action {
    pub fd: i32,
    pub newfd: i32,
}

#[derive(Copy, Clone)]
pub struct FchdirAction {
    pub fd: i32,
}

pub struct PosixSpawnFileActions {
    allocated: usize,
    used: usize,
    actions: Vec<SpawnAction>,
}

impl PosixSpawnFileActions {
    pub fn new() -> Self {
        PosixSpawnFileActions {
            allocated: 0,
            used: 0,
            actions: Vec::new(),
        }
    }

    pub fn reallocate(&mut self) -> Result<(), i32> {
        let new_size = self.allocated + 8;
        self.actions.reserve(new_size);
        self.allocated = new_size;
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), i32> {
        self.allocated = 0;
        self.used = 0;
        self.actions.clear();
        Ok(())
    }
}