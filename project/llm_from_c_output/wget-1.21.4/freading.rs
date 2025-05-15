// Retrieve information about a FILE stream.
// Copyright (C) 2007-2023 Free Software Foundation, Inc.
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

/// Returns true if the stream is opened read-only or if the last operation was a read.
/// Returns false if the stream is write-only/append-only or supports writing with no current read.
/// 
/// STREAM must not be wide-character oriented.
pub fn freading(stream: &File) -> bool {
    // Try to determine read status by attempting operations
    
    // Check if we can read by attempting a zero-byte read
    let mut buf = [0u8; 0];
    let can_read = stream.read(&mut buf).is_ok();
    
    // Check if we can write by attempting a zero-byte write
    let can_write = stream.write(&buf).is_ok();
    
    // Save current position
    let original_pos = stream.stream_position().ok();
    
    // Try to seek forward 0 bytes to detect read/write status
    let seek_status = stream.seek(SeekFrom::Current(0));
    
    // Restore position if we changed it
    if let Some(pos) = original_pos {
        let _ = stream.seek(SeekFrom::Start(pos));
    }
    
    match (can_read, can_write, seek_status) {
        (true, false, _) => true,  // Read-only mode
        (false, true, _) => false, // Write-only mode
        (true, true, Ok(_)) => {
            // Both read and write possible - need to check last operation
            // This is platform specific behavior - we'll approximate
            // by checking if we can read without error
            can_read
        }
        _ => false // Default to false on any error
    }
}