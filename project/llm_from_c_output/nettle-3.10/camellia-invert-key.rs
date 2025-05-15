// camellia-invert-key.rs

// Inverting a key means reversing order of subkeys.

// Copyright (C) 2010 Niels Möller

// This file is part of GNU Nettle.

// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:

//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.

// or

//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.

// or both in parallel, as here.

// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

/// Inverts the key by reversing the order of subkeys.
///
/// # Arguments
/// * `nkeys` - Number of keys to invert
/// * `dst` - Destination slice for inverted keys
/// * `src` - Source slice containing original keys
///
/// # Panics
/// Panics if the destination and source slices have different lengths.
pub fn camellia_invert_key(nkeys: usize, dst: &mut [u64], src: &[u64]) {
    assert!(dst.len() >= nkeys && src.len() >= nkeys,
            "Destination or source slice too small for nkeys");
    
    if dst.as_ptr() == src.as_ptr() {
        for i in 0..nkeys / 2 {
            let j = nkeys - 1 - i;
            dst.swap(i, j);
        }
    } else {
        for i in 0..nkeys {
            dst[i] = src[nkeys - 1 - i];
        }
    }
}