// Binary mode I/O with checking
// Copyright 2017-2023 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io;
use std::process;

#[cfg(target_family = "unix")]
pub fn set_binary_mode(fd: i32, mode: i32) -> io::Result<()> {
    // On Unix-like systems, text and binary modes are the same
    Ok(())
}

#[cfg(target_family = "windows")]
pub fn set_binary_mode(fd: i32, mode: i32) -> io::Result<()> {
    use std::os::windows::io::FromRawHandle;
    use std::os::windows::io::AsRawHandle;
    use std::fs::File;
    
    let file = unsafe { File::from_raw_handle(fd as _) };
    let result = file.set_binary_mode(mode == O_BINARY);
    std::mem::forget(file); // Prevent closing the handle
    result
}

#[cfg(O_BINARY)]
pub fn xset_binary_mode_error() -> ! {
    eprintln!("failed to set file descriptor text/binary mode");
    process::exit(1);
}

#[cfg(not(O_BINARY))]
pub fn xset_binary_mode_error() {}

/// Set the mode of FD to MODE, which should be either O_TEXT or O_BINARY.
/// Report an error and exit if this fails.
pub fn xset_binary_mode(fd: i32, mode: i32) {
    if let Err(_) = set_binary_mode(fd, mode) {
        xset_binary_mode_error();
    }
}

// Constants for binary/text mode (typically defined in libc)
pub const O_BINARY: i32 = 0x8000;
pub const O_TEXT: i32 = 0x4000;