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
    fn gsl_stats_long_double_mean(
        data: *const f128::f128,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_double_sd_m(
        data: *const f128::f128,
        stride: size_t,
        n: size_t,
        mean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_mean(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_sd_m(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
        mean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_float_mean(
        data: *const libc::c_float,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_float_sd_m(
        data: *const libc::c_float,
        stride: size_t,
        n: size_t,
        mean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_ulong_mean(
        data: *const u64,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ulong_sd_m(
        data: *const u64,
        stride: size_t,
        n: size_t,
        mean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_long_mean(
        data: *const i64,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_sd_m(
        data: *const i64,
        stride: size_t,
        n: size_t,
        mean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_uint_mean(
        data: *const u32,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uint_sd_m(
        data: *const u32,
        stride: size_t,
        n: size_t,
        mean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_int_mean(data: *const i32, stride: size_t, n: size_t) -> libc::c_double;
    fn gsl_stats_int_sd_m(
        data: *const i32,
        stride: size_t,
        n: size_t,
        mean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_ushort_mean(
        data: *const libc::c_ushort,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ushort_sd_m(
        data: *const libc::c_ushort,
        stride: size_t,
        n: size_t,
        mean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_short_mean(
        data: *const libc::c_short,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_short_sd_m(
        data: *const libc::c_short,
        stride: size_t,
        n: size_t,
        mean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_uchar_mean(
        data: *const u8,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uchar_sd_m(
        data: *const u8,
        stride: size_t,
        n: size_t,
        mean: libc::c_double,
    ) -> libc::c_double;
    fn gsl_stats_char_mean(data: *const i8, stride: size_t, n: size_t) -> libc::c_double;
    fn gsl_stats_char_sd_m(
        data: *const i8,
        stride: size_t,
        n: size_t,
        mean: libc::c_double,
    ) -> libc::c_double;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_skew(
    mut data: *const i8,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_char_mean(data, stride, n);
    let sd: libc::c_double = gsl_stats_char_sd_m(data, stride, n, mean);
    return gsl_stats_char_skew_m_sd(data, stride, n, mean, sd);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_skew(
    mut data: *const u64,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ulong_mean(data, stride, n);
    let sd: libc::c_double = gsl_stats_ulong_sd_m(data, stride, n, mean);
    return gsl_stats_ulong_skew_m_sd(data, stride, n, mean, sd);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_skew(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_double_mean(data, stride, n);
    let sd: libc::c_double = gsl_stats_long_double_sd_m(data, stride, n, mean);
    return gsl_stats_long_double_skew_m_sd(data, stride, n, mean, sd);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_skew(
    mut data: *const i64,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_long_mean(data, stride, n);
    let sd: libc::c_double = gsl_stats_long_sd_m(data, stride, n, mean);
    return gsl_stats_long_skew_m_sd(data, stride, n, mean, sd);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_skew(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_mean(data, stride, n);
    let sd: libc::c_double = gsl_stats_sd_m(data, stride, n, mean);
    return gsl_stats_skew_m_sd(data, stride, n, mean, sd);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_skew(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_short_mean(data, stride, n);
    let sd: libc::c_double = gsl_stats_short_sd_m(data, stride, n, mean);
    return gsl_stats_short_skew_m_sd(data, stride, n, mean, sd);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_skew(
    mut data: *const u32,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uint_mean(data, stride, n);
    let sd: libc::c_double = gsl_stats_uint_sd_m(data, stride, n, mean);
    return gsl_stats_uint_skew_m_sd(data, stride, n, mean, sd);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_skew(
    mut data: *const i32,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_int_mean(data, stride, n);
    let sd: libc::c_double = gsl_stats_int_sd_m(data, stride, n, mean);
    return gsl_stats_int_skew_m_sd(data, stride, n, mean, sd);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_skew(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_float_mean(data, stride, n);
    let sd: libc::c_double = gsl_stats_float_sd_m(data, stride, n, mean);
    return gsl_stats_float_skew_m_sd(data, stride, n, mean, sd);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_skew(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_ushort_mean(data, stride, n);
    let sd: libc::c_double = gsl_stats_ushort_sd_m(data, stride, n, mean);
    return gsl_stats_ushort_skew_m_sd(data, stride, n, mean, sd);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_skew(
    mut data: *const u8,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mean: libc::c_double = gsl_stats_uchar_mean(data, stride, n);
    let sd: libc::c_double = gsl_stats_uchar_sd_m(data, stride, n, mean);
    return gsl_stats_uchar_skew_m_sd(data, stride, n, mean, sd);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_skew_m_sd(
    mut data: *const i8,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
    sd: libc::c_double,
) -> libc::c_double {
    let mut skew: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        let x: f128::f128 = f128::f128::new(
            (*data.offset(i.wrapping_mul(stride) as isize) as i32 as libc::c_double
                - mean) / sd,
        );
        skew += (x * x * x - skew) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return skew.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_skew_m_sd(
    mut data: *const f128::f128,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
    sd: libc::c_double,
) -> libc::c_double {
    let mut skew: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        let x: f128::f128 = (*data.offset(i.wrapping_mul(stride) as isize)
            - f128::f128::new(mean)) / f128::f128::new(sd);
        skew += (x * x * x - skew) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return skew.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_skew_m_sd(
    mut data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
    sd: libc::c_double,
) -> libc::c_double {
    let mut skew: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        let x: f128::f128 = f128::f128::new(
            (*data.offset(i.wrapping_mul(stride) as isize) as i32 as libc::c_double
                - mean) / sd,
        );
        skew += (x * x * x - skew) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return skew.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_skew_m_sd(
    mut data: *const u8,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
    sd: libc::c_double,
) -> libc::c_double {
    let mut skew: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        let x: f128::f128 = f128::f128::new(
            (*data.offset(i.wrapping_mul(stride) as isize) as i32 as libc::c_double
                - mean) / sd,
        );
        skew += (x * x * x - skew) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return skew.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_skew_m_sd(
    mut data: *const u32,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
    sd: libc::c_double,
) -> libc::c_double {
    let mut skew: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        let x: f128::f128 = f128::f128::new(
            (*data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean) / sd,
        );
        skew += (x * x * x - skew) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return skew.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_skew_m_sd(
    mut data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
    sd: libc::c_double,
) -> libc::c_double {
    let mut skew: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        let x: f128::f128 = f128::f128::new(
            (*data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean) / sd,
        );
        skew += (x * x * x - skew) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return skew.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_skew_m_sd(
    mut data: *const i64,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
    sd: libc::c_double,
) -> libc::c_double {
    let mut skew: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        let x: f128::f128 = f128::f128::new(
            (*data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean) / sd,
        );
        skew += (x * x * x - skew) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return skew.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_skew_m_sd(
    mut data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
    sd: libc::c_double,
) -> libc::c_double {
    let mut skew: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        let x: f128::f128 = f128::f128::new(
            (*data.offset(i.wrapping_mul(stride) as isize) - mean) / sd,
        );
        skew += (x * x * x - skew) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return skew.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_skew_m_sd(
    mut data: *const i32,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
    sd: libc::c_double,
) -> libc::c_double {
    let mut skew: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        let x: f128::f128 = f128::f128::new(
            (*data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean) / sd,
        );
        skew += (x * x * x - skew) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return skew.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_skew_m_sd(
    mut data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
    sd: libc::c_double,
) -> libc::c_double {
    let mut skew: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        let x: f128::f128 = f128::f128::new(
            (*data.offset(i.wrapping_mul(stride) as isize) as i32 as libc::c_double
                - mean) / sd,
        );
        skew += (x * x * x - skew) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return skew.to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_skew_m_sd(
    mut data: *const u64,
    stride: size_t,
    n: size_t,
    mean: libc::c_double,
    sd: libc::c_double,
) -> libc::c_double {
    let mut skew: f128::f128 = f128::f128::new(0 as i32);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        let x: f128::f128 = f128::f128::new(
            (*data.offset(i.wrapping_mul(stride) as isize) as libc::c_double - mean) / sd,
        );
        skew += (x * x * x - skew) / f128::f128::new(i.wrapping_add(1 as i32 as u64));
        i = i.wrapping_add(1);
        i;
    }
    return skew.to_f64().unwrap();
}