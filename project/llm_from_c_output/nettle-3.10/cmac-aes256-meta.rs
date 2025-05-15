/* cmac-aes256-meta.rs

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

use std::mem::size_of;

use crate::cmac::{CmacAes256Ctx, cmac_aes256_set_key, cmac_aes256_update, cmac_aes256_digest};
use crate::nettle_meta::{NettleMac, MacFuncs};

pub const NETTLE_CMAC_AES256: NettleMac = NettleMac {
    name: "cmac_aes256",
    context_size: size_of::<CmacAes256Ctx>(),
    digest_size: crate::cmac::CMAC128_DIGEST_SIZE,
    key_size: crate::aes::AES256_KEY_SIZE,
    functions: MacFuncs {
        set_key: cmac_aes256_set_key,
        update: cmac_aes256_update,
        digest: cmac_aes256_digest,
    },
};