use std::ffi::{CStr, CString};
use std::process;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrToLError {
    Ok,
    Overflow,
    InvalidSuffixChar,
    InvalidSuffixCharWithOverflow,
    Invalid,
}

#[derive(Debug, Clone)]
pub struct Option {
    pub name: String,
    pub has_arg: i32,
    pub flag: Option<Box<i32>>,
    pub val: i32,
}

fn xstrtol_error(
    err: StrToLError,
    opt_idx: i32,
    c: char,
    long_options: &[Option],
    arg: &str,
    exit_status: i32,
) {
    let hyphens = if opt_idx < 0 {
        "-".repeat((-opt_idx) as usize)
    } else {
        "--".to_string()
    };

    let option = if opt_idx < 0 {
        c.to_string()
    } else {
        long_options[opt_idx as usize].name.clone()
    };

    let msg = match err {
        StrToLError::Invalid => format!("invalid {}{} argument '{}'", hyphens, option, arg),
        StrToLError::InvalidSuffixChar | StrToLError::InvalidSuffixCharWithOverflow => {
            format!("invalid suffix in {}{} argument '{}'", hyphens, option, arg)
        }
        StrToLError::Overflow => format!("{}{} argument '{}' too large", hyphens, option, arg),
        _ => unreachable!(),
    };

    eprintln!("{}", msg);
    process::exit(exit_status);
}

pub fn xstrtol_fatal(
    err: StrToLError,
    opt_idx: i32,
    c: char,
    long_options: &[Option],
    arg: &str,
) -> ! {
    xstrtol_error(err, opt_idx, c, long_options, arg, 1);
    process::abort();
}