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
    fn frexp(_: libc::c_double, _: *mut i32) -> libc::c_double;
    fn ldexp(_: libc::c_double, _: i32) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_fcmp(
    x1: libc::c_double,
    x2: libc::c_double,
    epsilon: libc::c_double,
) -> i32 {
    let mut exponent: i32 = 0;
    let mut delta: libc::c_double = 0.;
    let mut difference: libc::c_double = 0.;
    let mut max: libc::c_double = if fabs(x1) > fabs(x2) { x1 } else { x2 };
    frexp(max, &mut exponent);
    delta = ldexp(epsilon, exponent);
    difference = x1 - x2;
    if difference > delta {
        return 1 as i32
    } else if difference < -delta {
        return -(1 as i32)
    } else {
        return 0 as i32
    };
}