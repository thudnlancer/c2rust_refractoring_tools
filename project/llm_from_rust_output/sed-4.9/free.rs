use libc::{c_int, c_void};
use std::ptr;

pub fn rpl_free(p: *mut c_void) {
    let errno = unsafe { ptr::read(libc::__errno_location()) };
    unsafe { libc::free(p) };
    unsafe { ptr::write(libc::__errno_location(), errno) };
}