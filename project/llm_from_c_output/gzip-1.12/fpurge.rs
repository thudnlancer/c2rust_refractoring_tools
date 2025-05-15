// Flushing buffers of a FILE stream.
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

use std::io::{self, Write, Read, Seek};
use std::fs::File;

/// Flushes the buffers of a file stream, equivalent to C's fpurge function.
///
/// # Arguments
/// * `file` - A mutable reference to a File object
///
/// # Returns
/// * `io::Result<()>` - Returns Ok(()) on success, or an io::Error on failure
pub fn fpurge(file: &mut File) -> io::Result<()> {
    // On Unix-like systems, we can use flush() to clear write buffers
    // and seek to current position to clear read buffers
    file.flush()?;
    
    // Get current position to maintain it after clearing buffers
    let pos = file.stream_position()?;
    
    // Seek to current position to clear read buffers
    file.seek(io::SeekFrom::Start(pos))?;
    
    Ok(())
}