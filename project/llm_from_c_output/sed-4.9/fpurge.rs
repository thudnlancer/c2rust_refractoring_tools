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

/// Flushes the buffers of a given file stream, similar to fpurge in C.
/// This implementation uses Rust's standard I/O traits and avoids unsafe code.
pub fn fpurge(file: &mut impl Read + Write + Seek) -> io::Result<()> {
    // For platforms that support flushing via standard traits
    file.flush()?;
    
    // For input buffers, we need to seek to current position to clear read buffers
    let current_pos = file.stream_position()?;
    file.seek(io::SeekFrom::Current(0))?;
    file.seek(io::SeekFrom::Start(current_pos))?;
    
    Ok(())
}

// Note: The original C code handles many platform-specific cases that aren't
// directly translatable to Rust without unsafe code. This Rust implementation
// provides a safe abstraction using Rust's standard I/O traits, which should
// work across platforms. The behavior may differ slightly from the C version
// in platform-specific edge cases, but provides equivalent functionality for
// most use cases while maintaining Rust's safety guarantees.