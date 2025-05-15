// hmac-streebog-meta.rs

// Copyright (C) 2020 Dmitry Baryshkov
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

use std::marker::PhantomData;

pub struct NettleMac<C> {
    pub context: C,
    pub digest_size: usize,
    pub set_key: fn(&mut C, &[u8]),
    pub update: fn(&mut C, &[u8]),
    pub digest: fn(&mut C, &mut [u8]),
}

pub const STREEBOG256_DIGEST_SIZE: usize = 32;
pub const STREEBOG512_DIGEST_SIZE: usize = 64;

pub fn hmac_streebog256_set_key_wrapper(ctx: &mut HmacStreebog256, key: &[u8]) {
    hmac_streebog256_set_key(ctx, STREEBOG256_DIGEST_SIZE, key);
}

pub const NETTLE_HMAC_STREEBOG256: NettleMac<HmacStreebog256> = NettleMac {
    context: HmacStreebog256::new(),
    digest_size: STREEBOG256_DIGEST_SIZE,
    set_key: hmac_streebog256_set_key_wrapper,
    update: hmac_streebog256_update,
    digest: hmac_streebog256_digest,
};

pub fn hmac_streebog512_set_key_wrapper(ctx: &mut HmacStreebog512, key: &[u8]) {
    hmac_streebog512_set_key(ctx, STREEBOG512_DIGEST_SIZE, key);
}

pub const NETTLE_HMAC_STREEBOG512: NettleMac<HmacStreebog512> = NettleMac {
    context: HmacStreebog512::new(),
    digest_size: STREEBOG512_DIGEST_SIZE,
    set_key: hmac_streebog512_set_key_wrapper,
    update: hmac_streebog512_update,
    digest: hmac_streebog512_digest,
};

// Placeholder types and functions - these would need to be implemented
// according to the actual Streebog HMAC implementation
pub struct HmacStreebog256;
pub struct HmacStreebog512;

impl HmacStreebog256 {
    pub fn new() -> Self { HmacStreebog256 }
}

impl HmacStreebog512 {
    pub fn new() -> Self { HmacStreebog512 }
}

pub fn hmac_streebog256_set_key(ctx: &mut HmacStreebog256, _digest_size: usize, _key: &[u8]) {}
pub fn hmac_streebog256_update(_ctx: &mut HmacStreebog256, _data: &[u8]) {}
pub fn hmac_streebog256_digest(_ctx: &mut HmacStreebog256, _digest: &mut [u8]) {}

pub fn hmac_streebog512_set_key(ctx: &mut HmacStreebog512, _digest_size: usize, _key: &[u8]) {}
pub fn hmac_streebog512_update(_ctx: &mut HmacStreebog512, _data: &[u8]) {}
pub fn hmac_streebog512_digest(_ctx: &mut HmacStreebog512, _digest: &mut [u8]) {}