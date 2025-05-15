// Formatted output to strings.
// Copyright (C) 1999, 2002, 2006, 2009-2022 Free Software Foundation, Inc.
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

use std::fmt;
use std::io::{self, Write};

pub fn asnprintf(
    resultbuf: Option<&mut Vec<u8>>,
    lengthp: Option<&mut usize>,
    format: fmt::Arguments,
) -> io::Result<Vec<u8>> {
    let mut buffer = match resultbuf {
        Some(buf) => {
            buf.clear();
            buf.clone()
        }
        None => Vec::new(),
    };

    buffer.write_fmt(format)?;

    if let Some(len) = lengthp {
        *len = buffer.len();
    }

    Ok(buffer)
}

#[macro_export]
macro_rules! asnprintf {
    ($resultbuf:expr, $lengthp:expr, $($arg:tt)*) => {
        $crate::asnprintf($resultbuf, $lengthp, format_args!($($arg)*))
    };
}