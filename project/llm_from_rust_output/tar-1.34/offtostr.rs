use std::ffi::CString;

pub type off_t = i64;

pub fn offtostr(i: off_t) -> CString {
    let mut buf = Vec::with_capacity(24); // Sufficient size for i64
    let mut num = i;

    if num == 0 {
        buf.push(b'0');
    } else {
        let is_negative = num < 0;
        if is_negative {
            num = -num;
        }

        while num > 0 {
            let digit = (num % 10) as u8;
            buf.push(b'0' + digit);
            num /= 10;
        }

        if is_negative {
            buf.push(b'-');
        }

        buf.reverse();
    }

    // Add null terminator
    buf.push(0);
    
    unsafe { CString::from_vec_unchecked(buf) }
}