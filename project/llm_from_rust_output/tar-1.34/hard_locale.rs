use std::ffi::CStr;
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn hard_locale(category: c_int) -> bool {
    let mut locale = [0i8; 257];
    
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
    
    unsafe {
        let c_str = CStr::from_ptr(locale.as_ptr());
        let locale_str = c_str.to_str().unwrap_or("");
        locale_str != "C" && locale_str != "POSIX"
    }
}