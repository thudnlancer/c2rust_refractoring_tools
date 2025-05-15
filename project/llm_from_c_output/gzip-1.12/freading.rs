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
/// # Notes
/// - freading and fwriting will never both be true.
/// - For streams supporting both reads and writes:
///   - Both may be false when first opened, after read encounters EOF, or after flush
///   - freading may be false or true and fwriting may be false after repositioning
/// - The stream must not be wide-character oriented.
pub fn freading(stream: &File) -> std::io::Result<bool> {
    // Get current position before attempting operations
    let original_pos = stream.stream_position()?;
    
    // Try to read a single byte to determine read state
    let mut buf = [0u8; 1];
    match stream.read(&mut buf) {
        Ok(0) => {
            // EOF reached - restore position and check if readable
            stream.seek(SeekFrom::Start(original_pos))?;
            Ok(true) // EOF implies last operation was read
        }
        Ok(_) => {
            // Successfully read - restore position and return true
            stream.seek(SeekFrom::Start(original_pos))?;
            Ok(true)
        }
        Err(e) if e.kind() == std::io::ErrorKind::Unsupported => {
            // Read not supported - check if write is supported
            match stream.write(&buf) {
                Ok(_) => {
                    stream.seek(SeekFrom::Start(original_pos))?;
                    Ok(false) // Write supported but read not
                }
                Err(_) => {
                    // Neither read nor write supported - unusual case
                    stream.seek(SeekFrom::Start(original_pos))?;
                    Ok(false)
                }
            }
        }
        Err(_) => {
            // Other read error - restore position and assume not reading
            stream.seek(SeekFrom::Start(original_pos))?;
            Ok(false)
        }
    }
}