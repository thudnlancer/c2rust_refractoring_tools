/*!
Argmatch module for matching command line arguments against a list of possible values.

This is a Rust translation of the GNU argmatch C library.
*/

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::cmp;
use std::fmt;
use std::io::{self, Write};
use std::process;
use std::env;

/// Assert there are as many real arguments as there are values
/// (argument list ends with a NULL guard).
macro_rules! argmatch_verify {
    ($arglist:expr, $vallist:expr) => {
        assert_eq!($arglist.len(), $vallist.len() + 1);
    };
}

/// Return the index of the element of ARGLIST (NULL terminated) that
/// matches with ARG. If VALLIST is not NULL, then use it to resolve
/// false ambiguities.
pub fn argmatch(
    arg: &str,
    arglist: &[&str],
    vallist: Option<&[u8]>,
    valsize: usize,
) -> isize {
    let arglen = arg.len();
    let mut matchind = -1;
    let mut ambiguous = false;

    for (i, &argitem) in arglist.iter().enumerate() {
        if argitem.starts_with(arg) {
            if argitem.len() == arglen {
                // Exact match found
                return i as isize;
            } else if matchind == -1 {
                // First nonexact match found
                matchind = i as isize;
            } else if let Some(vallist) = vallist {
                // Second nonexact match found
                let match_ptr = unsafe {
                    vallist.as_ptr().offset(matchind * valsize as isize)
                };
                let current_ptr = unsafe {
                    vallist.as_ptr().offset(i as isize * valsize as isize)
                };
                if !eq_slice(match_ptr, current_ptr, valsize) {
                    ambiguous = true;
                }
            } else {
                ambiguous = true;
            }
        }
    }

    if ambiguous {
        -2
    } else {
        matchind
    }
}

/// Exact match version of argmatch
pub fn argmatch_exact(arg: &str, arglist: &[&str]) -> isize {
    for (i, &argitem) in arglist.iter().enumerate() {
        if argitem == arg {
            return i as isize;
        }
    }
    -1
}

/// Error reporting for argmatch
pub fn argmatch_invalid(context: &str, value: &str, problem: isize) {
    let format = if problem == -1 {
        "invalid argument {} for {}"
    } else {
        "ambiguous argument {} for {}"
    };
    eprintln!(format, value, context);
}

/// List the valid arguments for argmatch
pub fn argmatch_valid(arglist: &[&str], vallist: Option<&[u8]>, valsize: usize) {
    eprint!("Valid arguments are:");
    let mut last_val: Option<&[u8]> = None;

    for (i, &arg) in arglist.iter().enumerate() {
        let current_val = if let Some(vallist) = vallist {
            let start = i * valsize;
            let end = start + valsize;
            &vallist[start..end]
        } else {
            &[]
        };

        if last_val.is_none() || last_val != Some(current_val) {
            eprint!("\n  - {}", arg);
            last_val = Some(current_val);
        } else {
            eprint!(", {}", arg);
        }
    }
    eprintln!();
}

/// Never failing version of argmatch
pub fn xargmatch_internal(
    context: &str,
    arg: &str,
    arglist: &[&str],
    vallist: Option<&[u8]>,
    valsize: usize,
    exit_fn: fn(),
    allow_abbreviation: bool,
) -> isize {
    let res = if allow_abbreviation {
        argmatch(arg, arglist, vallist, valsize)
    } else {
        argmatch_exact(arg, arglist)
    };

    if res >= 0 {
        return res;
    }

    argmatch_invalid(context, arg, res);
    argmatch_valid(arglist, vallist, valsize);
    exit_fn();
    -1
}

/// Convert a value into a corresponding argument
pub fn argmatch_to_argument(
    value: &[u8],
    arglist: &[&str],
    vallist: &[u8],
    valsize: usize,
) -> Option<&str> {
    for (i, _) in arglist.iter().enumerate() {
        let start = i * valsize;
        let end = start + valsize;
        if value == &vallist[start..end] {
            return Some(arglist[i]);
        }
    }
    None
}

/// Helper function to compare memory slices
unsafe fn eq_slice(a: *const u8, b: *const u8, len: usize) -> bool {
    for i in 0..len {
        if *a.add(i) != *b.add(i) {
            return false;
        }
    }
    true
}

/// Default exit function
pub fn argmatch_die() {
    process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum BackupType {
        NoBackups,
        SimpleBackups,
        NumberedExistingBackups,
        NumberedBackups,
    }

    const BACKUP_ARGS: &[&str] = &[
        "no", "none", "off",
        "simple", "never",
        "existing", "nil",
        "numbered", "t",
    ];

    const BACKUP_VALS: &[BackupType] = &[
        BackupType::NoBackups,
        BackupType::NoBackups,
        BackupType::NoBackups,
        BackupType::SimpleBackups,
        BackupType::SimpleBackups,
        BackupType::NumberedExistingBackups,
        BackupType::NumberedExistingBackups,
        BackupType::NumberedBackups,
        BackupType::NumberedBackups,
    ];

    #[test]
    fn test_argmatch() {
        let res = argmatch("no", BACKUP_ARGS, None, 0);
        assert!(res >= 0);

        let res = argmatch("n", BACKUP_ARGS, None, 0);
        assert_eq!(res, -2); // ambiguous

        let res = argmatch("none", BACKUP_ARGS, None, 0);
        assert_eq!(res, 1);

        let res = argmatch("invalid", BACKUP_ARGS, None, 0);
        assert_eq!(res, -1);
    }

    #[test]
    fn test_xargmatch() {
        let res = xargmatch_internal(
            "test",
            "no",
            BACKUP_ARGS,
            None,
            0,
            argmatch_die,
            true,
        );
        assert!(res >= 0);

        let res = xargmatch_internal(
            "test",
            "none",
            BACKUP_ARGS,
            None,
            0,
            argmatch_die,
            false,
        );
        assert_eq!(res, 1);
    }
}