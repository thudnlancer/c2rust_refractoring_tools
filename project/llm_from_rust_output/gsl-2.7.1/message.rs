use std::ffi::CStr;

static GSL_MESSAGE_MASK: u32 = 0xffffffff;

pub fn gsl_message(reason: &CStr, file: &CStr, line: i32, mask: u32) {
    if mask & GSL_MESSAGE_MASK != 0 {
        let label = CStr::from_bytes_with_nul(b"MESSAGE\0").unwrap();
        unsafe {
            gsl_stream_printf(
                label.as_ptr(),
                file.as_ptr(),
                line,
                reason.as_ptr(),
            );
        }
    }
}

extern "C" {
    fn gsl_stream_printf(
        label: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        reason: *const libc::c_char,
    );
}