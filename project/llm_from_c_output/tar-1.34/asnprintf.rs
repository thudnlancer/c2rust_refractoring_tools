// Formatted output to strings.
// Copyright (C) 1999, 2002, 2006, 2009-2021 Free Software Foundation, Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, see <https://www.gnu.org/licenses/>.

use std::fmt;
use std::io::{self, Write};

pub fn asnprintf(
    resultbuf: Option<&mut Vec<u8>>,
    lengthp: Option<&mut usize>,
    format: &str,
    args: fmt::Arguments,
) -> io::Result<Vec<u8>> {
    let mut buffer = match resultbuf {
        Some(buf) => {
            buf.clear();
            buf
        }
        None => Vec::new(),
    };

    write!(&mut buffer, "{}", args)?;

    if let Some(len) = lengthp {
        *len = buffer.len();
    }

    Ok(buffer)
}

#[macro_export]
macro_rules! asnprintf {
    ($resultbuf:expr, $lengthp:expr, $format:expr) => {
        $crate::asnprintf($resultbuf, $lengthp, $format, format_args!($format))
    };
    ($resultbuf:expr, $lengthp:expr, $format:expr, $($arg:tt)*) => {
        $crate::asnprintf($resultbuf, $lengthp, $format, format_args!($format, $($arg)*))
    };
}