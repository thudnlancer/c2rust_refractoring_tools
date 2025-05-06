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
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_ran_hypergeometric_pdf(k: u32, n1: u32, n2: u32, t: u32) -> libc::c_double;
}
pub type C2RustUnnamed = i32;
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
unsafe extern "C" fn lower_tail(k: u32, n1: u32, n2: u32, t: u32) -> libc::c_double {
    let mut relerr: libc::c_double = 0.;
    let mut i: i32 = k as i32;
    let mut s: libc::c_double = 0.;
    let mut P: libc::c_double = 0.;
    s = gsl_ran_hypergeometric_pdf(i as u32, n1, n2, t);
    P = s;
    while i > 0 as i32 {
        let mut factor: libc::c_double = i as libc::c_double
            / (n1.wrapping_sub(i as u32) as libc::c_double + 1.0f64)
            * (n2.wrapping_add(i as u32).wrapping_sub(t) as libc::c_double
                / (t.wrapping_sub(i as u32) as libc::c_double + 1.0f64));
        s *= factor;
        P += s;
        relerr = s / P;
        if relerr < 2.2204460492503131e-16f64 {
            break;
        }
        i -= 1;
        i;
    }
    return P;
}
unsafe extern "C" fn upper_tail(k: u32, n1: u32, n2: u32, t: u32) -> libc::c_double {
    let mut relerr: libc::c_double = 0.;
    let mut i: u32 = k.wrapping_add(1 as i32 as u32);
    let mut s: libc::c_double = 0.;
    let mut Q: libc::c_double = 0.;
    s = gsl_ran_hypergeometric_pdf(i, n1, n2, t);
    Q = s;
    while i < t {
        let mut factor: libc::c_double = n1.wrapping_sub(i) as libc::c_double
            / (i as libc::c_double + 1.0f64)
            * (t.wrapping_sub(i) as libc::c_double
                / (n2.wrapping_add(i) as libc::c_double + 1.0f64 - t as libc::c_double));
        s *= factor;
        Q += s;
        relerr = s / Q;
        if relerr < 2.2204460492503131e-16f64 {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    return Q;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_hypergeometric_P(
    k: u32,
    n1: u32,
    n2: u32,
    t: u32,
) -> libc::c_double {
    let mut P: libc::c_double = 0.;
    if t > n1.wrapping_add(n2) {
        gsl_error(
            b"t larger than population size\0" as *const u8 as *const i8,
            b"hypergeometric.c\0" as *const u8 as *const i8,
            119 as i32,
            GSL_EDOM as i32,
        );
        return ::core::f32::NAN as libc::c_double;
    } else if k >= n1 || k >= t {
        P = 1.0f64;
    } else if (k as libc::c_double) < 0.0f64 {
        P = 0.0f64;
    } else {
        let mut midpoint: libc::c_double = t as libc::c_double * n1 as libc::c_double
            / (n1 as libc::c_double + n2 as libc::c_double);
        if k as libc::c_double >= midpoint {
            P = 1 as i32 as libc::c_double - upper_tail(k, n1, n2, t);
        } else {
            P = lower_tail(k, n1, n2, t);
        }
    }
    return P;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_cdf_hypergeometric_Q(
    k: u32,
    n1: u32,
    n2: u32,
    t: u32,
) -> libc::c_double {
    let mut Q: libc::c_double = 0.;
    if t > n1.wrapping_add(n2) {
        gsl_error(
            b"t larger than population size\0" as *const u8 as *const i8,
            b"hypergeometric.c\0" as *const u8 as *const i8,
            158 as i32,
            GSL_EDOM as i32,
        );
        return ::core::f32::NAN as libc::c_double;
    } else if k >= n1 || k >= t {
        Q = 0.0f64;
    } else if (k as libc::c_double) < 0.0f64 {
        Q = 1.0f64;
    } else {
        let mut midpoint: libc::c_double = t as libc::c_double * n1 as libc::c_double
            / (n1 as libc::c_double + n2 as libc::c_double);
        if (k as libc::c_double) < midpoint {
            Q = 1 as i32 as libc::c_double - lower_tail(k, n1, n2, t);
        } else {
            Q = upper_tail(k, n1, n2, t);
        }
    }
    return Q;
}