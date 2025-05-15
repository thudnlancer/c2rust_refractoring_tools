// basename.rs -- return the last element in a file name

// Copyright (C) 1990, 1998-2001, 2003-2006, 2009-2021 Free Software
// Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::path::{Component, Path, PathBuf};
use std::ffi::OsStr;

fn base_len(base: &OsStr) -> usize {
    let base_str = base.to_string_lossy();
    let mut length = base_str.len();
    
    // Collapse trailing slashes
    while length > 0 && base_str.chars().nth(length - 1).unwrap() == '/' {
        length -= 1;
    }
    length
}

pub fn base_name(name: &str) -> PathBuf {
    let path = Path::new(name);
    let components: Vec<_> = path.components().collect();
    
    // If there is no last component, return the original path (root or empty)
    if components.is_empty() {
        return path.to_path_buf();
    }
    
    let last_component = components.last().unwrap();
    let base = match last_component {
        Component::Normal(p) => p,
        _ => return path.to_path_buf(),
    };
    
    let mut length = base_len(base);
    
    // If the last character is a slash, include one slash
    let base_str = base.to_string_lossy();
    if length < base_str.len() {
        length += 1;
    }
    
    // Handle file system prefixes (like drive letters on Windows)
    if path.has_root() || path.components().count() > 1 {
        let mut result = PathBuf::new();
        result.push(".");
        result.push("/");
        result.push(&base_str[..length]);
        result
    } else {
        PathBuf::from(&base_str[..length])
    }
}