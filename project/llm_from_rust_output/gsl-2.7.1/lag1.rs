use num_traits::{ToPrimitive, NumCast};
use f128;

pub type size_t = usize;

pub fn gsl_stats_lag1_autocorrelation<T: NumCast + Copy>(
    data: &[T],
    stride: size_t,
) -> f64 {
    let mean = gsl_stats_mean(data, stride);
    gsl_stats_lag1_autocorrelation_m(data, stride, mean)
}

pub fn gsl_stats_lag1_autocorrelation_m<T: NumCast + Copy>(
    data: &[T],
    stride: size_t,
    mean: f64,
) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut q = f128::f128::ZERO;
    let mut v = f128::f128::ZERO;

    // Initialize with first element
    let first_val: f64 = NumCast::from(data[0]).unwrap();
    let delta = first_val - mean;
    v = f128::f128::new(delta * delta);

    for i in 1..data.len() {
        let prev_idx = i.wrapping_sub(1);
        if prev_idx >= data.len() {
            break;
        }

        let delta0: f64 = NumCast::from(data[prev_idx]).unwrap() - mean;
        let delta1: f64 = NumCast::from(data[i]).unwrap() - mean;

        let i_f128 = f128::f128::new((i + 1) as f64);
        q = q + (f128::f128::new(delta0 * delta1) - q) / i_f128;
        v = v + (f128::f128::new(delta1 * delta1) - v) / i_f128;
    }

    (q / v).to_f64().unwrap_or(0.0)
}

pub fn gsl_stats_mean<T: NumCast + Copy>(data: &[T], stride: size_t) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut sum = 0.0;
    let mut count = 0;

    for (i, item) in data.iter().enumerate() {
        if i % stride == 0 {
            sum += NumCast::from(*item).unwrap_or(0.0);
            count += 1;
        }
    }

    if count == 0 {
        0.0
    } else {
        sum / count as f64
    }
}

// Implement type-specific versions using macros to reduce code duplication
macro_rules! impl_autocorrelation {
    ($type:ty, $mean_fn:ident) => {
        pub fn $mean_fn(data: &[$type], stride: size_t) -> f64 {
            gsl_stats_mean(data, stride)
        }

        pub fn gsl_stats_${type}_lag1_autocorrelation(
            data: &[$type],
            stride: size_t,
        ) -> f64 {
            let mean = $mean_fn(data, stride);
            gsl_stats_lag1_autocorrelation_m(data, stride, mean)
        }
    };
}

impl_autocorrelation!(f32, gsl_stats_float_mean);
impl_autocorrelation!(f128::f128, gsl_stats_long_double_mean);
impl_autocorrelation!(u64, gsl_stats_ulong_mean);
impl_autocorrelation!(i64, gsl_stats_long_mean);
impl_autocorrelation!(u32, gsl_stats_uint_mean);
impl_autocorrelation!(i32, gsl_stats_int_mean);
impl_autocorrelation!(u16, gsl_stats_ushort_mean);
impl_autocorrelation!(i16, gsl_stats_short_mean);
impl_autocorrelation!(u8, gsl_stats_uchar_mean);
impl_autocorrelation!(i8, gsl_stats_char_mean);