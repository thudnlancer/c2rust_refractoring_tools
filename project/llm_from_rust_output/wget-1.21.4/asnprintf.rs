use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void, c_uint, c_ulong};

pub type size_t = c_ulong;

pub fn asnprintf(
    resultbuf: Option<&mut [c_char]>,
    lengthp: Option<&mut size_t>,
    format: &CStr,
    args: &mut std::ffi::VaList,
) -> Option<CString> {
    let resultbuf_ptr = resultbuf.map_or(std::ptr::null_mut(), |buf| buf.as_mut_ptr());
    let lengthp_ptr = lengthp.map_or(std::ptr::null_mut(), |p| p as *mut size_t);

    let result_ptr = unsafe {
        vasnprintf(
            resultbuf_ptr,
            lengthp_ptr,
            format.as_ptr(),
            args.as_va_list(),
        )
    };

    if result_ptr.is_null() {
        None
    } else {
        Some(unsafe { CString::from_raw(result_ptr) })
    }
}

extern "C" {
    fn vasnprintf(
        resultbuf: *mut c_char,
        lengthp: *mut size_t,
        format: *const c_char,
        args: *mut c_void,
    ) -> *mut c_char;
}