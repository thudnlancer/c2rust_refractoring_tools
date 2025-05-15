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

use std::path::{Path, Component};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::borrow::Cow;

/// Checks if a byte is a slash character
fn is_slash(c: u8) -> bool {
    c == b'/'
}

/// Returns the last component of a path
fn last_component(path: &OsStr) -> &OsStr {
    Path::new(path).components().last()
        .map(|c| match c {
            Component::Normal(p) => p.as_os_str(),
            Component::RootDir => OsStr::new("/"),
            Component::CurDir => OsStr::new("."),
            Component::ParentDir => OsStr::new(".."),
            Component::Prefix(_) => path, // shouldn't happen on Unix
        })
        .unwrap_or(OsStr::new(""))
}

/// Returns the length of the base component
fn base_len(base: &OsStr) -> usize {
    base.as_bytes().len()
}

/// Concatenate two file name components
pub fn mfile_name_concat(
    dir: &OsStr,
    base: &OsStr,
    base_in_result: Option<&mut Option<Cow<'_, OsStr>>>,
) -> Result<Cow<'static, OsStr>, std::io::Error> {
    let dir_bytes = dir.as_bytes();
    let base_bytes = base.as_bytes();
    
    let dirbase = last_component(dir);
    let dirbase_bytes = dirbase.as_bytes();
    let dirbaselen = base_len(dirbase);
    
    let dirbase_pos = dirbase_bytes.as_ptr() as usize - dir_bytes.as_ptr() as usize;
    let dirlen = dirbase_pos + dirbaselen;
    let baselen = base_bytes.len();
    
    let mut sep = None;
    
    if dirbaselen > 0 {
        // DIR is not a file system root, so separate with / if needed
        if !dir_bytes.is_empty() && !is_slash(dir_bytes[dirlen - 1]) && 
           !base_bytes.is_empty() && !is_slash(base_bytes[0]) {
            sep = Some(b'/');
        }
    } else if !base_bytes.is_empty() && is_slash(base_bytes[0]) {
        // DIR is a file system root and BASE begins with a slash
        sep = Some(b'.');
    }
    
    let sep_len = if sep.is_some() { 1 } else { 0 };
    let total_len = dirlen + sep_len + baselen;
    
    let mut result = Vec::with_capacity(total_len);
    result.extend_from_slice(&dir_bytes[..dirlen]);
    
    if let Some(s) = sep {
        result.push(s);
    }
    
    let base_start = result.len();
    result.extend_from_slice(base_bytes);
    
    if let Some(base_ptr) = base_in_result {
        *base_ptr = Some(Cow::Borrowed(OsStr::from_bytes(&result[base_start..])));
    }
    
    Ok(Cow::Owned(OsStr::from_bytes(&result).to_owned()))
}