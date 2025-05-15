// xmemdup0.rs -- copy a block of arbitrary bytes, plus a trailing NUL

// Copyright (C) 2008-2023 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::alloc::{alloc, Layout};
use std::ptr;

/// Clone an arbitrary block of bytes `p` of size `s`, with error checking,
/// and include a terminating NUL byte. The input `p` is of type `&[u8]`,
/// and the return type is `Vec<u8>` which includes a trailing NUL.
///
/// The terminating NUL makes it safe to use string operations on the result;
/// it also speeds up algorithms such as escape sequence processing on arbitrary
/// memory, by making it always safe to read the byte after the escape character
/// rather than having to check if each escape character is the last byte in the object.
pub fn xmemdup0(p: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(p.len() + 1);
    result.extend_from_slice(p);
    result.push(0);
    result
}

/// A version that takes a raw pointer and size for FFI compatibility
/// # Safety
/// The caller must ensure that:
/// - `p` is valid for reads of `s` bytes
/// - `p` is properly aligned
/// - The memory pointed to by `p` isn't modified for the duration of the function
pub unsafe fn xmemdup0_raw(p: *const u8, s: usize) -> Vec<u8> {
    let slice = std::slice::from_raw_parts(p, s);
    xmemdup0(slice)
}