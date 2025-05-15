//! floatcomp.rs - Isolate floating point details.
//!
//! This is a Rust translation of the original C code from GAWK.
//! Maintains the same functionality while adhering to Rust's safety principles.

/*
 * Copyright (C) 1986, 1988, 1989, 1991-2011, 2016, 2021,
 * the Free Software Foundation, Inc.
 *
 * This file is part of GAWK, the GNU implementation of the
 * AWK Programming Language.
 *
 * GAWK is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * GAWK is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

use std::mem;

// Assume IEEE-754 arithmetic if not specified
const FLT_RADIX: u32 = 2;
const FLT_MANT_DIG: u32 = 24;
const DBL_MANT_DIG: u32 = 53;

/// The number of base-FLT_RADIX digits in an AWKNUM fraction, assuming
/// that AWKNUM is not long double.
fn awk_small_mant_dig() -> u32 {
    if mem::size_of::<f64>() == mem::size_of::<f64>() {
        DBL_MANT_DIG
    } else {
        FLT_MANT_DIG
    }
}

/// The number of base-FLT_DIGIT digits in an AWKNUM fraction, even if
/// AWKNUM is long double.
fn awknum_mant_dig() -> u32 {
    #[cfg(has_long_double)]
    {
        if mem::size_of::<f64>() == mem::size_of::<f64>() {
            f64::MANTISSA_DIGITS
        } else {
            awk_small_mant_dig()
        }
    }
    #[cfg(not(has_long_double))]
    {
        awk_small_mant_dig()
    }
}

/// The number of bits in an AWKNUM fraction, assuming FLT_RADIX is
/// either 2 or 16. IEEE and VAX formats use radix 2, and IBM
/// mainframe format uses radix 16.
fn awknum_fraction_bits() -> u32 {
    let mant_dig = awknum_mant_dig();
    if FLT_RADIX == 2 {
        mant_dig
    } else if FLT_RADIX == 16 {
        mant_dig * 4
    } else {
        panic!("Unsupported FLT_RADIX value, please port this code to your platform");
    }
}

fn dbl_fraction_bits() -> u32 {
    if FLT_RADIX == 2 {
        DBL_MANT_DIG
    } else if FLT_RADIX == 16 {
        DBL_MANT_DIG * 4
    } else {
        panic!("Unsupported FLT_RADIX value");
    }
}

/// Return the number of trailing zeros in n. n must be nonzero.
fn count_trailing_zeros(n: u64) -> u32 {
    if n == 0 {
        panic!("n must be nonzero");
    }
    n.trailing_zeros()
}

/// Adjust unsigned integer values to fit within AWKNUM representation
/// 
/// If the integer type is wider than what AWKNUM can represent exactly,
/// this function strips leading nonzero bits of large integers so that
/// their low order bits are represented exactly without rounding errors.
pub fn adjust_uint(n: u64) -> u64 {
    let wordbits = (mem::size_of::<u64>() * 8) as u32;
    let awknum_fraction_bits = awknum_fraction_bits();
    
    if awknum_fraction_bits < wordbits {
        let sentinel = 1u64 << (wordbits - awknum_fraction_bits);
        let shift = count_trailing_zeros(n | sentinel);
        let mask = (1u64 << awknum_fraction_bits) - 1;
        
        n & (mask << shift)
    } else {
        n
    }
}