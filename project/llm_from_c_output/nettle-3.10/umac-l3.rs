/* umac-l3.rs

   Copyright (C) 2013 Niels Möller

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

// 2^36 - 5
const P: u64 = 0x0000000FFFFFFFFB;

/// Initialize UMAC L3 keys
pub fn umac_l3_init(size: usize, k: &mut [u64]) {
    for item in k.iter_mut().take(size) {
        let w = item.to_be();
        *item = w % P;
    }
}

fn umac_l3_word(k: &[u64], mut w: u64) -> u64 {
    let mut y = 0;

    // Since it's easiest to process the input word from the low end,
    // loop over keys in reverse order.
    for i in 0..4 {
        y += (w & 0xffff) * k[3 - i];
        w >>= 16;
    }

    y
}

pub fn umac_l3(key: &[u64], m: &[u64]) -> u32 {
    let y = (umac_l3_word(&key[0..4], m[0]) + umac_l3_word(&key[4..8], m[1])) % P;
    (y as u32).to_be()
}