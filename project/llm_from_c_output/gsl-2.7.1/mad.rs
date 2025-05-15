// Rust implementation of GSL's median absolute deviation (MAD) calculation
// for various numeric types using generics instead of C templates

use std::cmp::Ordering;

pub fn mad_long_double(data: &[f64]) -> Option<f64> {
    mad_generic(data)
}

pub fn mad_double(data: &[f64]) -> Option<f64> {
    mad_generic(data)
}

pub fn mad_float(data: &[f32]) -> Option<f32> {
    mad_generic(data)
}

pub fn mad_ulong(data: &[u64]) -> Option<u64> {
    mad_generic(data)
}

pub fn mad_long(data: &[i64]) -> Option<i64> {
    mad_generic(data)
}

pub fn mad_uint(data: &[u32]) -> Option<u32> {
    mad_generic(data)
}

pub fn mad_int(data: &[i32]) -> Option<i32> {
    mad_generic(data)
}

pub fn mad_ushort(data: &[u16]) -> Option<u16> {
    mad_generic(data)
}

pub fn mad_short(data: &[i16]) -> Option<i16> {
    mad_generic(data)
}

pub fn mad_uchar(data: &[u8]) -> Option<u8> {
    mad_generic(data)
}

pub fn mad_char(data: &[i8]) -> Option<i8> {
    mad_generic(data)
}

fn mad_generic<T>(data: &[T]) -> Option<T>
where
    T: Copy + Ord + std::ops::Sub<Output = T> + std::ops::Div<Output = T> + From<u8>,
{
    if data.is_empty() {
        return None;
    }

    let median = median(data)?;
    let mut deviations: Vec<T> = data.iter().map(|&x| if x > median { x - median } else { median - x }).collect();
    
    deviations.sort_unstable();
    median_generic(&deviations)
}

fn median_generic<T>(data: &[T]) -> Option<T>
where
    T: Copy + Ord,
{
    if data.is_empty() {
        return None;
    }

    let mut sorted = data.to_vec();
    sorted.sort_unstable();

    let n = sorted.len();
    let middle = n / 2;

    if n % 2 == 1 {
        Some(sorted[middle])
    } else {
        Some(sorted[middle - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mad_double() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        assert_eq!(mad_double(&data), Some(2.0));
    }

    #[test]
    fn test_mad_int() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(mad_int(&data), Some(2));
    }

    #[test]
    fn test_empty() {
        let empty: [i32; 0] = [];
        assert_eq!(mad_int(&empty), None);
    }
}