use std::ffi::CString;
use std::ptr;
use std::io::Error;

pub type Pid = i32;

static mut REMOTE_DESCRIPTION: Option<CString> = None;

pub fn remote_setup() {}

pub fn remote_cleanup() {
    unsafe {
        REMOTE_DESCRIPTION = None;
    }
}

pub fn start_remote_job_p(_first_p: i32) -> i32 {
    0
}

pub fn start_remote_job(
    _argv: Vec<CString>,
    _envp: Vec<CString>,
    _stdin_fd: i32,
) -> Result<(bool, Pid, bool), Error> {
    Err(Error::from_raw_os_error(1))
}

pub fn remote_status(
    _block: bool,
) -> Result<(i32, i32, bool), Error> {
    Err(Error::from_raw_os_error(10))
}

pub fn block_remote_children() {}

pub fn unblock_remote_children() {}

pub fn remote_kill(_id: Pid, _sig: i32) -> Result<(), Error> {
    Err(Error::from_raw_os_error(1))
}