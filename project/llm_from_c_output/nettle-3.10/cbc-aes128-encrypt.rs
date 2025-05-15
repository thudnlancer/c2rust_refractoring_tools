/*
   Copyright (C) 2021 Niels Möller

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

use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use block_modes::BlockModeError;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn cbc_aes128_encrypt(
    ctx: &Aes128,
    iv: &mut [u8; 16],
    src: &[u8],
) -> Result<Vec<u8>, BlockModeError> {
    let cipher = Aes128Cbc::new_from_slices(ctx, iv)?;
    let ciphertext = cipher.encrypt_vec(src);
    iv.copy_from_slice(&ciphertext[ciphertext.len() - 16..]);
    Ok(ciphertext)
}