//! Bit rotation functions for various integer types.
//!
//! This module provides functions for rotating bits in integers of various sizes.
//! The functions are implemented safely in Rust without using any unsafe blocks.
//!
//! The original C code was:
//! Copyright (C) 2008-2021 Free Software Foundation, Inc.
//! Licensed under GNU General Public License version 3 or later.

#![allow(dead_code)]

/// Rotate a 64-bit unsigned integer left by `n` bits.
/// `n` must be between 1 and 63 inclusive.
#[inline]
pub fn rotl64(x: u64, n: u32) -> u64 {
    assert!(n >= 1 && n <= 63, "n must be between 1 and 63");
    x.rotate_left(n)
}

/// Rotate a 64-bit unsigned integer right by `n` bits.
/// `n` must be between 1 and 63 inclusive.
#[inline]
pub fn rotr64(x: u64, n: u32) -> u64 {
    assert!(n >= 1 && n <= 63, "n must be between 1 and 63");
    x.rotate_right(n)
}

/// Rotate a 32-bit unsigned integer left by `n` bits.
/// `n` must be between 1 and 31 inclusive.
#[inline]
pub fn rotl32(x: u32, n: u32) -> u32 {
    assert!(n >= 1 && n <= 31, "n must be between 1 and 31");
    x.rotate_left(n)
}

/// Rotate a 32-bit unsigned integer right by `n` bits.
/// `n` must be between 1 and 31 inclusive.
#[inline]
pub fn rotr32(x: u32, n: u32) -> u32 {
    assert!(n >= 1 && n <= 31, "n must be between 1 and 31");
    x.rotate_right(n)
}

/// Rotate a size_t value left by `n` bits.
/// `n` must be between 1 and (bit_size - 1) inclusive.
#[inline]
pub fn rotl_sz(x: usize, n: u32) -> usize {
    let max_n = (std::mem::size_of::<usize>() * 8) - 1;
    assert!(n >= 1 && n <= max_n as u32, "n must be between 1 and {}", max_n);
    x.rotate_left(n)
}

/// Rotate a size_t value right by `n` bits.
/// `n` must be between 1 and (bit_size - 1) inclusive.
#[inline]
pub fn rotr_sz(x: usize, n: u32) -> usize {
    let max_n = (std::mem::size_of::<usize>() * 8) - 1;
    assert!(n >= 1 && n <= max_n as u32, "n must be between 1 and {}", max_n);
    x.rotate_right(n)
}

/// Rotate a 16-bit unsigned integer left by `n` bits.
/// On most platforms, `n` can also be 0 and 16 due to integer promotion rules.
#[inline]
pub fn rotl16(x: u16, n: u32) -> u16 {
    x.rotate_left(n)
}

/// Rotate a 16-bit unsigned integer right by `n` bits.
/// On most platforms, `n` can also be 0 and 16 due to integer promotion rules.
#[inline]
pub fn rotr16(x: u16, n: u32) -> u16 {
    x.rotate_right(n)
}

/// Rotate an 8-bit unsigned integer left by `n` bits.
/// On most platforms, `n` can also be 0 and 8 due to integer promotion rules.
#[inline]
pub fn rotl8(x: u8, n: u32) -> u8 {
    x.rotate_left(n)
}

/// Rotate an 8-bit unsigned integer right by `n` bits.
/// On most platforms, `n` can also be 0 and 8 due to integer promotion rules.
#[inline]
pub fn rotr8(x: u8, n: u32) -> u8 {
    x.rotate_right(n)
}