// xgetcwd.rs -- return current directory with unlimited length

// Copyright (C) 2001, 2003-2004, 2006-2007, 2009-2021 Free Software
// Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Jim Meyering.

// Return the current directory, newly allocated.
// Upon an out-of-memory error, call xalloc_die.
// Upon any other type of error, return None with errno set.

use std::env;
use std::io;

pub fn xgetcwd() -> Option<String> {
    match env::current_dir() {
        Ok(path) => Some(path.to_string_lossy().into_owned()),
        Err(e) => {
            if e.kind() == io::ErrorKind::OutOfMemory {
                // In Rust, we can't directly match to ENOMEM as in C,
                // but OutOfMemory is the closest equivalent
                xalloc_die();
            }
            None
        }
    }
}

// Placeholder for xalloc_die function
// In a real implementation, this would handle out-of-memory conditions
fn xalloc_die() -> ! {
    panic!("Out of memory");
}