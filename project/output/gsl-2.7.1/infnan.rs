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
    fn __isinfl(__value: f128::f128) -> i32;
    fn __isinff(__value: libc::c_float) -> i32;
    fn __isinf(__value: libc::c_double) -> i32;
    fn __finitel(_: f128::f128) -> i32;
    fn __finitef(_: libc::c_float) -> i32;
    fn __finite(_: libc::c_double) -> i32;
    fn __isnan(__value: libc::c_double) -> i32;
    fn __isnanl(__value: f128::f128) -> i32;
    fn __isnanf(__value: libc::c_float) -> i32;
    fn gsl_fdiv(x: libc::c_double, y: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_nan() -> libc::c_double {
    return gsl_fdiv(0.0f64, 0.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_posinf() -> libc::c_double {
    return gsl_fdiv(1.0f64, 0.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_neginf() -> libc::c_double {
    return gsl_fdiv(-1.0f64, 0.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_finite(x: libc::c_double) -> i32 {
    return if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __finitef(x as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_double>() as u64
    {
        __finite(x)
    } else {
        __finitel(f128::f128::new(x))
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_isnan(x: libc::c_double) -> i32 {
    return if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __isnanf(x as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_double>() as u64
    {
        __isnan(x)
    } else {
        __isnanl(f128::f128::new(x))
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_isinf(x: libc::c_double) -> i32 {
    if if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_float>() as u64
    {
        __isinff(x as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<libc::c_double>() as u64
    {
        __isinf(x)
    } else {
        __isinfl(f128::f128::new(x))
    } != 0
    {
        return if x > 0 as i32 as libc::c_double { 1 as i32 } else { -(1 as i32) }
    } else {
        return 0 as i32
    };
}