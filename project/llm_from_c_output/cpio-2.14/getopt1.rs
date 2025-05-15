//! Implementation of getopt_long and getopt_long_only functionality in Rust.

use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_char};
use std::ptr;

#[repr(C)]
pub struct option {
    pub name: *const c_char,
    pub has_arg: c_int,
    pub flag: *mut c_int,
    pub val: c_int,
}

extern "C" {
    fn _getopt_internal(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
        long_only: c_int,
        posixly_correct: c_int,
    ) -> c_int;

    fn _getopt_internal_r(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
        long_only: c_int,
        data: *mut getopt_data,
        posixly_correct: c_int,
    ) -> c_int;
}

#[repr(C)]
pub struct getopt_data {
    // Internal fields would go here
    // (omitted as they're not used in the public interface)
}

pub fn getopt_long(
    argc: c_int,
    argv: *mut *mut c_char,
    options: *const c_char,
    long_options: *const option,
    opt_index: *mut c_int,
) -> c_int {
    unsafe {
        _getopt_internal(argc, argv, options, long_options, opt_index, 0, 0)
    }
}

pub fn _getopt_long_r(
    argc: c_int,
    argv: *mut *mut c_char,
    options: *const c_char,
    long_options: *const option,
    opt_index: *mut c_int,
    d: *mut getopt_data,
) -> c_int {
    unsafe {
        _getopt_internal_r(argc, argv, options, long_options, opt_index, 0, d, 0)
    }
}

pub fn getopt_long_only(
    argc: c_int,
    argv: *mut *mut c_char,
    options: *const c_char,
    long_options: *const option,
    opt_index: *mut c_int,
) -> c_int {
    unsafe {
        _getopt_internal(argc, argv, options, long_options, opt_index, 1, 0)
    }
}

pub fn _getopt_long_only_r(
    argc: c_int,
    argv: *mut *mut c_char,
    options: *const c_char,
    long_options: *const option,
    opt_index: *mut c_int,
    d: *mut getopt_data,
) -> c_int {
    unsafe {
        _getopt_internal_r(argc, argv, options, long_options, opt_index, 1, d, 0)
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

        let mut argv: Vec<*mut c_char> = args.iter().map(|arg| arg.as_ptr() as *mut c_char).collect();
        argv.push(ptr::null_mut());

        let options = CString::new("abc:d:0123456789").unwrap();
        let long_options = [
            option {
                name: CString::new("add").unwrap().into_raw(),
                has_arg: 1,
                flag: ptr::null_mut(),
                val: 0,
            },
            option {
                name: CString::new("append").unwrap().into_raw(),
                has_arg: 0,
                flag: ptr::null_mut(),
                val: 0,
            },
            option {
                name: ptr::null(),
                has_arg: 0,
                flag: ptr::null_mut(),
                val: 0,
            },
        ];

        let argc = (argv.len() - 1) as c_int;
        let mut opt_index = 0;

        unsafe {
            let c = getopt_long(
                argc,
                argv.as_mut_ptr(),
                options.as_ptr(),
                long_options.as_ptr(),
                &mut opt_index,
            );

            assert_eq!(c, 0);
            assert_eq!(opt_index, 0);

            // Clean up
            for opt in &long_options {
                if !opt.name.is_null() {
                    drop(CString::from_raw(opt.name as *mut c_char));
                }
            }
        }
    }
}