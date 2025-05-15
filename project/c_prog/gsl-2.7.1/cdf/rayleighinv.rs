use ::libc;
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn log1p(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_rayleigh_Pinv(
    P: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if P == 1.0f64 {
        return ::core::f32::INFINITY as libc::c_double
    } else if P == 0.0f64 {
        return 0.0f64
    }
    x = sigma * 1.41421356237309504880f64 * sqrt(-log1p(-P));
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_rayleigh_Qinv(
    Q: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if Q == 0.0f64 {
        return ::core::f32::INFINITY as libc::c_double
    } else if Q == 1.0f64 {
        return 0.0f64
    }
    x = sigma * 1.41421356237309504880f64 * sqrt(-log(Q));
    return x;
}
