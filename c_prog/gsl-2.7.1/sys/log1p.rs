#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_log1p(x: libc::c_double) -> libc::c_double {
    let mut y: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    ::core::ptr::write_volatile(
        &mut y as *mut libc::c_double,
        1 as libc::c_int as libc::c_double + x,
    );
    ::core::ptr::write_volatile(
        &mut z as *mut libc::c_double,
        y - 1 as libc::c_int as libc::c_double,
    );
    return log(y) - (z - x) / y;
}
