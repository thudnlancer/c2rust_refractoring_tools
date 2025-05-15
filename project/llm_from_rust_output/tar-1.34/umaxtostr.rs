use std::ffi::CString;

pub type uintmax_t = u64;

#[no_mangle]
pub extern "C" fn umaxtostr(i: uintmax_t, buf: *mut libc::c_char) -> *mut libc::c_char {
    let mut num = i;
    let mut digits = Vec::new();

    if num == 0 {
        digits.push(b'0');
    } else {
        while num > 0 {
            digits.push((b'0' + (num % 10) as u8);
            num /= 10;
        }
    }

    let mut c_str = CString::new(digits.into_iter().rev().collect::<Vec<u8>>()).unwrap();
    unsafe {
        std::ptr::copy_nonoverlapping(c_str.as_ptr(), buf, c_str.as_bytes().len() + 1);
    }
    buf
}