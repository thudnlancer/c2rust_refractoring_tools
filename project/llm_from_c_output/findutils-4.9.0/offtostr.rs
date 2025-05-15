/* Convert 'off_t' integer to printable string.

   Copyright (C) 2004-2022 Free Software Foundation, Inc.

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

use std::io::{self, Write};

/// Converts an off_t integer to a printable string
pub fn offtostr(value: off_t) -> String {
    value.to_string()
}

type off_t = i64; // Assuming off_t is 64-bit signed integer