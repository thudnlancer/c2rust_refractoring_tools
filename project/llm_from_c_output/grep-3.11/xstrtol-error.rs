//! Error reporting interface for xstrto* functions.

use std::process;
use std::ffi::CStr;
use std::os::raw::c_char;
use libc::{exit, EXIT_FAILURE};

#[derive(Debug)]
pub enum StrToLError {
    Invalid,
    InvalidSuffixChar,
    InvalidSuffixCharWithOverflow,
    Overflow,
}

pub struct Option {
    pub name: *const c_char,
    // Other fields omitted for brevity
}

/// Report an error for an invalid integer in an option argument.
///
/// `err` is the error code returned by one of the xstrto* functions.
///
/// Use `opt_idx` to decide whether to print the short option string "C"
/// or "-C" or a long option string derived from `long_options`. `opt_idx`
/// is -2 if the short option "C" was used, without any leading "-"; it
/// is -1 if the short option "-C" was used; otherwise it is an index
/// into `long_options`, which should have a name preceded by two '-'
/// characters.
///
/// `arg` is the option-argument containing the integer.
///
/// After reporting an error, exit with status `exit_status` if it is
/// nonzero.
fn xstrtol_error(
    err: StrToLError,
    opt_idx: i32,
    c: char,
    long_options: &[Option],
    arg: &str,
    exit_status: i32,
) {
    let hyphens = "--";
    let msgid = match err {
        StrToLError::Invalid => "invalid %s%s argument '%s'",
        StrToLError::InvalidSuffixChar | StrToLError::InvalidSuffixCharWithOverflow => {
            "invalid suffix in %s%s argument '%s'"
        }
        StrToLError::Overflow => "%s%s argument '%s' too large",
    };

    let (hyphens, option) = if opt_idx < 0 {
        let adjusted_hyphens = &hyphens[..(2 - (-opt_idx) as usize)];
        let option = c.to_string();
        (adjusted_hyphens, option)
    } else {
        let option_name = unsafe { CStr::from_ptr(long_options[opt_idx as usize].name) };
        (hyphens, option_name.to_string_lossy().into_owned())
    };

    eprintln!("{}", format!(msgid, hyphens, option, arg));
    if exit_status != 0 {
        unsafe { exit(exit_status) };
    }
}

/// Like xstrtol_error, except exit with a failure status.
pub fn xstrtol_fatal(
    err: StrToLError,
    opt_idx: i32,
    c: char,
    long_options: &[Option],
    arg: &str,
) -> ! {
    xstrtol_error(err, opt_idx, c, long_options, arg, EXIT_FAILURE);
    process::abort();
}