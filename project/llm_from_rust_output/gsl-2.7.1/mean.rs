use num_traits::{ToPrimitive, NumCast};
use f128;

pub type size_t = usize;

fn calculate_mean<T: NumCast + Copy>(data: &[T], stride: usize) -> f64 {
    let mut mean = f128::f128::new(0);
    for (i, &value) in data.iter().step_by(stride).enumerate() {
        let value_f128 = f128::f128::new(value.to_f64().unwrap());
        mean += (value_f128 - mean) / f128::f128::new((i + 1) as f64);
    }
    mean.to_f64().unwrap()
}

macro_rules! impl_mean {
    ($name:ident, $type:ty) => {
        #[no_mangle]
        pub extern "C" fn $name(data: *const $type, stride: size_t, size: size_t) -> f64 {
            let data_slice = unsafe { std::slice::from_raw_parts(data, size) };
            calculate_mean(data_slice, stride)
        }
    };
}

impl_mean!(gsl_stats_ulong_mean, libc::c_ulong);
impl_mean!(gsl_stats_uchar_mean, libc::c_uchar);
impl_mean!(gsl_stats_uint_mean, libc::c_uint);
impl_mean!(gsl_stats_long_double_mean, f128::f128);
impl_mean!(gsl_stats_mean, libc::c_double);
impl_mean!(gsl_stats_float_mean, libc::c_float);
impl_mean!(gsl_stats_char_mean, libc::c_char);
impl_mean!(gsl_stats_long_mean, libc::c_long);
impl_mean!(gsl_stats_int_mean, libc::c_int);
impl_mean!(gsl_stats_ushort_mean, libc::c_ushort);
impl_mean!(gsl_stats_short_mean, libc::c_short);