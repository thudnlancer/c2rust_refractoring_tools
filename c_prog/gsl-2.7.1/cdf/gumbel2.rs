#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
    fn expm1(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_gumbel2_P(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    if x == 0 as libc::c_int as libc::c_double {
        P = 0 as libc::c_int as libc::c_double;
    } else {
        let mut u: libc::c_double = pow(x, a);
        P = exp(-b / u);
    }
    return P;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_gumbel2_Q(
    x: libc::c_double,
    a: libc::c_double,
    b: libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    if x == 0 as libc::c_int as libc::c_double {
        Q = 1 as libc::c_int as libc::c_double;
    } else {
        let mut u: libc::c_double = pow(x, a);
        Q = -expm1(-b / u);
    }
    return Q;
}
