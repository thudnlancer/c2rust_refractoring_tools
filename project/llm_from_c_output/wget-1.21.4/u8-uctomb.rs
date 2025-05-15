/* Store a character in UTF-8 string.
   Copyright (C) 2002, 2005-2006, 2009-2023 Free Software Foundation, Inc.
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

/// Result type for UTF-8 encoding operations
#[derive(Debug, PartialEq)]
pub enum Utf8EncodeResult {
    Success(usize),
    InvalidCodePoint,
    BufferTooSmall,
}

/// Encodes a Unicode code point into UTF-8 bytes in a buffer.
///
/// # Arguments
/// * `s` - The buffer to write the UTF-8 bytes into
/// * `uc` - The Unicode code point to encode
/// * `n` - The size of the buffer
///
/// # Returns
/// * `Utf8EncodeResult::Success(count)` if encoding succeeded (count is bytes written)
/// * `Utf8EncodeResult::InvalidCodePoint` if the code point is invalid
/// * `Utf8EncodeResult::BufferTooSmall` if the buffer is too small
pub fn u8_uctomb(s: &mut [u8], uc: u32) -> Utf8EncodeResult {
    if uc < 0x80 {
        if s.len() > 0 {
            s[0] = uc as u8;
            return Utf8EncodeResult::Success(1);
        }
        return Utf8EncodeResult::BufferTooSmall;
    } else {
        let count = if uc < 0x800 {
            2
        } else if uc < 0x10000 {
            if uc < 0xD800 || uc >= 0xE000 {
                3
            } else {
                return Utf8EncodeResult::InvalidCodePoint;
            }
        } else if uc < 0x110000 {
            4
        } else {
            return Utf8EncodeResult::InvalidCodePoint;
        };

        if s.len() >= count {
            let mut value = uc;
            match count {
                4 => {
                    s[3] = 0x80 | (value as u8 & 0x3F);
                    value >>= 6;
                    value |= 0x10000;
                    s[2] = 0x80 | (value as u8 & 0x3F);
                    value >>= 6;
                    value |= 0x800;
                    s[1] = 0x80 | (value as u8 & 0x3F);
                    value >>= 6;
                    value |= 0xC0;
                    s[0] = value as u8;
                }
                3 => {
                    s[2] = 0x80 | (value as u8 & 0x3F);
                    value >>= 6;
                    value |= 0x800;
                    s[1] = 0x80 | (value as u8 & 0x3F);
                    value >>= 6;
                    value |= 0xC0;
                    s[0] = value as u8;
                }
                2 => {
                    s[1] = 0x80 | (value as u8 & 0x3F);
                    value >>= 6;
                    value |= 0xC0;
                    s[0] = value as u8;
                }
                _ => unreachable!(),
            }
            return Utf8EncodeResult::Success(count);
        } else {
            return Utf8EncodeResult::BufferTooSmall;
        }
    }
}