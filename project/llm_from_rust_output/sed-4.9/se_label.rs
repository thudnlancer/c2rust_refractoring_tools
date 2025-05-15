use std::ffi::{CStr, CString};
use std::ptr;

#[repr(C)]
pub struct SelabelHandle;

#[repr(C)]
pub struct SelinuxOpt;

#[derive(Debug, Clone, Copy)]
pub enum SelabelError {
    Unsupported,
}

impl SelabelError {
    fn errno(&self) -> i32 {
        95 // ENOTSUP
    }
}

pub fn selabel_lookup(
    hnd: *mut SelabelHandle,
    context: *mut *mut libc::c_char,
    key: &CStr,
    file_type: i32,
) -> Result<(), SelabelError> {
    unsafe { *libc::__errno_location() = SelabelError::Unsupported.errno() };
    Err(SelabelError::Unsupported)
}

pub fn selabel_open(
    backend: i32,
    options: *const SelinuxOpt,
    nopt: u32,
) -> Result<*mut SelabelHandle, SelabelError> {
    unsafe { *libc::__errno_location() = SelabelError::Unsupported.errno() };
    Err(SelabelError::Unsupported)
}

pub fn selabel_close(hnd: *mut SelabelHandle) -> Result<(), SelabelError> {
    unsafe { *libc::__errno_location() = SelabelError::Unsupported.errno() };
    Err(SelabelError::Unsupported)
}