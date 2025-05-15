// Note: The original C code uses template-based code generation for different numeric types.
// In Rust, we can use generics to achieve similar functionality in a type-safe manner.

use std::convert::TryInto;

pub fn p_variance<T>(data: &[T], mean: T) -> Result<T, &'static str>
where
    T: Copy
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Div<Output = T>
        + From<f64>
        + Default,
{
    if data.is_empty() {
        return Err("empty dataset");
    }

    let n = T::from(data.len() as f64);
    let mut sum = T::default();

    for &x in data {
        let delta = x - mean;
        sum = sum + delta * delta;
    }

    Ok(sum / n)
}

// Implement specialized versions for common numeric types
// This provides better performance than the generic version
// while maintaining type safety

macro_rules! impl_p_variance {
    ($type:ty) => {
        pub fn p_variance_$type(data: &[$type], mean: $type) -> Result<$type, &'static str> {
            if data.is_empty() {
                return Err("empty dataset");
            }

            let n = data.len() as $type;
            let mut sum = 0.0 as $type;

            for &x in data {
                let delta = x - mean;
                sum += delta * delta;
            }

            Ok(sum / n)
        }
    };
}

impl_p_variance!(f64);
impl_p_variance!(f32);
impl_p_variance!(i64);
impl_p_variance!(u64);
impl_p_variance!(i32);
impl_p_variance!(u32);
impl_p_variance!(i16);
impl_p_variance!(u16);
impl_p_variance!(i8);
impl_p_variance!(u8);