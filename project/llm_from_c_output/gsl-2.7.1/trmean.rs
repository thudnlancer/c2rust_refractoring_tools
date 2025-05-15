// Note: Since the original C code is using template-based code generation via includes,
// we'll use Rust generics to achieve similar functionality in a type-safe manner.

use std::cmp::Ordering;

pub fn trmean<T>(data: &[T], alpha: f64) -> Option<T>
where
    T: Copy + PartialOrd + Into<f64> + From<f64>,
{
    if data.is_empty() || alpha < 0.0 || alpha >= 0.5 {
        return None;
    }

    let n = data.len();
    let k = (alpha * n as f64).floor() as usize;

    if 2 * k >= n {
        return None;
    }

    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let trimmed = &sorted[k..n - k];
    let sum: f64 = trimmed.iter().map(|&x| x.into()).sum();
    let mean = sum / (n - 2 * k) as f64;

    Some(T::from(mean))
}

// Implement specialized versions for common numeric types
#[allow(dead_code)]
pub mod trmean_impl {
    use super::trmean;

    pub fn trmean_long_double(data: &[f64], alpha: f64) -> Option<f64> {
        trmean(data, alpha)
    }

    pub fn trmean_double(data: &[f64], alpha: f64) -> Option<f64> {
        trmean(data, alpha)
    }

    pub fn trmean_float(data: &[f32], alpha: f64) -> Option<f32> {
        trmean(data, alpha)
    }

    pub fn trmean_ulong(data: &[u64], alpha: f64) -> Option<u64> {
        trmean(data, alpha)
    }

    pub fn trmean_long(data: &[i64], alpha: f64) -> Option<i64> {
        trmean(data, alpha)
    }

    pub fn trmean_uint(data: &[u32], alpha: f64) -> Option<u32> {
        trmean(data, alpha)
    }

    pub fn trmean_int(data: &[i32], alpha: f64) -> Option<i32> {
        trmean(data, alpha)
    }

    pub fn trmean_ushort(data: &[u16], alpha: f64) -> Option<u16> {
        trmean(data, alpha)
    }

    pub fn trmean_short(data: &[i16], alpha: f64) -> Option<i16> {
        trmean(data, alpha)
    }

    pub fn trmean_uchar(data: &[u8], alpha: f64) -> Option<u8> {
        trmean(data, alpha)
    }

    pub fn trmean_char(data: &[i8], alpha: f64) -> Option<i8> {
        trmean(data, alpha)
    }
}