//! Bit rotation functions for various integer types.
//!
//! This module provides functions to rotate bits left and right for integers of various sizes.
//! The implementation is safe and uses Rust's built-in rotation operations where possible.
//!
//! The original C code was Copyright (C) 2008-2023 Free Software Foundation, Inc.
//! and was written by Simon Josefsson <simon@josefsson.org> in 2008.
//!
//! This Rust translation maintains the same functionality while adhering to Rust's safety guarantees.

#![no_std]

use core::num::{Wrapping, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroUsize};

/// Rotates the bits of a 64-bit unsigned integer to the left by `n` bits.
/// `n` must be between 1 and 63 inclusive.
#[inline]
pub fn rotl64(x: u64, n: NonZeroU64) -> u64 {
    x.rotate_left(n.get())
}

/// Rotates the bits of a 64-bit unsigned integer to the right by `n` bits.
/// `n` must be between 1 and 63 inclusive.
#[inline]
pub fn rotr64(x: u64, n: NonZeroU64) -> u64 {
    x.rotate_right(n.get())
}

/// Rotates the bits of a 32-bit unsigned integer to the left by `n` bits.
/// `n` must be between 1 and 31 inclusive.
#[inline]
pub fn rotl32(x: u32, n: NonZeroU32) -> u32 {
    x.rotate_left(n.get())
}

/// Rotates the bits of a 32-bit unsigned integer to the right by `n` bits.
/// `n` must be between 1 and 31 inclusive.
#[inline]
pub fn rotr32(x: u32, n: NonZeroU32) -> u32 {
    x.rotate_right(n.get())
}

/// Rotates the bits of a size_t value to the left by `n` bits.
/// `n` must be between 1 and (bit_size - 1) inclusive.
#[inline]
pub fn rotl_sz(x: usize, n: NonZeroUsize) -> usize {
    x.rotate_left(n.get())
}

/// Rotates the bits of a size_t value to the right by `n` bits.
/// `n` must be between 1 and (bit_size - 1) inclusive.
#[inline]
pub fn rotr_sz(x: usize, n: NonZeroUsize) -> usize {
    x.rotate_right(n.get())
}

/// Rotates the bits of a 16-bit unsigned integer to the left by `n` bits.
/// `n` must be between 1 and 15 inclusive.
#[inline]
pub fn rotl16(x: u16, n: NonZeroU16) -> u16 {
    x.rotate_left(n.get())
}

/// Rotates the bits of a 16-bit unsigned integer to the right by `n` bits.
/// `n` must be between 1 and 15 inclusive.
#[inline]
pub fn rotr16(x: u16, n: NonZeroU16) -> u16 {
    x.rotate_right(n.get())
}

/// Rotates the bits of an 8-bit unsigned integer to the left by `n` bits.
/// `n` must be between 1 and 7 inclusive.
#[inline]
pub fn rotl8(x: u8, n: NonZeroU8) -> u8 {
    x.rotate_left(n.get())
}

/// Rotates the bits of an 8-bit unsigned integer to the right by `n` bits.
/// `n` must be between 1 and 7 inclusive.
#[inline]
pub fn rotr8(x: u8, n: NonZeroU8) -> u8 {
    x.rotate_right(n.get())
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::*;

    #[test]
    fn test_rotations() {
        // Test 64-bit rotations
        assert_eq!(rotl64(0x0123456789ABCDEF, NonZeroU64::new(4).unwrap(), 0x123456789ABCDEF0);
        assert_eq!(rotr64(0x0123456789ABCDEF, NonZeroU64::new(4).unwrap(), 0xF0123456789ABCDE);

        // Test 32-bit rotations
        assert_eq!(rotl32(0x01234567, NonZeroU32::new(4).unwrap(), 0x12345670);
        assert_eq!(rotr32(0x01234567, NonZeroU32::new(4).unwrap(), 0x70123456);

        // Test size_t rotations
        assert_eq!(rotl_sz(0x0123, NonZeroUsize::new(4).unwrap(), 0x1230);
        assert_eq!(rotr_sz(0x0123, NonZeroUsize::new(4).unwrap(), 0x3012);

        // Test 16-bit rotations
        assert_eq!(rotl16(0x0123, NonZeroU16::new(4).unwrap(), 0x1230);
        assert_eq!(rotr16(0x0123, NonZeroU16::new(4).unwrap(), 0x3012);

        // Test 8-bit rotations
        assert_eq!(rotl8(0x12, NonZeroU8::new(4).unwrap(), 0x21);
        assert_eq!(rotr8(0x12, NonZeroU8::new(4).unwrap(), 0x21);
    }
}