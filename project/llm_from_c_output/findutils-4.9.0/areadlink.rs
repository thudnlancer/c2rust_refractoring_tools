// Read symbolic links without size limitation.

// Copyright (C) 2001, 2003-2004, 2007, 2009-2022 Free Software Foundation,
// Inc.

// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.

// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Jim Meyering <jim@meyering.net>

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Read symbolic link and return its target path.
/// Similar to the C function `areadlink`, but returns a `PathBuf` instead of a C string.
/// Returns `io::Result<PathBuf>` which will contain the error if the operation fails.
pub fn areadlink(filename: &Path) -> io::Result<PathBuf> {
    fs::read_link(filename)
}

#[cfg(feature = "areadlinkat")]
/// Read symbolic link at given directory file descriptor and return its target path.
/// Similar to the C function `areadlinkat`.
/// Returns `io::Result<PathBuf>` which will contain the error if the operation fails.
pub fn areadlinkat(dirfd: &fs::File, filename: &Path) -> io::Result<PathBuf> {
    use std::os::unix::fs::FileExt;
    let cwd = std::env::current_dir()?;
    std::env::set_current_dir(dirfd)?;
    let result = fs::read_link(filename);
    std::env::set_current_dir(cwd)?;
    result
}

#[cfg(feature = "areadlinkat_with_size")]
/// Read symbolic link at given directory file descriptor with size hint and return its target path.
/// Similar to the C function `areadlinkat_with_size`.
/// The size hint is ignored in this implementation as Rust's `read_link` handles allocation.
/// Returns `io::Result<PathBuf>` which will contain the error if the operation fails.
pub fn areadlinkat_with_size(dirfd: &fs::File, filename: &Path, _size_hint: usize) -> io::Result<PathBuf> {
    areadlinkat(dirfd, filename)
}

// The following is the Rust equivalent of the careadlinkatcwd function,
// but in Rust we don't need it as separate since fs::read_link handles everything.
// The original C implementation's buffer management is handled automatically by Rust's PathBuf.