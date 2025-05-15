/* cast128-meta.rs

   Copyright (C) 2002 Niels Möller

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

use crate::cast128::{Cast128, CAST128_BLOCK_SIZE, CAST128_KEY_SIZE};
use crate::nettle_meta::NettleCipher;

pub const NETTLE_CAST128: NettleCipher = NettleCipher {
    name: "cast128",
    context_size: std::mem::size_of::<Cast128>(),
    block_size: CAST128_BLOCK_SIZE,
    key_size: CAST128_KEY_SIZE,
    set_encrypt_key: Cast128::set_key,
    set_decrypt_key: Cast128::set_key,
    encrypt: Cast128::encrypt,
    decrypt: Cast128::decrypt,
};