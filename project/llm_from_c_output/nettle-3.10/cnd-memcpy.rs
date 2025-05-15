/*
   Copyright (C) 2018 Niels Möller

   This file is part of GNU Nettle.

   GNU Nettle is free software: you can redistribute it and/or
   modify it under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at your
       option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at your
       option) any later version.

   or both in parallel, as here.

   GNU Nettle is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/

use std::ptr::{read_volatile, write_volatile};

/// Conditional memory copy function
/// 
/// # Arguments
/// * `cnd` - Condition flag (0 or non-zero)
/// * `dst` - Destination slice (volatile)
/// * `src` - Source slice (volatile)
/// 
/// # Safety
/// The function uses volatile reads/writes to ensure the operations aren't optimized away.
/// The slices must be of equal length.
pub fn cnd_memcpy(cnd: i32, dst: &mut [u8], src: &[u8]) {
    assert_eq!(dst.len(), src.len(), "Destination and source slices must have equal length");
    
    let m = if cnd != 0 { 0xff } else { 0x00 };

    for (d, s) in dst.iter_mut().zip(src.iter()) {
        let c = (s & m) | (*d & !m);
        *d = c;
    }
}