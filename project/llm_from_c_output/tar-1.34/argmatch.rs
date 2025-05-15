/*!
Argmatch module for finding matches in argument lists.

This is a Rust translation of the GNU argmatch C library.
*/

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::fmt;
use std::error::Error;
use std::io::{self, Write};

/// Type alias for exit function
pub type ArgmatchExitFn = fn();

/// Global exit function, defaults to `std::process::exit(1)`
pub static mut ARGMATCH_DIE: ArgmatchExitFn = default_argmatch_die;

fn default_argmatch_die() {
    std::process::exit(1);
}

/// Error types for argmatch
#[derive(Debug)]
pub enum ArgmatchError {
    InvalidArgument,
    AmbiguousArgument,
    IoError(io::Error),
}

impl fmt::Display for ArgmatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgmatchError::InvalidArgument => write!(f, "invalid argument"),
            ArgmatchError::AmbiguousArgument => write!(f, "ambiguous argument"),
            ArgmatchError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for ArgmatchError {}

impl From<io::Error> for ArgmatchError {
    fn from(err: io::Error) -> Self {
        ArgmatchError::IoError(err)
    }
}

/// Find a match for a string in an array
///
/// Returns index of match, or error if not found or ambiguous
pub fn argmatch(
    arg: &str,
    arglist: &[&str],
    vallist: Option<&[u8]>,
    valsize: usize,
) -> Result<usize, ArgmatchError> {
    let arglen = arg.len();
    let mut matchind = None;
    let mut ambiguous = false;

    for (i, &item) in arglist.iter().enumerate() {
        if item.starts_with(arg) {
            if item.len() == arglen {
                // Exact match found
                return Ok(i);
            } else if matchind.is_none() {
                // First nonexact match found
                matchind = Some(i);
            } else {
                // Second nonexact match found
                if let Some(vallist) = vallist {
                    let match_start = matchind.unwrap() * valsize;
                    let current_start = i * valsize;
                    if vallist[match_start..match_start + valsize]
                        != vallist[current_start..current_start + valsize]
                    {
                        ambiguous = true;
                    }
                } else {
                    ambiguous = true;
                }
            }
        }
    }

    if ambiguous {
        Err(ArgmatchError::AmbiguousArgument)
    } else if let Some(idx) = matchind {
        Ok(idx)
    } else {
        Err(ArgmatchError::InvalidArgument)
    }
}

/// Report invalid argument error
pub fn argmatch_invalid(context: &str, value: &str, problem: ArgmatchError) -> io::Result<()> {
    let format = match problem {
        ArgmatchError::InvalidArgument => "invalid argument {} for {}",
        ArgmatchError::AmbiguousArgument => "ambiguous argument {} for {}",
        _ => unreachable!(),
    };

    writeln!(
        io::stderr(),
        format,
        value, // TODO: Add proper quoting
        context
    )
}

/// List valid arguments
pub fn argmatch_valid(arglist: &[&str], vallist: Option<&[u8]>, valsize: usize) -> io::Result<()> {
    let mut stderr = io::stderr();
    write!(stderr, "Valid arguments are:")?;

    let mut last_val: Option<&[u8]> = None;

    for (i, &arg) in arglist.iter().enumerate() {
        let current_val = if let Some(vallist) = vallist {
            &vallist[i * valsize..(i + 1) * valsize]
        } else {
            &[]
        };

        if i == 0 || last_val != Some(current_val) {
            write!(stderr, "\n  - {}", arg)?;
            last_val = Some(current_val);
        } else {
            write!(stderr, ", {}", arg)?;
        }
    }

    writeln!(stderr)?;
    Ok(())
}

