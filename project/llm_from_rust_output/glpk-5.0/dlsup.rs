use std::ffi::CStr;
use std::ptr;

#[no_mangle]
pub extern "C" fn _glp_dlopen(module: *const libc::c_char) -> *mut libc::c_void {
    assert!(!module.is_null(), "module pointer is null");
    unsafe {
        _glp_put_err_msg(
            CStr::from_bytes_with_nul_unchecked(b"Shared libraries not supported\0").as_ptr()
        );
    }
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn _glp_dlsym(h: *mut libc::c_void, symbol: *const libc::c_char) -> *mut libc::c_void {
    assert!(!h.is_null(), "handle pointer is null");
    assert!(!symbol.is_null(), "symbol pointer is null");
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn _glp_dlclose(h: *mut libc::c_void) {
    assert!(!h.is_null(), "handle pointer is null");
}

extern "C" {
    fn _glp_put_err_msg(msg: *const libc::c_char);
}