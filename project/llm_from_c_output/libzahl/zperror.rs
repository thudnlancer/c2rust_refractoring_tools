// See LICENSE file for copyright and license details.
use std::io::{self, Write};
use std::ffi::CStr;

extern "C" {
    static libzahl_error: i32;
    fn zerror(desc: *mut *const libc::c_char) -> i32;
}

pub fn zperror(prefix: Option<&str>) {
    unsafe {
        if libzahl_error >= 0 {
            let err = libzahl_error;
            let prefix_cstr = prefix.map(|s| s.to_string()).unwrap_or_default();
            io::Error::from_raw_os_error(err).print(prefix_cstr);
        } else {
            let mut desc_ptr: *const libc::c_char = std::ptr::null();
            zerror(&mut desc_ptr);
            let desc = CStr::from_ptr(desc_ptr).to_string_lossy();

            match prefix {
                Some(p) if !p.is_empty() => {
                    writeln!(io::stderr(), "{}: {}", p, desc).ok();
                }
                _ => {
                    writeln!(io::stderr(), "{}", desc).ok();
                }
            }
        }
    }
}

trait PrintErrorExt {
    fn print(&self, prefix: String);
}

impl PrintErrorExt for io::Error {
    fn print(&self, prefix: String) {
        if !prefix.is_empty() {
            eprintln!("{}: {}", prefix, self);
        } else {
            eprintln!("{}", self);
        }
    }
}