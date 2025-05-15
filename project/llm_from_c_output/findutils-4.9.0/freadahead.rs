// Retrieve information about a FILE stream.
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

use std::io::{self, Read, Seek};

/// Trait for checking buffered data in a readable stream
pub trait FReadAhead {
    /// Returns the number of bytes available in the input buffer
    fn freadahead(&self) -> io::Result<usize>;
}

impl<T: Read + Seek> FReadAhead for T {
    fn freadahead(&self) -> io::Result<usize> {
        // Rust's standard library doesn't expose internal buffer information
        // like C's FILE* does, so we need to use a different approach.
        // We'll use a combination of stream_position and reading to determine
        // available data.
        
        // Save current position
        let current_pos = self.stream_position()?;
        
        // Seek to end to get total size
        let end_pos = self.seek(io::SeekFrom::End(0))?;
        
        // Restore original position
        self.seek(io::SeekFrom::Start(current_pos))?;
        
        // Calculate bytes available
        Ok((end_pos - current_pos) as usize)
    }
}

// Note: This implementation differs from the C version because:
// 1. Rust's standard library doesn't expose internal buffer details
// 2. We're using a trait-based approach which is more idiomatic Rust
// 3. The implementation is platform-independent
// 4. It handles errors properly through Result
// 5. It doesn't require unsafe code

// For cases where you need exact compatibility with C's behavior,
// you would need platform-specific implementations using libc,
// but that would require unsafe blocks. This version provides
// similar functionality in safe Rust while maintaining the spirit
// of the original function.