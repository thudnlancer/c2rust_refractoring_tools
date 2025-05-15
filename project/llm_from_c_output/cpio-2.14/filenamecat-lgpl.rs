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

use std::path::{Path, Component};
use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;
use std::borrow::Cow;

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
    dir: &str,
    base: &str,
    base_in_result: Option<&mut &str>,
) -> Option<String> {
    let dir_path = Path::new(dir);
    let dir_components: Vec<_> = dir_path.components().collect();
    let dirbase = dir_components.last().map(|c| c.as_os_str().as_bytes()).unwrap_or(&[]);
    let dirbaselen = base_len(dirbase);
    
    let dirbase_pos = dir.rfind(std::str::from_utf8(dirbase).ok()?).unwrap_or(0);
    let dirlen = dirbase_pos + dirbaselen;
    
    let baselen = base.len();
    let mut sep = None;
    
    if dirbaselen > 0 {
        // DIR is not a file system root, so separate with / if needed
        if !dir[..dirlen].ends_with('/') && !base.starts_with('/') {
            sep = Some('/');
        }
    } else if base.starts_with('/') {
        // DIR is a file system root and BASE begins with a slash
        sep = Some('.');
    }
    
    let sep_len = sep.is_some() as usize;
    let total_len = dirlen + sep_len + baselen;
    
    let mut result = String::with_capacity(total_len);
    result.push_str(&dir[..dirlen]);
    
    if let Some(s) = sep {
        result.push(s);
    }
    
    let base_start = result.len();
    result.push_str(base);
    
    if let Some(base_ptr) = base_in_result {
        *base_ptr = &result[base_start..];
    }
    
    Some(result)
}

fn base_len(base: &[u8]) -> usize {
    let len = base.len();
    if len > 1 && base[len - 1] == b'/' {
        // Skip trailing slash
        len - 1
    } else {
        len
    }
}

fn is_slash(c: u8) -> bool {
    c == b'/'
}