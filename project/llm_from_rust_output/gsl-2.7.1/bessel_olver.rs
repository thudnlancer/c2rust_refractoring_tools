use libc::{c_double, c_int, c_uint, c_char};
use std::f64::consts::PI;

const GSL_SUCCESS: c_int = 0;
const GSL_EDOM: c_int = 1;
const GSL_ERANGE: c_int = 2;
const GSL_EFAULT: c_int = 3;
const GSL_EINVAL: c_int = 4;
const GSL_EFAILED: c_int = 5;
const GSL_EFACTOR: c_int = 6;
const GSL_ESANITY: c_int = 7;
const GSL_ENOMEM: c_int = 8;
const GSL_EBADFUNC: c_int = 9;
const GSL_ERUNAWAY: c_int = 10;
const GSL_EMAXITER: c_int = 11;
const GSL_EZERODIV: c_int = 12;
const GSL_EBADTOL: c_int = 13;
const GSL_ETOL: c_int = 14;
const GSL_EUNDRFLW: c_int = 15;
const GSL_EOVRFLW: c_int = 16;
const GSL_ELOSS: c_int = 17;
const GSL_EROUND: c_int = 18;
const GSL_EBADLEN: c_int = 19;
const GSL_ENOTSQR: c_int = 20;
const GSL_ESING: c_int = 21;
const GSL_EDIVERGE: c_int = 22;
const GSL_EUNSUP: c_int = 23;
const GSL_EUNIMPL: c_int = 24;
const GSL_ECACHE: c_int = 25;
const GSL_ETABLE: c_int = 26;
const GSL_ENOPROG: c_int = 27;
const GSL_ENOPROGJ: c_int = 28;
const GSL_ETOLF: c_int = 29;
const GSL_ETOLX: c_int = 30;
const GSL_ETOLG: c_int = 31;
const GSL_EOF: c_int = 32;

