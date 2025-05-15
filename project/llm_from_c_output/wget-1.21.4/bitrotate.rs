//! Bit rotation functions for various integer types.
//!
//! This module provides functions for rotating bits in integers of various sizes.
//! The functions are implemented safely in Rust without using any `unsafe` blocks.
//!
//! The original C code was part of the GNU C Library and was released under the
//! GNU Lesser General Public License. This Rust translation maintains the same
//! functionality while adhering to Rust's safety guarantees.

#![no_std]

use core::mem::size_of;
use core::num::{Wrapping, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroUsize};

/// Rotates the bits of a 64-bit unsigned integer to the left.
///
/// # Arguments
/// * `x` - The value to rotate
/// * `n` - Number of bits to rotate (1-63)
///
/// # Panics
/// Panics if `n` is 0 or greater than 63.
#[inline]
pub fn rotl64(x: u64, n: NonZeroU64) -> u64 {
    let n = n.get();
    debug_assert!(n <= 63, "n must be between 1 and 63");
    (x << n) | (x >> (64 - n))
}

/// Rotates the bits of a 64-bit unsigned integer to the right.
///
/// # Arguments
/// * `x` - The value to rotate
/// * `n` - Number of bits to rotate (1-63)
///
/// # Panics
/// Panics if `n` is 0 or greater than 63.
#[inline]
pub fn rotr64(x: u64, n: NonZeroU64) -> u64 {
    let n = n.get();
    debug_assert!(n <= 63, "n must be between 1 and 63");
    (x >> n) | (x << (64 - n))
}

/// Rotates the bits of a 32-bit unsigned integer to the left.
///
/// # Arguments
/// * `x` - The value to rotate
/// * `n` - Number of bits to rotate (1-31)
///
/// # Panics
/// Panics if `n` is 0 or greater than 31.
#[inline]
pub fn rotl32(x: u32, n: NonZeroU32) -> u32 {
    let n = n.get();
    debug_assert!(n <= 31, "n must be between 1 and 31");
    (x << n) | (x >> (32 - n))
}

/// Rotates the bits of a 32-bit unsigned integer to the right.
///
/// # Arguments
/// * `x` - The value to rotate
/// * `n` - Number of bits to rotate (1-31)
///
/// # Panics
/// Panics if `n` is 0 or greater than 31.
#[inline]
pub fn rotr32(x: u32, n: NonZeroU32) -> u32 {
    let n = n.get();
    debug_assert!(n <= 31, "n must be between 1 and 31");
    (x >> n) | (x << (32 - n))
}

/// Rotates the bits of a `usize` value to the left.
///
/// # Arguments
/// * `x` - The value to rotate
/// * `n` - Number of bits to rotate (1 to (8 * sizeof(usize) - 1))
///
/// # Panics
/// Panics if `n` is 0 or greater than or equal to the number of bits in `usize`.
#[inline]
pub fn rotl_sz(x: usize, n: NonZeroUsize) -> usize {
    let bits = 8 * size_of::<usize>();
    let n = n.get();
    debug_assert!(n < bits, "n must be between 1 and {}", bits - 1);
    (x << n) | (x >> (bits - n))
}

/// Rotates the bits of a `usize` value to the right.
///
/// # Arguments
/// * `x` - The value to rotate
/// * `n` - Number of bits to rotate (1 to (8 * sizeof(usize) - 1))
///
/// # Panics
/// Panics if `n` is 0 or greater than or equal to the number of bits in `usize`.
#[inline]
pub fn rotr_sz(x: usize, n: NonZeroUsize) -> usize {
    let bits = 8 * size_of::<usize>();
    let n = n.get();
    debug_assert!(n < bits, "n must be between 1 and {}", bits - 1);
    (x >> n) | (x << (bits - n))
}

/// Rotates the bits of a 16-bit unsigned integer to the left.
///
/// # Arguments
/// * `x` - The value to rotate
/// * `n` - Number of bits to rotate (0-16)
///
/// # Note
/// Unlike other functions, this allows n=0 and n=16 because of automatic widening.
#[inline]
pub fn rotl16(x: u16, n: u32) -> u16 {
    let x = Wrapping(x as u32);
    ((x << n) | (x >> (16 - n))).0 as u16
}

/// Rotates the bits of a 16-bit unsigned integer to the right.
///
/// # Arguments
/// * `x` - The value to rotate
/// * `n` - Number of bits to rotate (0-16)
///
/// # Note
/// Unlike other functions, this allows n=0 and n=16 because of automatic widening.
#[inline]
pub fn rotr16(x: u16, n: u32) -> u16 {
    let x = Wrapping(x as u32);
    ((x >> n) | (x << (16 - n))).0 as u16
}

/// Rotates the bits of an 8-bit unsigned integer to the left.
///
/// # Arguments
/// * `x` - The value to rotate
/// * `n` - Number of bits to rotate (0-8)
///
/// # Note
/// Unlike other functions, this allows n=0 and n=8 because of automatic widening.
#[inline]
pub fn rotl8(x: u8, n: u32) -> u8 {
    let x = Wrapping(x as u32);
    ((x << n) | (x >> (8 - n))).0 as u8
}

/// Rotates the bits of an 8-bit unsigned integer to the right.
///
/// # Arguments
/// * `x` - The value to rotate
/// * `n` - Number of bits to rotate (0-8)
///
/// # Note
/// Unlike other functions, this allows n=0 and n=8 because of automatic widening.
#[inline]
pub fn rotr8(x: u8, n: u32) -> u8 {
    let x = Wrapping(x as u32);
    ((x >> n) | (x << (8 - n))).0 as u8
}