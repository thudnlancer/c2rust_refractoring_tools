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
    fn tan(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_cauchy_Pinv(
    P: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if P == 1.0f64 {
        return ::core::f32::INFINITY as libc::c_double
    } else if P == 0.0f64 {
        return -::core::f32::INFINITY as libc::c_double
    }
    if P > 0.5f64 {
        x = a * tan(3.14159265358979323846f64 * (P - 0.5f64));
    } else {
        x = -a / tan(3.14159265358979323846f64 * P);
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_cauchy_Qinv(
    Q: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if Q == 0.0f64 {
        return ::core::f32::INFINITY as libc::c_double
    } else if Q == 1.0f64 {
        return -::core::f32::INFINITY as libc::c_double
    }
    if Q > 0.5f64 {
        x = a * tan(3.14159265358979323846f64 * (0.5f64 - Q));
    } else {
        x = a / tan(3.14159265358979323846f64 * Q);
    }
    return x;
}