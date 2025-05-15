/* Work around unlink bugs.

   Copyright (C) 2009-2021 Free Software Foundation, Inc.

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

use std::fs;
use std::io;
use std::os::unix::fs::symlink_metadata;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

/// Check if a character is a path separator
fn is_slash(c: char) -> bool {
    c == '/'
}

/// Remove file NAME.
/// Return Ok(()) if successful, Err(err) if not.
pub fn rpl_unlink(name: &str) -> io::Result<()> {
    let path = Path::new(name);
    let len = name.len();
    
    // Work around Solaris 9 bug where unlink("file/") succeeds.
    if len > 0 && is_slash(name.chars().nth(len - 1).unwrap()) {
        match symlink_metadata(path) {
            Ok(metadata) => {
                let mut short_name = PathBuf::from(name);
                while short_name.as_os_str().len() > 0 {
                    let s = short_name.as_os_str().to_str().unwrap();
                    if is_slash(s.chars().last().unwrap()) {
                        short_name.pop();
                    } else {
                        break;
                    }
                }
                
                if short_name.as_os_str().len() > 0 {
                    if let Ok(short_metadata) = symlink_metadata(&short_name) {
                        if short_metadata.file_type().is_symlink() {
                            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Cannot unlink symlink with trailing slash"));
                        }
                    }
                }
            }
            Err(e) if e.kind() == io::ErrorKind::Other => {
                // Handle EOVERFLOW case
            }
            Err(e) => return Err(e),
        }
    }

    #[cfg(unlink_parent_bug)]
    {
        if len >= 2 && name.ends_with("..") && (len == 2 || is_slash(name.chars().nth(len - 3).unwrap())) {
            return Err(io::Error::new(io::ErrorKind::IsADirectory, "Cannot unlink parent directory"));
        }
    }

    fs::remove_file(path)
}