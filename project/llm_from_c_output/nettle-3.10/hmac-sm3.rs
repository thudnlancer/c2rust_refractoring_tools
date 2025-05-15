// hmac-sm3.rs
//
// HMAC-SM3 message authentication code.
//
// Copyright (C) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>
//
// This file is part of Rust Nettle.
//
// Rust Nettle is free software: you can redistribute it and/or
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
// Rust Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use crate::hmac::Hmac;
use crate::sm3::Sm3;

pub struct HmacSm3 {
    ctx: Hmac<Sm3>,
}

impl HmacSm3 {
    pub fn new() -> Self {
        HmacSm3 {
            ctx: Hmac::new(Sm3::new()),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.ctx.set_key(key);
    }

    pub fn update(&mut self, data: &[u8]) {
        self.ctx.update(data);
    }

    pub fn digest(&mut self, digest: &mut [u8]) -> Result<(), &'static str> {
        self.ctx.digest(digest)
    }
}