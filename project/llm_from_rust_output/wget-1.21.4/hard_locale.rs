use std::ffi::CStr;
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn hard_locale(category: c_int) -> bool {
    let mut locale = [0u8; 257];
    
    let result = unsafe {
        libc::setlocale_null_r(
            category,
            locale.as_mut_ptr() as *mut libc::c_char,
            locale.len() as libc::size_t,
        )
    };
    
    if result != 0 {
        return false;
    }
    
    let c_str = unsafe { CStr::from_ptr(locale.as_ptr() as *const libc::c_char) };
    let locale_str = c_str.to_str().unwrap_or("");
    
    locale_str != "C" && locale_str != "POSIX"
}