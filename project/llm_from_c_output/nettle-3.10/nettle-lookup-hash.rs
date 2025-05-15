// nettle-lookup-hash.rs

// Copyright (C) 2016 Niels Möller.
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

use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub struct NettleHash {
    pub name: *const c_char,
    // Other fields omitted for brevity
}

extern "C" {
    static _nettle_hashes: [*const NettleHash; 0]; // Actual size determined by implementation
}

pub fn nettle_lookup_hash(name: &str) -> Option<&'static NettleHash> {
    let name_cstr = std::ffi::CString::new(name).ok()?;
    let name_ptr = name_cstr.as_ptr();

    let mut i = 0;
    loop {
        let hash_ptr = unsafe { _nettle_hashes.get(i)? };
        if hash_ptr.is_null() {
            break;
        }
        let hash = unsafe { &**hash_ptr };
        let hash_name = unsafe { CStr::from_ptr(hash.name) };
        if hash_name.to_str().ok()? == name {
            return Some(hash);
        }
        i += 1;
    }
    None
}