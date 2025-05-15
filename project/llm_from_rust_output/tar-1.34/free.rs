use libc::{c_int, c_void};

#[no_mangle]
pub extern "C" fn rpl_free(p: *mut c_void) {
    let err = unsafe { *libc::__errno_location() };
    unsafe { libc::free(p) };
    unsafe { *libc::__errno_location() = err };
}