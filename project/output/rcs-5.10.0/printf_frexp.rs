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
}
#[no_mangle]
pub unsafe extern "C" fn printf_frexp(
    mut x: libc::c_double,
    mut expptr: *mut i32,
) -> libc::c_double {
    let mut exponent: i32 = 0;
    x = frexp(x, &mut exponent);
    x = x + x;
    exponent -= 1 as i32;
    if exponent < -(1021 as i32) - 1 as i32 {
        x = ldexp(x, exponent - (-(1021 as i32) - 1 as i32));
        exponent = -(1021 as i32) - 1 as i32;
    }
    *expptr = exponent;
    return x;
}