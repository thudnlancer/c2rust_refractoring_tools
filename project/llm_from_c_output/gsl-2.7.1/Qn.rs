// Note: The original C code uses template-based code generation for different data types.
// In Rust, we'll use generics to achieve similar functionality in a type-safe manner.

use std::cmp::Ordering;

// Define a trait for the required operations on different numeric types
pub trait Numeric: 
    Copy + 
    PartialOrd + 
    Into<f64> + 
    std::ops::Add<Output = Self> + 
    std::ops::Sub<Output = Self> + 
    std::ops::Div<Output = Self> + 
    std::ops::Mul<Output = Self> {
    fn zero() -> Self;
    fn one() -> Self;
}

// Implement Numeric trait for all relevant numeric types
macro_rules! impl_numeric {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {
                fn zero() -> Self { 0 as $t }
                fn one() -> Self { 1 as $t }
            }
        )*
    };
}

impl_numeric!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

// Qn calculation function using generics
pub fn qn<T: Numeric>(data: &[T]) -> Result<f64, &'static str> {
    if data.len() < 2 {
        return Err("Input array must have at least 2 elements");
    }

    // Create a sorted copy of the data
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    // Calculate the Qn statistic
    // (Implementation details would go here, similar to the original Qn_source.c)
    // This is a placeholder for the actual calculation logic
    let n = sorted.len();
    let h = n / 2 + 1;
    let mut diffs = Vec::with_capacity(h * (h - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..std::cmp::min(i + h, n) {
            let diff = sorted[j] - sorted[i];
            diffs.push(diff.into());
        }
    }

    diffs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let k = h * (h - 1) / 2;
    let qn = if diffs.len() > k { diffs[k] } else { 0.0 };

    Ok(qn)
}

// Public interface functions for different numeric types
pub fn qn_char(data: &[i8]) -> Result<f64, &'static str> { qn(data) }
pub fn qn_uchar(data: &[u8]) -> Result<f64, &'static str> { qn(data) }
pub fn qn_short(data: &[i16]) -> Result<f64, &'static str> { qn(data) }
pub fn qn_ushort(data: &[u16]) -> Result<f64, &'static str> { qn(data) }
pub fn qn_int(data: &[i32]) -> Result<f64, &'static str> { qn(data) }
pub fn qn_uint(data: &[u32]) -> Result<f64, &'static str> { qn(data) }
pub fn qn_long(data: &[i64]) -> Result<f64, &'static str> { qn(data) }
pub fn qn_ulong(data: &[u64]) -> Result<f64, &'static str> { qn(data) }
pub fn qn_float(data: &[f32]) -> Result<f64, &'static str> { qn(data) }
pub fn qn_double(data: &[f64]) -> Result<f64, &'static str> { qn(data) }