//! Implementation of getopt_long and getopt_long_only functionality in Rust.

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

extern "C" {
    fn _getopt_internal(
        argc: c_int,
        argv: *const *const c_char,
        optstring: *const c_char,
        longopts: *const Option,
        longindex: *mut c_int,
        long_only: c_int,
    ) -> c_int;

    fn _getopt_internal_r(
        argc: c_int,
        argv: *const *const c_char,
        optstring: *const c_char,
        longopts: *const Option,
        longindex: *mut c_int,
        long_only: c_int,
        data: *mut getopt_data,
    ) -> c_int;
}

#[repr(C)]
pub struct getopt_data {
    // Internal fields would go here
    // This is a placeholder since the actual implementation details
    // of getopt_data aren't shown in the C code
}

pub fn getopt_long(
    argc: c_int,
    argv: *const *const c_char,
    options: *const c_char,
    long_options: *const Option,
    opt_index: *mut c_int,
) -> c_int {
    unsafe {
        _getopt_internal(argc, argv, options, long_options, opt_index, 0)
    }
}

pub fn _getopt_long_r(
    argc: c_int,
    argv: *const *const c_char,
    options: *const c_char,
    long_options: *const Option,
    opt_index: *mut c_int,
    data: *mut getopt_data,
) -> c_int {
    unsafe {
        _getopt_internal_r(argc, argv, options, long_options, opt_index, 0, data)
    }
}

pub fn getopt_long_only(
    argc: c_int,
    argv: *const *const c_char,
    options: *const c_char,
    long_options: *const Option,
    opt_index: *mut c_int,
) -> c_int {
    unsafe {
        _getopt_internal(argc, argv, options, long_options, opt_index, 1)
    }
}

pub fn _getopt_long_only_r(
    argc: c_int,
    argv: *const *const c_char,
    options: *const c_char,
    long_options: *const Option,
    opt_index: *mut c_int,
    data: *mut getopt_data,
) -> c_int {
    unsafe {
        _getopt_internal_r(argc, argv, options, long_options, opt_index, 1, data)
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
            CString::new("-c").unwrap(),
            CString::new("option").unwrap(),
        ];

        let argv: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
        let argc = argv.len() as c_int;

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

        // This is just a basic test to verify linking
        // Actual option parsing would need more comprehensive tests
        let result = getopt_long(
            argc,
            argv.as_ptr(),
            options.as_ptr(),
            long_options.as_ptr(),
            &mut opt_index,
        );

        assert_ne!(result, -1);
    }
}