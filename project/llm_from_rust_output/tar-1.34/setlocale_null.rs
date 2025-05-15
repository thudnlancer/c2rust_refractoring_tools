use std::ffi::CStr;
use std::ptr;

pub fn setlocale_null_r(category: i32, buf: &mut [u8]) -> Result<(), i32> {
    let locale = match unsafe { CStr::from_ptr(libc::setlocale(category, ptr::null())) } {
        Ok(s) => s,
        Err(_) => {
            if !buf.is_empty() {
                buf[0] = 0;
            }
            return Err(22);
        }
    };

    let bytes = locale.to_bytes_with_nul();
    if bytes.len() <= buf.len() {
        buf[..bytes.len()].copy_from_slice(bytes);
        Ok(())
    } else {
        if !buf.is_empty() {
            let copy_len = buf.len() - 1;
            buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
            buf[copy_len] = 0;
        }
        Err(34)
    }
}

pub fn setlocale_null(category: i32) -> Option<&'static CStr> {
    unsafe { CStr::from_ptr(libc::setlocale(category, ptr::null()) }.ok()
}