use std::ffi::CStr;
use std::os::raw::c_int;

extern "C" {
    fn locale_charset() -> *const libc::c_char;
    fn setlocale(category: c_int, locale: *const libc::c_char) -> *mut libc::c_char;
}

fn main() {
    let result = unsafe {
        setlocale(libc::LC_ALL, b"\0".as_ptr() as *const libc::c_char);
        let charset_ptr = locale_charset();
        if !charset_ptr.is_null() {
            CStr::from_ptr(charset_ptr).to_string_lossy().into_owned()
        } else {
            String::from("unknown")
        }
    };

    println!("{}", result);
}