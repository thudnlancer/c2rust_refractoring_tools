// hmac-md5-meta.rs

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

use md5::Md5;
use hmac::{Hmac, Mac};
use std::marker::PhantomData;

pub struct HmacMd5 {
    hmac: Hmac<Md5>,
}

impl HmacMd5 {
    pub fn new() -> Self {
        HmacMd5 {
            hmac: Hmac::new_from_slice(&[]).expect("HMAC initialization failed"),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.hmac = Hmac::new_from_slice(key).expect("HMAC key initialization failed");
    }
}

pub const NETTLE_HMAC_MD5: HmacMd5 = HmacMd5 {
    hmac: Hmac::new_from_slice(&[]).expect("HMAC initialization failed"),
};