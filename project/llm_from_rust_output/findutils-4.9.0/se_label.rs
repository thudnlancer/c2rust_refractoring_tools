use std::ffi::{CStr, CString};
use std::ptr;

#[repr(C)]
pub struct SelabelHandle;

#[repr(C)]
pub struct SelinuxOpt;

#[derive(Debug)]
pub enum SelabelError {
    Unsupported,
}

impl From<SelabelError> for i32 {
    fn from(err: SelabelError) -> Self {
        match err {
            SelabelError::Unsupported => -1,
        }
    }
}

pub fn selabel_lookup(
    hnd: *mut SelabelHandle,
    context: *mut *mut libc::c_char,
    key: &CStr,
    file_type: libc::c_int,
) -> Result<(), SelabelError> {
    unsafe { *libc::__errno_location() = 95 };
    Err(SelabelError::Unsupported)
}

pub fn selabel_open(
    backend: libc::c_int,
    options: *mut SelinuxOpt,
    nopt: libc::c_uint,
) -> Result<*mut SelabelHandle, SelabelError> {
    unsafe { *libc::__errno_location() = 95 };
    Ok(ptr::null_mut())
}

pub fn selabel_close(hnd: *mut SelabelHandle) -> Result<(), SelabelError> {
    unsafe { *libc::__errno_location() = 95 };
    Ok(())
}