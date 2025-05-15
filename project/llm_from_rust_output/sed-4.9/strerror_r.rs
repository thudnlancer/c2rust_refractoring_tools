use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::str;

pub type size_t = usize;

fn safe_copy(buf: &mut [c_char], msg: &[c_char]) -> c_int {
    let msg_len = msg.len();
    let buf_len = buf.len();
    let copy_len = if msg_len < buf_len {
        msg_len
    } else {
        buf_len.saturating_sub(1)
    };

    unsafe {
        ptr::copy_nonoverlapping(msg.as_ptr(), buf.as_mut_ptr(), copy_len);
        *buf.get_unchecked_mut(copy_len) = 0;
    }

    if msg_len < buf_len { 0 } else { 34 }
}

#[no_mangle]
pub extern "C" fn rpl_strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int {
    if buflen <= 1 {
        if !buf.is_null() && buflen != 0 {
            unsafe { *buf = 0 };
        }
        return 34;
    }

    let buf_slice = unsafe { std::slice::from_raw_parts_mut(buf, buflen) };
    buf_slice[0] = 0;

    let saved_errno = unsafe { *libc::__errno_location() };
    let mut ret = 0;

    ret = unsafe { libc::__xpg_strerror_r(errnum, buf, buflen) };

    if unsafe { *buf } == 0 {
        let mut stackbuf = [0 as c_char; 80];
        let errstring = unsafe {
            libc::strerror_r(
                errnum,
                stackbuf.as_mut_ptr(),
                std::mem::size_of_val(&stackbuf),
            )
        };

        ret = if !errstring.is_null() {
            let msg = unsafe { CStr::from_ptr(errstring) }.to_bytes();
            let msg_chars = unsafe { std::slice::from_raw_parts(msg.as_ptr() as *const c_char, msg.len()) };
            safe_copy(buf_slice, msg_chars)
        } else {
            unsafe { *libc::__errno_location() }
        };
    }

    if ret == 22 && unsafe { *buf } == 0 {
        let msg = format!("Unknown error {}", errnum);
        let msg_bytes = msg.as_bytes();
        let copy_len = std::cmp::min(msg_bytes.len(), buflen - 1);
        unsafe {
            ptr::copy_nonoverlapping(msg_bytes.as_ptr() as *const c_char, buf, copy_len);
            *buf.add(copy_len) = 0;
        }
    }

    unsafe { *libc::__errno_location() = saved_errno };
    ret
}