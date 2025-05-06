#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn gsl_stats_float_variance(
        data: *const libc::c_float,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_double_variance(
        data: *const f128::f128,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_variance(
        data: *const libc::c_double,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ulong_variance(
        data: *const u64,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_long_variance(
        data: *const i64,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uint_variance(
        data: *const u32,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_int_variance(
        data: *const i32,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_ushort_variance(
        data: *const libc::c_ushort,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_short_variance(
        data: *const libc::c_short,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_uchar_variance(
        data: *const u8,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
    fn gsl_stats_char_variance(
        data: *const i8,
        stride: size_t,
        n: size_t,
    ) -> libc::c_double;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_pvariance(
    mut data1: *const i8,
    stride1: size_t,
    n1: size_t,
    mut data2: *const i8,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_char_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_char_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1.wrapping_sub(1 as i32 as u64)
        as libc::c_double * var1
        + n2.wrapping_sub(1 as i32 as u64) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as i32 as u64) as libc::c_double;
    return pooled_variance;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_pvariance(
    mut data1: *const i64,
    stride1: size_t,
    n1: size_t,
    mut data2: *const i64,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_long_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_long_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1.wrapping_sub(1 as i32 as u64)
        as libc::c_double * var1
        + n2.wrapping_sub(1 as i32 as u64) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as i32 as u64) as libc::c_double;
    return pooled_variance;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_pvariance(
    mut data1: *const u32,
    stride1: size_t,
    n1: size_t,
    mut data2: *const u32,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_uint_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_uint_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1.wrapping_sub(1 as i32 as u64)
        as libc::c_double * var1
        + n2.wrapping_sub(1 as i32 as u64) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as i32 as u64) as libc::c_double;
    return pooled_variance;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_pvariance(
    mut data1: *const i32,
    stride1: size_t,
    n1: size_t,
    mut data2: *const i32,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_int_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_int_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1.wrapping_sub(1 as i32 as u64)
        as libc::c_double * var1
        + n2.wrapping_sub(1 as i32 as u64) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as i32 as u64) as libc::c_double;
    return pooled_variance;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_pvariance(
    mut data1: *const libc::c_ushort,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_ushort,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_ushort_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_ushort_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1.wrapping_sub(1 as i32 as u64)
        as libc::c_double * var1
        + n2.wrapping_sub(1 as i32 as u64) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as i32 as u64) as libc::c_double;
    return pooled_variance;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_pvariance(
    mut data1: *const libc::c_short,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_short,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_short_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_short_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1.wrapping_sub(1 as i32 as u64)
        as libc::c_double * var1
        + n2.wrapping_sub(1 as i32 as u64) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as i32 as u64) as libc::c_double;
    return pooled_variance;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_pvariance(
    mut data1: *const u64,
    stride1: size_t,
    n1: size_t,
    mut data2: *const u64,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_ulong_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_ulong_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1.wrapping_sub(1 as i32 as u64)
        as libc::c_double * var1
        + n2.wrapping_sub(1 as i32 as u64) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as i32 as u64) as libc::c_double;
    return pooled_variance;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_pvariance(
    mut data1: *const libc::c_float,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_float,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_float_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_float_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1.wrapping_sub(1 as i32 as u64)
        as libc::c_double * var1
        + n2.wrapping_sub(1 as i32 as u64) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as i32 as u64) as libc::c_double;
    return pooled_variance;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_pvariance(
    mut data1: *const u8,
    stride1: size_t,
    n1: size_t,
    mut data2: *const u8,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_uchar_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_uchar_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1.wrapping_sub(1 as i32 as u64)
        as libc::c_double * var1
        + n2.wrapping_sub(1 as i32 as u64) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as i32 as u64) as libc::c_double;
    return pooled_variance;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_pvariance(
    mut data1: *const libc::c_double,
    stride1: size_t,
    n1: size_t,
    mut data2: *const libc::c_double,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1.wrapping_sub(1 as i32 as u64)
        as libc::c_double * var1
        + n2.wrapping_sub(1 as i32 as u64) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as i32 as u64) as libc::c_double;
    return pooled_variance;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_pvariance(
    mut data1: *const f128::f128,
    stride1: size_t,
    n1: size_t,
    mut data2: *const f128::f128,
    stride2: size_t,
    n2: size_t,
) -> libc::c_double {
    let var1: libc::c_double = gsl_stats_long_double_variance(data1, stride1, n1);
    let var2: libc::c_double = gsl_stats_long_double_variance(data2, stride2, n2);
    let pooled_variance: libc::c_double = (n1.wrapping_sub(1 as i32 as u64)
        as libc::c_double * var1
        + n2.wrapping_sub(1 as i32 as u64) as libc::c_double * var2)
        / n1.wrapping_add(n2).wrapping_sub(2 as i32 as u64) as libc::c_double;
    return pooled_variance;
}