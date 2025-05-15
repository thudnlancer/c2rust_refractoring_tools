// Look at first character in UTF-8 string.
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

/// Unsafely look at first character in UTF-8 string.
/// Returns the Unicode character and the number of bytes consumed.
/// If the string is invalid, returns U+FFFD (replacement character) and the number of bytes to skip.
pub fn u8_mbtouc_unsafe(s: &[u8]) -> (char, usize) {
    if s.is_empty() {
        return ('\0', 0);
    }

    let c = s[0];

    if c < 0x80 {
        return (c as char, 1);
    } else if c >= 0xc2 {
        if c < 0xe0 {
            if s.len() >= 2 {
                if (s[1] ^ 0x80) < 0x40 {
                    let uc = ((u32::from(c & 0x1f) << 6) | u32::from(s[1] ^ 0x80));
                    return (char::from_u32(uc).unwrap_or('\u{FFFD}'), 2);
                }
                // invalid multibyte character
            } else {
                // incomplete multibyte character
                return ('\u{FFFD}', 1);
            }
        } else if c < 0xf0 {
            if s.len() >= 3 {
                if (s[1] ^ 0x80) < 0x40 {
                    if (s[2] ^ 0x80) < 0x40 {
                        if (c >= 0xe1 || s[1] >= 0xa0) && (c != 0xed || s[1] < 0xa0) {
                            let uc = ((u32::from(c & 0x0f) << 12)
                                | (u32::from(s[1] ^ 0x80) << 6
                                | u32::from(s[2] ^ 0x80);
                            return (char::from_u32(uc).unwrap_or('\u{FFFD}'), 3);
                        }
                        // invalid multibyte character
                        return ('\u{FFFD}', 3);
                    }
                    // invalid multibyte character
                    return ('\u{FFFD}', 2);
                }
                // invalid multibyte character
            } else {
                // incomplete multibyte character
                if s.len() == 1 || (s[1] ^ 0x80) >= 0x40 {
                    return ('\u{FFFD}', 1);
                } else {
                    return ('\u{FFFD}', 2);
                }
            }
        } else if c < 0xf8 {
            if s.len() >= 4 {
                if (s[1] ^ 0x80) < 0x40 {
                    if (s[2] ^ 0x80) < 0x40 {
                        if (s[3] ^ 0x80) < 0x40 {
                            if (c >= 0xf1 || s[1] >= 0x90) && (c < 0xf4 || (c == 0xf4 && s[1] < 0x90)) {
                                let uc = ((u32::from(c & 0x07) << 18)
                                    | (u32::from(s[1] ^ 0x80) << 12)
                                    | (u32::from(s[2] ^ 0x80) << 6)
                                    | u32::from(s[3] ^ 0x80));
                                return (char::from_u32(uc).unwrap_or('\u{FFFD}'), 4);
                            }
                            // invalid multibyte character
                            return ('\u{FFFD}', 4);
                        }
                        // invalid multibyte character
                        return ('\u{FFFD}', 3);
                    }
                    // invalid multibyte character
                    return ('\u{FFFD}', 2);
                }
                // invalid multibyte character
            } else {
                // incomplete multibyte character
                if s.len() == 1 || (s[1] ^ 0x80) >= 0x40 {
                    return ('\u{FFFD}', 1);
                } else if s.len() == 2 || (s[2] ^ 0x80) >= 0x40 {
                    return ('\u{FFFD}', 2);
                } else {
                    return ('\u{FFFD}', 3);
                }
            }
        }
    }
    // invalid multibyte character
    ('\u{FFFD}', 1)
}