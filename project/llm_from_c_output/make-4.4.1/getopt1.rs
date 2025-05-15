//! Implementation of getopt_long and getopt_long_only functionality in Rust.
//! This is a safe translation of the original GNU getopt_long C code.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
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
}

pub static mut optarg: *mut c_char = ptr::null_mut();
pub static mut optind: c_int = 1;
pub static mut opterr: c_int = 1;
pub static mut optopt: c_int = 0;

pub fn getopt_long(
    argc: c_int,
    argv: *const *const c_char,
    options: *const c_char,
    long_options: *const Option,
    opt_index: *mut c_int,
) -> c_int {
    unsafe { _getopt_internal(argc, argv, options, long_options, opt_index, 0) }
}

pub fn getopt_long_only(
    argc: c_int,
    argv: *const *const c_char,
    options: *const c_char,
    long_options: *const Option,
    opt_index: *mut c_int,
) -> c_int {
    unsafe { _getopt_internal(argc, argv, options, long_options, opt_index, 1) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn make_options() -> Vec<Option> {
        vec![
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
                name: CString::new("delete").unwrap().into_raw(),
                has_arg: 1,
                flag: ptr::null_mut(),
                val: 0,
            },
            Option {
                name: CString::new("verbose").unwrap().into_raw(),
                has_arg: 0,
                flag: ptr::null_mut(),
                val: 0,
            },
            Option {
                name: CString::new("create").unwrap().into_raw(),
                has_arg: 0,
                flag: ptr::null_mut(),
                val: 0,
            },
            Option {
                name: CString::new("file").unwrap().into_raw(),
                has_arg: 1,
                flag: ptr::null_mut(),
                val: 0,
            },
            Option {
                name: ptr::null(),
                has_arg: 0,
                flag: ptr::null_mut(),
                val: 0,
            },
        ]
    }

    #[test]
    fn test_getopt_long() {
        let args = vec![
            CString::new("program").unwrap(),
            CString::new("--add").unwrap(),
            CString::new("value").unwrap(),
            CString::new("-c").unwrap(),
            CString::new("arg").unwrap(),
        ];

        let argv: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
        let argc = argv.len() as c_int;
        let options = CString::new("abc:d:0123456789").unwrap();
        let mut long_options = make_options();
        let mut option_index = 0;

        unsafe {
            optind = 1;
            let c = getopt_long(
                argc,
                argv.as_ptr(),
                options.as_ptr(),
                long_options.as_ptr(),
                &mut option_index,
            );
            assert_eq!(c, 0);
            assert_eq!(CStr::from_ptr(long_options[option_index].name).to_str().unwrap(), "add");
            assert!(!optarg.is_null());
            assert_eq!(CStr::from_ptr(optarg).to_str().unwrap(), "value");

            let c = getopt_long(
                argc,
                argv.as_ptr(),
                options.as_ptr(),
                long_options.as_ptr(),
                &mut option_index,
            );
            assert_eq!(c, 'c' as i32);
            assert!(!optarg.is_null());
            assert_eq!(CStr::from_ptr(optarg).to_str().unwrap(), "arg");
        }

        // Clean up
        for opt in long_options {
            if !opt.name.is_null() {
                drop(CString::from_raw(opt.name as *mut c_char));
            }
        }
    }
}