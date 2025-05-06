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
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_quantile_from_sorted_data(
    mut sorted_data: *const u8,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f * n.wrapping_sub(1 as i32 as u64) as libc::c_double;
    let lhs: size_t = index as i32 as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as i32 as u64) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as i32 as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as i32
                as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as i32 as libc::c_double;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_quantile_from_sorted_data(
    mut sorted_data: *const u64,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f * n.wrapping_sub(1 as i32 as u64) as libc::c_double;
    let lhs: size_t = index as i32 as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as i32 as u64) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as i32 as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as libc::c_double;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_quantile_from_sorted_data(
    mut sorted_data: *const i8,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f * n.wrapping_sub(1 as i32 as u64) as libc::c_double;
    let lhs: size_t = index as i32 as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as i32 as u64) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as i32 as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as i32
                as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as i32 as libc::c_double;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_quantile_from_sorted_data(
    mut sorted_data: *const i64,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f * n.wrapping_sub(1 as i32 as u64) as libc::c_double;
    let lhs: size_t = index as i32 as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as i32 as u64) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as i32 as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as libc::c_double;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_quantile_from_sorted_data(
    mut sorted_data: *const u32,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f * n.wrapping_sub(1 as i32 as u64) as libc::c_double;
    let lhs: size_t = index as i32 as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as i32 as u64) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as i32 as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as libc::c_double;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_quantile_from_sorted_data(
    mut sorted_data: *const i32,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f * n.wrapping_sub(1 as i32 as u64) as libc::c_double;
    let lhs: size_t = index as i32 as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as i32 as u64) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as i32 as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as libc::c_double;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f * n.wrapping_sub(1 as i32 as u64) as libc::c_double;
    let lhs: size_t = index as i32 as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as i32 as u64) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as i32 as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as i32
                as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as i32 as libc::c_double;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f * n.wrapping_sub(1 as i32 as u64) as libc::c_double;
    let lhs: size_t = index as i32 as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as i32 as u64) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as i32 as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as libc::c_double;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f * n.wrapping_sub(1 as i32 as u64) as libc::c_double;
    let lhs: size_t = index as i32 as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as i32 as u64) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        result = (1 as i32 as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize) as i32
                as libc::c_double
            + delta
                * *sorted_data
                    .offset(
                        lhs.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as i32 as libc::c_double;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_quantile_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f * n.wrapping_sub(1 as i32 as u64) as libc::c_double;
    let lhs: size_t = index as i32 as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as i32 as u64) {
        result = *sorted_data.offset(lhs.wrapping_mul(stride) as isize);
    } else {
        result = (1 as i32 as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            + delta
                * *sorted_data
                    .offset(
                        lhs.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_quantile_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
    f: libc::c_double,
) -> libc::c_double {
    let index: libc::c_double = f * n.wrapping_sub(1 as i32 as u64) as libc::c_double;
    let lhs: size_t = index as i32 as size_t;
    let delta: libc::c_double = index - lhs as libc::c_double;
    let mut result: libc::c_double = 0.;
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == n.wrapping_sub(1 as i32 as u64) {
        result = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize))
            .to_f64()
            .unwrap();
    } else {
        result = (f128::f128::new(1 as i32 as libc::c_double - delta)
            * *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            + f128::f128::new(delta)
                * *sorted_data
                    .offset(
                        lhs.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ))
            .to_f64()
            .unwrap();
    }
    return result;
}