#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn exp(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_logistic_P(
    x: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    let mut u: libc::c_double = x / a;
    if u >= 0 as libc::c_int as libc::c_double {
        P = 1 as libc::c_int as libc::c_double
            / (1 as libc::c_int as libc::c_double + exp(-u));
    } else {
        P = exp(u) / (1 as libc::c_int as libc::c_double + exp(u));
    }
    return P;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_logistic_Q(
    x: libc::c_double,
    a: libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    let mut u: libc::c_double = x / a;
    if u >= 0 as libc::c_int as libc::c_double {
        Q = exp(-u) / (1 as libc::c_int as libc::c_double + exp(-u));
    } else {
        Q = 1 as libc::c_int as libc::c_double
            / (1 as libc::c_int as libc::c_double + exp(u));
    }
    return Q;
}
