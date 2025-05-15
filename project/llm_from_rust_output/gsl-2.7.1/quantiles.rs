use num_traits::{ToPrimitive, Num, NumCast, Float};
use std::marker::PhantomData;

pub type size_t = usize;

trait QuantileType: Num + NumCast + Copy + ToPrimitive {}
impl<T: Num + NumCast + Copy + ToPrimitive> QuantileType for T {}

struct QuantileCalculator<T: QuantileType> {
    _marker: PhantomData<T>,
}

impl<T: QuantileType> QuantileCalculator<T> {
    fn quantile_from_sorted_data(
        sorted_data: &[T],
        stride: size_t,
        n: size_t,
        f: f64,
    ) -> f64 {
        if n == 0 {
            return 0.0;
        }

        let index = f * (n - 1) as f64;
        let lhs = index as size_t;
        let delta = index - lhs as f64;

        if lhs == n - 1 {
            sorted_data[lhs * stride].to_f64().unwrap()
        } else {
            let lower = sorted_data[lhs * stride].to_f64().unwrap();
            let upper = sorted_data[(lhs + 1) * stride].to_f64().unwrap();
            (1.0 - delta) * lower + delta * upper
        }
    }
}

// Type-specific wrapper functions
pub fn gsl_stats_uchar_quantile_from_sorted_data(
    sorted_data: &[u8],
    stride: size_t,
    n: size_t,
    f: f64,
) -> f64 {
    QuantileCalculator::<u8>::quantile_from_sorted_data(sorted_data, stride, n, f)
}

pub fn gsl_stats_ulong_quantile_from_sorted_data(
    sorted_data: &[u64],
    stride: size_t,
    n: size_t,
    f: f64,
) -> f64 {
    QuantileCalculator::<u64>::quantile_from_sorted_data(sorted_data, stride, n, f)
}

pub fn gsl_stats_char_quantile_from_sorted_data(
    sorted_data: &[i8],
    stride: size_t,
    n: size_t,
    f: f64,
) -> f64 {
    QuantileCalculator::<i8>::quantile_from_sorted_data(sorted_data, stride, n, f)
}

pub fn gsl_stats_long_quantile_from_sorted_data(
    sorted_data: &[i64],
    stride: size_t,
    n: size_t,
    f: f64,
) -> f64 {
    QuantileCalculator::<i64>::quantile_from_sorted_data(sorted_data, stride, n, f)
}

pub fn gsl_stats_uint_quantile_from_sorted_data(
    sorted_data: &[u32],
    stride: size_t,
    n: size_t,
    f: f64,
) -> f64 {
    QuantileCalculator::<u32>::quantile_from_sorted_data(sorted_data, stride, n, f)
}

pub fn gsl_stats_int_quantile_from_sorted_data(
    sorted_data: &[i32],
    stride: size_t,
    n: size_t,
    f: f64,
) -> f64 {
    QuantileCalculator::<i32>::quantile_from_sorted_data(sorted_data, stride, n, f)
}

pub fn gsl_stats_ushort_quantile_from_sorted_data(
    sorted_data: &[u16],
    stride: size_t,
    n: size_t,
    f: f64,
) -> f64 {
    QuantileCalculator::<u16>::quantile_from_sorted_data(sorted_data, stride, n, f)
}

pub fn gsl_stats_float_quantile_from_sorted_data(
    sorted_data: &[f32],
    stride: size_t,
    n: size_t,
    f: f64,
) -> f64 {
    QuantileCalculator::<f32>::quantile_from_sorted_data(sorted_data, stride, n, f)
}

pub fn gsl_stats_short_quantile_from_sorted_data(
    sorted_data: &[i16],
    stride: size_t,
    n: size_t,
    f: f64,
) -> f64 {
    QuantileCalculator::<i16>::quantile_from_sorted_data(sorted_data, stride, n, f)
}

pub fn gsl_stats_quantile_from_sorted_data(
    sorted_data: &[f64],
    stride: size_t,
    n: size_t,
    f: f64,
) -> f64 {
    QuantileCalculator::<f64>::quantile_from_sorted_data(sorted_data, stride, n, f)
}

pub fn gsl_stats_long_double_quantile_from_sorted_data(
    sorted_data: &[f64],
    stride: size_t,
    n: size_t,
    f: f64,
) -> f64 {
    QuantileCalculator::<f64>::quantile_from_sorted_data(sorted_data, stride, n, f)
}