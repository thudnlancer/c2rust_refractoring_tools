// Determine length of UTF-8 string.
// Copyright (C) 2002, 2006, 2009-2023 Free Software Foundation, Inc.
// Written by Bruno Haible <bruno@clisp.org>, 2002.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// Calculates the length of a UTF-8 string.
/// 
/// # Arguments
/// * `s` - A reference to a UTF-8 string slice
/// 
/// # Returns
/// The number of bytes in the string
pub fn u8_strlen(s: &[u8]) -> usize {
    s.len()
}