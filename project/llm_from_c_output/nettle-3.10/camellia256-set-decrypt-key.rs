/* camellia256-set-decrypt-key.rs

   Inverse key setup for the camellia block cipher.

   Copyright (C) 2010, 2013 Niels Möller

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

use crate::camellia_internal::*;

pub fn camellia256_invert_key(dst: &mut Camellia256Ctx, src: &Camellia256Ctx) {
    _nettle_camellia_invert_key(_CAMELLIA256_NKEYS, &mut dst.keys, &src.keys);
}

pub fn camellia256_set_decrypt_key(ctx: &mut Camellia256Ctx, key: &[u8]) -> Result<(), &'static str> {
    camellia256_set_encrypt_key(ctx, key)?;
    camellia256_invert_key(ctx, ctx);
    Ok(())
}

pub fn camellia192_set_decrypt_key(ctx: &mut Camellia256Ctx, key: &[u8]) -> Result<(), &'static str> {
    camellia192_set_encrypt_key(ctx, key)?;
    camellia256_invert_key(ctx, ctx);
    Ok(())
}