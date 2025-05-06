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
    fn log(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_logistic_Pinv(
    P: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if P == 1.0f64 {
        return ::core::f32::INFINITY as libc::c_double
    } else if P == 0.0f64 {
        return -::core::f32::INFINITY as libc::c_double
    }
    x = a * log(P / (1 as i32 as libc::c_double - P));
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_logistic_Qinv(
    Q: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if Q == 0.0f64 {
        return ::core::f32::INFINITY as libc::c_double
    } else if Q == 1.0f64 {
        return -::core::f32::INFINITY as libc::c_double
    }
    x = a * log((1 as i32 as libc::c_double - Q) / Q);
    return x;
}