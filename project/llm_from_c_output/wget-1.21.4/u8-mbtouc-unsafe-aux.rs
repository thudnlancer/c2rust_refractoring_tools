// Conversion UTF-8 to UCS-4.
// Copyright (C) 2001-2002, 2006-2007, 2009-2023 Free Software Foundation, Inc.
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

/// Converts a UTF-8 sequence to a UCS-4 character (unsafe variant).
/// Returns the number of bytes consumed and writes the UCS-4 character to `puc`.
/// This function performs minimal validation and may return invalid characters.
pub fn u8_mbtouc_unsafe_aux(puc: &mut u32, s: &[u8]) -> usize {
    if s.is_empty() {
        *puc = 0xfffd;
        return 1;
    }

    let c = s[0];

    if c >= 0xc2 {
        if c < 0xe0 {
            if s.len() >= 2 {
                if (s[1] ^ 0x80) < 0x40 {
                    *puc = ((u32::from(c) & 0x1f) << 6) | u32::from(s[1] ^ 0x80);
                    return 2;
                }
                // invalid multibyte character
            } else {
                // incomplete multibyte character
                *puc = 0xfffd;
                return 1;
            }
        } else if c < 0xf0 {
            if s.len() >= 3 {
                if (s[1] ^ 0x80) < 0x40 {
                    if (s[2] ^ 0x80) < 0x40 {
                        if (c >= 0xe1 || s[1] >= 0xa0) && (c != 0xed || s[1] < 0xa0) {
                            *puc = ((u32::from(c) & 0x0f) << 12
                                | (u32::from(s[1] ^ 0x80) << 6
                                | u32::from(s[2] ^ 0x80);
                            return 3;
                        }
                        // invalid multibyte character
                        *puc = 0xfffd;
                        return 3;
                    }
                    // invalid multibyte character
                    *puc = 0xfffd;
                    return 2;
                }
                // invalid multibyte character
            } else {
                // incomplete multibyte character
                *puc = 0xfffd;
                if s.len() == 1 || (s[1] ^ 0x80) >= 0x40 {
                    return 1;
                } else {
                    return 2;
                }
            }
        } else if c < 0xf8 {
            if s.len() >= 4 {
                if (s[1] ^ 0x80) < 0x40 {
                    if (s[2] ^ 0x80) < 0x40 {
                        if (s[3] ^ 0x80) < 0x40 {
                            if (c >= 0xf1 || s[1] >= 0x90)
                                && (c < 0xf4 || (c == 0xf4 && s[1] < 0x90))
                            {
                                *puc = ((u32::from(c) & 0x07) << 18)
                                    | (u32::from(s[1] ^ 0x80) << 12)
                                    | (u32::from(s[2] ^ 0x80) << 6
                                    | u32::from(s[3] ^ 0x80);
                                return 4;
                            }
                            // invalid multibyte character
                            *puc = 0xfffd;
                            return 4;
                        }
                        // invalid multibyte character
                        *puc = 0xfffd;
                        return 3;
                    }
                    // invalid multibyte character
                    *puc = 0xfffd;
                    return 2;
                }
                // invalid multibyte character
            } else {
                // incomplete multibyte character
                *puc = 0xfffd;
                if s.len() == 1 || (s[1] ^ 0x80) >= 0x40 {
                    return 1;
                } else if s.len() == 2 || (s[2] ^ 0x80) >= 0x40 {
                    return 2;
                } else {
                    return 3;
                }
            }
        }
    }
    // invalid multibyte character
    *puc = 0xfffd;
    1
}