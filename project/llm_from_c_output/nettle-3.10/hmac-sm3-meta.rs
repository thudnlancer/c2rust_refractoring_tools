/* hmac-sm3-meta.rs

   Copyright (C) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>

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
use crate::hmac::{Hmac, Mac};
use crate::sm3::Sm3;

const SM3_DIGEST_SIZE: usize = 32;

pub fn hmac_sm3_set_key_wrapper(ctx: &mut Hmac<Sm3>, key: &[u8]) {
    ctx.set_key(key);
}

pub struct NettleHmacSm3;

impl NettleHmacSm3 {
    pub fn new() -> Hmac<Sm3> {
        Hmac::<Sm3>::new()
    }
}