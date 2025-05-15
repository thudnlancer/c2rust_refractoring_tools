use std::ffi::{CString, CStr, VaList};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::io::Error;

#[repr(C)]
pub struct VaListTag {
    gp_offset: u32,
    fp_offset: u32,
    overflow_arg_area: *mut std::ffi::c_void,
    reg_save_area: *mut std::ffi::c_void,
}

pub type VaListImpl = [VaListTag; 1];

pub fn rpl_sprintf(str: &mut [u8], format: &CStr, args: VaList) -> Result<i32, Error> {
    let mut lenbuf = std::usize::MAX.min(i32::MAX as usize);
    lenbuf = lenbuf.min(!(str.as_ptr() as usize));

    let output = unsafe {
        let mut len = 0;
        let output_ptr = libc::vasnprintf(
            str.as_mut_ptr() as *mut c_char,
            &mut len,
            format.as_ptr(),
            args.as_va_list(),
        );

        if output_ptr.is_null() {
            return Err(Error::last_os_error());
        }

        if output_ptr != str.as_mut_ptr() as *mut c_char {
            libc::free(output_ptr as *mut std::ffi::c_void);
            return Err(Error::from_raw_os_error(75));
        }

        if len > i32::MAX as usize {
            return Err(Error::from_raw_os_error(75));
        }

        len as i32
    };

    Ok(output)
}