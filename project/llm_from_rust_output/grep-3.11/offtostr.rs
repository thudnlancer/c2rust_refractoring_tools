use std::ffi::CString;

pub type off_t = i64;

#[no_mangle]
pub extern "C" fn offtostr(i: off_t, buf: *mut libc::c_char) -> *mut libc::c_char {
    let mut s = if i < 0 {
        let mut num = -i;
        let mut digits = Vec::new();
        
        if num == 0 {
            digits.push(b'0');
        } else {
            while num != 0 {
                digits.push(b'0' + (num % 10) as u8);
                num /= 10;
            }
        }
        
        digits.push(b'-');
        digits.reverse();
        CString::new(digits).unwrap()
    } else {
        let mut num = i;
        let mut digits = Vec::new();
        
        if num == 0 {
            digits.push(b'0');
        } else {
            while num != 0 {
                digits.push(b'0' + (num % 10) as u8);
                num /= 10;
            }
        }
        
        digits.reverse();
        CString::new(digits).unwrap()
    };

    unsafe {
        if !buf.is_null() {
            std::ptr::copy_nonoverlapping(s.as_ptr(), buf, s.as_bytes().len() + 1);
            buf
        } else {
            std::ptr::null_mut()
        }
    }
}