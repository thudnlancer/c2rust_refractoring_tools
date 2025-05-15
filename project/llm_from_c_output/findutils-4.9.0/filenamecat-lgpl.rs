/* Concatenate two arbitrary file names.

   Copyright (C) 1996-2007, 2009-2022 Free Software Foundation, Inc.

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
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::io;

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
/// Returns an error if memory allocation fails.
pub fn mfile_name_concat(
    dir: &Path,
    base: &OsStr,
    base_in_result: Option<&mut &OsStr>,
) -> io::Result<PathBuf> {
    let dir_components: Vec<_> = dir.components().collect();
    let dir_last_component = dir_components.last().map(|c| c.as_os_str()).unwrap_or_default();
    let dirbaselen = dir_last_component.len();
    let dirlen = dir.as_os_str().len() - dir_last_component.len() + dirbaselen;
    
    let base_bytes = base.as_bytes();
    let baselen = base_bytes.len();
    
    let mut sep = None;
    if dirbaselen > 0 {
        // DIR is not a file system root, so separate with / if needed
        let dir_bytes = dir.as_os_str().as_bytes();
        if !(dir_bytes[dirlen - 1] == b'/' || base_bytes[0] == b'/') {
            sep = Some(b'/');
        }
    } else if base_bytes[0] == b'/' {
        // DIR is a file system root and BASE begins with a slash
        sep = Some(b'.');
    }
    
    let sep_len = sep.is_some() as usize;
    let total_len = dirlen + sep_len + baselen;
    
    let mut result = Vec::with_capacity(total_len);
    result.extend_from_slice(&dir.as_os_str().as_bytes()[..dirlen]);
    
    if let Some(s) = sep {
        result.push(s);
    }
    
    let base_start = result.len();
    result.extend_from_slice(base_bytes);
    
    if let Some(base_ptr) = base_in_result {
        *base_ptr = OsStr::from_bytes(&result[base_start..]);
    }
    
    Ok(PathBuf::from(OsStr::from_bytes(&result)))
}