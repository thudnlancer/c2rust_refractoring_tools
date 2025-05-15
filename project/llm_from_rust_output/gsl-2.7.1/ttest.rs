use libc::{c_double, size_t};
use std::f64::consts::SQRT_2;

pub trait GslStats {
    fn mean(data: &[Self], stride: usize) -> c_double;
    fn pvariance(data1: &[Self], stride1: usize, data2: &[Self], stride2: usize) -> c_double;
}

macro_rules! impl_gsl_stats {
    ($type:ty, $mean_fn:ident, $pvariance_fn:ident) => {
        impl GslStats for $type {
            fn mean(data: &[Self], stride: usize) -> c_double {
                unsafe { $mean_fn(data.as_ptr(), stride as size_t, data.len() as size_t) }
            }

            fn pvariance(data1: &[Self], stride1: usize, data2: &[Self], stride2: usize) -> c_double {
                unsafe {
                    $pvariance_fn(
                        data1.as_ptr(),
                        stride1 as size_t,
                        data1.len() as size_t,
                        data2.as_ptr(),
                        stride2 as size_t,
                        data2.len() as size_t,
                    )
                }
            }
        }
    };
}

impl_gsl_stats!(libc::c_char, gsl_stats_char_mean, gsl_stats_char_pvariance);
impl_gsl_stats!(libc::c_uchar, gsl_stats_uchar_mean, gsl_stats_uchar_pvariance);
impl_gsl_stats!(libc::c_short, gsl_stats_short_mean, gsl_stats_short_pvariance);
impl_gsl_stats!(libc::c_ushort, gsl_stats_ushort_mean, gsl_stats_ushort_pvariance);
impl_gsl_stats!(libc::c_int, gsl_stats_int_mean, gsl_stats_int_pvariance);
impl_gsl_stats!(libc::c_uint, gsl_stats_uint_mean, gsl_stats_uint_pvariance);
impl_gsl_stats!(libc::c_long, gsl_stats_long_mean, gsl_stats_long_pvariance);
impl_gsl_stats!(libc::c_ulong, gsl_stats_ulong_mean, gsl_stats_ulong_pvariance);
impl_gsl_stats!(libc::c_float, gsl_stats_float_mean, gsl_stats_float_pvariance);
impl_gsl_stats!(libc::c_double, gsl_stats_mean, gsl_stats_pvariance);
impl_gsl_stats!(f128::f128, gsl_stats_long_double_mean, gsl_stats_long_double_pvariance);

pub fn ttest<T: GslStats>(data1: &[T], stride1: usize, data2: &[T], stride2: usize) -> c_double {
    let mean1 = T::mean(data1, stride1);
    let mean2 = T::mean(data2, stride2);
    let pv = T::pvariance(data1, stride1, data2, stride2);
    let n1 = data1.len() as c_double;
    let n2 = data2.len() as c_double;

    (mean1 - mean2) / (pv * (1.0 / n1 + 1.0 / n2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ttest() {
        let data1 = [1.0, 2.0, 3.0];
        let data2 = [4.0, 5.0, 6.0];
        let result = ttest(&data1, 1, &data2, 1);
        assert!(result < 0.0);
    }
}