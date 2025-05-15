use std::ffi::CString;

pub type intmax_t = i64;

pub fn imaxtostr(i: intmax_t) -> CString {
    let mut buf = Vec::with_capacity(24); // Enough for i64::MIN
    let mut n = i;

    if n == 0 {
        buf.push(b'0');
    } else {
        let negative = n < 0;
        if negative {
            n = -n;
        }

        while n > 0 {
            buf.push((b'0' + (n % 10) as u8) as i8 as u8);
            n /= 10;
        }

        if negative {
            buf.push(b'-');
        }

        buf.reverse();
    }

    unsafe { CString::from_vec_unchecked(buf) }
}