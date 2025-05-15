// Determine display width of Unicode character.
// Copyright (C) 2001-2002, 2006-2022 Free Software Foundation, Inc.
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

use std::convert::TryFrom;

mod cjk {
    pub fn is_cjk_encoding(encoding: &str) -> bool {
        // Implementation of CJK encoding check
        // This would need to be implemented based on the original C code's logic
        false
    }
}

mod uniwidth {
    include!("uniwidth/width0.rs");
    include!("uniwidth/width2.rs");
}

mod unictype {
    pub mod bitmap {
        pub fn lookup(bitmap: &[u32], uc: u32) -> bool {
            // Implementation of bitmap lookup
            // This would need to be implemented based on the original C code's logic
            false
        }
    }
}

/// Determine number of column positions required for a Unicode character.
pub fn uc_width(uc: u32, encoding: &str) -> i32 {
    // Test for non-spacing or control character
    if (uc >> 9) < (uniwidth::nonspacing_table_ind.len() as u32) {
        let ind = uniwidth::nonspacing_table_ind[(uc >> 9) as usize];
        if ind >= 0 {
            let index = 64 * ind as usize + ((uc >> 3) & 63) as usize;
            if let Some(&data) = uniwidth::nonspacing_table_data.get(index) {
                if (data >> (uc & 7)) & 1 != 0 {
                    if uc > 0 && uc < 0xa0 {
                        return -1;
                    } else {
                        return 0;
                    }
                }
            }
        }
    } else if (uc >> 9) == (0xe0000 >> 9) {
        if uc >= 0xe0100 {
            if uc <= 0xe01ef {
                return 0;
            }
        } else {
            if uc >= 0xe0020 {
                if uc <= 0xe007f {
                    return 0;
                }
            } else if uc == 0xe0001 {
                return 0;
            }
        }
    }

    // Test for double-width character
    if unictype::bitmap::lookup(&uniwidth::u_width2, uc) {
        return 2;
    }

    // In ancient CJK encodings, Cyrillic and most other characters are
    // double-width as well
    if uc >= 0x00A1 && uc < 0xFF61 && uc != 0x20A9 && cjk::is_cjk_encoding(encoding) {
        return 2;
    }

    1
}