use std::ffi::CStr;
use std::io::{self, Write};
use std::os::raw::{c_char, c_int};

pub type zerror = u32;
pub const ZERROR_INVALID_RADIX: zerror = 5;
pub const ZERROR_NEGATIVE: zerror = 4;
pub const ZERROR_DIV_0: zerror = 3;
pub const ZERROR_0_DIV_0: zerror = 2;
pub const ZERROR_0_POW_0: zerror = 1;
pub const ZERROR_ERRNO_SET: zerror = 0;

extern "C" {
    fn zerror(desc: *mut *const c_char) -> zerror;
    static libzahl_error: c_int;
}

#[no_mangle]
pub extern "C" fn zperror(prefix: *const c_char) {
    let prefix_str = if !prefix.is_null() {
        unsafe { CStr::from_ptr(prefix).to_string_lossy() }
    } else {
        "".into()
    };

    let libzahl_error = unsafe { libzahl_error };
    if libzahl_error >= 0 {
        unsafe {
            let errno = libc::__errno_location();
            *errno = libzahl_error;
        }
        let prefix = if prefix_str.is_empty() {
            None
        } else {
            Some(prefix_str.as_ref())
        };
        io::Error::last_os_error().print_custom(prefix);
    } else {
        let mut desc_ptr: *const c_char = std::ptr::null();
        let error_code = unsafe { zerror(&mut desc_ptr) };
        
        let desc = if desc_ptr.is_null() {
            "Unknown error".to_string()
        } else {
            unsafe { CStr::from_ptr(desc_ptr).to_string_lossy().into_owned() }
        };

        let stderr = io::stderr();
        let mut handle = stderr.lock();
        if !prefix_str.is_empty() {
            writeln!(handle, "{}: {}", prefix_str, desc).unwrap();
        } else {
            writeln!(handle, "{}", desc).unwrap();
        }
    }
}

trait PrintCustom {
    fn print_custom(&self, prefix: Option<&str>);
}

impl PrintCustom for io::Error {
    fn print_custom(&self, prefix: Option<&str>) {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
        match prefix {
            Some(p) => writeln!(handle, "{}: {}", p, self).unwrap(),
            None => writeln!(handle, "{}", self).unwrap(),
        }
    }
}