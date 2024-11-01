#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn expm1(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_rayleigh_P(
    x: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = x / sigma;
    let mut P: libc::c_double = -expm1(-u * u / 2 as libc::c_int as libc::c_double);
    return P;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_rayleigh_Q(
    x: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = x / sigma;
    let mut Q: libc::c_double = exp(-u * u / 2 as libc::c_int as libc::c_double);
    return Q;
}
