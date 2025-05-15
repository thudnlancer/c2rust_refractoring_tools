// Formatted output to a stream.
// Copyright (C) 2004, 2006-2022 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fmt;
use std::io::{self, Write};
use std::convert::TryInto;

/// Print formatted output to the given writer.
/// Returns the string length of formatted string or an error.
pub fn fprintf<W: Write>(fp: &mut W, format: fmt::Arguments) -> Result<usize, io::Error> {
    let mut output = String::new();
    fmt::write(&mut output, format).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, e)
    })?;

    let len = output.len();
    fp.write_all(output.as_bytes())?;

    if len > std::i32::MAX as usize {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "formatted string length exceeds INT_MAX",
        ));
    }

    Ok(len)
}