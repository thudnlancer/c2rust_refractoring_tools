use libc::{c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort};
use f128;

pub type size_t = c_ulong;

// Define a trait for all the types that can compute Gastwirth statistics
pub trait GastwirthStats {
    fn median_from_sorted_data(sorted_data: &[Self], stride: size_t) -> c_double;
    fn quantile_from_sorted_data(sorted_data: &[Self], stride: size_t, f: c_double) -> c_double;
}

// Implement the trait for all supported types
macro_rules! impl_gastwirth_stats {
    ($type:ty, $median_fn:ident, $quantile_fn:ident) => {
        impl GastwirthStats for $type {
            fn median_from_sorted_data(sorted_data: &[Self], stride: size_t) -> c_double {
                unsafe { $median_fn(sorted_data.as_ptr(), stride, sorted_data.len() as size_t) }
            }
            
            fn quantile_from_sorted_data(sorted_data: &[Self], stride: size_t, f: c_double) -> c_double {
                unsafe { $quantile_fn(sorted_data.as_ptr(), stride, sorted_data.len() as size_t, f) }
            }
        }
    };
}

// Implement for each type using the GSL functions
impl_gastwirth_stats!(c_uchar, gsl_stats_uchar_median_from_sorted_data, gsl_stats_uchar_quantile_from_sorted_data);
impl_gastwirth_stats!(c_char, gsl_stats_char_median_from_sorted_data, gsl_stats_char_quantile_from_sorted_data);
impl_gastwirth_stats!(c_ulong, gsl_stats_ulong_median_from_sorted_data, gsl_stats_ulong_quantile_from_sorted_data);
impl_gastwirth_stats!(c_long, gsl_stats_long_median_from_sorted_data, gsl_stats_long_quantile_from_sorted_data);
impl_gastwirth_stats!(c_uint, gsl_stats_uint_median_from_sorted_data, gsl_stats_uint_quantile_from_sorted_data);
impl_gastwirth_stats!(c_int, gsl_stats_int_median_from_sorted_data, gsl_stats_int_quantile_from_sorted_data);
impl_gastwirth_stats!(c_ushort, gsl_stats_ushort_median_from_sorted_data, gsl_stats_ushort_quantile_from_sorted_data);
impl_gastwirth_stats!(c_short, gsl_stats_short_median_from_sorted_data, gsl_stats_short_quantile_from_sorted_data);
impl_gastwirth_stats!(c_float, gsl_stats_float_median_from_sorted_data, gsl_stats_float_quantile_from_sorted_data);
impl_gastwirth_stats!(c_double, gsl_stats_median_from_sorted_data, gsl_stats_quantile_from_sorted_data);
impl_gastwirth_stats!(f128::f128, gsl_stats_long_double_median_from_sorted_data, gsl_stats_long_double_quantile_from_sorted_data);

// Generic Gastwirth function that works for any type implementing GastwirthStats
pub fn gastwirth_from_sorted_data<T: GastwirthStats>(
    sorted_data: &[T],
    stride: size_t,
) -> c_double {
    if sorted_data.is_empty() {
        return 0.0;
    }

    let a = T::quantile_from_sorted_data(sorted_data, stride, 1.0 / 3.0);
    let b = T::median_from_sorted_data(sorted_data, stride);
    let c = T::quantile_from_sorted_data(sorted_data, stride, 2.0 / 3.0);
    
    0.3 * a + 0.4 * b + 0.3 * c
}

// Type-specific functions for C compatibility
#[no_mangle]
pub extern "C" fn gsl_stats_uchar_gastwirth_from_sorted_data(
    sorted_data: *const c_uchar,
    stride: size_t,
    n: size_t,
) -> c_double {
    if n == 0 {
        return 0.0;
    }
    let data = unsafe { std::slice::from_raw_parts(sorted_data, n as usize) };
    gastwirth_from_sorted_data(data, stride)
}

