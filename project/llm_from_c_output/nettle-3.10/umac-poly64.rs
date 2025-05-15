// umac-poly64.rs

// Copyright (C) 2013 Niels Möller
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

/// Multiplies two 64-bit polynomials modulo a special prime.
/// Implements the same algorithm as the C version.
fn poly64_mul(kh: u32, kl: u32, y: u64) -> u64 {
    let yl = y & 0xffffffff;
    let yh = y >> 32;
    let pl = (yl as u64) * (kl as u64);
    let ph = (yh as u64) * (kh as u64);
    let ml = (yh as u64) * (kl as u64) + (yl as u64) * (kh as u64); // No overflow, thanks to special form
    let mh = ml >> 32;
    let ml = ml << 32;
    let (pl1, overflow1) = pl.overflowing_add(ml);
    let (ph1, overflow2) = ph.overflowing_add(mh);
    let ph1 = ph1 + if overflow1 { 1 } else { 0 };

    // Reduce, using 2^64 = UMAC_P64_OFFSET (mod p)
    debug_assert!(ph1 < (1 << 57));
    let ph1 = ph1 * UMAC_P64_OFFSET;
    let (mut pl1, overflow) = pl1.overflowing_add(ph1);
    if overflow {
        pl1 = pl1.wrapping_add(UMAC_P64_OFFSET);
    }

    pl1
}

/// Computes the UMAC polynomial hash function.
/// Implements the same algorithm as the C version.
pub fn umac_poly64(kh: u32, kl: u32, mut y: u64, m: u64) -> u64 {
    if (m >> 32) == 0xffffffff {
        y = poly64_mul(kh, kl, y);
        y = if y == 0 {
            UMAC_P64 - 1
        } else {
            y - 1
        };
        let m = m.wrapping_sub(UMAC_P64_OFFSET);
        y = poly64_mul(kh, kl, y);
        let (y1, overflow) = y.overflowing_add(m);
        if overflow {
            y1.wrapping_add(UMAC_P64_OFFSET)
        } else {
            y1
        }
    } else {
        y = poly64_mul(kh, kl, y);
        let (y1, overflow) = y.overflowing_add(m);
        if overflow {
            y1.wrapping_add(UMAC_P64_OFFSET)
        } else {
            y1
        }
    }
}

// Constants from umac.h
const UMAC_P64_OFFSET: u64 = /* value from umac.h */;
const UMAC_P64: u64 = /* value from umac.h */;