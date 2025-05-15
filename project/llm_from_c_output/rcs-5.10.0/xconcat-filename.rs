/* Construct a full filename from a directory and a relative filename.
   Copyright (C) 2001-2004, 2006-2020 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by the
   Free Software Foundation; either version 3 of the License, or any
   later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Bruno Haible <haible@clisp.cons.org>.  */

use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::io;

/* Concatenate a directory filename, a relative filename and an optional
   suffix.  The directory may end with the directory separator.  The second
   argument may not start with the directory separator (it is relative).
   Return a freshly allocated filename.  */
pub fn xconcatenated_filename(
    directory: &str,
    filename: &str,
    suffix: Option<&str>,
) -> Result<PathBuf, io::Error> {
    let mut path = PathBuf::from(directory);
    path.push(filename);
    
    if let Some(suffix) = suffix {
        let mut os_string = OsString::from(path);
        os_string.push(suffix);
        path = PathBuf::from(os_string);
    }
    
    Ok(path)
}