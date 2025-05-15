// hmac-gosthash94.rs
//
// HMAC-GOSTHASH94 message authentication code.
//
// Copyright (C) 2016 Dmitry Eremin-Solenikov
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

pub struct HmacGostHash94Ctx {
    state: gosthash94::State,
    outer_state: gosthash94::State,
    key: Vec<u8>,
}

pub struct HmacGostHash94CpCtx {
    state: gosthash94cp::State,
    outer_state: gosthash94cp::State,
    key: Vec<u8>,
}

pub fn hmac_gosthash94_set_key(
    ctx: &mut HmacGostHash94Ctx,
    key_length: usize,
    key: &[u8],
) {
    ctx.key = key[..key_length].to_vec();
    hmac::set_key(&mut ctx.state, &nettle::gosthash94, key_length, key);
}

pub fn hmac_gosthash94_update(
    ctx: &mut HmacGostHash94Ctx,
    length: usize,
    data: &[u8],
) {
    gosthash94::update(&mut ctx.state, length, data);
}

pub fn hmac_gosthash94_digest(
    ctx: &mut HmacGostHash94Ctx,
    length: usize,
    digest: &mut [u8],
) {
    hmac::digest(ctx, &nettle::gosthash94, length, digest);
}

pub fn hmac_gosthash94cp_set_key(
    ctx: &mut HmacGostHash94CpCtx,
    key_length: usize,
    key: &[u8],
) {
    ctx.key = key[..key_length].to_vec();
    hmac::set_key(&mut ctx.state, &nettle::gosthash94cp, key_length, key);
}

pub fn hmac_gosthash94cp_update(
    ctx: &mut HmacGostHash94CpCtx,
    length: usize,
    data: &[u8],
) {
    gosthash94cp::update(&mut ctx.state, length, data);
}

pub fn hmac_gosthash94cp_digest(
    ctx: &mut HmacGostHash94CpCtx,
    length: usize,
    digest: &mut [u8],
) {
    hmac::digest(ctx, &nettle::gosthash94cp, length, digest);
}

mod hmac {
    use super::*;

    pub fn set_key<T>(
        ctx: &mut T,
        hash: &dyn HashFunction,
        key_length: usize,
        key: &[u8],
    ) {
        // HMAC key setup implementation
    }

    pub fn digest<T>(
        ctx: &mut T,
        hash: &dyn HashFunction,
        length: usize,
        digest: &mut [u8],
    ) {
        // HMAC digest implementation
    }
}

mod gosthash94 {
    pub struct State {
        // GOST hash state
    }

    pub fn update(state: &mut State, length: usize, data: &[u8]) {
        // GOST hash update implementation
    }
}

mod gosthash94cp {
    pub struct State {
        // GOST hash cp state
    }

    pub fn update(state: &mut State, length: usize, data: &[u8]) {
        // GOST hash cp update implementation
    }
}

mod nettle {
    pub static gosthash94: dyn HashFunction = // ...;
    pub static gosthash94cp: dyn HashFunction = // ...;
}

trait HashFunction {
    // Hash function trait definition
}