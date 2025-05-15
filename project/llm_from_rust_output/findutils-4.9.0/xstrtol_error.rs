use std::ffi::{CStr, CString};
use std::process;

#[derive(Debug, PartialEq, Eq)]
pub enum StrToLError {
    Ok,
    Overflow,
    InvalidSuffixChar,
    InvalidSuffixCharWithOverflow,
    Invalid,
}

#[derive(Clone)]
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
    let hyphens = if opt_idx < 0 { "-" } else { "--" };
    let msgid = match err {
        StrToLError::Invalid => "invalid {}{} argument '{}'",
        StrToLError::InvalidSuffixChar | StrToLError::InvalidSuffixCharWithOverflow => {
            "invalid suffix in {}{} argument '{}'"
        }
        StrToLError::Overflow => "{}{} argument '{}' too large",
        _ => unreachable!(),
    };

    let option = if opt_idx < 0 {
        c.to_string()
    } else {
        long_options[opt_idx as usize].name.clone()
    };

    eprintln!("{}", format!(msgid, hyphens, option, arg));
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