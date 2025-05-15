// basename.rs -- return the last element in a file name

// Copyright (C) 1990, 1998-2001, 2003-2006, 2009-2023 Free Software
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

/// Returns the base name of a file path, similar to the C function `base_name`.
/// 
/// # Arguments
/// * `name` - The input file path as a string slice
/// 
/// # Returns
/// A `PathBuf` containing the base name of the path
pub fn base_name(name: &str) -> PathBuf {
    let path = Path::new(name);
    let mut components = path.components();
    
    // Get the last component
    let last_component = components.next_back().unwrap_or(Component::RootDir);
    
    // Handle empty path or root
    if last_component == Component::RootDir || name.is_empty() {
        return if name.is_empty() {
            PathBuf::new()
        } else {
            PathBuf::from(name)
        };
    }
    
    // Handle drive letters or prefixes (Windows-specific behavior)
    let needs_dotslash = if cfg!(windows) {
        let prefix = path.prefix();
        prefix.is_some() && path.components().count() > 1
    } else {
        false
    };
    
    // Construct the result path
    let mut result = PathBuf::new();
    if needs_dotslash {
        result.push("./");
    }
    result.push(last_component);
    
    // Collapse trailing slashes
    let result_str = result.to_string_lossy().into_owned();
    let trimmed = result_str.trim_end_matches(std::path::is_separator);
    let final_str = if trimmed.is_empty() {
        "/"
    } else {
        trimmed
    };
    
    PathBuf::from(final_str)
}