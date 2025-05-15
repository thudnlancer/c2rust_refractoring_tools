// -*- buffer-read-only: t -*- vi: set ro:
// DO NOT EDIT! GENERATED AUTOMATICALLY!
// stripslash.rs -- remove redundant trailing slashes from a file name

// Copyright (C) 1990, 2001, 2003-2006 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// Remove trailing slashes from FILE. Return true if a trailing slash
// was removed. This is useful when using file name completion from a
// shell that adds a "/" after directory names (such as tcsh and
// bash), because on symlinks to directories, several system calls
// have different semantics according to whether a trailing slash is
// present.

/// Removes trailing slashes from the given path string.
/// Returns true if any trailing slashes were removed.
pub fn strip_trailing_slashes(file: &mut String) -> bool {
    let path = file.as_str();
    
    // Find the last component of the path
    let last_comp = last_component(path);
    
    // Handle filesystem roots (empty last component)
    let base = if last_comp.is_empty() {
        path
    } else {
        last_comp
    };
    
    let base_len = base_len(base);
    let base_end = base.len().min(base_len);
    let had_slash = path.len() > base_end;
    
    if had_slash {
        file.truncate(base_end);
    }
    
    had_slash
}

/// Returns the last component of the path
fn last_component(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or("")
}

/// Returns the length of the base component (without trailing slashes)
fn base_len(base: &str) -> usize {
    base.trim_end_matches('/').len()
}