// Similar implementations for all other types...
#[no_mangle]
pub extern "C" fn gsl_stats_char_gastwirth_from_sorted_data(
    sorted_data: *const c_char,
    stride: size_t,
    n: size_t,
) -> c_double {
    if n == 0 {
        return 0.0;
    }
    let data = unsafe { std::slice::from_raw_parts(sorted_data, n as usize) };
    gastwirth_from_sorted_data(data, stride)
}

#[no_mangle]
pub extern "C" fn gsl_stats_ulong_gastwirth_from_sorted_data(
    sorted_data: *const c_ulong,
    stride: size_t,
    n: size_t,
) -> c_double {
    if n == 0 {
        return 0.0;
    }
    let data = unsafe { std::slice::from_raw_parts(sorted_data, n as usize) };
    gastwirth_from_sorted_data(data, stride)
}

#[no_mangle]
pub extern "C" fn gsl_stats_long_gastwirth_from_sorted_data(
    sorted_data: *const c_long,
    stride: size_t,
    n: size_t,
) -> c_double {
    if n == 0 {
        return 0.0;
    }
    let data = unsafe { std::slice::from_raw_parts(sorted_data, n as usize) };
    gastwirth_from_sorted_data(data, stride)
}

#[no_mangle]
pub extern "C" fn gsl_stats_uint_gastwirth_from_sorted_data(
    sorted_data: *const c_uint,
    stride: size_t,
    n: size_t,
) -> c_double {
    if n == 0 {
        return 0.0;
    }
    let data = unsafe { std::slice::from_raw_parts(sorted_data, n as usize) };
    gastwirth_from_sorted_data(data, stride)
}

#[no_mangle]
pub extern "C" fn gsl_stats_int_gastwirth_from_sorted_data(
    sorted_data: *const c_int,
    stride: size_t,
    n: size_t,
) -> c_double {
    if n == 0 {
        return 0.0;
    }
    let data = unsafe { std::slice::from_raw_parts(sorted_data, n as usize) };
    gastwirth_from_sorted_data(data, stride)
}

#[no_mangle]
pub extern "C" fn gsl_stats_ushort_gastwirth_from_sorted_data(
    sorted_data: *const c_ushort,
    stride: size_t,
    n: size_t,
) -> c_double {
    if n == 0 {
        return 0.0;
    }
    let data = unsafe { std::slice::from_raw_parts(sorted_data, n as usize) };
    gastwirth_from_sorted_data(data, stride)
}

#[no_mangle]
pub extern "C" fn gsl_stats_short_gastwirth_from_sorted_data(
    sorted_data: *const c_short,
    stride: size_t,
    n: size_t,
) -> c_double {
    if n == 0 {
        return 0.0;
    }
    let data = unsafe { std::slice::from_raw_parts(sorted_data, n as usize) };
    gastwirth_from_sorted_data(data, stride)
}

#[no_mangle]
pub extern "C" fn gsl_stats_float_gastwirth_from_sorted_data(
    sorted_data: *const c_float,
    stride: size_t,
    n: size_t,
) -> c_double {
    if n == 0 {
        return 0.0;
    }
    let data = unsafe { std::slice::from_raw_parts(sorted_data, n as usize) };
    gastwirth_from_sorted_data(data, stride)
}

#[no_mangle]
pub extern "C" fn gsl_stats_gastwirth_from_sorted_data(
    sorted_data: *const c_double,
    stride: size_t,
    n: size_t,
) -> c_double {
    if n == 0 {
        return 0.0;
    }
    let data = unsafe { std::slice::from_raw_parts(sorted_data, n as usize) };
    gastwirth_from_sorted_data(data, stride)
}

#[no_mangle]
pub extern "C" fn gsl_stats_long_double_gastwirth_from_sorted_data(
    sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> c_double {
    if n == 0 {
        return 0.0;
    }
    let data = unsafe { std::slice::from_raw_parts(sorted_data, n as usize) };
    gastwirth_from_sorted_data(data, stride)
}