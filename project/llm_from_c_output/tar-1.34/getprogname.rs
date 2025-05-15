// Program name management.
// Copyright (C) 2016-2021 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::ffi::OsString;
use std::path::Path;

/// Returns the base name of the executing program.
/// On Windows this will usually end in ".exe" or ".EXE".
pub fn getprogname() -> Option<String> {
    env::args_os()
        .next()
        .as_ref()
        .map(OsString::as_os_str)
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(|os_str| os_str.to_str())
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getprogname() {
        let progname = getprogname();
        assert!(progname.is_some());
        let name = progname.unwrap();
        assert!(!name.is_empty());
    }
}