// Look at first character in UTF-8 string, returning an error code.
// Copyright (C) 1999-2002, 2006-2007, 2009-2023 Free Software Foundation, Inc.
// Written by Bruno Haible <bruno@clisp.org>, 2001.
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

use std::convert::TryFrom;

/// Decodes the first UTF-8 character in the given byte slice.
///
/// Returns:
/// - Ok(positive) if a valid character was found, where positive is the number of bytes used
/// - Err(negative) if an error occurred, where negative is the error code:
///   - -1: invalid multibyte character
///   - -2: incomplete multibyte character
pub fn u8_mbtoucr(s: &[u8]) -> Result<(u32, usize), i32> {
    if s.is_empty() {
        return Err(-2);
    }

    let c = s[0];

    if c < 0x80 {
        Ok((c as u32, 1))
    } else if c >= 0xc2 {
        if c < 0xe0 {
            if s.len() >= 2 {
                if (s[1] ^ 0x80) < 0x40 {
                    let puc = ((u32::from(c & 0x1f) << 6) | u32::from(s[1] ^ 0x80));
                    Ok((puc, 2))
                } else {
                    // invalid multibyte character
                    Err(-1)
                }
            } else {
                // incomplete multibyte character
                Err(-2)
            }
        } else if c < 0xf0 {
            if s.len() >= 2 {
                if (s[1] ^ 0x80) < 0x40
                    && (c >= 0xe1 || s[1] >= 0xa0)
                    && (c != 0xed || s[1] < 0xa0)
                {
                    if s.len() >= 3 {
                        if (s[2] ^ 0x80) < 0x40 {
                            let puc = ((u32::from(c & 0x0f) << 12)
                                | (u32::from(s[1] ^ 0x80) << 6)
                                | u32::from(s[2] ^ 0x80));
                            Ok((puc, 3))
                        } else {
                            // invalid multibyte character
                            Err(-1)
                        }
                    } else {
                        // incomplete multibyte character
                        Err(-2)
                    }
                } else {
                    // invalid multibyte character
                    Err(-1)
                }
            } else {
                // incomplete multibyte character
                Err(-2)
            }
        } else if c < 0xf8 {
            if s.len() >= 2 {
                if (s[1] ^ 0x80) < 0x40
                    && (c >= 0xf1 || s[1] >= 0x90)
                    && (c < 0xf4 || (c == 0xf4 && s[1] < 0x90))
                {
                    if s.len() >= 3 {
                        if (s[2] ^ 0x80) < 0x40 {
                            if s.len() >= 4 {
                                if (s[3] ^ 0x80) < 0x40 {
                                    let puc = ((u32::from(c & 0x07) << 18)
                                        | (u32::from(s[1] ^ 0x80) << 12)
                                        | (u32::from(s[2] ^ 0x80) << 6)
                                        | u32::from(s[3] ^ 0x80);
                                    Ok((puc, 4))
                                } else {
                                    // invalid multibyte character
                                    Err(-1)
                                }
                            } else {
                                // incomplete multibyte character
                                Err(-2)
                            }
                        } else {
                            // invalid multibyte character
                            Err(-1)
                        }
                    } else {
                        // incomplete multibyte character
                        Err(-2)
                    }
                } else {
                    // invalid multibyte character
                    Err(-1)
                }
            } else {
                // incomplete multibyte character
                Err(-2)
            }
        } else {
            // invalid multibyte character
            Err(-1)
        }
    } else {
        // invalid multibyte character
        Err(-1)
    }
}

/// Wrapper function that matches the C function signature more closely
pub fn u8_mbtoucr_cstyle(puc: &mut u32, s: &[u8]) -> i32 {
    match u8_mbtoucr(s) {
        Ok((ch, len)) => {
            *puc = ch;
            i32::try_from(len).unwrap()
        }
        Err(-2) => {
            *puc = 0xfffd;
            -2
        }
        Err(_) => {
            *puc = 0xfffd;
            -1
        }
    }
}