// Note: The original C code appears to be using template-based code generation
// for different numeric types. In Rust, we can achieve similar functionality
// using generics and traits.

// We'll define a module structure similar to GSL's statistics functionality
// but implemented in safe Rust.

pub mod statistics {
    use num_traits::{Float, Num, NumCast};
    use std::cmp::Ordering;

    // Generic statistical functions will be implemented here
    // For example:
    pub fn mean<T: Float + NumCast>(data: &[T]) -> Option<T> {
        if data.is_empty() {
            return None;
        }
        
        let sum: T = data.iter().fold(T::zero(), |acc, &x| acc + x);
        let count = T::from(data.len()).unwrap();
        Some(sum / count)
    }

    // More statistical functions would be implemented similarly
    // for all supported numeric types through generic implementations
}

// The original C code uses template instantiation for different types.
// In Rust, we can implement traits for all needed numeric types.

// Supported numeric types:
// - Floating point: f32, f64
// - Signed integers: i8, i16, i32, i64, isize
// - Unsigned integers: u8, u16, u32, u64, usize

// The actual implementations would be generic over these types
// using traits from the num-traits crate for numeric operations.

// Error handling would use Rust's Result type where appropriate
// rather than GSL's error codes.