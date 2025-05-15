use std::fmt;
use std::io::{Error, ErrorKind};
use std::mem;
use std::ptr;

/// Print formatted output to string `str`.
/// Returns string length of formatted string. On error, returns an error.
pub fn sprintf(str: &mut [u8], format: &str, args: fmt::Arguments<'_>) -> Result<usize, Error> {
    // Calculate maximum possible buffer size
    let lenbuf = {
        let max_size = usize::MAX.min(i32::MAX as usize);
        let str_ptr = str.as_ptr() as usize;
        max_size.min(!str_ptr)
    };

    // Format into a String first
    let output = format!("{}", args);

    // Check if output fits in provided buffer
    if output.len() > lenbuf {
        return Err(Error::new(ErrorKind::Other, "Buffer overflow"));
    }

    // Check if output exceeds INT_MAX
    if output.len() > i32::MAX as usize {
        return Err(Error::new(ErrorKind::Other, "Output too large"));
    }

    // Copy formatted string to output buffer
    let bytes = output.as_bytes();
    if bytes.len() > str.len() {
        return Err(Error::new(ErrorKind::Other, "Buffer too small"));
    }

    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), str.as_mut_ptr(), bytes.len());
    }

    Ok(bytes.len())
}

// Convenience wrapper for variable arguments
#[macro_export]
macro_rules! sprintf {
    ($buf:expr, $($arg:tt)*) => {
        sprintf($buf, format_args!($($arg)*))
    }
}