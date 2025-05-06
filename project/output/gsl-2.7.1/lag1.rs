#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use num_traits::ToPrimitive;
extern "C" {
    fn gsl_stats_float_mean(
        data: *const libc::c_float,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_double_mean(
        data: *const f128::f128,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_mean(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ulong_mean(
        data: *const u64,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_mean(
        data: *const i64,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uint_mean(
        data: *const u32,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_int_mean(data: *const i32, stride: size_t, n: size_t) -> libc::c_double;
    fn gsl_stats_ushort_mean(
        data: *const libc::c_ushort,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_short_mean(
        data: *const libc::c_short,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uchar_mean(
        data: *const u8,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_char_mean(data: *const i8, stride: size_t, n: size_t) -> libc::c_double;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_lag1_autocorrelation(
    mut data: *const u32,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uint_mean(data, stride, n);
    return gsl_stats_uint_lag1_autocorrelation_m(data, stride, n, mean);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_lag1_autocorrelation(
    mut data: *const u64,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ulong_mean(data, stride, n);
    return gsl_stats_ulong_lag1_autocorrelation_m(data, stride, n, mean);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_lag1_autocorrelation(
    mut data: *const i8,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_char_mean(data, stride, n);
    return gsl_stats_char_lag1_autocorrelation_m(data, stride, n, mean);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_lag1_autocorrelation(
    mut data: *const i32,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_int_mean(data, stride, n);
    return gsl_stats_int_lag1_autocorrelation_m(data, stride, n, mean);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_lag1_autocorrelation(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_double_mean(data, stride, n);
    return gsl_stats_long_double_lag1_autocorrelation_m(data, stride, n, mean);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_lag1_autocorrelation(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_mean(data, stride, n);
    return gsl_stats_lag1_autocorrelation_m(data, stride, n, mean);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_lag1_autocorrelation(
    mut data: *const u8,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uchar_mean(data, stride, n);
    return gsl_stats_uchar_lag1_autocorrelation_m(data, stride, n, mean);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_lag1_autocorrelation(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_float_mean(data, stride, n);
    return gsl_stats_float_lag1_autocorrelation_m(data, stride, n, mean);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_lag1_autocorrelation(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ushort_mean(data, stride, n);
    return gsl_stats_ushort_lag1_autocorrelation_m(data, stride, n, mean);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_lag1_autocorrelation(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_short_mean(data, stride, n);
    return gsl_stats_short_lag1_autocorrelation_m(data, stride, n, mean);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_lag1_autocorrelation(
    mut data: *const i64,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_mean(data, stride, n);
    return gsl_stats_long_lag1_autocorrelation_m(data, stride, n, mean);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_lag1_autocorrelation_m(
    mut data: *const libc::c_float,
    stride: size_t,
    size: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r1: f128::f128 = f128::f128::ZERO;
    let mut q: f128::f128 = f128::f128::new(0 as i32);
    let mut v: f128::f128 = f128::f128::new(
        (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as libc::c_double
            - mean)
            * (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize)
                as libc::c_double - mean),
    );
    i = 1 as i32 as size_t;
    while i < size {
        let delta0: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
                as libc::c_double - mean,
        );
        let delta1: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        q += (delta0 * delta1 - q) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        v += (delta1 * delta1 - v) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    r1 = q / v;
    return r1.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_lag1_autocorrelation_m(
    mut data: *const u64,
    stride: size_t,
    size: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r1: f128::f128 = f128::f128::ZERO;
    let mut q: f128::f128 = f128::f128::new(0 as i32);
    let mut v: f128::f128 = f128::f128::new(
        (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as libc::c_double
            - mean)
            * (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize)
                as libc::c_double - mean),
    );
    i = 1 as i32 as size_t;
    while i < size {
        let delta0: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
                as libc::c_double - mean,
        );
        let delta1: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        q += (delta0 * delta1 - q) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        v += (delta1 * delta1 - v) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    r1 = q / v;
    return r1.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_lag1_autocorrelation_m(
    mut data: *const i32,
    stride: size_t,
    size: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r1: f128::f128 = f128::f128::ZERO;
    let mut q: f128::f128 = f128::f128::new(0 as i32);
    let mut v: f128::f128 = f128::f128::new(
        (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as libc::c_double
            - mean)
            * (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize)
                as libc::c_double - mean),
    );
    i = 1 as i32 as size_t;
    while i < size {
        let delta0: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
                as libc::c_double - mean,
        );
        let delta1: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        q += (delta0 * delta1 - q) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        v += (delta1 * delta1 - v) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    r1 = q / v;
    return r1.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_lag1_autocorrelation_m(
    mut data: *const u32,
    stride: size_t,
    size: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r1: f128::f128 = f128::f128::ZERO;
    let mut q: f128::f128 = f128::f128::new(0 as i32);
    let mut v: f128::f128 = f128::f128::new(
        (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as libc::c_double
            - mean)
            * (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize)
                as libc::c_double - mean),
    );
    i = 1 as i32 as size_t;
    while i < size {
        let delta0: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
                as libc::c_double - mean,
        );
        let delta1: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        q += (delta0 * delta1 - q) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        v += (delta1 * delta1 - v) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    r1 = q / v;
    return r1.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_lag1_autocorrelation_m(
    mut data: *const libc::c_ushort,
    stride: size_t,
    size: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r1: f128::f128 = f128::f128::ZERO;
    let mut q: f128::f128 = f128::f128::new(0 as i32);
    let mut v: f128::f128 = f128::f128::new(
        (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as i32
            as libc::c_double - mean)
            * (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as i32
                as libc::c_double - mean),
    );
    i = 1 as i32 as size_t;
    while i < size {
        let delta0: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
                as i32 as libc::c_double - mean,
        );
        let delta1: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as i32 as libc::c_double - mean,
        );
        q += (delta0 * delta1 - q) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        v += (delta1 * delta1 - v) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    r1 = q / v;
    return r1.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_lag1_autocorrelation_m(
    mut data: *const f128::f128,
    stride: size_t,
    size: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r1: f128::f128 = f128::f128::ZERO;
    let mut q: f128::f128 = f128::f128::new(0 as i32);
    let mut v: f128::f128 = (*data
        .offset((0 as i32 as u64).wrapping_mul(stride) as isize) - f128::f128::new(mean))
        * (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize)
            - f128::f128::new(mean));
    i = 1 as i32 as size_t;
    while i < size {
        let delta0: f128::f128 = *data
            .offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
            - f128::f128::new(mean);
        let delta1: f128::f128 = *data.offset(i.wrapping_mul(stride) as isize)
            - f128::f128::new(mean);
        q += (delta0 * delta1 - q) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        v += (delta1 * delta1 - v) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    r1 = q / v;
    return r1.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_lag1_autocorrelation_m(
    mut data: *const libc::c_short,
    stride: size_t,
    size: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r1: f128::f128 = f128::f128::ZERO;
    let mut q: f128::f128 = f128::f128::new(0 as i32);
    let mut v: f128::f128 = f128::f128::new(
        (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as i32
            as libc::c_double - mean)
            * (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as i32
                as libc::c_double - mean),
    );
    i = 1 as i32 as size_t;
    while i < size {
        let delta0: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
                as i32 as libc::c_double - mean,
        );
        let delta1: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as i32 as libc::c_double - mean,
        );
        q += (delta0 * delta1 - q) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        v += (delta1 * delta1 - v) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    r1 = q / v;
    return r1.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_lag1_autocorrelation_m(
    mut data: *const libc::c_double,
    stride: size_t,
    size: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r1: f128::f128 = f128::f128::ZERO;
    let mut q: f128::f128 = f128::f128::new(0 as i32);
    let mut v: f128::f128 = f128::f128::new(
        (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) - mean)
            * (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) - mean),
    );
    i = 1 as i32 as size_t;
    while i < size {
        let delta0: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
                - mean,
        );
        let delta1: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) - mean,
        );
        q += (delta0 * delta1 - q) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        v += (delta1 * delta1 - v) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    r1 = q / v;
    return r1.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_lag1_autocorrelation_m(
    mut data: *const u8,
    stride: size_t,
    size: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r1: f128::f128 = f128::f128::ZERO;
    let mut q: f128::f128 = f128::f128::new(0 as i32);
    let mut v: f128::f128 = f128::f128::new(
        (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as i32
            as libc::c_double - mean)
            * (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as i32
                as libc::c_double - mean),
    );
    i = 1 as i32 as size_t;
    while i < size {
        let delta0: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
                as i32 as libc::c_double - mean,
        );
        let delta1: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as i32 as libc::c_double - mean,
        );
        q += (delta0 * delta1 - q) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        v += (delta1 * delta1 - v) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    r1 = q / v;
    return r1.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_lag1_autocorrelation_m(
    mut data: *const i64,
    stride: size_t,
    size: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r1: f128::f128 = f128::f128::ZERO;
    let mut q: f128::f128 = f128::f128::new(0 as i32);
    let mut v: f128::f128 = f128::f128::new(
        (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as libc::c_double
            - mean)
            * (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize)
                as libc::c_double - mean),
    );
    i = 1 as i32 as size_t;
    while i < size {
        let delta0: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
                as libc::c_double - mean,
        );
        let delta1: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean,
        );
        q += (delta0 * delta1 - q) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        v += (delta1 * delta1 - v) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    r1 = q / v;
    return r1.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_lag1_autocorrelation_m(
    mut data: *const i8,
    stride: size_t,
    size: size_t,
    mean: libc::c_double,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut r1: f128::f128 = f128::f128::ZERO;
    let mut q: f128::f128 = f128::f128::new(0 as i32);
    let mut v: f128::f128 = f128::f128::new(
        (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as i32
            as libc::c_double - mean)
            * (*data.offset((0 as i32 as u64).wrapping_mul(stride) as isize) as i32
                as libc::c_double - mean),
    );
    i = 1 as i32 as size_t;
    while i < size {
        let delta0: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_sub(1 as i32 as u64).wrapping_mul(stride) as isize)
                as i32 as libc::c_double - mean,
        );
        let delta1: f128::f128 = f128::f128::new(
            *data.offset(i.wrapping_mul(stride) as isize) as i32 as libc::c_double - mean,
        );
        q += (delta0 * delta1 - q) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        v += (delta1 * delta1 - v) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    r1 = q / v;
    return r1.to_f64().unwrap();
}