// Copy piece of UTF-8 string.
// Copyright (C) 2002, 2006, 2009-2023 Free Software Foundation, Inc.
// Written by Bruno Haible <bruno@clisp.org>, 2002.
//
// This file is free software.
// It is dual-licensed under "the GNU LGPLv3+ or the GNU GPLv2+".
// You can redistribute it and/or modify it under either
//   - the terms of the GNU Lesser General Public License as published
//     by the Free Software Foundation, either version 3, or (at your
//     option) any later version, or
//   - the terms of the GNU General Public License as published by the
//     Free Software Foundation; either version 2, or (at your option)
//     any later version, or
//   - the same dual license "the GNU LGPLv3+ or the GNU GPLv2+".
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License and the GNU General Public License
// for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License and of the GNU General Public License along with this
// program.  If not, see <https://www.gnu.org/licenses/>.

/// Copies a piece of UTF-8 string from source to destination.
///
/// # Arguments
/// * `src` - The source UTF-8 string slice to copy from
/// * `dest` - The mutable destination UTF-8 string slice to copy to
///
/// # Returns
/// Result containing the number of bytes copied or an error if:
/// - The destination is smaller than required
/// - Invalid UTF-8 sequence encountered
pub fn u8_cpy(src: &str, dest: &mut [u8]) -> Result<usize, &'static str> {
    if !src.is_ascii() && !src.is_char_boundary(src.len()) {
        return Err("Invalid UTF-8 sequence in source string");
    }

    if dest.len() < src.len() {
        return Err("Destination buffer too small");
    }

    dest[..src.len()].copy_from_slice(src.as_bytes());
    Ok(src.len())
}