// Note: The original C code uses template-based code generation for different numeric types.
// In Rust, we'll use generics to achieve similar functionality in a type-safe manner.

use std::ops::{Add, Sub, Mul, Div};
use std::iter::Sum;

trait Numeric:
    Copy +
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    Sum<Self> +
    Into<f64>
{
    fn zero() -> Self;
    fn one() -> Self;
}

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

impl_numeric!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

macro_rules! impl_numeric_float {
    ($($t:ty),*) => {
        $(
            impl Numeric for $t {
                fn zero() -> Self { 0.0 }
                fn one() -> Self { 1.0 }
            }
        )*
    };
}

impl_numeric_float!(f32, f64);

fn variance<T: Numeric>(data: &[T]) -> f64 {
    let mean = mean(data);
    let sum_sq_diff: f64 = data.iter()
        .map(|&x| {
            let diff = x.into() - mean;
            diff * diff
        })
        .sum();
    sum_sq_diff / data.len() as f64
}

fn mean<T: Numeric>(data: &[T]) -> f64 {
    let sum: f64 = data.iter().map(|&x| x.into()).sum();
    sum / data.len() as f64
}

// For long double (f128) support, we'd need to use a crate like `num-traits`
// or wait for f128 to stabilize in Rust
#[cfg(feature = "f128")]
mod f128_support {
    use super::*;
    use num_traits::Float;
    
    impl Numeric for f128 {
        fn zero() -> Self { 0.0 }
        fn one() -> Self { 1.0 }
    }
}

// The original C code generates specialized versions for each type via templates.
// In Rust, we use generic functions that work for all Numeric types.
// The compiler will generate specialized versions as needed during monomorphization.