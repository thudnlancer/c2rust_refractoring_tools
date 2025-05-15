// Rust doesn't use preprocessor macros like C, so we'll implement generic functions
// for each numeric type using Rust's generics and traits.

use num_traits::{Float, Num, NumCast, ToPrimitive};
use std::iter::Sum;

/// Calculate the mean of a slice of values for any numeric type
pub fn mean<T>(data: &[T]) -> Option<f64>
where
    T: Copy + Sum<T> + Num + NumCast + ToPrimitive,
{
    if data.is_empty() {
        return None;
    }

    let sum: T = data.iter().copied().sum();
    let count = T::from(data.len())?;
    let mean = sum.to_f64()? / count.to_f64()?;
    Some(mean)
}

// Specialized implementations for different numeric types
// These provide type-specific versions similar to the C template approach

pub fn mean_long_double(data: &[f64]) -> Option<f64> {
    mean(data)
}

pub fn mean_double(data: &[f64]) -> Option<f64> {
    mean(data)
}

pub fn mean_float(data: &[f32]) -> Option<f64> {
    mean(data)
}

pub fn mean_ulong(data: &[u64]) -> Option<f64> {
    mean(data)
}

pub fn mean_long(data: &[i64]) -> Option<f64> {
    mean(data)
}

pub fn mean_uint(data: &[u32]) -> Option<f64> {
    mean(data)
}

pub fn mean_int(data: &[i32]) -> Option<f64> {
    mean(data)
}

pub fn mean_ushort(data: &[u16]) -> Option<f64> {
    mean(data)
}

pub fn mean_short(data: &[i16]) -> Option<f64> {
    mean(data)
}

pub fn mean_uchar(data: &[u8]) -> Option<f64> {
    mean(data)
}

pub fn mean_char(data: &[i8]) -> Option<f64> {
    mean(data)
}