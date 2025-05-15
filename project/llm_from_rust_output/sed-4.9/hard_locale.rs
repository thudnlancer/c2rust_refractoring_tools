use std::ffi::CStr;
use std::ptr;

#[no_mangle]
pub extern "C" fn hard_locale(category: libc::c_int) -> bool {
    let mut locale: [libc::c_char; 257] = [0; 257];
    
    let result = unsafe {
        libc::setlocale_null_r(
            category,
            locale.as_mut_ptr(),
            std::mem::size_of_val(&locale) as libc::size_t,
        )
    };
    
    if result != 0 {
        return false;
    }
    
    let c_str = unsafe { CStr::from_ptr(locale.as_ptr()) };
    
    c_str.to_bytes() != b"C" && c_str.to_bytes() != b"POSIX"
}