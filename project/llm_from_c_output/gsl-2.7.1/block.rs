// Note: The original C code uses template-based code generation for different data types.
// In Rust, we can achieve similar functionality using generics and traits.

use std::mem;
use std::ptr;
use std::slice;

pub struct GslBlock<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> GslBlock<T> {
    pub fn new(size: usize) -> Option<Self> {
        if size == 0 {
            return None;
        }
        Some(GslBlock {
            size,
            data: Vec::with_capacity(size),
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

// Implementations for various numeric types
macro_rules! impl_gsl_block {
    ($type:ty) => {
        impl GslBlock<$type> {
            pub fn new_initialized(size: usize, value: $type) -> Option<Self> {
                if size == 0 {
                    return None;
                }
                Some(GslBlock {
                    size,
                    data: vec![value; size],
                })
            }
        }
    };
}

impl_gsl_block!(f32);
impl_gsl_block!(f64);
impl_gsl_block!(i8);
impl_gsl_block!(u8);
impl_gsl_block!(i16);
impl_gsl_block!(u16);
impl_gsl_block!(i32);
impl_gsl_block!(u32);
impl_gsl_block!(i64);
impl_gsl_block!(u64);
impl_gsl_block!(isize);
impl_gsl_block!(usize);

// For complex numbers, we would need a complex number type
// Here's a basic implementation for complex numbers
#[derive(Debug, Clone, Copy)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl_gsl_block!(Complex<f32>);
impl_gsl_block!(Complex<f64>);
impl_gsl_block!(Complex<i64>);