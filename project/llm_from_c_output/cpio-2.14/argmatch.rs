/*!
Argmatch module for matching and validating command line arguments.

This is a Rust translation of the GNU argmatch C library, providing similar
functionality for argument matching and validation.
*/

use std::cmp::Ordering;
use std::env;
use std::ffi::{CStr, CString};
use std::fmt;
use std::mem;
use std::os::raw::c_char;
use std::ptr;
use std::slice;

/// Assert there are as many real arguments as there are values
macro_rules! argmatch_verify {
    ($arglist:expr, $vallist:expr) => {
        assert_eq!(
            $arglist.len(),
            $vallist.len() + 1,
            "Argument and value lists must have matching lengths"
        );
    };
}

/// Return the index of the element of ARGLIST (NULL terminated) that matches with ARG.
/// If VALLIST is not NULL, then use it to resolve false ambiguities.
pub fn argmatch(
    arg: &str,
    arglist: &[&str],
    vallist: Option<&[u8]>,
    valsize: usize,
) -> Result<isize, ArgmatchError> {
    let arglen = arg.len();
    let mut matchind = -1;
    let mut ambiguous = false;

    for (i, &argitem) in arglist.iter().enumerate() {
        if argitem.starts_with(arg) {
            if argitem.len() == arglen {
                // Exact match found
                return Ok(i as isize);
            } else if matchind == -1 {
                // First nonexact match found
                matchind = i as isize;
            } else if let Some(vals) = vallist {
                // Check if values are different
                let val1 = &vals[(matchind as usize) * valsize..][..valsize];
                let val2 = &vals[i * valsize..][..valsize];
                if val1 != val2 {
                    ambiguous = true;
                }
            } else {
                ambiguous = true;
            }
        }
    }

    if ambiguous {
        Err(ArgmatchError::Ambiguous)
    } else if matchind != -1 {
        Ok(matchind)
    } else {
        Err(ArgmatchError::Invalid)
    }
}

/// Exact version of argmatch that doesn't allow abbreviations
pub fn argmatch_exact(arg: &str, arglist: &[&str]) -> Result<isize, ArgmatchError> {
    arglist
        .iter()
        .position(|&x| x == arg)
        .map(|i| i as isize)
        .ok_or(ArgmatchError::Invalid)
}

/// Error types for argmatch
#[derive(Debug, PartialEq)]
pub enum ArgmatchError {
    Invalid,
    Ambiguous,
}

impl fmt::Display for ArgmatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgmatchError::Invalid => write!(f, "invalid argument"),
            ArgmatchError::Ambiguous => write!(f, "ambiguous argument"),
        }
    }
}

/// Report invalid argument
pub fn argmatch_invalid(context: &str, value: &str, problem: ArgmatchError) {
    eprintln!(
        "{}: {} argument '{}' for '{}'",
        env::args().next().unwrap_or_default(),
        problem,
        value,
        context
    );
}

/// List valid arguments
pub fn argmatch_valid(arglist: &[&str], vallist: Option<&[u8]>, valsize: usize) {
    eprint!("Valid arguments are:");
    let mut last_val: Option<&[u8]> = None;

    for (i, &arg) in arglist.iter().enumerate() {
        let current_val = vallist.map(|vals| &vals[i * valsize..][..valsize]);

        if i == 0 || current_val != last_val {
            eprint!("\n  - {}", arg);
        } else {
            eprint!(", {}", arg);
        }
        last_val = current_val;
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
    allow_abbreviation: bool,
) -> isize {
    let res = if allow_abbreviation {
        argmatch(arg, arglist, vallist, valsize)
    } else {
        argmatch_exact(arg, arglist)
    };

    match res {
        Ok(idx) => idx,
        Err(err) => {
            argmatch_invalid(context, arg, err);
            argmatch_valid(arglist, vallist, valsize);
            std::process::exit(1);
        }
    }
}

/// Convert value back to argument
pub fn argmatch_to_argument(
    value: &[u8],
    arglist: &[&str],
    vallist: &[u8],
    valsize: usize,
) -> Option<&str> {
    for (i, _) in arglist.iter().enumerate() {
        let val = &vallist[i * valsize..][..valsize];
        if val == value {
            return Some(arglist[i]);
        }
    }
    None
}

/// Macro for convenient argmatch usage
#[macro_export]
macro_rules! xargmatch {
    ($context:expr, $arg:expr, $arglist:expr, $vallist:expr) => {
        $crate::xargmatch_internal(
            $context,
            $arg,
            $arglist,
            Some($vallist),
            mem::size_of_val(&$vallist[0]),
            true,
        )
    };
}

/// Macro for exact argmatch usage
#[macro_export]
macro_rules! xargmatch_exact {
    ($context:expr, $arg:expr, $arglist:expr, $vallist:expr) => {
        $crate::xargmatch_internal(
            $context,
            $arg,
            $arglist,
            Some($vallist),
            mem::size_of_val(&$vallist[0]),
            false,
        )
    };
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

    const BACKUP_VALS: [BackupType; 9] = [
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
        let res = argmatch(
            "no",
            BACKUP_ARGS,
            Some(unsafe {
                std::slice::from_raw_parts(
                    BACKUP_VALS.as_ptr() as *const u8,
                    BACKUP_VALS.len() * mem::size_of::<BackupType>(),
                )
            }),
            mem::size_of::<BackupType>(),
        );
        assert_eq!(res, Ok(0));

        let res = argmatch("n", BACKUP_ARGS, None, 0);
        assert_eq!(res, Err(ArgmatchError::Ambiguous));
    }

    #[test]
    fn test_argmatch_exact() {
        let res = argmatch_exact("no", BACKUP_ARGS);
        assert_eq!(res, Ok(0));

        let res = argmatch_exact("n", BACKUP_ARGS);
        assert_eq!(res, Err(ArgmatchError::Invalid));
    }

    #[test]
    fn test_xargmatch() {
        let idx = xargmatch_internal(
            "test",
            "no",
            BACKUP_ARGS,
            Some(unsafe {
                std::slice::from_raw_parts(
                    BACKUP_VALS.as_ptr() as *const u8,
                    BACKUP_VALS.len() * mem::size_of::<BackupType>(),
                )
            }),
            mem::size_of::<BackupType>(),
            true,
        );
        assert_eq!(idx, 0);
    }
}