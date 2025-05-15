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
    dir: &CStr,
    base: &CStr,
    base_in_result: Option<&mut *const libc::c_char>,
) -> Option<CString> {
    let dir_bytes = dir.to_bytes();
    let base_bytes = base.to_bytes();

    let dir_path = Path::new(dir.to_str().unwrap_or(""));
    let dir_last_component = dir_path.components().last().unwrap_or(Component::RootDir);
    let dirbaselen = dir_last_component.as_os_str().len();
    let dirlen = dir_bytes.len() - (dir_bytes.len() - dir_last_component.as_os_str().as_bytes().len());

    let baselen = base_bytes.len();
    let mut sep = b'\0';

    if dirbaselen > 0 {
        if !dir_bytes.ends_with(&[b'/']) && !base_bytes.starts_with(&[b'/']) {
            sep = b'/';
        }
    } else if base_bytes.starts_with(&[b'/']) {
        sep = b'.';
    }

    let total_len = dirlen + (sep != b'\0') as usize + baselen + 1;
    let mut result = Vec::with_capacity(total_len);

    result.extend_from_slice(&dir_bytes[..dirlen]);
    if sep != b'\0' {
        result.push(sep);
    }
    
    let base_start = result.len();
    result.extend_from_slice(base_bytes);
    result.push(b'\0');

    let c_string = match CString::new(result) {
        Ok(s) => s,
        Err(_) => return None,
    };

    if let Some(base_ptr) = base_in_result {
        let base_ptr_slice = &c_string.as_bytes()[base_start..base_start + baselen];
        *base_ptr = base_ptr_slice.as_ptr() as *const libc::c_char;
    }

    Some(c_string)
}