//! Bit rotation functions for various integer types.
//!
//! This module provides functions for rotating bits in integers of various sizes.
//! The functions are implemented without unsafe code and follow Rust's safety practices.
//!
//! The original C code was Copyright (C) 2008-2022 Free Software Foundation, Inc.
//! and was licensed under the GNU Lesser General Public License.

#![allow(dead_code)]

/// Rotates the bits of a 64-bit unsigned integer to the left by `n` bits.
/// `n` must be between 1 and 63 inclusive.
#[inline]
pub fn rotl64(x: u64, n: u32) -> u64 {
    debug_assert!(n >= 1 && n <= 63, "n must be between 1 and 63");
    x.rotate_left(n)
}

/// Rotates the bits of a 64-bit unsigned integer to the right by `n` bits.
/// `n` must be between 1 and 63 inclusive.
#[inline]
pub fn rotr64(x: u64, n: u32) -> u64 {
    debug_assert!(n >= 1 && n <= 63, "n must be between 1 and 63");
    x.rotate_right(n)
}

/// Rotates the bits of a 32-bit unsigned integer to the left by `n` bits.
/// `n` must be between 1 and 31 inclusive.
#[inline]
pub fn rotl32(x: u32, n: u32) -> u32 {
    debug_assert!(n >= 1 && n <= 31, "n must be between 1 and 31");
    x.rotate_left(n)
}

/// Rotates the bits of a 32-bit unsigned integer to the right by `n` bits.
/// `n` must be between 1 and 31 inclusive.
#[inline]
pub fn rotr32(x: u32, n: u32) -> u32 {
    debug_assert!(n >= 1 && n <= 31, "n must be between 1 and 31");
    x.rotate_right(n)
}

/// Rotates the bits of a size_t value to the left by `n` bits.
/// `n` must be between 1 and (bit_size - 1) inclusive.
#[inline]
pub fn rotl_sz(x: usize, n: u32) -> usize {
    let bit_size = (std::mem::size_of::<usize>() * 8) as u32;
    debug_assert!(n >= 1 && n <= bit_size - 1, "n out of range");
    x.rotate_left(n)
}

/// Rotates the bits of a size_t value to the right by `n` bits.
/// `n` must be between 1 and (bit_size - 1) inclusive.
#[inline]
pub fn rotr_sz(x: usize, n: u32) -> usize {
    let bit_size = (std::mem::size_of::<usize>() * 8) as u32;
    debug_assert!(n >= 1 && n <= bit_size - 1, "n out of range");
    x.rotate_right(n)
}

/// Rotates the bits of a 16-bit unsigned integer to the left by `n` bits.
/// On most targets, `n` can also be 0 or 16 due to promotion rules.
#[inline]
pub fn rotl16(x: u16, n: u32) -> u16 {
    x.rotate_left(n)
}

/// Rotates the bits of a 16-bit unsigned integer to the right by `n` bits.
/// On most targets, `n` can also be 0 or 16 due to promotion rules.
#[inline]
pub fn rotr16(x: u16, n: u32) -> u16 {
    x.rotate_right(n)
}

/// Rotates the bits of an 8-bit unsigned integer to the left by `n` bits.
/// On most targets, `n` can also be 0 or 8 due to promotion rules.
#[inline]
pub fn rotl8(x: u8, n: u32) -> u8 {
    x.rotate_left(n)
}

/// Rotates the bits of an 8-bit unsigned integer to the right by `n` bits.
/// On most targets, `n` can also be 0 or 8 due to promotion rules.
#[inline]
pub fn rotr8(x: u8, n: u32) -> u8 {
    x.rotate_right(n)
}