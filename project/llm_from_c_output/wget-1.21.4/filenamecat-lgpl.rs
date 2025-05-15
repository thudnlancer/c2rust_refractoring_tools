/* Concatenate two arbitrary file names.

   Copyright (C) 1996-2007, 2009-2023 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Jim Meyering.  */

use std::path::{Component, Path, PathBuf};
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::io;

/// Checks if a character is a slash
fn is_slash(c: u8) -> bool {
    c == b'/'
}

/// Returns the length of the base component of a path
fn base_len(path: &[u8]) -> usize {
    if let Some(last_slash) = path.iter().rposition(|&c| is_slash(c)) {
        path.len() - last_slash - 1
    } else {
        path.len()
    }
}

/// Returns the last component of a path
fn last_component(path: &[u8]) -> &[u8] {
    if let Some(last_slash) = path.iter().rposition(|&c| is_slash(c)) {
        &path[last_slash + 1..]
    } else {
        path
    }
}

/// Concatenate two file name components
pub fn mfile_name_concat(
    dir: &[u8],
    base: &[u8],
    base_in_result: Option<&mut &[u8]>,
) -> io::Result<Vec<u8>> {
    let dirbase = last_component(dir);
    let dirbaselen = base_len(dirbase);
    let dirlen = (dirbase.as_ptr() as usize - dir.as_ptr() as usize) + dirbaselen;
    let baselen = base.len();
    
    let mut sep = None;
    
    if dirbaselen > 0 {
        // DIR is not a file system root, so separate with / if needed
        if !is_slash(dir[dirlen - 1]) && !base.is_empty() && !is_slash(base[0]) {
            sep = Some(b'/');
        }
    } else if !base.is_empty() && is_slash(base[0]) {
        // DIR is a file system root and BASE begins with a slash
        sep = Some(b'.');
    }
    
    let total_len = dirlen + sep.map_or(0, |_| 1) + baselen;
    let mut result = Vec::with_capacity(total_len + 1);
    
    // Copy dir part
    result.extend_from_slice(&dir[..dirlen]);
    
    // Add separator if needed
    if let Some(s) = sep {
        result.push(s);
    }
    
    // Set base_in_result if requested
    let base_start = result.len();
    result.extend_from_slice(base);
    
    if let Some(bir) = base_in_result {
        *bir = &result[base_start..];
    }
    
    Ok(result)
}