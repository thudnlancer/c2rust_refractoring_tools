// Note: The original C code uses template-based code generation for different data types.
// In Rust, we'll use generics to achieve similar functionality in a type-safe manner.

use std::ops::{Add, Mul, Sub};
use std::iter::zip;

/// Computes the lag-1 autocorrelation of a dataset
pub fn lag1_autocorrelation<T>(data: &[T], mean: T) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Default,
{
    let n = data.len();
    if n < 2 {
        return T::default();
    }

    let mut q = T::default();
    let mut v = T::default();

    for i in 0..n - 1 {
        let delta0 = data[i] - mean;
        let delta1 = data[i + 1] - mean;
        q = q + delta0 * delta1;
        v = v + delta0 * delta0;
    }

    q / v
}

// Implementations for specific numeric types can be provided as needed
// Rust's type inference and generics will handle the different cases at compile time
// without needing the template approach used in C.

// The actual implementations would be used like:
// lag1_autocorrelation::<f64>(&data, mean)
// lag1_autocorrelation::<f32>(&data, mean)
// etc.