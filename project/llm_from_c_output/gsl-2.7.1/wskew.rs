// Note: Since the original C code is using template-based code generation
// through header includes, and Rust doesn't have an exact equivalent,
// we'll implement the functionality using Rust generics and traits.

use std::cmp::Ordering;

pub trait Float: num_traits::Float + num_traits::FromPrimitive {}
impl Float for f32 {}
impl Float for f64 {}
impl Float for f64 {} // long double typically maps to f64 in Rust

pub fn wskew<T: Float>(data: &[T], stride: usize, n: usize) -> T {
    if n == 0 {
        return T::nan();
    }

    let mean = data.iter().step_by(stride).fold(T::zero(), |sum, &x| sum + x) / T::from_usize(n).unwrap();
    
    let (sum2, sum3) = data.iter().step_by(stride).fold((T::zero(), T::zero()), |(s2, s3), &x| {
        let delta = x - mean;
        let delta2 = delta * delta;
        (s2 + delta2, s3 + delta2 * delta)
    });

    let variance = sum2 / T::from_usize(n - 1).unwrap();
    let std_dev = variance.sqrt();
    
    if std_dev == T::zero() {
        return T::nan();
    }

    (sum3 / T::from_usize(n).unwrap()) / (std_dev * std_dev * std_dev)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wskew_f32() {
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        assert!((wskew(&data, 1, 5) - 0.0).abs() < 1e-6);
    }
    
    #[test]
    fn test_wskew_f64() {
        let data = [1.0f64, 2.0, 3.0, 4.0, 5.0];
        assert!((wskew(&data, 1, 5) - 0.0).abs() < 1e-12);
    }
}