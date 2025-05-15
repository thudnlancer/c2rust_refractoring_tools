// Convert 'unsigned int' integer to printable string.
//
// Copyright (C) 2006-2022 Free Software Foundation, Inc.
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

/// Converts an unsigned integer to its string representation
pub fn uinttostr(mut num: u32) -> String {
    let mut buffer = Vec::with_capacity(20); // Maximum digits for u32
    
    if num == 0 {
        buffer.push(b'0');
    } else {
        while num > 0 {
            let digit = (num % 10) as u8;
            buffer.push(digit + b'0');
            num /= 10;
        }
    }
    
    buffer.reverse();
    String::from_utf8(buffer).expect("Invalid UTF-8 sequence")
}