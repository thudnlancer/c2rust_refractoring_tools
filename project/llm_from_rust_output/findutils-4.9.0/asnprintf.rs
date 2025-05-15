use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};
use std::mem;

#[repr(C)]
struct VaListTag {
    gp_offset: u32,
    fp_offset: u32,
    overflow_arg_area: *mut c_void,
    reg_save_area: *mut c_void,
}

type VaList = [VaListTag; 1];

pub fn asnprintf(
    resultbuf: Option<&mut [u8]>,
    lengthp: Option<&mut usize>,
    format: &CStr,
    args: VaList,
) -> Option<CString> {
    unsafe {
        let mut result_ptr: *mut c_char = std::ptr::null_mut();
        let mut length = 0;
        let resultbuf_ptr = resultbuf.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr() as *mut c_char);
        let lengthp_ptr = lengthp.map_or(std::ptr::null_mut(), |l| l as *mut usize as *mut _);

        result_ptr = vasnprintf(
            resultbuf_ptr,
            lengthp_ptr,
            format.as_ptr(),
            mem::transmute(args),
        );

        if result_ptr.is_null() {
            None
        } else {
            Some(CString::from_raw(result_ptr))
        }
    }
}

extern "C" {
    fn vasnprintf(
        resultbuf: *mut c_char,
        lengthp: *mut usize,
        format: *const c_char,
        args: VaList,
    ) -> *mut c_char;
}