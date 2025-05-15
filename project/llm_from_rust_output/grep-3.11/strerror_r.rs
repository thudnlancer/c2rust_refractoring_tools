use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::str::Utf8Error;

const ERANGE: c_int = 34;
const EINVAL: c_int = 22;
const UNKNOWN_ERROR_MSG: &str = "Unknown error";

fn safe_copy(buf: &mut [c_char], msg: &CStr) -> Result<(), c_int> {
    let msg_bytes = msg.to_bytes_with_nul();
    if msg_bytes.len() > buf.len() {
        let copy_len = buf.len() - 1;
        unsafe {
            ptr::copy_nonoverlapping(msg_bytes.as_ptr() as *const c_char, buf.as_mut_ptr(), copy_len);
            *buf.get_unchecked_mut(copy_len) = 0;
        }
        Err(ERANGE)
    } else {
        unsafe {
            ptr::copy_nonoverlapping(msg_bytes.as_ptr() as *const c_char, buf.as_mut_ptr(), msg_bytes.len());
        }
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn rpl_strerror_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> c_int {
    if buflen <= 1 {
        if !buf.is_null() && buflen != 0 {
            unsafe { *buf = 0 };
        }
        return ERANGE;
    }

    let buf_slice = unsafe { std::slice::from_raw_parts_mut(buf, buflen) };
    buf_slice[0] = 0;

    let saved_errno = unsafe { *libc::__errno_location() };

    let mut stackbuf = [0; 80];
    let ret = unsafe {
        libc::__xpg_strerror_r(errnum, buf, buflen)
    };

    if ret == 0 && buf_slice[0] == 0 {
        let errstring = unsafe {
            let ptr = libc::strerror_r(errnum, stackbuf.as_mut_ptr(), stackbuf.len());
            CStr::from_ptr(ptr)
        };

        match safe_copy(buf_slice, &errstring) {
            Ok(()) => 0,
            Err(e) => e,
        }
    } else if ret == EINVAL && buf_slice[0] == 0 {
        let msg = format!("{} {}", UNKNOWN_ERROR_MSG, errnum);
        if let Ok(c_msg) = CStr::from_bytes_with_nul(msg.as_bytes()) {
            match safe_copy(buf_slice, c_msg) {
                Ok(()) => 0,
                Err(e) => e,
            }
        } else {
            EINVAL
        }
    } else {
        ret
    };

    unsafe { *libc::__errno_location() = saved_errno };
    ret
}