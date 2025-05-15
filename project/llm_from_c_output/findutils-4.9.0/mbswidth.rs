// Determine the number of screen columns needed for a string.
// Copyright (C) 2000-2022 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::char;
use std::cmp;
use std::str;

bitflags::bitflags! {
    pub struct MbswFlags: i32 {
        const REJECT_INVALID = 1;
        const REJECT_UNPRINTABLE = 2;
    }
}

/// Returns the number of screen columns needed for the string.
pub fn mbswidth(string: &str, flags: MbswFlags) -> Option<usize> {
    mbsnwidth(string, string.len(), flags)
}

/// Returns the number of screen columns needed for the first `nbytes` bytes of the string.
pub fn mbsnwidth(string: &str, nbytes: usize, flags: MbswFlags) -> Option<usize> {
    let mut width = 0;
    let mut chars = string.chars();
    let mut bytes_processed = 0;

    while bytes_processed < nbytes {
        let c = match chars.next() {
            Some(c) => c,
            None => break,
        };

        bytes_processed += c.len_utf8();
        if bytes_processed > nbytes {
            // Incomplete character at end
            if flags.contains(MbswFlags::REJECT_INVALID) {
                return None;
            }
            width += 1;
            break;
        }

        match c {
            ' '..='~' => {
                // Printable ASCII characters
                width += 1;
            }
            _ => {
                // Non-ASCII character
                let w = c.width().unwrap_or(0);
                if w > 0 {
                    // Printable character
                    width = width.checked_add(w)?;
                } else {
                    // Unprintable character
                    if flags.contains(MbswFlags::REJECT_UNPRINTABLE) {
                        return None;
                    }
                    if !c.is_control() {
                        width = width.checked_add(1)?;
                    }
                }
            }
        }
    }

    Some(width)
}