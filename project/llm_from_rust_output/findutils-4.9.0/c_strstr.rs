use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn c_strstr(
    haystack: *const libc::c_char,
    needle: *const libc::c_char,
) -> *mut libc::c_char {
    if haystack.is_null() || needle.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let haystack_cstr = CStr::from_ptr(haystack);
        let needle_cstr = CStr::from_ptr(needle);
        
        if let Some(found) = haystack_cstr.to_bytes().windows(needle_cstr.to_bytes().len())
            .position(|window| window == needle_cstr.to_bytes())
        {
            (haystack as *mut libc::c_char).add(found)
        } else {
            std::ptr::null_mut()
        }
    }
}