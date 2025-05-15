/*
 * Scan memory for the first of two bytes.
 * Copyright (C) 2008-2023 Free Software Foundation, Inc.
 *
 * This file is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of the
 * License, or (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::ptr;

/// Return the first address of either C1 or C2 (treated as unsigned
/// char) that occurs within N bytes of the memory region S.  If
/// neither byte appears, return None.
pub fn memchr2(s: &[u8], c1: u8, c2: u8) -> Option<&u8> {
    if c1 == c2 {
        return s.iter().find(|&&x| x == c1);
    }

    // On 32-bit hardware, using u32 tends to give better performance.
    // On 64-bit hardware, u64 is generally better.
    type Longword = usize;

    let repeated_one = {
        let mut val = 0x01u64;
        for _ in 0..(std::mem::size_of::<Longword>() / 8 - 1) {
            val = (val << 32) | val;
        }
        val as Longword
    };

    let repeated_c1 = {
        let mut val = c1 as Longword;
        for i in 1..std::mem::size_of::<Longword>() {
            val |= (c1 as Longword) << (i * 8);
        }
        val
    };

    let repeated_c2 = {
        let mut val = c2 as Longword;
        for i in 1..std::mem::size_of::<Longword>() {
            val |= (c2 as Longword) << (i * 8);
        }
        val
    };

    let mut offset = 0;
    let len = s.len();

    // Handle unaligned bytes at the beginning
    while offset < len && (s.as_ptr() as usize + offset) % std::mem::align_of::<Longword>() != 0 {
        if s[offset] == c1 || s[offset] == c2 {
            return Some(&s[offset]);
        }
        offset += 1;
    }

    // Process aligned longwords
    while len - offset >= std::mem::size_of::<Longword>() {
        let longword_ptr = unsafe {
            (s.as_ptr().add(offset) as *const Longword).read_unaligned()
        };

        let longword1 = longword_ptr ^ repeated_c1;
        let longword2 = longword_ptr ^ repeated_c2;

        let tmp1 = (longword1 - repeated_one) & !longword1;
        let tmp2 = (longword2 - repeated_one) & !longword2;
        let mask = repeated_one << 7;

        if ((tmp1 | tmp2) & mask) != 0 {
            break;
        }

        offset += std::mem::size_of::<Longword>();
    }

    // Check remaining bytes
    for i in offset..len {
        if s[i] == c1 || s[i] == c2 {
            return Some(&s[i]);
        }
    }

    None
}