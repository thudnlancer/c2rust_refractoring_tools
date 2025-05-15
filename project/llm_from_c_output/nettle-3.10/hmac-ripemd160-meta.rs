// hmac-ripemd160-meta.rs

// Copyright (C) 2020 Daiki Ueno
//
// This file is part of GNU Nettle.
//
// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:
//
//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.
//
// or
//
//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.
//
// or both in parallel, as here.
//
// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use std::mem::size_of;
use crate::hmac::Hmac;
use crate::ripemd160::Ripemd160;
use crate::nettle_mac::NettleMac;

const RIPEMD160_DIGEST_SIZE: usize = 20;

pub struct HmacRipemd160 {
    hmac: Hmac<Ripemd160>,
}

impl HmacRipemd160 {
    pub fn new() -> Self {
        HmacRipemd160 {
            hmac: Hmac::new(Ripemd160::new()),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.hmac.set_key(key);
    }
}

impl NettleMac for HmacRipemd160 {
    fn digest_size(&self) -> usize {
        RIPEMD160_DIGEST_SIZE
    }

    fn update(&mut self, data: &[u8]) {
        self.hmac.update(data);
    }

    fn digest(&mut self, digest: &mut [u8]) {
        self.hmac.digest(digest);
    }
}

pub static NETTLE_HMAC_RIPEMD160: HmacRipemd160 = HmacRipemd160::new();