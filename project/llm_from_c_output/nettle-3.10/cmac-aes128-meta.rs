/* cmac-aes128-meta.rs

   Copyright (C) 2013, 2014 Niels Möller

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

use crate::cmac::{CmacAes128Context, cmac_aes128_set_key, cmac_aes128_update, cmac_aes128_digest};
use crate::nettle_meta::{NettleMac, MacContext};

pub const CMAC128_DIGEST_SIZE: usize = 16;
pub const AES128_KEY_SIZE: usize = 16;

pub const NETTLE_CMAC_AES128: NettleMac = NettleMac {
    name: "cmac_aes128",
    context_size: std::mem::size_of::<CmacAes128Context>(),
    digest_size: CMAC128_DIGEST_SIZE,
    key_size: AES128_KEY_SIZE,

    set_key: cmac_aes128_set_key,
    update: cmac_aes128_update,
    digest: cmac_aes128_digest,
};