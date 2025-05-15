use num_traits::{ToPrimitive, Float};
use std::ops::{Add, Sub, Mul, Div};

pub type size_t = usize;

trait Numeric: Copy + ToPrimitive + Float {}
impl<T: Copy + ToPrimitive + Float> Numeric for T {}

fn compute_variance<T: Numeric>(
    data: &[T],
    stride: size_t,
    n: size_t,
    mean: f64,
) -> f64 {
    let mut variance = 0.0;
    for i in 0..n {
        let delta = data[i * stride].to_f64().unwrap() - mean;
        variance += (delta * delta - variance) / (i + 1) as f64;
    }
    variance
}

fn compute_tss<T: Numeric>(
    data: &[T],
    stride: size_t,
    n: size_t,
    mean: f64,
) -> f64 {
    let mut tss = 0.0;
    for i in 0..n {
        let delta = data[i * stride].to_f64().unwrap() - mean;
        tss += delta * delta;
    }
    tss
}

// Generic variance functions
pub fn variance_with_fixed_mean<T: Numeric>(
    data: &[T],
    stride: size_t,
    n: size_t,
    mean: f64,
) -> f64 {
    compute_variance(data, stride, n, mean)
}

pub fn variance_m<T: Numeric>(
    data: &[T],
    stride: size_t,
    n: size_t,
    mean: f64,
) -> f64 {
    let variance = compute_variance(data, stride, n, mean);
    variance * (n as f64 / (n - 1) as f64)
}

pub fn variance<T: Numeric>(
    data: &[T],
    stride: size_t,
    n: size_t,
) -> f64 {
    let mean = data.iter().take(n).step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / n as f64;
    variance_m(data, stride, n, mean)
}

// Generic standard deviation functions
pub fn sd_with_fixed_mean<T: Numeric>(
    data: &[T],
    stride: size_t,
    n: size_t,
    mean: f64,
) -> f64 {
    compute_variance(data, stride, n, mean).sqrt()
}

pub fn sd_m<T: Numeric>(
    data: &[T],
    stride: size_t,
    n: size_t,
    mean: f64,
) -> f64 {
    (variance_m(data, stride, n, mean)).sqrt()
}

pub fn sd<T: Numeric>(
    data: &[T],
    stride: size_t,
    n: size_t,
) -> f64 {
    let mean = data.iter().take(n).step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / n as f64;
    sd_m(data, stride, n, mean)
}

// Generic total sum of squares functions
pub fn tss_m<T: Numeric>(
    data: &[T],
    stride: size_t,
    n: size_t,
    mean: f64,
) -> f64 {
    compute_tss(data, stride, n, mean)
}

pub fn tss<T: Numeric>(
    data: &[T],
    stride: size_t,
    n: size_t,
) -> f64 {
    let mean = data.iter().take(n).step_by(stride).map(|&x| x.to_f64().unwrap()).sum::<f64>() / n as f64;
    tss_m(data, stride, n, mean)
}

// Type-specific wrappers
macro_rules! impl_stats {
    ($type:ty, $prefix:ident) => {
        paste::item! {
            pub fn [<$prefix _variance_with_fixed_mean>](
                data: &[$type],
                stride: size_t,
                n: size_t,
                mean: f64,
            ) -> f64 {
                variance_with_fixed_mean(data, stride, n, mean)
            }

            pub fn [<$prefix _variance_m>](
                data: &[$type],
                stride: size_t,
                n: size_t,
                mean: f64,
            ) -> f64 {
                variance_m(data, stride, n, mean)
            }

            pub fn [<$prefix _variance>](
                data: &[$type],
                stride: size_t,
                n: size_t,
            ) -> f64 {
                variance(data, stride, n)
            }

            pub fn [<$prefix _sd_with_fixed_mean>](
                data: &[$type],
                stride: size_t,
                n: size_t,
                mean: f64,
            ) -> f64 {
                sd_with_fixed_mean(data, stride, n, mean)
            }

            pub fn [<$prefix _sd_m>](
                data: &[$type],
                stride: size_t,
                n: size_t,
                mean: f64,
            ) -> f64 {
                sd_m(data, stride, n, mean)
            }

            pub fn [<$prefix _sd>](
                data: &[$type],
                stride: size_t,
                n: size_t,
            ) -> f64 {
                sd(data, stride, n)
            }

            pub fn [<$prefix _tss_m>](
                data: &[$type],
                stride: size_t,
                n: size_t,
                mean: f64,
            ) -> f64 {
                tss_m(data, stride, n, mean)
            }

            pub fn [<$prefix _tss>](
                data: &[$type],
                stride: size_t,
                n: size_t,
            ) -> f64 {
                tss(data, stride, n)
            }
        }
    };
}

impl_stats!(i8, char);
impl_stats!(u8, uchar);
impl_stats!(i16, short);
impl_stats!(u16, ushort);
impl_stats!(i32, int);
impl_stats!(u32, uint);
impl_stats!(i64, long);
impl_stats!(u64, ulong);
impl_stats!(f32, float);
impl_stats!(f64, double);