/* -*- buffer-read-only: t -*- vi: set ro: */
/* DO NOT EDIT! GENERATED AUTOMATICALLY! */
/* Formatted output to strings.
   Copyright (C) 1999, 2002, 2006 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License along
   with this program; if not, write to the Free Software Foundation,
   Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.  */

use std::fmt;
use std::io::{self, Write};
use std::mem;

pub fn asnprintf(
    resultbuf: Option<&mut Vec<u8>>,
    lengthp: Option<&mut usize>,
    format: fmt::Arguments,
) -> io::Result<Vec<u8>> {
    let mut output = Vec::new();
    output.write_fmt(format)?;

    if let Some(buf) = resultbuf {
        buf.clear();
        buf.extend_from_slice(&output);
    }

    if let Some(len) = lengthp {
        *len = output.len();
    }

    Ok(output)
}

#[macro_export]
macro_rules! asnprintf {
    ($resultbuf:expr, $lengthp:expr, $($arg:tt)*) => {
        $crate::asnprintf($resultbuf, $lengthp, format_args!($($arg)*))
    };
}