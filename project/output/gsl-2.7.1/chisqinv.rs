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
    fn gsl_cdf_gamma_Pinv(
        P: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_cdf_gamma_Qinv(
        Q: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_chisq_Pinv(
    P: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    return gsl_cdf_gamma_Pinv(P, nu / 2 as i32 as libc::c_double, 2.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_chisq_Qinv(
    Q: libc::c_double,
    nu: libc::c_double,
) -> libc::c_double {
    return gsl_cdf_gamma_Qinv(Q, nu / 2 as i32 as libc::c_double, 2.0f64);
}