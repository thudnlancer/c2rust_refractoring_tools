use std::fmt;
use std::io::{Error, ErrorKind};
use std::mem;
use std::ptr;
use std::str;
use std::vec::Vec;

/// Error type for xvasprintf operations
#[derive(Debug)]
pub enum XvasprintfError {
    Overflow,
    InvalidFormat,
    EncodingError,
    Other(Error),
}

impl From<Error> for XvasprintfError {
    fn from(err: Error) -> Self {
        XvasprintfError::Other(err)
    }
}

impl fmt::Display for XvasprintfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            XvasprintfError::Overflow => write!(f, "String length exceeds INT_MAX"),
            XvasprintfError::InvalidFormat => write!(f, "Invalid format string"),
            XvasprintfError::EncodingError => write!(f, "Encoding conversion error"),
            XvasprintfError::Other(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for XvasprintfError {}

/// Concatenates strings from variable arguments
fn xstrcat(args: &[&str]) -> Result<String, XvasprintfError> {
    // Calculate total size
    let totalsize = args.iter().map(|s| s.len()).sum::<usize>();

    // Check for overflow
    if totalsize > i32::MAX as usize {
        return Err(XvasprintfError::Overflow);
    }

    // Build the result string
    let mut result = String::with_capacity(totalsize);
    for s in args {
        result.push_str(s);
    }

    Ok(result)
}

/// Formats arguments into a string, with memory allocation error handling
pub fn xvasprintf(format: &str, args: &[&str]) -> Result<String, XvasprintfError> {
    // Check for simple string concatenation case
    if format.chars().all(|c| c == '%' || c == 's') {
        let percent_count = format.matches('%').count();
        if percent_count == args.len() {
            return xstrcat(args);
        }
    }

    // Use standard Rust formatting for other cases
    if let Ok(result) = format!(
        "{}",
        args.iter()
            .fold(format.to_string(), |acc, &arg| acc.replacen("%s", arg, 1))
    {
        Ok(result)
    } else {
        Err(XvasprintfError::InvalidFormat)
    }
}

/// Formats arguments into a string (variadic version)
#[macro_export]
macro_rules! xasprintf {
    ($format:expr, $($arg:expr),*) => {
        xvasprintf($format, &[$($arg),*])
    };
}

/// Handles memory allocation failure by panicking
fn xalloc_die() -> ! {
    panic!("Memory allocation failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xstrcat() {
        let args = ["hello", " ", "world"];
        assert_eq!(xstrcat(&args).unwrap(), "hello world");
    }

    #[test]
    fn test_xvasprintf_concat() {
        let args = ["hello", " ", "world"];
        assert_eq!(xvasprintf("%s%s%s", &args).unwrap(), "hello world");
    }

    #[test]
    fn test_xvasprintf_format() {
        let args = ["world"];
        assert_eq!(xvasprintf("hello %s", &args).unwrap(), "hello world");
    }

    #[test]
    fn test_xasprintf_macro() {
        assert_eq!(xasprintf!("%s %s", "hello", "world").unwrap(), "hello world");
    }
}