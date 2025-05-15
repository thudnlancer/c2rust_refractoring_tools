use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};
use std::ptr;

pub type ConvertFn = fn(*const c_char, *mut *mut c_char) -> c_double;

pub fn xstrtod(
    str: &CStr,
    ptr: Option<&mut *const c_char>,
    result: &mut c_double,
    convert: ConvertFn,
) -> bool {
    let mut terminator: *mut c_char = ptr::null_mut();
    let mut ok = true;
    
    unsafe {
        *libc::__errno_location() = 0;
    }
    
    let val = convert(str.as_ptr(), &mut terminator);
    
    if terminator == str.as_ptr() as *mut c_char 
        || ptr.is_none() && unsafe { *terminator } != 0 
    {
        ok = false;
    } else if val != 0.0 && unsafe { *libc::__errno_location() } == 34 {
        ok = false;
    }
    
    if let Some(p) = ptr {
        *p = terminator;
    }
    
    *result = val;
    ok
}