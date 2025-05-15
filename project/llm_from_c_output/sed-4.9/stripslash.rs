// stripslash.rs -- remove redundant trailing slashes from a file name

// Original Copyright (C) 1990, 2001, 2003-2006, 2009-2022 Free Software Foundation,
// Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Remove trailing slashes from FILE. Return true if a trailing slash
// was removed. This is useful when using file name completion from a
// shell that adds a "/" after directory names (such as tcsh and
// bash), because on symlinks to directories, several system calls
// have different semantics according to whether a trailing slash is
// present.

use std::path::{Component, Path, PathBuf};

pub fn strip_trailing_slashes(file: &mut PathBuf) -> bool {
    let mut had_slash = false;
    
    // Normalize the path first to handle redundant slashes
    let normalized = file.clone().into_os_string().into_string().unwrap();
    let mut normalized = PathBuf::from(normalized);
    
    // Get the last component
    let last_component = normalized.components().last();
    
    match last_component {
        Some(Component::RootDir) => {
            // For root directory, we want to keep exactly one slash
            normalized = PathBuf::from("/");
            had_slash = true;
        },
        Some(_) => {
            // For non-root paths, remove trailing slashes
            let s = normalized.to_str().unwrap();
            let trimmed = s.trim_end_matches('/');
            had_slash = trimmed.len() < s.len();
            normalized = PathBuf::from(trimmed);
        },
        None => {
            // Empty path, nothing to do
        }
    }
    
    *file = normalized;
    had_slash
}