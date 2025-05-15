// nettle-meta-hashes.rs

// Copyright (C) 2011 Daniel Kahn Gillmor
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

use std::ptr;

#[repr(C)]
pub struct NettleHash {
    // Define the fields of nettle_hash struct as needed
}

extern {
    static nettle_gosthash94: NettleHash;
    static nettle_gosthash94cp: NettleHash;
    static nettle_md2: NettleHash;
    static nettle_md4: NettleHash;
    static nettle_md5: NettleHash;
    static nettle_ripemd160: NettleHash;
    static nettle_sha1: NettleHash;
    static nettle_sha224: NettleHash;
    static nettle_sha256: NettleHash;
    static nettle_sha384: NettleHash;
    static nettle_sha512: NettleHash;
    static nettle_sha512_224: NettleHash;
    static nettle_sha512_256: NettleHash;
    static nettle_sha3_224: NettleHash;
    static nettle_sha3_256: NettleHash;
    static nettle_sha3_384: NettleHash;
    static nettle_sha3_512: NettleHash;
    static nettle_streebog256: NettleHash;
    static nettle_streebog512: NettleHash;
    static nettle_sm3: NettleHash;
}

static _NETTLE_HASHES: &[Option<&NettleHash>] = &[
    Some(&nettle_gosthash94),
    Some(&nettle_gosthash94cp),
    Some(&nettle_md2),
    Some(&nettle_md4),
    Some(&nettle_md5),
    Some(&nettle_ripemd160),
    Some(&nettle_sha1),
    Some(&nettle_sha224),
    Some(&nettle_sha256),
    Some(&nettle_sha384),
    Some(&nettle_sha512),
    Some(&nettle_sha512_224),
    Some(&nettle_sha512_256),
    Some(&nettle_sha3_224),
    Some(&nettle_sha3_256),
    Some(&nettle_sha3_384),
    Some(&nettle_sha3_512),
    Some(&nettle_streebog256),
    Some(&nettle_streebog512),
    Some(&nettle_sm3),
    None,
];

pub fn nettle_get_hashes() -> &'static [Option<&'static NettleHash>] {
    _NETTLE_HASHES
}