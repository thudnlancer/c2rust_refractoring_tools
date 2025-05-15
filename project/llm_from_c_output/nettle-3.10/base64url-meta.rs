// base64url_meta.rs

// Copyright (C) 2015 Niels Möller
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

use crate::base64::{Base64EncodeContext, Base64DecodeContext};
use crate::nettle_meta::{Armor, ArmorLengthFunc};

/// Same as the macro with the same name
fn base64url_encode_length(length: usize) -> usize {
    (length + 2) / 3 * 4
}

/// Same as the macro with the same name
fn base64url_decode_length(length: usize) -> usize {
    (length + 3) / 4 * 3
}

pub const NETTLE_BASE64URL: Armor = Armor {
    encode_length: base64url_encode_length as ArmorLengthFunc,
    decode_length: base64url_decode_length as ArmorLengthFunc,
    encode_ctx: Base64EncodeContext::new,
    encode_update: Base64EncodeContext::update,
    encode_final: Base64EncodeContext::finalize,
    decode_ctx: Base64DecodeContext::new,
    decode_update: Base64DecodeContext::update,
    decode_final: Base64DecodeContext::finalize,
};