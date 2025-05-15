/* Determine display width of Unicode character.
   Copyright (C) 2001-2002, 2006-2023 Free Software Foundation, Inc.
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

use crate::cjk::is_cjk_encoding;
use crate::uniwidth::width0::{nonspacing_table_ind, nonspacing_table_data};
use crate::uniwidth::width2::u_width2;
use crate::unictype::bitmap::bitmap_lookup;

/// Determine number of column positions required for Unicode character.
pub fn uc_width(uc: u32, encoding: &str) -> i32 {
    /* Test for non-spacing or control character.  */
    if (uc >> 9) < nonspacing_table_ind.len() as u32 {
        let ind = nonspacing_table_ind[(uc >> 9) as usize];
        if ind >= 0 {
            let index = 64 * ind as usize + ((uc >> 3) & 63) as usize;
            if ((nonspacing_table_data[index] >> (uc & 7)) & 1) != 0 {
                return if uc > 0 && uc < 0xa0 { -1 } else { 0 };
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
    /* Test for double-width character.  */
    if bitmap_lookup(&u_width2, uc) {
        return 2;
    }
    /* In ancient CJK encodings, Cyrillic and most other characters are
       double-width as well.  */
    if uc >= 0x00A1 && uc < 0xFF61 && uc != 0x20A9 && is_cjk_encoding(encoding) {
        return 2;
    }
    1
}