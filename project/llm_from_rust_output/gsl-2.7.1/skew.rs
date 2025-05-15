use num_traits::{ToPrimitive, AsPrimitive};
use f128;

pub type size_t = usize;

pub trait GslStatsSkew {
    fn mean(data: &[Self], stride: size_t) -> f64;
    fn sd_m(data: &[Self], stride: size_t, mean: f64) -> f64;
}

macro_rules! impl_gsl_stats_skew {
    ($type:ty, $mean_fn:ident, $sd_m_fn:ident) => {
        impl GslStatsSkew for $type {
            fn mean(data: &[Self], stride: size_t) -> f64 {
                unsafe { $mean_fn(data.as_ptr(), stride, data.len() as size_t) }
            }

            fn sd_m(data: &[Self], stride: size_t, mean: f64) -> f64 {
                unsafe { $sd_m_fn(data.as_ptr(), stride, data.len() as size_t, mean) }
            }
        }

        pub fn skew(data: &[$type], stride: size_t) -> f64 {
            let mean = <$type as GslStatsSkew>::mean(data, stride);
            let sd = <$type as GslStatsSkew>::sd_m(data, stride, mean);
            skew_m_sd(data, stride, mean, sd)
        }

        pub fn skew_m_sd(data: &[$type], stride: size_t, mean: f64, sd: f64) -> f64 {
            let mut skew = f128::f128::new(0);
            for i in 0..data.len() {
                let idx = i.wrapping_mul(stride);
                if idx >= data.len() {
                    continue;
                }
                let x = f128::f128::new((data[idx] as f64 - mean) / sd);
                skew += (x * x * x - skew) / f128::f128::new((i + 1) as f64);
            }
            skew.to_f64().unwrap()
        }
    };
}

impl_gsl_stats_skew!(i8, gsl_stats_char_mean, gsl_stats_char_sd_m);
impl_gsl_stats_skew!(u8, gsl_stats_uchar_mean, gsl_stats_uchar_sd_m);
impl_gsl_stats_skew!(i16, gsl_stats_short_mean, gsl_stats_short_sd_m);
impl_gsl_stats_skew!(u16, gsl_stats_ushort_mean, gsl_stats_ushort_sd_m);
impl_gsl_stats_skew!(i32, gsl_stats_int_mean, gsl_stats_int_sd_m);
impl_gsl_stats_skew!(u32, gsl_stats_uint_mean, gsl_stats_uint_sd_m);
impl_gsl_stats_skew!(i64, gsl_stats_long_mean, gsl_stats_long_sd_m);
impl_gsl_stats_skew!(u64, gsl_stats_ulong_mean, gsl_stats_ulong_sd_m);
impl_gsl_stats_skew!(f32, gsl_stats_float_mean, gsl_stats_float_sd_m);
impl_gsl_stats_skew!(f64, gsl_stats_mean, gsl_stats_sd_m);
impl_gsl_stats_skew!(f128::f128, gsl_stats_long_double_mean, gsl_stats_long_double_sd_m);

extern "C" {
    fn gsl_stats_char_mean(data: *const i8, stride: size_t, n: size_t) -> f64;
    fn gsl_stats_char_sd_m(data: *const i8, stride: size_t, n: size_t, mean: f64) -> f64;
    fn gsl_stats_uchar_mean(data: *const u8, stride: size_t, n: size_t) -> f64;
    fn gsl_stats_uchar_sd_m(data: *const u8, stride: size_t, n: size_t, mean: f64) -> f64;
    fn gsl_stats_short_mean(data: *const i16, stride: size_t, n: size_t) -> f64;
    fn gsl_stats_short_sd_m(data: *const i16, stride: size_t, n: size_t, mean: f64) -> f64;
    fn gsl_stats_ushort_mean(data: *const u16, stride: size_t, n: size_t) -> f64;
    fn gsl_stats_ushort_sd_m(data: *const u16, stride: size_t, n: size_t, mean: f64) -> f64;
    fn gsl_stats_int_mean(data: *const i32, stride: size_t, n: size_t) -> f64;
    fn gsl_stats_int_sd_m(data: *const i32, stride: size_t, n: size_t, mean: f64) -> f64;
    fn gsl_stats_uint_mean(data: *const u32, stride: size_t, n: size_t) -> f64;
    fn gsl_stats_uint_sd_m(data: *const u32, stride: size_t, n: size_t, mean: f64) -> f64;
    fn gsl_stats_long_mean(data: *const i64, stride: size_t, n: size_t) -> f64;
    fn gsl_stats_long_sd_m(data: *const i64, stride: size_t, n: size_t, mean: f64) -> f64;
    fn gsl_stats_ulong_mean(data: *const u64, stride: size_t, n: size_t) -> f64;
    fn gsl_stats_ulong_sd_m(data: *const u64, stride: size_t, n: size_t, mean: f64) -> f64;
    fn gsl_stats_float_mean(data: *const f32, stride: size_t, n: size_t) -> f64;
    fn gsl_stats_float_sd_m(data: *const f32, stride: size_t, n: size_t, mean: f64) -> f64;
    fn gsl_stats_mean(data: *const f64, stride: size_t, n: size_t) -> f64;
    fn gsl_stats_sd_m(data: *const f64, stride: size_t, n: size_t, mean: f64) -> f64;
    fn gsl_stats_long_double_mean(data: *const f128::f128, stride: size_t, n: size_t) -> f64;
    fn gsl_stats_long_double_sd_m(data: *const f128::f128, stride: size_t, n: size_t, mean: f64) -> f64;
}