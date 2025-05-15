/* fflush.rs -- allow flushing input streams
   Copyright (C) 2007-2022 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::io::{self, Seek, SeekFrom, Write};
use std::fs::File;
use std::os::unix::io::AsRawFd;

/// Flush all pending data on STREAM according to POSIX rules.
pub fn rpl_fflush(stream: Option<&mut File>) -> io::Result<()> {
    match stream {
        None => {
            // When stream is NULL, flush all output streams
            io::stdout().flush()?;
            io::stderr().flush()
        }
        Some(file) => {
            if !is_reading(file)? {
                // For output streams, just flush normally
                file.flush()
            } else {
                // For input streams, handle according to POSIX rules
                flush_input_stream(file)
            }
        }
    }
}

fn is_reading(file: &File) -> io::Result<bool> {
    let metadata = file.metadata()?;
    Ok(!metadata.permissions().readonly())
}

fn flush_input_stream(file: &mut File) -> io::Result<()> {
    let pos = file.stream_position()?;
    
    // Clear the ungetc buffer equivalent
    clear_ungetc_buffer(file)?;
    
    // Flush the buffer
    file.flush()?;
    
    // Restore position
    file.seek(SeekFrom::Start(pos))?;
    
    Ok(())
}

fn clear_ungetc_buffer(file: &mut File) -> io::Result<()> {
    // Equivalent to fseeko(fp, 0, SEEK_CUR) to clear ungetc buffer
    file.seek(SeekFrom::Current(0))
}

// Note: The original C code had platform-specific implementations.
// This Rust version provides a simplified cross-platform implementation.
// For full compatibility, platform-specific code would need to be added.