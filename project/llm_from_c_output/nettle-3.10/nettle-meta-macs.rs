// nettle-meta-macs.rs

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

use std::ptr;

#[repr(C)]
pub struct nettle_mac {
    // Define the fields of nettle_mac as needed
}

// Declare the external MAC implementations
extern {
    pub static nettle_cmac_aes128: nettle_mac;
    pub static nettle_cmac_aes256: nettle_mac;
    pub static nettle_cmac_des3: nettle_mac;
    pub static nettle_hmac_md5: nettle_mac;
    pub static nettle_hmac_ripemd160: nettle_mac;
    pub static nettle_hmac_sha1: nettle_mac;
    pub static nettle_hmac_sha224: nettle_mac;
    pub static nettle_hmac_sha256: nettle_mac;
    pub static nettle_hmac_sha384: nettle_mac;
    pub static nettle_hmac_sha512: nettle_mac;
    pub static nettle_hmac_streebog256: nettle_mac;
    pub static nettle_hmac_streebog512: nettle_mac;
    pub static nettle_hmac_sm3: nettle_mac;
}

static _NETTLE_MACS: &[&nettle_mac] = &[
    &nettle_cmac_aes128,
    &nettle_cmac_aes256,
    &nettle_cmac_des3,
    &nettle_hmac_md5,
    &nettle_hmac_ripemd160,
    &nettle_hmac_sha1,
    &nettle_hmac_sha224,
    &nettle_hmac_sha256,
    &nettle_hmac_sha384,
    &nettle_hmac_sha512,
    &nettle_hmac_streebog256,
    &nettle_hmac_streebog512,
    &nettle_hmac_sm3,
];

#[no_mangle]
pub extern "C" fn nettle_get_macs() -> *const *const nettle_mac {
    _NETTLE_MACS.as_ptr()
}