//! Bit rotation functions for various integer types.
//!
//! This module provides functions to rotate bits left and right for integers of
//! various sizes (8, 16, 32, 64 bits and size_t equivalent).
//!
//! The implementation follows the same logic as the original C code but uses
//! Rust's built-in rotate operations which are guaranteed to be safe and
//! efficient.

#![allow(non_snake_case)]

/// Rotates a 64-bit unsigned integer left by `n` bits.
/// `n` must be between 1 and 63 inclusive.
#[inline]
pub fn rotl64(x: u64, n: u32) -> u64 {
    x.rotate_left(n)
}

/// Rotates a 64-bit unsigned integer right by `n` bits.
/// `n` must be between 1 and 63 inclusive.
#[inline]
pub fn rotr64(x: u64, n: u32) -> u64 {
    x.rotate_right(n)
}

/// Rotates a 32-bit unsigned integer left by `n` bits.
/// `n` must be between 1 and 31 inclusive.
#[inline]
pub fn rotl32(x: u32, n: u32) -> u32 {
    x.rotate_left(n)
}

/// Rotates a 32-bit unsigned integer right by `n` bits.
/// `n` must be between 1 and 31 inclusive.
#[inline]
pub fn rotr32(x: u32, n: u32) -> u32 {
    x.rotate_right(n)
}

/// Rotates a `usize` value left by `n` bits.
/// `n` must be between 1 and (bits in usize - 1) inclusive.
#[inline]
pub fn rotl_sz(x: usize, n: u32) -> usize {
    x.rotate_left(n)
}

/// Rotates a `usize` value right by `n` bits.
/// `n` must be between 1 and (bits in usize - 1) inclusive.
#[inline]
pub fn rotr_sz(x: usize, n: u32) -> usize {
    x.rotate_right(n)
}

/// Rotates a 16-bit unsigned integer left by `n` bits.
/// On most platforms, `n` can also be 0 and 16 due to promotion rules.
#[inline]
pub fn rotl16(x: u16, n: u32) -> u16 {
    x.rotate_left(n)
}

/// Rotates a 16-bit unsigned integer right by `n` bits.
/// On most platforms, `n` can also be 0 and 16 due to promotion rules.
#[inline]
pub fn rotr16(x: u16, n: u32) -> u16 {
    x.rotate_right(n)
}

/// Rotates an 8-bit unsigned integer left by `n` bits.
/// On most platforms, `n` can also be 0 and 8 due to promotion rules.
#[inline]
pub fn rotl8(x: u8, n: u32) -> u8 {
    x.rotate_left(n)
}

/// Rotates an 8-bit unsigned integer right by `n` bits.
/// On most platforms, `n` can also be 0 and 8 due to promotion rules.
#[inline]
pub fn rotr8(x: u8, n: u32) -> u8 {
    x.rotate_right(n)
}