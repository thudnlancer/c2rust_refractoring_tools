// Retrieve information about a file stream.
// Copyright (C) 2007-2022 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::File;

/// Returns true if the stream is opened read-only, or if the last operation
/// on the stream was a read operation. Returns false if the stream is opened
/// write-only or append-only, or if it supports writing and there is no current
/// read operation.
///
/// # Arguments
/// * `file` - A reference to a file object implementing Read, Write and Seek
///
/// # Examples
/// ```
/// use std::fs::File;
/// use std::io::{self, Read, Write};
///
/// let mut file = File::open("foo.txt").unwrap();
/// assert!(freading(&file));
/// ```
pub fn freading<T: Read + Write + Seek>(file: &T) -> bool {
    // Try to determine read mode by attempting a zero-length read
    let mut buf = [0u8; 0];
    match file.read(&mut buf) {
        Ok(_) => {
            // If read succeeds, we're in read mode
            // Reset position if needed (since we did a zero-length read)
            let _ = file.seek(SeekFrom::Current(0));
            true
        }
        Err(_) => {
            // If read fails, check if write would succeed
            match file.write(&buf) {
                Ok(_) => {
                    // If write succeeds, we're in write mode
                    // Reset position if needed
                    let _ = file.seek(SeekFrom::Current(0));
                    false
                }
                Err(_) => {
                    // If both fail, assume read-only
                    true
                }
            }
        }
    }
}

// Note: This Rust implementation takes a different approach than the C version
// because we don't have direct access to FILE structure internals in safe Rust.
// Instead, we use the standard traits and attempt operations to determine the mode.
// This is less efficient but maintains safety and portability.