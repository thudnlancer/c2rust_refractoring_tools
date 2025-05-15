//! Convert integers to printable strings
//!
//! This module provides functions to convert various integer types to their string representations.
//! 
//! The code is translated from C to Rust with safety and idiomatic Rust practices in mind.

use std::convert::TryInto;
use std::fmt::Write;

/// Converts an intmax_t to a string representation.
/// 
/// # Arguments
/// * `value` - The integer value to convert
/// * `buffer` - A mutable slice of bytes to store the result
/// 
/// # Returns
/// A reference to the string within the buffer on success
/// 
/// # Panics
/// Panics if the buffer is too small to hold the result
#[must_use]
pub fn imaxtostr(value: i64, buffer: &mut [u8]) -> &str {
    write_int_to_str(value, buffer)
}

/// Converts an int to a string representation.
/// 
/// # Arguments
/// * `value` - The integer value to convert
/// * `buffer` - A mutable slice of bytes to store the result
/// 
/// # Returns
/// A reference to the string within the buffer on success
/// 
/// # Panics
/// Panics if the buffer is too small to hold the result
#[must_use]
pub fn inttostr(value: i32, buffer: &mut [u8]) -> &str {
    write_int_to_str(value as i64, buffer)
}

/// Converts an off_t to a string representation.
/// 
/// # Arguments
/// * `value` - The integer value to convert
/// * `buffer` - A mutable slice of bytes to store the result
/// 
/// # Returns
/// A reference to the string within the buffer on success
/// 
/// # Panics
/// Panics if the buffer is too small to hold the result
#[must_use]
pub fn offtostr(value: i64, buffer: &mut [u8]) -> &str {
    write_int_to_str(value, buffer)
}

/// Converts an unsigned int to a string representation.
/// 
/// # Arguments
/// * `value` - The integer value to convert
/// * `buffer` - A mutable slice of bytes to store the result
/// 
/// # Returns
/// A reference to the string within the buffer on success
/// 
/// # Panics
/// Panics if the buffer is too small to hold the result
#[must_use]
pub fn uinttostr(value: u32, buffer: &mut [u8]) -> &str {
    write_uint_to_str(value as u64, buffer)
}

/// Converts a uintmax_t to a string representation.
/// 
/// # Arguments
/// * `value` - The integer value to convert
/// * `buffer` - A mutable slice of bytes to store the result
/// 
/// # Returns
/// A reference to the string within the buffer on success
/// 
/// # Panics
/// Panics if the buffer is too small to hold the result
#[must_use]
pub fn umaxtostr(value: u64, buffer: &mut [u8]) -> &str {
    write_uint_to_str(value, buffer)
}

fn write_int_to_str<T: Into<i64>>(value: T, buffer: &mut [u8]) -> &str {
    let value = value.into();
    let mut writer = BufferWriter::new(buffer);
    write!(&mut writer, "{}", value).expect("Buffer too small for integer conversion");
    writer.as_str()
}

fn write_uint_to_str<T: Into<u64>>(value: T, buffer: &mut [u8]) -> &str {
    let value = value.into();
    let mut writer = BufferWriter::new(buffer);
    write!(&mut writer, "{}", value).expect("Buffer too small for integer conversion");
    writer.as_str()
}

struct BufferWriter<'a> {
    buffer: &'a mut [u8],
    position: usize,
}

impl<'a> BufferWriter<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        BufferWriter {
            buffer,
            position: 0,
        }
    }

    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buffer[..self.position]).unwrap()
    }
}

impl<'a> std::fmt::Write for BufferWriter<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let bytes = s.as_bytes();
        let available = self.buffer.len() - self.position;
        if bytes.len() > available {
            return Err(std::fmt::Error);
        }
        self.buffer[self.position..self.position + bytes.len()].copy_from_slice(bytes);
        self.position += bytes.len();
        Ok(())
    }
}