// Note: Since the original C code is using GSL (GNU Scientific Library) templates
// to generate permutation functions for various data types, we'll implement similar
// functionality in Rust using generics and traits. Rust doesn't have an exact
// equivalent to C's template system, but we can achieve similar functionality.

use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PermutationError {
    LengthMismatch,
    InvalidIndex,
}

impl fmt::Display for PermutationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PermutationError::LengthMismatch => write!(f, "Length mismatch between vectors"),
            PermutationError::InvalidIndex => write!(f, "Invalid index in permutation"),
        }
    }
}

impl Error for PermutationError {}

pub trait Permutable: Clone + PartialEq {}
impl<T: Clone + PartialEq> Permutable for T {}

pub fn permute<T: Permutable>(
    data: &mut [T],
    p: &[usize],
) -> Result<(), PermutationError> {
    if data.len() != p.len() {
        return Err(PermutationError::LengthMismatch);
    }

    let mut visited = vec![false; data.len()];

    for i in 0..data.len() {
        if visited[i] {
            continue;
        }

        let mut current = i;
        let mut cycle_start = data[current].clone();

        loop {
            let next = p[current];
            if next >= data.len() {
                return Err(PermutationError::InvalidIndex);
            }

            if visited[next] {
                break;
            }

            visited[current] = true;
            data[current] = data[next].clone();
            current = next;
        }

        data[current] = cycle_start;
        visited[current] = true;
    }

    Ok(())
}

// Implement permutation functions for various numeric types using the generic permute function
// This replaces the template-based approach in C

pub fn permute_complex_long(data: &mut [num_complex::Complex<i64>], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_complex(data: &mut [num_complex::Complex<f64>], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_complex_float(data: &mut [num_complex::Complex<f32>], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_long_double(data: &mut [f64], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_double(data: &mut [f64], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_float(data: &mut [f32], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_ulong(data: &mut [u64], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_long(data: &mut [i64], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_uint(data: &mut [u32], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_int(data: &mut [i32], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_ushort(data: &mut [u16], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_short(data: &mut [i16], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_uchar(data: &mut [u8], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}

pub fn permute_char(data: &mut [i8], p: &[usize]) -> Result<(), PermutationError> {
    permute(data, p)
}