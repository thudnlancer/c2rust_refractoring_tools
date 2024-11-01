#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn GSL_MAX_INT(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn GSL_MIN_INT(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn GSL_MAX_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn GSL_MIN_DBL(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn GSL_MAX_LDBL(
    mut a: f128::f128,
    mut b: f128::f128,
) -> f128::f128 {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn GSL_MIN_LDBL(
    mut a: f128::f128,
    mut b: f128::f128,
) -> f128::f128 {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_max(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_min(
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    return if a < b { a } else { b };
}
