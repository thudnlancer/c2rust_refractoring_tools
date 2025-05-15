//! Construct a full filename from a directory and a relative filename.
//! Copyright (C) 2001-2004, 2007-2023 Free Software Foundation, Inc.
//!
//! This file is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Lesser General Public License as
//! published by the Free Software Foundation; either version 2.1 of the
//! License, or (at your option) any later version.
//!
//! This file is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU Lesser General Public License for more details.
//!
//! You should have received a copy of the GNU Lesser General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::io;

/// Concatenate a directory filename, a relative filename and an optional suffix.
/// The directory may end with the directory separator. The second argument may
/// not start with the directory separator (it is relative).
/// Returns a newly allocated PathBuf or an error upon memory allocation failure.
pub fn concatenated_filename(
    directory: &str,
    filename: &str,
    suffix: Option<&str>,
) -> io::Result<PathBuf> {
    let mut path = if directory == "." {
        PathBuf::new()
    } else {
        let mut path = PathBuf::from(directory);
        let needs_slash = !directory.ends_with(std::path::MAIN_SEPARATOR)
            && !directory.is_empty();
        if needs_slash {
            path.push(std::path::MAIN_SEPARATOR.to_string());
        }
        path
    };

    path.push(filename);
    if let Some(suffix) = suffix {
        path.push(suffix);
    }

    Ok(path)
}

/// Concatenate a directory filename, a relative filename and an optional suffix.
/// The directory may end with the directory separator. The second argument may
/// not start with the directory separator (it is relative).
/// Returns a newly allocated PathBuf (guaranteed to succeed).
pub fn xconcatenated_filename(
    directory: &str,
    filename: &str,
    suffix: Option<&str>,
) -> PathBuf {
    concatenated_filename(directory, filename, suffix)
        .expect("Memory allocation failed in xconcatenated_filename")
}