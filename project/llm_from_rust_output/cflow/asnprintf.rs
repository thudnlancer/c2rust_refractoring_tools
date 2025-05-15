use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void, c_uint, c_ulong};
use std::ptr;
use std::mem;

type size_t = c_ulong;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct VaListTag {
    gp_offset: c_uint,
    fp_offset: c_uint,
    overflow_arg_area: *mut c_void,
    reg_save_area: *mut c_void,
}

type VaList = [VaListTag; 1];

extern "C" {
    fn vasnprintf(
        resultbuf: *mut c_char,
        lengthp: *mut size_t,
        format: *const c_char,
        args: VaList,
    ) -> *mut c_char;
}

pub fn asnprintf(
    resultbuf: Option<&mut [u8]>,
    lengthp: Option<&mut size_t>,
    format: &CStr,
    args: VaList,
) -> Option<CString> {
    let resultbuf_ptr = resultbuf.map_or(ptr::null_mut(), |b| b.as_mut_ptr() as *mut c_char);
    let lengthp_ptr = lengthp.map_or(ptr::null_mut(), |p| p as *mut size_t);

    let result = unsafe {
        vasnprintf(
            resultbuf_ptr,
            lengthp_ptr,
            format.as_ptr(),
            args,
        )
    };

    if result.is_null() {
        None
    } else {
        unsafe { Some(CString::from_raw(result)) }
    }
}