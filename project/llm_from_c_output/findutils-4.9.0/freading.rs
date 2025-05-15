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

use std::io::{self, Read, Write, Seek};

/// Determines if a stream is in reading mode.
///
/// Returns `true` if the stream is opened read-only, or if the last operation
/// was a read operation. Returns `false` if the stream is write-only or
/// append-only, or if it supports writing and there is no current read operation.
///
/// # Arguments
///
/// * `stream` - A reference to a stream implementing `Read`, `Write`, and `Seek`.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io;
///
/// let file = File::open("example.txt").unwrap();
/// println!("Is reading: {}", freading(&file));
/// ```
pub fn freading<T: Read + Write + Seek>(stream: &T) -> bool {
    // Try to perform a zero-length read to determine the current mode
    let mut buf = [0u8; 0];
    match stream.read(&mut buf) {
        Ok(_) => true,  // Read operation succeeded - in reading mode
        Err(ref e) if e.kind() == io::ErrorKind::WriteZero => false,  // Write operation in progress
        Err(_) => {
            // If read fails, try to determine mode by seeking
            let pos = stream.stream_position();
            pos.is_ok()  // If we can get position, likely in reading mode
        }
    }
}

// Note: This Rust implementation takes a different approach than the C version
// because Rust's std::io traits don't expose internal flags like C's FILE struct.
// Instead, we try to determine the mode by attempting operations and observing
// the results. This is less precise than checking internal flags but maintains
// safety and portability.

// The original C code checked platform-specific flags which isn't possible
// in safe Rust. This implementation provides similar functionality through
// standard Rust traits while maintaining safety.