// basename.rs -- return the last element in a file name

// Copyright (C) 1990, 1998-2001, 2003-2006, 2009-2022 Free Software
// Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::path::{Component, Path, PathBuf};
use std::ffi::OsStr;

/// Returns the base name of the given path, similar to the C function.
/// 
/// # Arguments
/// * `name` - A reference to a path string
/// 
/// # Returns
/// A new `PathBuf` containing just the base name component
pub fn base_name(name: &str) -> PathBuf {
    let path = Path::new(name);
    let mut components = path.components();
    
    let last_component = components.next_back()
        .unwrap_or_else(|| Component::Normal(OsStr::new("")));
    
    let base = match last_component {
        Component::Normal(s) if !s.is_empty() => {
            // Handle the case where we have a valid last component
            let base_str = s.to_str().unwrap_or("");
            let mut length = base_str.len();
            
            // Collapse trailing slashes
            if length > 0 && is_slash(base_str.chars().nth(length - 1).unwrap()) {
                length = base_str.trim_end_matches(is_slash).len() + 1;
            }
            
            // Handle drive letter case on Windows
            let dotslash_len = if cfg!(windows) && has_drive_prefix(base_str) {
                2
            } else {
                0
            };
            
            let mut result = String::with_capacity(dotslash_len + length + 1);
            if dotslash_len > 0 {
                result.push_str("./");
            }
            result.push_str(&base_str[..length]);
            PathBuf::from(result)
        }
        _ => {
            // Handle root or empty path case
            let base_str = path.to_str().unwrap_or("");
            let length = base_str.trim_end_matches(is_slash).len();
            PathBuf::from(&base_str[..length])
        }
    };
    
    base
}

/// Helper function to check if a character is a slash
fn is_slash(c: char) -> bool {
    c == '/' || (cfg!(windows) && c == '\\')
}

/// Helper function to check for Windows drive prefix
fn has_drive_prefix(s: &str) -> bool {
    if s.len() >= 2 {
        let mut chars = s.chars();
        let first = chars.next().unwrap();
        let second = chars.next().unwrap();
        first.is_ascii_alphabetic() && second == ':'
    } else {
        false
    }
}