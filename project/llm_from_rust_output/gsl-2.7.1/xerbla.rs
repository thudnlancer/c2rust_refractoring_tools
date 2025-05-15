use std::ffi::CStr;
use std::io::{stderr, Write};
use std::process;

pub fn cblas_xerbla(p: i32, rout: &CStr, form: &CStr, args: std::fmt::Arguments) {
    if p != 0 {
        let _ = writeln!(
            stderr(),
            "Parameter {} to routine {} was incorrect",
            p,
            rout.to_string_lossy()
        );
    }
    
    let _ = stderr().write_fmt(args);
    process::abort();
}