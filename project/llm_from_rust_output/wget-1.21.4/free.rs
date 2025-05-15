use libc::{c_int, c_void};
use std::ptr;

#[no_mangle]
pub extern "C" fn rpl_free(p: *mut c_void) {
    let err = unsafe { ptr::read(__errno_location()) };
    unsafe { libc::free(p) };
    unsafe { ptr::write(__errno_location(), err) };
}

extern "C" {
    fn __errno_location() -> *mut c_int;
}