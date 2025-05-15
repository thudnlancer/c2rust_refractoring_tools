// Convert 'i64' integer to printable string.
// 
// This code is based on the original C implementation from the GNU project,
// adapted to Rust while maintaining the same functionality.
// 
// Copyright (C) 2004-2023 Free Software Foundation, Inc.
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

/// Converts an i64 (equivalent to C's intmax_t) to its string representation.
/// 
/// # Arguments
/// * `num` - The integer to convert
/// 
/// # Returns
/// A String containing the string representation of the number
pub fn imaxtostr(num: i64) -> String {
    num.to_string()
}