// base16-meta.rs

// Copyright (C) 2002 Niels Möller
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

use crate::base16::{base16_encode_update, BASE16_ENCODE_LENGTH, BASE16_DECODE_LENGTH};

/// Same as the macro with the same name
fn base16_encode_length(length: usize) -> usize {
    BASE16_ENCODE_LENGTH(length)
}

/// Same as the macro with the same name
fn base16_decode_length(length: usize) -> usize {
    BASE16_DECODE_LENGTH(length)
}

fn base16_encode_init(_ctx: &mut ()) {}

fn base16_encode_update_wrapper(
    _ctx: &mut (),
    dst: &mut [u8],
    src: &[u8],
) -> usize {
    base16_encode_update(dst, src);
    BASE16_ENCODE_LENGTH(src.len())
}

fn base16_encode_final(_ctx: &mut (), _dst: &mut [u8]) -> usize {
    0
}

const BASE16_ENCODE_FINAL_LENGTH: usize = 0;

pub struct NettleArmor {
    pub encode_length: fn(usize) -> usize,
    pub decode_length: fn(usize) -> usize,
    pub encode_init: fn(&mut ()),
    pub encode_update: fn(&mut (), &mut [u8], &[u8]) -> usize,
    pub encode_final: fn(&mut (), &mut [u8]) -> usize,
    pub encode_final_length: usize,
}

pub const NETTLE_BASE16: NettleArmor = NettleArmor {
    encode_length: base16_encode_length,
    decode_length: base16_decode_length,
    encode_init: base16_encode_init,
    encode_update: base16_encode_update_wrapper,
    encode_final: base16_encode_final,
    encode_final_length: BASE16_ENCODE_FINAL_LENGTH,
};