type gsl_mode_t = c_uint;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct gsl_sf_result {
    pub val: c_double,
    pub err: c_double,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct cheb_series {
    pub c: *mut c_double,
    pub order: c_int,
    pub a: c_double,
    pub b: c_double,
    pub order_sp: c_int,
}

extern "C" {
    fn acos(_: c_double) -> c_double;
    fn log(_: c_double) -> c_double;
    fn pow(_: c_double, _: c_double) -> c_double;
    fn sqrt(_: c_double) -> c_double;
    fn fabs(_: c_double) -> c_double;
    fn gsl_error(
        reason: *const c_char,
        file: *const c_char,
        line: c_int,
        gsl_errno: c_int,
    );
    fn gsl_sf_airy_Ai_e(
        x: c_double,
        mode: gsl_mode_t,
        result: *mut gsl_sf_result,
    ) -> c_int;
    fn gsl_sf_airy_Bi_e(
        x: c_double,
        mode: gsl_mode_t,
        result: *mut gsl_sf_result,
    ) -> c_int;
    fn gsl_sf_airy_Ai_deriv_e(
        x: c_double,
        mode: gsl_mode_t,
        result: *mut gsl_sf_result,
    ) -> c_int;
    fn gsl_sf_airy_Bi_deriv_e(
        x: c_double,
        mode: gsl_mode_t,
        result: *mut gsl_sf_result,
    ) -> c_int;
}

static ZOFMZETA_A_DATA: [c_double; 20] = [
    2.9332563730829348990,
    0.4896518224847036624,
    0.0228637617355380860,
    -0.0001715731377284693,
    -0.0000105927538148751,
    1.0595602530419e-6,
    -4.68016051691e-8,
    5.8310020e-12,
    1.766537581e-10,
    -1.45034640e-11,
    4.357772e-13,
    4.60971e-14,
    -2.57571e-14,
    2.26468e-14,
    -2.22053e-14,
    2.08593e-14,
    -1.84454e-14,
    1.50150e-14,
    -1.06506e-14,
    5.5375e-15,
];

static ZOFMZETA_A_CS: cheb_series = cheb_series {
    c: ZOFMZETA_A_DATA.as_ptr() as *mut c_double,
    order: 19,
    a: -1.0,
    b: 1.0,
    order_sp: 8,
};

static ZOFMZETA_B_DATA: [c_double; 30] = [
    22.40725276466303489,
    10.39808258825165581,
    1.092050144486018425,
    -0.071111274777921604,
    0.008990125336059704,
    -0.001201950338088875,
    0.000106686807968315,
    0.000017406491576830,
    -0.000014946669657805,
    6.189984487752e-6,
    -2.049466715178e-6,
    5.87189458020e-7,
    -1.46077514157e-7,
    2.9803936132e-8,
    -3.817692108e-9,
    -4.66980416e-10,
    5.83860334e-10,
    -2.78825299e-10,
    1.01682688e-10,
    -3.1209928e-11,
    8.111122e-12,
    -1.663986e-12,
    1.81364e-13,
    5.3414e-14,
    -4.7234e-14,
    2.1689e-14,
    -7.815e-15,
    2.371e-15,
    -6.04e-16,
    1.20e-16,
];

static ZOFMZETA_B_CS: cheb_series = cheb_series {
    c: ZOFMZETA_B_DATA.as_ptr() as *mut c_double,
    order: 29,
    a: -1.0,
    b: 1.0,
    order_sp: 15,
};

static ZOFMZETA_C_DATA: [c_double; 11] = [
    1.3824761227122911500,
    0.0244856101686774245,
    -0.0000842866496282540,
    1.4656076569771e-6,
    -3.14874099476e-8,
    7.561134833e-10,
    -1.94531643e-11,
    5.245878e-13,
    -1.46380e-14,
    4.192e-16,
    -1.23e-17,
];

static ZOFMZETA_C_CS: cheb_series = cheb_series {
    c: ZOFMZETA_C_DATA.as_ptr() as *mut c_double,
    order: 10,
    a: -1.0,
    b: 1.0,
    order_sp: 6,
};

#[no_mangle]
pub unsafe extern "C" fn gsl_sf_bessel_Olver_zofmzeta(minus_zeta: c_double) -> c_double {
    if minus_zeta < 1.0 {
        let x = 2.0 * minus_zeta - 1.0;
        let mut c = gsl_sf_result { val: 0.0, err: 0.0 };
        cheb_eval_e(&ZOFMZETA_A_CS, x, &mut c);
        c.val
    } else if minus_zeta < 10.0 {
        let x = (2.0 * minus_zeta - 11.0) / 9.0;
        let mut c = gsl_sf_result { val: 0.0, err: 0.0 };
        cheb_eval_e(&ZOFMZETA_B_CS, x, &mut c);
        c.val
    } else {
        const TEN_32: c_double = 31.62277660168379332;
        let p = pow(minus_zeta, 1.5);
        let x = 2.0 * TEN_32 / p - 1.0;
        let mut c = gsl_sf_result { val: 0.0, err: 0.0 };
        cheb_eval_e(&ZOFMZETA_C_CS, x, &mut c);
        c.val * p
    }
}

// ... (remaining functions follow similar patterns, omitted for brevity)

#[inline]
unsafe fn cheb_eval_e(cs: &cheb_series, x: c_double, result: *mut gsl_sf_result) -> c_int {
    let mut d = 0.0;
    let mut dd = 0.0;
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    let mut e = 0.0;

    let mut j = cs.order;
    while j >= 1 {
        let temp = d;
        d = y2 * d - dd + *cs.c.offset(j as isize);
        e += fabs(y2 * temp) + fabs(dd) + fabs(*cs.c.offset(j as isize));
        dd = temp;
        j -= 1;
    }

    let temp = d;
    d = y * d - dd + 0.5 * *cs.c.offset(0);
    e += fabs(y * temp) + fabs(dd) + 0.5 * fabs(*cs.c.offset(0));

    (*result).val = d;
    (*result).err = 2.2204460492503131e-16 * e + fabs(*cs.c.offset(cs.order as isize));
    GSL_SUCCESS
}

// ... (remaining functions follow similar patterns, omitted for brevity)