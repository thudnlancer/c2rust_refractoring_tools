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
    fn gsl_stats_long_double_select(
        data: *mut f128::f128,
        stride: size_t,
        n: size_t,
        k: size_t,
    ) -> f128::f128;
    fn gsl_stats_select(
        data: *mut libc::c_double,
        stride: size_t,
        n: size_t,
        k: size_t,
    ) -> libc::c_double;
    fn gsl_stats_float_select(
        data: *mut libc::c_float,
        stride: size_t,
        n: size_t,
        k: size_t,
    ) -> libc::c_float;
    fn gsl_stats_char_select(data: *mut i8, stride: size_t, n: size_t, k: size_t) -> i8;
    fn gsl_stats_ulong_select(
        data: *mut u64,
        stride: size_t,
        n: size_t,
        k: size_t,
    ) -> u64;
    fn gsl_stats_long_select(
        data: *mut i64,
        stride: size_t,
        n: size_t,
        k: size_t,
    ) -> i64;
    fn gsl_stats_uint_select(
        data: *mut u32,
        stride: size_t,
        n: size_t,
        k: size_t,
    ) -> u32;
    fn gsl_stats_int_select(data: *mut i32, stride: size_t, n: size_t, k: size_t) -> i32;
    fn gsl_stats_ushort_select(
        data: *mut libc::c_ushort,
        stride: size_t,
        n: size_t,
        k: size_t,
    ) -> libc::c_ushort;
    fn gsl_stats_short_select(
        data: *mut libc::c_short,
        stride: size_t,
        n: size_t,
        k: size_t,
    ) -> libc::c_short;
    fn gsl_stats_uchar_select(data: *mut u8, stride: size_t, n: size_t, k: size_t) -> u8;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_median_from_sorted_data(
    mut sorted_data: *const i64,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        median = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            + *sorted_data.offset(rhs.wrapping_mul(stride) as isize)) as libc::c_double
            / 2.0f64;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_median_from_sorted_data(
    mut sorted_data: *const f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize))
            .to_f64()
            .unwrap();
    } else {
        median = ((*sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            + *sorted_data.offset(rhs.wrapping_mul(stride) as isize))
            / f128::f128::new(2.0f64))
            .to_f64()
            .unwrap();
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_median_from_sorted_data(
    mut sorted_data: *const libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        median = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize) as i32
            + *sorted_data.offset(rhs.wrapping_mul(stride) as isize) as i32)
            as libc::c_double / 2.0f64;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_median_from_sorted_data(
    mut sorted_data: *const u8,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        median = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize) as i32
            + *sorted_data.offset(rhs.wrapping_mul(stride) as isize) as i32)
            as libc::c_double / 2.0f64;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_median_from_sorted_data(
    mut sorted_data: *const libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        median = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize) as i32
            + *sorted_data.offset(rhs.wrapping_mul(stride) as isize) as i32)
            as libc::c_double / 2.0f64;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_median_from_sorted_data(
    mut sorted_data: *const u64,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        median = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize))
            .wrapping_add(*sorted_data.offset(rhs.wrapping_mul(stride) as isize))
            as libc::c_double / 2.0f64;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_median_from_sorted_data(
    mut sorted_data: *const i8,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        median = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize) as i32
            + *sorted_data.offset(rhs.wrapping_mul(stride) as isize) as i32)
            as libc::c_double / 2.0f64;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_median_from_sorted_data(
    mut sorted_data: *const libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = *sorted_data.offset(lhs.wrapping_mul(stride) as isize);
    } else {
        median = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            + *sorted_data.offset(rhs.wrapping_mul(stride) as isize)) / 2.0f64;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_median_from_sorted_data(
    mut sorted_data: *const i32,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        median = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            + *sorted_data.offset(rhs.wrapping_mul(stride) as isize)) as libc::c_double
            / 2.0f64;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_median_from_sorted_data(
    mut sorted_data: *const libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        median = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            + *sorted_data.offset(rhs.wrapping_mul(stride) as isize)) as libc::c_double
            / 2.0f64;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_median_from_sorted_data(
    mut sorted_data: *const u32,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = *sorted_data.offset(lhs.wrapping_mul(stride) as isize)
            as libc::c_double;
    } else {
        median = (*sorted_data.offset(lhs.wrapping_mul(stride) as isize))
            .wrapping_add(*sorted_data.offset(rhs.wrapping_mul(stride) as isize))
            as libc::c_double / 2.0f64;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_median(
    mut data: *mut i64,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = gsl_stats_long_select(data, stride, n, lhs) as libc::c_double;
    } else {
        let mut a: i64 = gsl_stats_long_select(data, stride, n, lhs);
        let mut b: i64 = gsl_stats_long_select(data, stride, n, rhs);
        median = 0.5f64 * (a + b) as libc::c_double;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_median(
    mut data: *mut i8,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = gsl_stats_char_select(data, stride, n, lhs) as libc::c_double;
    } else {
        let mut a: i8 = gsl_stats_char_select(data, stride, n, lhs);
        let mut b: i8 = gsl_stats_char_select(data, stride, n, rhs);
        median = 0.5f64 * (a as i32 + b as i32) as libc::c_double;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_median(
    mut data: *mut libc::c_short,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = gsl_stats_short_select(data, stride, n, lhs) as libc::c_double;
    } else {
        let mut a: libc::c_short = gsl_stats_short_select(data, stride, n, lhs);
        let mut b: libc::c_short = gsl_stats_short_select(data, stride, n, rhs);
        median = 0.5f64 * (a as i32 + b as i32) as libc::c_double;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_median(
    mut data: *mut u32,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = gsl_stats_uint_select(data, stride, n, lhs) as libc::c_double;
    } else {
        let mut a: u32 = gsl_stats_uint_select(data, stride, n, lhs);
        let mut b: u32 = gsl_stats_uint_select(data, stride, n, rhs);
        median = 0.5f64 * a.wrapping_add(b) as libc::c_double;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_median(
    mut data: *mut u8,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = gsl_stats_uchar_select(data, stride, n, lhs) as libc::c_double;
    } else {
        let mut a: u8 = gsl_stats_uchar_select(data, stride, n, lhs);
        let mut b: u8 = gsl_stats_uchar_select(data, stride, n, rhs);
        median = 0.5f64 * (a as i32 + b as i32) as libc::c_double;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_median(
    mut data: *mut i32,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = gsl_stats_int_select(data, stride, n, lhs) as libc::c_double;
    } else {
        let mut a: i32 = gsl_stats_int_select(data, stride, n, lhs);
        let mut b: i32 = gsl_stats_int_select(data, stride, n, rhs);
        median = 0.5f64 * (a + b) as libc::c_double;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_median(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = gsl_stats_select(data, stride, n, lhs);
    } else {
        let mut a: libc::c_double = gsl_stats_select(data, stride, n, lhs);
        let mut b: libc::c_double = gsl_stats_select(data, stride, n, rhs);
        median = 0.5f64 * (a + b);
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_median(
    mut data: *mut libc::c_ushort,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = gsl_stats_ushort_select(data, stride, n, lhs) as libc::c_double;
    } else {
        let mut a: libc::c_ushort = gsl_stats_ushort_select(data, stride, n, lhs);
        let mut b: libc::c_ushort = gsl_stats_ushort_select(data, stride, n, rhs);
        median = 0.5f64 * (a as i32 + b as i32) as libc::c_double;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_median(
    mut data: *mut f128::f128,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = (gsl_stats_long_double_select(data, stride, n, lhs)).to_f64().unwrap();
    } else {
        let mut a: f128::f128 = gsl_stats_long_double_select(data, stride, n, lhs);
        let mut b: f128::f128 = gsl_stats_long_double_select(data, stride, n, rhs);
        median = (f128::f128::new(0.5f64) * (a + b)).to_f64().unwrap();
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_median(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = gsl_stats_float_select(data, stride, n, lhs) as libc::c_double;
    } else {
        let mut a: libc::c_float = gsl_stats_float_select(data, stride, n, lhs);
        let mut b: libc::c_float = gsl_stats_float_select(data, stride, n, rhs);
        median = 0.5f64 * (a + b) as libc::c_double;
    }
    return median;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_median(
    mut data: *mut u64,
    stride: size_t,
    n: size_t,
) -> libc::c_double {
    let mut median: libc::c_double = 0.;
    let lhs: size_t = n.wrapping_sub(1 as i32 as u64).wrapping_div(2 as i32 as u64);
    let rhs: size_t = n.wrapping_div(2 as i32 as u64);
    if n == 0 as i32 as u64 {
        return 0.0f64;
    }
    if lhs == rhs {
        median = gsl_stats_ulong_select(data, stride, n, lhs) as libc::c_double;
    } else {
        let mut a: u64 = gsl_stats_ulong_select(data, stride, n, lhs);
        let mut b: u64 = gsl_stats_ulong_select(data, stride, n, rhs);
        median = 0.5f64 * a.wrapping_add(b) as libc::c_double;
    }
    return median;
}