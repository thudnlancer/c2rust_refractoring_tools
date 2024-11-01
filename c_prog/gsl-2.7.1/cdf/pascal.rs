#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn gsl_cdf_negative_binomial_P(
        k: libc::c_uint,
        p: libc::c_double,
        n: libc::c_double,
    ) -> libc::c_double;
    fn gsl_cdf_negative_binomial_Q(
        k: libc::c_uint,
        p: libc::c_double,
        n: libc::c_double,
    ) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_pascal_P(
    k: libc::c_uint,
    p: libc::c_double,
    n: libc::c_uint,
) -> libc::c_double {
    let mut P: libc::c_double = gsl_cdf_negative_binomial_P(k, p, n as libc::c_double);
    return P;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_pascal_Q(
    k: libc::c_uint,
    p: libc::c_double,
    n: libc::c_uint,
) -> libc::c_double {
    let mut Q: libc::c_double = gsl_cdf_negative_binomial_Q(k, p, n as libc::c_double);
    return Q;
}
