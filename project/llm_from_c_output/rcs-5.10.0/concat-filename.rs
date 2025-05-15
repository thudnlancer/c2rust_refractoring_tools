//! Construct a full filename from a directory and a relative filename.
//! Copyright (C) 2001-2004, 2007-2020 Free Software Foundation, Inc.
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation; either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::io;

/// Checks if a character is a directory separator
fn is_slash(c: char) -> bool {
    c == '/' || c == '\\'
}

/// Concatenate a directory filename, a relative filename and an optional
/// suffix. The directory may end with the directory separator. The second
/// argument may not start with the directory separator (it is relative).
/// Returns a freshly allocated filename or an error upon memory allocation failure.
pub fn concatenated_filename(
    directory: &str,
    filename: &str,
    suffix: Option<&str>,
) -> io::Result<PathBuf> {
    if filename.starts_with(is_slash) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Filename must be relative",
        ));
    }

    let mut result = if directory == "." {
        PathBuf::from(filename)
    } else {
        let mut path = PathBuf::from(directory);
        let needs_slash = !directory.ends_with(is_slash)
            && directory.len() > 0
            && filename.len() > 0;

        if needs_slash {
            path.push("/");
        }
        path.push(filename);
        path
    };

    if let Some(suffix) = suffix {
        result.push(suffix);
    }

    Ok(result)
}

/// Concatenate a directory filename, a relative filename and an optional
/// suffix. The directory may end with the directory separator. The second
/// argument may not start with the directory separator (it is relative).
/// Returns a freshly allocated filename.
/// 
/// # Panics
/// Panics if memory allocation fails.
pub fn xconcatenated_filename(
    directory: &str,
    filename: &str,
    suffix: Option<&str>,
) -> PathBuf {
    concatenated_filename(directory, filename, suffix)
        .expect("Memory allocation failed")
}