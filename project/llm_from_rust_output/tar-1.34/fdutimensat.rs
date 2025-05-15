use std::ffi::CStr;
use std::os::raw::{c_int, c_char, c_long};
use std::io;
use libc::{timespec, utimensat, futimens};

pub type TimeT = c_long;
pub type SyscallSLongT = c_long;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Timespec {
    pub tv_sec: TimeT,
    pub tv_nsec: SyscallSLongT,
}

#[no_mangle]
pub fn fdutimensat(
    fd: c_int,
    dir: c_int,
    file: Option<&CStr>,
    ts: *const Timespec,
    atflag: c_int,
) -> io::Result<()> {
    let mut result = Err(io::Error::from_raw_os_error(9));

    if fd >= 0 {
        result = unsafe { futimens(fd, ts as *const timespec) }
            .and_then(|_| Ok(()))
            .or_else(|e| {
                if e.raw_os_error() == Some(38) && file.is_some() {
                    None
                } else {
                    Some(e)
                }
            });
    }

    if let Some(file_path) = file {
        if fd < 0 || result.as_ref().err().map(|e| e.raw_os_error()) == Some(Some(38)) {
            result = unsafe { utimensat(dir, file_path.as_ptr(), ts as *const timespec, atflag) }
                .and_then(|_| Ok(()));
        }
    }

    result.or_else(|e| {
        if e.raw_os_error() == Some(9) {
            Err(io::Error::from_raw_os_error(9))
        } else {
            Err(e)
        }
    })
}