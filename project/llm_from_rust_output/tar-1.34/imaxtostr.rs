use std::ffi::CString;

pub type intmax_t = i64;

pub fn imaxtostr(i: intmax_t) -> CString {
    let mut buf = Vec::with_capacity(24); // Enough space for i64::MIN
    let mut i = i;
    let negative = i < 0;

    if negative {
        i = -i;
    }

    if i == 0 {
        buf.push(b'0');
    } else {
        while i != 0 {
            buf.push((b'0' + (i % 10) as u8) as _);
            i /= 10;
        }
    }

    if negative {
        buf.push(b'-' as _);
    }

    buf.reverse();
    unsafe { CString::from_vec_unchecked(buf) }
}