/// Never-failing version of argmatch
pub fn xargmatch_internal(
    context: &str,
    arg: &str,
    arglist: &[&str],
    vallist: Option<&[u8]>,
    valsize: usize,
    exit_fn: ArgmatchExitFn,
) -> usize {
    match argmatch(arg, arglist, vallist, valsize) {
        Ok(res) => res,
        Err(problem) => {
            argmatch_invalid(context, arg, problem).unwrap();
            argmatch_valid(arglist, vallist, valsize).unwrap();
            exit_fn();
            0 // Unreachable
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
        let current_val = &vallist[i * valsize..(i + 1) * valsize];
        if current_val == value {
            return Some(arglist[i]);
        }
    }
    None
}

/// Macro to define an argmatch group
#[macro_export]
macro_rules! argmatch_define_group {
    ($name:ident, $type:ty) => {
        pub mod $name {
            use super::*;
            
            pub type Type = $type;
            pub const SIZE: usize = std::mem::size_of::<Type>();
            
            pub struct Arg {
                pub arg: &'static str,
                pub val: Type,
            }
            
            pub struct Doc {
                pub arg: &'static str,
                pub doc: &'static str,
            }
            
            pub struct Group {
                pub args: &'static [Arg],
                pub docs: &'static [Doc],
                pub doc_pre: Option<&'static str>,
                pub doc_post: Option<&'static str>,
            }
            
            impl Group {
                pub fn usage(&self, out: &mut dyn Write) -> io::Result<()> {
                    if let Some(pre) = self.doc_pre {
                        writeln!(out, "{}", pre)?;
                    }
                    
                    // TODO: Implement full usage formatting
                    for doc in self.docs {
                        writeln!(out, "  {} - {}", doc.arg, doc.doc)?;
                    }
                    
                    if let Some(post) = self.doc_post {
                        writeln!(out, "{}", post)?;
                    }
                    
                    Ok(())
                }
                
                pub fn choice(&self, arg: &str) -> Result<usize, ArgmatchError> {
                    let arglen = arg.len();
                    let mut matchind = None;
                    let mut ambiguous = false;
                    
                    for (i, item) in self.args.iter().enumerate() {
                        if item.arg.starts_with(arg) {
                            if item.arg.len() == arglen {
                                return Ok(i);
                            } else if matchind.is_none() {
                                matchind = Some(i);
                            } else {
                                if self.args[matchind.unwrap()].val != item.val {
                                    ambiguous = true;
                                }
                            }
                        }
                    }
                    
                    if ambiguous {
                        Err(ArgmatchError::AmbiguousArgument)
                    } else if let Some(idx) = matchind {
                        Ok(idx)
                    } else {
                        Err(ArgmatchError::InvalidArgument)
                    }
                }
                
                pub fn value(&self, context: &str, arg: &str) -> &Type {
                    match self.choice(arg) {
                        Ok(res) => &self.args[res].val,
                        Err(problem) => {
                            argmatch_invalid(context, arg, problem).unwrap();
                            self.valid().unwrap();
                            unsafe { ARGMATCH_DIE() };
                            unreachable!()
                        }
                    }
                }
                
                pub fn argument(&self, val: &Type) -> Option<&str> {
                    for arg in self.args {
                        if &arg.val == val {
                            return Some(arg.arg);
                        }
                    }
                    None
                }
                
                fn valid(&self) -> io::Result<()> {
                    let mut stderr = io::stderr();
                    write!(stderr, "Valid arguments are:")?;
                    
                    let mut last_val: Option<&Type> = None;
                    
                    for arg in self.args {
                        if last_val != Some(&arg.val) {
                            write!(stderr, "\n  - {}", arg.arg)?;
                            last_val = Some(&arg.val);
                        } else {
                            write!(stderr, ", {}", arg.arg)?;
                        }
                    }
                    
                    writeln!(stderr)?;
                    Ok(())
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_argmatch() {
        let args = ["no", "simple", "existing", "numbered"];
        let vals = [0u8, 1u8, 2u8, 3u8];
        
        assert_eq!(argmatch("no", &args, Some(&vals), Ok(0));
        assert_eq!(argmatch("sim", &args, Some(&vals), Ok(1));
        assert_eq!(argmatch("x", &args, Some(&vals), Err(ArgmatchError::InvalidArgument));
    }
    
    argmatch_define_group!(backup, u8);
    
    #[test]
    fn test_argmatch_group() {
        const BACKUP_ARGS: [backup::Arg; 4] = [
            backup::Arg { arg: "no", val: 0 },
            backup::Arg { arg: "simple", val: 1 },
            backup::Arg { arg: "existing", val: 2 },
            backup::Arg { arg: "numbered", val: 3 },
        ];
        
        let group = backup::Group {
            args: &BACKUP_ARGS,
            docs: &[],
            doc_pre: None,
            doc_post: None,
        };
        
        assert_eq!(group.choice("no"), Ok(0));
        assert_eq!(group.choice("sim"), Ok(1));
        assert_eq!(group.choice("x"), Err(ArgmatchError::InvalidArgument));
    }
}