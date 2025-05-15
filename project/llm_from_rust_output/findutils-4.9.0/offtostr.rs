use std::ffi::CString;

pub type off_t = i64;

pub fn offtostr(i: off_t) -> CString {
    let mut buf = Vec::with_capacity(24); // Sufficient for i64 max digits
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
    
    // SAFETY: We've only pushed ASCII digits and '-'
    unsafe { CString::from_vec_unchecked(buf) }
}