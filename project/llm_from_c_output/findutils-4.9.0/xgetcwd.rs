// Copyright (C) 1995, 2001, 2003, 2009-2022 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// xgetcwd -- return current directory with unlimited length
//
// Copyright (C) 2001, 2003-2004, 2006-2007, 2009-2022 Free Software
// Foundation, Inc.
//
// Written by Jim Meyering.

use std::env;
use std::io;

/// Return the current directory, newly allocated.
/// Upon an out-of-memory error, panic (like xalloc_die in C).
/// Upon any other type of error, return None with errno set.
pub fn xgetcwd() -> Option<String> {
    match env::current_dir() {
        Ok(path) => Some(path.to_string_lossy().into_owned()),
        Err(e) => {
            if e.kind() == io::ErrorKind::OutOfMemory {
                panic!("Out of memory while getting current directory");
            }
            None
        }
    }
}