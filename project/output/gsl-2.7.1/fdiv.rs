#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn gsl_fdiv(
    x: libc::c_double,
    y: libc::c_double,
) -> libc::c_double {
    return x / y;
}