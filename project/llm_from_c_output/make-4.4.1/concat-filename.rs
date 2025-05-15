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
use std::io;
use std::path::{Path, PathBuf};

/// Checks if a character is a path separator
fn is_slash(c: char) -> bool {
    c == '/' || c == '\\'
}

/// Gets the length of the filesystem prefix if any
fn file_system_prefix_len(path: &str) -> usize {
    if path.starts_with(r"\\?\") || path.starts_with(r"\\.\") {
        4
    } else if path.starts_with(r"\\") {
        2
    } else if path.len() >= 2 && path.chars().nth(1) == Some(':') {
        2
    } else {
        0
    }
}

/// Concatenate a directory filename, a relative filename and an optional suffix.
/// The directory may end with the directory separator. The second argument may not
/// start with the directory separator (it is relative).
/// Returns a PathBuf or an IO error upon memory allocation failure.
pub fn concatenated_filename(
    directory: &str,
    filename: &str,
    suffix: Option<&str>,
) -> io::Result<PathBuf> {
    let mut result = PathBuf::new();

    if directory == "." {
        // No need to prepend the directory
        result.push(filename);
    } else {
        let directory_len = directory.len();
        let need_slash = directory_len > file_system_prefix_len(directory)
            && !directory.ends_with(is_slash);

        result.push(directory);
        if need_slash {
            result.push("/");
        }
        result.push(filename);
    }

    if let Some(s) = suffix {
        result.push(s);
    }

    Ok(result)
}

/// Concatenate a directory filename, a relative filename and an optional suffix.
/// The directory may end with the directory separator. The second argument may not
/// start with the directory separator (it is relative).
/// Returns a PathBuf (guaranteed to not be null).
pub fn xconcatenated_filename(
    directory: &str,
    filename: &str,
    suffix: Option<&str>,
) -> PathBuf {
    concatenated_filename(directory, filename, suffix).expect("Memory allocation failed")
}