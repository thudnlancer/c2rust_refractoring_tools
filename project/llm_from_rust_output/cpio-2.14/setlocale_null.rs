use std::ffi::{CStr, CString};
use std::ptr;

pub type size_t = usize;

fn setlocale_null_androidfix(category: i32) -> Option<&'static CStr> {
    unsafe {
        let result = libc::setlocale(category, ptr::null());
        if result.is_null() {
            None
        } else {
            Some(CStr::from_ptr(result))
        }
    }
}

fn setlocale_null_unlocked(
    category: i32,
    buf: &mut [u8],
) -> Result<(), (i32, Option<usize>)> {
    let result = match setlocale_null_androidfix(category) {
        Some(s) => s,
        None => {
            if !buf.is_empty() {
                buf[0] = 0;
            }
            return Err((22, None));
        }
    };

    let bytes = result.to_bytes_with_nul();
    let required_len = bytes.len();

    if required_len <= buf.len() {
        buf[..required_len].copy_from_slice(bytes);
        Ok(())
    } else {
        if !buf.is_empty() {
            let copy_len = buf.len() - 1;
            buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
            buf[copy_len] = 0;
        }
        Err((34, Some(required_len)))
    }
}

#[no_mangle]
pub extern "C" fn setlocale_null_r(
    category: libc::c_int,
    buf: *mut libc::c_char,
    bufsize: size_t,
) -> libc::c_int {
    if buf.is_null() || bufsize == 0 {
        return 22;
    }

    let buf_slice = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, bufsize) };
    match setlocale_null_unlocked(category, buf_slice) {
        Ok(_) => 0,
        Err((code, _)) => code,
    }
}

#[no_mangle]
pub extern "C" fn setlocale_null(category: libc::c_int) -> *const libc::c_char {
    match setlocale_null_androidfix(category) {
        Some(s) => s.as_ptr(),
        None => ptr::null(),
    }
}