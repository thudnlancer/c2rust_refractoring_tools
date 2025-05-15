/* cmac-des3-meta.rs

   Copyright (C) 2020 Dmitry Baryshkov

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

use crate::cmac::{cmac_des3_set_key, cmac_des3_update, cmac_des3_digest, CmacDes3Ctx};
use crate::nettle_meta::{NettleMac, MacContext};

pub const CMAC64_DIGEST_SIZE: usize = 8;
pub const DES3_KEY_SIZE: usize = 24;

pub const NETTLE_CMAC_DES3: NettleMac = NettleMac {
    name: "cmac_des3",
    context_size: size_of::<CmacDes3Ctx>(),
    digest_size: CMAC64_DIGEST_SIZE,
    key_size: DES3_KEY_SIZE,

    set_key: cmac_des3_set_key,
    update: cmac_des3_update,
    digest: cmac_des3_digest,
};