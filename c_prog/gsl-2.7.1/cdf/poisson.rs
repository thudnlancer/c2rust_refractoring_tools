#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_cdf_gamma_P(
        x: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
    fn gsl_cdf_gamma_Q(
        x: libc::c_double,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_poisson_P(
    k: libc::c_uint,
    mu: libc::c_double,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    if mu <= 0.0f64 {
        gsl_error(
            b"mu <= 0\0" as *const u8 as *const libc::c_char,
            b"poisson.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::core::f32::NAN as libc::c_double;
    }
    a = k as libc::c_double + 1.0f64;
    P = gsl_cdf_gamma_Q(mu, a, 1.0f64);
    return P;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_poisson_Q(
    k: libc::c_uint,
    mu: libc::c_double,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    if mu <= 0.0f64 {
        gsl_error(
            b"mu <= 0\0" as *const u8 as *const libc::c_char,
            b"poisson.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return ::core::f32::NAN as libc::c_double;
    }
    a = k as libc::c_double + 1.0f64;
    Q = gsl_cdf_gamma_P(mu, a, 1.0f64);
    return Q;
}
