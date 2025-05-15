use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::process;

type ArgMatchExitFn = fn();

const EXIT_FAILURE: i32 = 1;

#[derive(Debug, PartialEq)]
enum ArgMatchError {
    Invalid,
    Ambiguous,
}

fn argmatch(
    arg: &str,
    arglist: &[&str],
    vallist: Option<&[u8]>,
    valsize: usize,
) -> Result<usize, ArgMatchError> {
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
                let match_offset = matchind.unwrap() * valsize;
                let current_offset = i * valsize;
                if &vals[match_offset..match_offset + valsize] != &vals[current_offset..current_offset + valsize] {
                    ambiguous = true;
                }
            }
        }
    }

    if ambiguous {
        Err(ArgMatchError::Ambiguous)
    } else if let Some(idx) = matchind {
        Ok(idx)
    } else {
        Err(ArgMatchError::Invalid)
    }
}

fn argmatch_invalid(context: &str, value: &str, problem: ArgMatchError) {
    let format = match problem {
        ArgMatchError::Invalid => "invalid argument {} for {}",
        ArgMatchError::Ambiguous => "ambiguous argument {} for {}",
    };
    eprintln!(format, value, context);
}

fn argmatch_valid(arglist: &[&str], vallist: Option<&[u8]>, valsize: usize) {
    eprint!("Valid arguments are:");
    
    let mut last_val: Option<&[u8]> = None;
    for (i, arg) in arglist.iter().enumerate() {
        let current_val = vallist.map(|vals| &vals[i * valsize..(i + 1) * valsize]);
        
        match (last_val, current_val) {
            (None, _) | (Some(_), None) => {
                eprint!("\n  - {}", arg);
            }
            (Some(last), Some(current)) if last != current => {
                eprint!("\n  - {}", arg);
            }
            _ => {
                eprint!(", {}", arg);
            }
        }
        
        last_val = current_val;
    }
    eprintln!();
}

fn xargmatch_internal(
    context: &str,
    arg: &str,
    arglist: &[&str],
    vallist: Option<&[u8]>,
    valsize: usize,
    exit_fn: ArgMatchExitFn,
) -> usize {
    match argmatch(arg, arglist, vallist, valsize) {
        Ok(idx) => idx,
        Err(err) => {
            argmatch_invalid(context, arg, err);
            argmatch_valid(arglist, vallist, valsize);
            exit_fn();
            process::exit(EXIT_FAILURE);
        }
    }
}

fn argmatch_to_argument(
    value: &[u8],
    arglist: &[&str],
    vallist: &[u8],
    valsize: usize,
) -> Option<&str> {
    arglist.iter().enumerate().find_map(|(i, arg)| {
        let offset = i * valsize;
        if value == &vallist[offset..offset + valsize] {
            Some(*arg)
        } else {
            None
        }
    })
}

fn default_argmatch_die() {
    process::exit(EXIT_FAILURE);
}

static ARGMATCH_DIE: ArgMatchExitFn = default_argmatch_die;