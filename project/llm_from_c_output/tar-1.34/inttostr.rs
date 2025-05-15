// inttostr.rs -- convert integers to printable strings

// Copyright (C) 2001-2006, 2009-2021 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Paul Eggert

use std::convert::TryInto;
use std::fmt::Write;

/// Converts intmax_t to string
pub fn imaxtostr(value: i64, buffer: &mut [u8]) -> Result<&str, std::fmt::Error> {
    write_int_to_str(value, buffer)
}

/// Converts int to string
pub fn inttostr(value: i32, buffer: &mut [u8]) -> Result<&str, std::fmt::Error> {
    write_int_to_str(value as i64, buffer)
}

/// Converts off_t to string
pub fn offtostr(value: i64, buffer: &mut [u8]) -> Result<&str, std::fmt::Error> {
    write_int_to_str(value, buffer)
}

/// Converts unsigned int to string
pub fn uinttostr(value: u32, buffer: &mut [u8]) -> Result<&str, std::fmt::Error> {
    write_uint_to_str(value as u64, buffer)
}

/// Converts uintmax_t to string
pub fn umaxtostr(value: u64, buffer: &mut [u8]) -> Result<&str, std::fmt::Error> {
    write_uint_to_str(value, buffer)
}

fn write_int_to_str<T: Into<i64>>(value: T, buffer: &mut [u8]) -> Result<&str, std::fmt::Error> {
    let mut s = String::new();
    write!(&mut s, "{}", value.into())?;
    let bytes = s.as_bytes();
    if bytes.len() > buffer.len() {
        return Err(std::fmt::Error);
    }
    buffer[..bytes.len()].copy_from_slice(bytes);
    Ok(std::str::from_utf8(&buffer[..bytes.len()]).unwrap())
}

fn write_uint_to_str<T: Into<u64>>(value: T, buffer: &mut [u8]) -> Result<&str, std::fmt::Error> {
    let mut s = String::new();
    write!(&mut s, "{}", value.into())?;
    let bytes = s.as_bytes();
    if bytes.len() > buffer.len() {
        return Err(std::fmt::Error);
    }
    buffer[..bytes.len()].copy_from_slice(bytes);
    Ok(std::str::from_utf8(&buffer[..bytes.len()]).unwrap())
}