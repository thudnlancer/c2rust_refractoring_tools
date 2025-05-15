use ::libc;
use ::f128;
extern "C" {
    fn __isinfl(__value: f128::f128) -> libc::c_int;
    fn __isinff(__value: libc::c_float) -> libc::c_int;
    fn __isinf(__value: libc::c_double) -> libc::c_int;
    fn __finitel(_: f128::f128) -> libc::c_int;
    fn __finitef(_: libc::c_float) -> libc::c_int;
    fn __finite(_: libc::c_double) -> libc::c_int;
    fn __isnan(__value: libc::c_double) -> libc::c_int;
    fn __isnanl(__value: f128::f128) -> libc::c_int;
    fn __isnanf(__value: libc::c_float) -> libc::c_int;
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
pub unsafe extern "C" fn gsl_finite(x: libc::c_double) -> libc::c_int {
    return if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
    {
        __finitef(x as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
    {
        __finite(x)
    } else {
        __finitel(f128::f128::new(x))
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_isnan(x: libc::c_double) -> libc::c_int {
    return if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
    {
        __isnanf(x as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
    {
        __isnan(x)
    } else {
        __isnanl(f128::f128::new(x))
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_isinf(x: libc::c_double) -> libc::c_int {
    if if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
    {
        __isinff(x as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
    {
        __isinf(x)
    } else {
        __isinfl(f128::f128::new(x))
    } != 0
    {
        return if x > 0 as libc::c_int as libc::c_double {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }
    } else {
        return 0 as libc::c_int
    };
}
