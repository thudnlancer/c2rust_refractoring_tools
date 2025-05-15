/* Counting the multibyte characters in a string.
   Copyright (C) 2007-2022 Free Software Foundation, Inc.
   Written by Bruno Haible <bruno@clisp.org>, 2007.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation, either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::env;
use std::str;

/// Return the number of multibyte characters in the character string STRING.
pub fn mbslen(string: &str) -> usize {
    match env::var_os("LC_CTYPE")
        .and_then(|s| s.into_string().ok())
        .as_deref()
    {
        Some("C") | Some("POSIX") => string.len(),
        _ => string.chars().count(),
    }
}