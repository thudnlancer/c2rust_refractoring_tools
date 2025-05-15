use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};
use std::ptr;

type PtrDiffT = c_long;
type SizeT = c_ulong;

#[derive(Debug, Clone, Copy)]
pub enum QuotingStyle {
    Literal,
    Shell,
    ShellAlways,
    ShellEscape,
    ShellEscapeAlways,
    C,
    CMaybe,
    Escape,
    Locale,
    Clocale,
    Custom,
}

pub type ArgMatchExitFn = fn() -> !;

fn argmatch_die() -> ! {
    std::process::exit(1);
}

pub fn argmatch(
    arg: &str,
    arglist: &[&str],
    vallist: Option<&[u8]>,
    valsize: usize,
) -> Result<usize, (Option<usize>, bool)> {
    let arglen = arg.len();
    let mut matchind = None;
    let mut ambiguous = false;

    for (i, candidate) in arglist.iter().enumerate() {
        if candidate.starts_with(arg) {
            if candidate.len() == arglen {
                return Ok(i);
            } else if matchind.is_none() {
                matchind = Some(i);
            } else if let Some(vals) = vallist {
                let match_idx = matchind.unwrap();
                let match_slice = &vals[match_idx * valsize..(match_idx + 1) * valsize];
                let current_slice = &vals[i * valsize..(i + 1) * valsize];
                if match_slice != current_slice {
                    ambiguous = true;
                }
            }
        }
    }

    Err((matchind, ambiguous))
}

pub fn argmatch_exact(arg: &str, arglist: &[&str]) -> Option<usize> {
    arglist.iter().position(|&candidate| candidate == arg)
}

pub fn argmatch_invalid(context: &str, value: &str, problem: Result<(), ()>) {
    let format = if problem.is_err() {
        "invalid argument {} for {}"
    } else {
        "ambiguous argument {} for {}"
    };
    eprintln!(format, value, context);
}

pub fn argmatch_valid(arglist: &[&str], vallist: Option<&[u8]>, valsize: usize) {
    eprint!("Valid arguments are:");
    let mut last_val: Option<&[u8]> = None;

    for (i, arg) in arglist.iter().enumerate() {
        let current_val = vallist.map(|vals| &vals[i * valsize..(i + 1) * valsize]);

        if i == 0 || Some(current_val) != last_val.map(|_| current_val) {
            eprint!("\n  - {}", arg);
            last_val = current_val;
        } else {
            eprint!(", {}", arg);
        }
    }
    eprintln!();
}

pub fn xargmatch_internal(
    context: &str,
    arg: &str,
    arglist: &[&str],
    vallist: Option<&[u8]>,
    valsize: usize,
    exit_fn: ArgMatchExitFn,
    allow_abbreviation: bool,
) -> usize {
    let res = if allow_abbreviation {
        argmatch(arg, arglist, vallist, valsize)
    } else {
        argmatch_exact(arg, arglist).ok_or((None, false))
    };

    match res {
        Ok(idx) => idx,
        Err((_, true)) => {
            argmatch_invalid(context, arg, Err(()));
            argmatch_valid(arglist, vallist, valsize);
            exit_fn();
        }
        Err((None, _)) => {
            argmatch_invalid(context, arg, Ok(()));
            argmatch_valid(arglist, vallist, valsize);
            exit_fn();
        }
        Err((Some(idx), _)) => idx,
    }
}

pub fn argmatch_to_argument(
    value: &[u8],
    arglist: &[&str],
    vallist: &[u8],
    valsize: usize,
) -> Option<&str> {
    arglist.iter().enumerate().find_map(|(i, &arg)| {
        let val_slice = &vallist[i * valsize..(i + 1) * valsize];
        if val_slice == value {
            Some(arg)
        } else {
            None
        }
    })
}