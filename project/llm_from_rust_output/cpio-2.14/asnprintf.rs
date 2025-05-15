use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void, c_uint, c_ulong};

pub type size_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VaListTag {
    pub gp_offset: c_uint,
    pub fp_offset: c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}

pub type VaList = [VaListTag; 1];

pub fn asnprintf(
    resultbuf: Option<&mut [u8]>,
    lengthp: Option<&mut size_t>,
    format: &CStr,
    args: VaList,
) -> Option<CString> {
    unsafe {
        let resultbuf_ptr = resultbuf.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr() as *mut c_char);
        let lengthp_ptr = lengthp.map_or(std::ptr::null_mut(), |p| p as *mut size_t);
        
        let result = vasnprintf(
            resultbuf_ptr,
            lengthp_ptr,
            format.as_ptr(),
            args.as_ptr() as *mut _,
        );
        
        if result.is_null() {
            None
        } else {
            Some(CString::from_raw(result))
        }
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