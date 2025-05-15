/* Conversion UCS-4 to UTF-8.
   Copyright (C) 2002, 2006-2007, 2009-2023 Free Software Foundation, Inc.
   Written by Bruno Haible <bruno@clisp.org>, 2002.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/// Converts a UCS-4 character to UTF-8 and stores it in the given buffer.
///
/// # Arguments
/// * `s` - A mutable slice of bytes where the UTF-8 sequence will be stored
/// * `uc` - The UCS-4 character to convert
///
/// # Returns
/// * `Ok(count)` - The number of bytes written (2-4)
/// * `Err(-1)` - Invalid UCS-4 character (surrogate or > 0x10FFFF)
/// * `Err(-2)` - Buffer too small
pub fn u8_uctomb_aux(s: &mut [u8], uc: u32) -> Result<usize, i32> {
    let count = if uc < 0x80 {
        return Err(-2);
    } else if uc < 0x800 {
        2
    } else if uc < 0x10000 {
        if uc < 0xd800 || uc >= 0xe000 {
            3
        } else {
            return Err(-1);
        }
    } else if uc < 0x110000 {
        4
    } else {
        return Err(-1);
    };

    if s.len() < count {
        return Err(-2);
    }

    let mut uc = uc;
    match count {
        4 => {
            s[3] = 0x80 | (uc as u8 & 0x3f);
            uc >>= 6;
            uc |= 0x10000;
            s[2] = 0x80 | (uc as u8 & 0x3f);
            uc >>= 6;
            uc |= 0x800;
            s[1] = 0x80 | (uc as u8 & 0x3f);
            uc >>= 6;
            uc |= 0xc0;
            s[0] = uc as u8;
        }
        3 => {
            s[2] = 0x80 | (uc as u8 & 0x3f);
            uc >>= 6;
            uc |= 0x800;
            s[1] = 0x80 | (uc as u8 & 0x3f);
            uc >>= 6;
            uc |= 0xc0;
            s[0] = uc as u8;
        }
        2 => {
            s[1] = 0x80 | (uc as u8 & 0x3f);
            uc >>= 6;
            uc |= 0xc0;
            s[0] = uc as u8;
        }
        _ => unreachable!(),
    }

    Ok(count)
}