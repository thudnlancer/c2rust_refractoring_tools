/* Program name management.
   Copyright (C) 2016-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as published by
   the Free Software Foundation; either version 2.1 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::env;
use std::ffi::CString;
use std::os::unix::ffi::OsStringExt;
use std::path::Path;

#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;

/// Returns the base name of the executing program.
/// On native Windows this will usually end in ".exe" or ".EXE".
pub fn getprogname() -> &'static str {
    if let Ok(progname) = env::current_exe() {
        if let Some(name) = progname.file_name() {
            if let Some(name_str) = name.to_str() {
                return name_str;
            }
        }
    }
    "?"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getprogname() {
        let progname = getprogname();
        assert!(!progname.is_empty());
        assert_ne!(progname, "?");
    }
}