// Note: The original C code uses template-based code generation for different data types.
// In Rust, we'll use generics to achieve similar functionality in a type-safe manner.

use std::cmp::Ordering;
use std::iter::Sum;
use std::ops::{Add, Div, Sub};
use num_traits::{Float, Signed, Zero, NumCast};

pub fn absdev<T>(data: &[T]) -> Option<T>
where
    T: Copy + Sum<T> + Div<Output = T> + Sub<Output = T> + Add<Output = T> + Zero + NumCast + PartialOrd,
    f64: Into<T>,
{
    let len = data.len();
    if len == 0 {
        return None;
    }

    let mean = {
        let sum: T = data.iter().copied().sum();
        sum / T::from(len).unwrap()
    };

    let abs_dev: T = data.iter()
        .map(|&x| {
            let diff = if x > mean { x - mean } else { mean - x };
            diff
        })
        .sum();

    Some(abs_dev / T::from(len).unwrap())
}

// Specialized implementations for floating point types for better precision
pub fn absdev_float(data: &[f32]) -> Option<f32> {
    absdev(data)
}

pub fn absdev_double(data: &[f64]) -> Option<f64> {
    absdev(data)
}

pub fn absdev_long_double(data: &[f64]) -> Option<f64> {
    absdev(data)
}

// Signed integer implementations
pub fn absdev_char(data: &[i8]) -> Option<f64> {
    absdev_signed(data)
}

pub fn absdev_short(data: &[i16]) -> Option<f64> {
    absdev_signed(data)
}

pub fn absdev_int(data: &[i32]) -> Option<f64> {
    absdev_signed(data)
}

pub fn absdev_long(data: &[i64]) -> Option<f64> {
    absdev_signed(data)
}

// Unsigned integer implementations
pub fn absdev_uchar(data: &[u8]) -> Option<f64> {
    absdev_unsigned(data)
}

pub fn absdev_ushort(data: &[u16]) -> Option<f64> {
    absdev_unsigned(data)
}

pub fn absdev_uint(data: &[u32]) -> Option<f64> {
    absdev_unsigned(data)
}

pub fn absdev_ulong(data: &[u64]) -> Option<f64> {
    absdev_unsigned(data)
}

// Helper functions for integer types (which need to be converted to floating point)
fn absdev_signed<T>(data: &[T]) -> Option<f64>
where
    T: Copy + Sum<T> + Into<f64> + PartialOrd,
{
    if data.is_empty() {
        return None;
    }

    let len = data.len() as f64;
    let mean = data.iter().copied().sum::<T>().into() / len;

    let abs_dev: f64 = data.iter()
        .map(|&x| {
            let val: f64 = x.into();
            (val - mean).abs()
        })
        .sum();

    Some(abs_dev / len)
}

fn absdev_unsigned<T>(data: &[T]) -> Option<f64>
where
    T: Copy + Sum<T> + Into<f64> + PartialOrd,
{
    absdev_signed(data)
}