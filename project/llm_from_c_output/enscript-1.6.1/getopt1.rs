//! getopt_long and getopt_long_only implementation in Rust

use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_char};
use std::ptr;

#[repr(C)]
pub struct Option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

extern {
    fn _getopt_internal(
        argc: c_int,
        argv: *const *const c_char,
        optstring: *const c_char,
        longopts: *const Option,
        longindex: *mut c_int,
        long_only: c_int,
    ) -> c_int;
}

pub fn getopt_long(
    argc: c_int,
    argv: &[*const c_char],
    options: &CStr,
    long_options: &[Option],
    opt_index: &mut c_int,
) -> c_int {
    unsafe {
        _getopt_internal(
            argc,
            argv.as_ptr(),
            options.as_ptr(),
            long_options.as_ptr(),
            opt_index,
            0,
        )
    }
}

pub fn getopt_long_only(
    argc: c_int,
    argv: &[*const c_char],
    options: &CStr,
    long_options: &[Option],
    opt_index: &mut c_int,
) -> c_int {
    unsafe {
        _getopt_internal(
            argc,
            argv.as_ptr(),
            options.as_ptr(),
            long_options.as_ptr(),
            opt_index,
            1,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_getopt_long() {
        let args = vec![
            CString::new("program").unwrap(),
            CString::new("--add").unwrap(),
            CString::new("value").unwrap(),
        ];
        let argv: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
        let options = CString::new("abc:d:0123456789").unwrap();
        let long_options = [
            Option {
                name: CString::new("add").unwrap().into_raw(),
                has_arg: 1,
                flag: ptr::null_mut(),
                val: 0,
            },
            Option {
                name: CString::new("append").unwrap().into_raw(),
                has_arg: 0,
                flag: ptr::null_mut(),
                val: 0,
            },
            Option {
                name: ptr::null(),
                has_arg: 0,
                flag: ptr::null_mut(),
                val: 0,
            },
        ];
        let mut opt_index = 0;

        let result = getopt_long(
            argv.len() as c_int,
            &argv,
            &options,
            &long_options,
            &mut opt_index,
        );

        // Clean up
        for opt in &long_options {
            if !opt.name.is_null() {
                drop(CString::from_raw(opt.name as *mut c_char));
            }
        }

        assert_eq!(result, 0);
    }
}