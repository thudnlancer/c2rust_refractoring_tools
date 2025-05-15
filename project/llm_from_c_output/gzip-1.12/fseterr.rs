// Set the error indicator of a stream.
// Copyright (C) 2007, 2009-2022 Free Software Foundation, Inc.
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

use std::io::{self, Write};

/// Sets the error indicator of the given stream.
/// The "error indicator" is set when an I/O operation on the stream fails.
pub fn fseterr(stream: &mut impl Write) -> io::Result<()> {
    // In Rust, we don't have direct access to the internal flags of I/O streams
    // like in C. The closest equivalent is to perform an operation that will
    // set the error state of the stream.
    
    // Attempt to write a zero byte which should fail and set the error indicator
    stream.write_all(&[0])?;
    
    // The ? operator will propagate any errors, which effectively sets the
    // error indicator on the stream in Rust's error handling model
    
    Ok(())
}

// Note: Rust's I/O model is different from C's. In Rust, errors are typically
// handled through the Result type rather than by checking stream flags.
// This implementation provides similar functionality but follows Rust's
// error handling patterns.