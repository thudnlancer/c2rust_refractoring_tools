#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_flat_Pinv(
    P: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if P == 1.0f64 { return b } else if P == 0.0f64 { return a }
    x = (1 as libc::c_int as libc::c_double - P) * a + P * b;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_flat_Qinv(
    Q: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if Q == 0.0f64 { return b } else if Q == 1.0f64 { return a }
    x = Q * a + (1 as libc::c_int as libc::c_double - Q) * b;
    return x;
}
