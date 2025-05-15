use std::ffi::CString;

pub type off_t = i64;

pub fn offtostr(i: off_t) -> CString {
    let mut buf = Vec::with_capacity(24); // Enough space for i64::MIN
    let mut num = i;

    if num < 0 {
        while num != 0 {
            buf.push(b'0' - (num % 10) as u8);
            num /= 10;
        }
        buf.push(b'-');
    } else {
        while num != 0 {
            buf.push(b'0' + (num % 10) as u8);
            num /= 10;
        }
    }

    buf.reverse();
    if buf.is_empty() {
        buf.push(b'0');
    }

    unsafe { CString::from_vec_unchecked(buf) }
}