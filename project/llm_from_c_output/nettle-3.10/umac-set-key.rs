/* umac-set-key.rs

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

use std::convert::TryInto;
use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::cipher::generic_array::GenericArray;

const AES_BLOCK_SIZE: usize = 16;
const UMAC_KEY_SIZE: usize = 16;
const UMAC_BLOCK_SIZE: usize = 16;

fn umac_kdf(aes: &Aes128, index: u32, length: usize, dst: &mut [u8]) {
    let mut block = [0u8; AES_BLOCK_SIZE];
    block[..8].copy_from_slice(&index.to_le_bytes());
    
    let mut count = 1u64;
    let mut remaining = length;
    let mut offset = 0;
    
    while remaining >= AES_BLOCK_SIZE {
        block[8..].copy_from_slice(&count.to_le_bytes());
        let mut output_block = GenericArray::clone_from_slice(&block);
        aes.encrypt_block(&mut output_block);
        dst[offset..offset+AES_BLOCK_SIZE].copy_from_slice(&output_block);
        
        remaining -= AES_BLOCK_SIZE;
        offset += AES_BLOCK_SIZE;
        count += 1;
    }
    
    if remaining > 0 {
        block[8..].copy_from_slice(&count.to_le_bytes());
        let mut output_block = GenericArray::clone_from_slice(&block);
        aes.encrypt_block(&mut output_block);
        dst[offset..offset+remaining].copy_from_slice(&output_block[..remaining]);
    }
}

pub fn _nettle_umac_set_key(
    l1_key: &mut [u32],
    l2_key: &mut [u32],
    l3_key1: &mut [u64],
    l3_key2: &mut [u32],
    aes: &mut Aes128,
    key: &[u8],
    n: usize,
) {
    let mut buffer = [0u8; UMAC_KEY_SIZE];
    *aes = Aes128::new_from_slice(key).expect("Invalid key length");

    let size = UMAC_BLOCK_SIZE / 4 + 4 * (n - 1);
    umac_kdf(aes, 1, size * std::mem::size_of::<u32>(), bytemuck::cast_slice_mut(l1_key));
    bswap32_n_if_le(size, l1_key);

    let size = 6 * n;
    umac_kdf(aes, 2, size * std::mem::size_of::<u32>(), bytemuck::cast_slice_mut(l2_key));
    _nettle_umac_l2_init(size, l2_key);

    let size = 8 * n;
    umac_kdf(aes, 3, size * std::mem::size_of::<u64>(), bytemuck::cast_slice_mut(l3_key1));
    _nettle_umac_l3_init(size, l3_key1);

    umac_kdf(aes, 4, n * std::mem::size_of::<u32>(), bytemuck::cast_slice_mut(l3_key2));

    umac_kdf(aes, 0, UMAC_KEY_SIZE, &mut buffer);
    *aes = Aes128::new_from_slice(&buffer).expect("Invalid key length");
}

// Helper functions (assumed to be implemented elsewhere)
fn bswap32_n_if_le(n: usize, data: &mut [u32]) {
    if cfg!(target_endian = "little") {
        for i in 0..n {
            data[i] = data[i].swap_bytes();
        }
    }
}

fn _nettle_umac_l2_init(size: usize, l2_key: &mut [u32]) {
    // Implementation omitted
}

fn _nettle_umac_l3_init(size: usize, l3_key1: &mut [u64]) {
    // Implementation omitted
}