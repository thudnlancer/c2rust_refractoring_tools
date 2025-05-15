/* Concatenate two arbitrary file names.

   Copyright (C) 1996-2007, 2009-2021 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Jim Meyering.  */

use std::path::{Component, Path, PathBuf};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

/// Concatenate two file name components, DIR and BASE, in
/// newly-allocated storage and return the result.
/// The resulting file name F is such that the commands "ls F" and "(cd
/// DIR; ls ./BASE)" refer to the same file. If necessary, put
/// a separator between DIR and BASE in the result. Typically this
/// separator is "/", but in rare cases it might be ".".
/// In any case, if BASE_IN_RESULT is Some, set
/// *BASE_IN_RESULT to point to the copy of BASE at the end of the
/// returned concatenation.
///
/// Returns None if memory allocation fails.
pub fn mfile_name_concat(
    dir: &Path,
    base: &OsStr,
    base_in_result: Option<&mut &OsStr>,
) -> Option<PathBuf> {
    let dir_components: Vec<_> = dir.components().collect();
    let dirbase = dir_components.last().unwrap_or(&Component::CurDir);
    
    let dirbaselen = dirbase.as_os_str().as_bytes().len();
    let dirlen = dir.as_os_str().as_bytes().len() - dirbase.as_os_str().as_bytes().len() + dirbaselen;
    let baselen = base.as_bytes().len();
    
    let mut sep = None;
    if dirbaselen > 0 {
        // DIR is not a file system root, so separate with / if needed
        let dir_bytes = dir.as_os_str().as_bytes();
        if !(dir_bytes[dirlen - 1] == b'/') && !(base.as_bytes()[0] == b'/') {
            sep = Some(b'/');
        }
    } else if base.as_bytes().get(0) == Some(&b'/') {
        // DIR is a file system root and BASE begins with a slash
        sep = Some(b'.');
    }
    
    let total_len = dirlen + sep.map_or(0, |_| 1) + baselen;
    let mut result = Vec::with_capacity(total_len);
    
    // Copy dir part
    result.extend_from_slice(&dir.as_os_str().as_bytes()[..dirlen]);
    
    // Add separator if needed
    if let Some(s) = sep {
        result.push(s);
    }
    
    // Set base_in_result pointer if requested
    let base_start = result.len();
    result.extend_from_slice(base.as_bytes());
    
    if let Some(ptr) = base_in_result {
        *ptr = OsStr::from_bytes(&result[base_start..]);
    }
    
    Some(PathBuf::from(OsStr::from_bytes(&result)))
}