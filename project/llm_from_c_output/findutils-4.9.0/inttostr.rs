// inttostr.rs -- convert integers to printable strings

// Copyright (C) 2001-2006, 2009-2022 Free Software Foundation, Inc.

// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.

// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Paul Eggert

use std::convert::TryInto;
use std::fmt::Write;

/// Converts an integer to a string representation, storing it in the provided buffer.
/// Returns the buffer if successful, or None if the buffer is too small.
pub fn imaxtostr(value: i64, buffer: &mut [u8]) -> Option<&str> {
    anytostr(value, buffer)
}

/// Converts an integer to a string representation, storing it in the provided buffer.
/// Returns the buffer if successful, or None if the buffer is too small.
pub fn inttostr(value: i32, buffer: &mut [u8]) -> Option<&str> {
    anytostr(value as i64, buffer)
}

/// Converts an offset to a string representation, storing it in the provided buffer.
/// Returns the buffer if successful, or None if the buffer is too small.
pub fn offtostr(value: i64, buffer: &mut [u8]) -> Option<&str> {
    anytostr(value, buffer)
}

/// Converts an unsigned integer to a string representation, storing it in the provided buffer.
/// Returns the buffer if successful, or None if the buffer is too small.
pub fn uinttostr(value: u32, buffer: &mut [u8]) -> Option<&str> {
    anytostr(value as u64 as i64, buffer)
}

/// Converts an unsigned integer to a string representation, storing it in the provided buffer.
/// Returns the buffer if successful, or None if the buffer is too small.
pub fn umaxtostr(value: u64, buffer: &mut [u8]) -> Option<&str> {
    anytostr(value as i64, buffer)
}

/// Generic function to convert any integer type to string representation.
/// Returns the buffer if successful, or None if the buffer is too small.
fn anytostr<T: Into<i64>>(value: T, buffer: &mut [u8]) -> Option<&str> {
    let value = value.into();
    let mut s = String::new();
    write!(&mut s, "{}", value).ok()?;
    
    if s.len() > buffer.len() {
        return None;
    }
    
    buffer[..s.len()].copy_from_slice(s.as_bytes());
    std::str::from_utf8(&buffer[..s.len()]).ok()
}