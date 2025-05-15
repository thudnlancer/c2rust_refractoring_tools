// gcm-camellia128.rs

// Copyright (C) 2011, 2014 Niels Möller
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

use std::convert::TryInto;
use crate::gcm::GcmContext;
use crate::camellia::{camellia128_set_encrypt_key, camellia128_crypt};

pub struct GcmCamellia128Context {
    ctx: GcmContext,
}

impl GcmCamellia128Context {
    pub fn new() -> Self {
        GcmCamellia128Context {
            ctx: GcmContext::new(),
        }
    }

    pub fn set_key(&mut self, key: &[u8; 16]) {
        self.ctx.set_key(
            |k| camellia128_set_encrypt_key(k, key),
            |k, l, d| camellia128_crypt(k, l, d),
        );
    }

    pub fn set_iv(&mut self, iv: &[u8]) {
        self.ctx.set_iv(iv);
    }

    pub fn update(&mut self, data: &[u8]) {
        self.ctx.update(data);
    }

    pub fn encrypt(&mut self, src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
        if src.len() != dst.len() {
            return Err("Source and destination buffers must be of equal length");
        }
        self.ctx.encrypt(
            |k, l, d| camellia128_crypt(k, l, d),
            src,
            dst,
        );
        Ok(())
    }

    pub fn decrypt(&mut self, src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
        if src.len() != dst.len() {
            return Err("Source and destination buffers must be of equal length");
        }
        self.ctx.decrypt(
            |k, l, d| camellia128_crypt(k, l, d),
            src,
            dst,
        );
        Ok(())
    }

    pub fn digest(&mut self, digest: &mut [u8]) -> Result<(), &'static str> {
        if digest.len() < 16 {
            return Err("Digest buffer must be at least 16 bytes");
        }
        self.ctx.digest(
            |k, l, d| camellia128_crypt(k, l, d),
            digest,
        );
        Ok(())
    }
}