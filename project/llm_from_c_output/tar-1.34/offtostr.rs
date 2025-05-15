// This is a Rust equivalent of the C macro-based code that defines anytostr as offtostr
// and uses off_t as the integer type. In Rust, we'll implement a similar conversion
// function for the equivalent Rust type (typically i64 for off_t).

use std::fmt::Write;

/// Converts an offset (off_t equivalent) to a string representation
pub fn offtostr(value: i64) -> Result<String, std::fmt::Error> {
    let mut buffer = String::new();
    write!(&mut buffer, "{}", value)?;
    Ok(buffer)
}

// Alias anytostr to offtostr for compatibility
pub use offtostr as anytostr;