#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn log(_: libc::c_double) -> libc::c_double;
    fn gsl_cdf_ugaussian_P(x: libc::c_double) -> libc::c_double;
    fn gsl_cdf_ugaussian_Q(x: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_lognormal_P(
    x: libc::c_double,
    zeta: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = (log(x) - zeta) / sigma;
    let mut P: libc::c_double = gsl_cdf_ugaussian_P(u);
    return P;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_lognormal_Q(
    x: libc::c_double,
    zeta: libc::c_double,
    sigma: libc::c_double,
) -> libc::c_double {
    let mut u: libc::c_double = (log(x) - zeta) / sigma;
    let mut Q: libc::c_double = gsl_cdf_ugaussian_Q(u);
    return Q;
}
