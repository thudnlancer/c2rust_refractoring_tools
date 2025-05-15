use libc::{c_double, c_int, c_uint};
use std::f64::consts::PI;

pub const GSL_SUCCESS: c_int = 0;
pub const GSL_FAILURE: c_int = -1;
pub const GSL_CONTINUE: c_int = -2;
pub const GSL_EDOM: c_int = 1;
pub const GSL_ERANGE: c_int = 2;
pub const GSL_EFAULT: c_int = 3;
pub const GSL_EINVAL: c_int = 4;
pub const GSL_EFAILED: c_int = 5;
pub const GSL_EFACTOR: c_int = 6;
pub const GSL_ESANITY: c_int = 7;
pub const GSL_ENOMEM: c_int = 8;
pub const GSL_EBADFUNC: c_int = 9;
pub const GSL_ERUNAWAY: c_int = 10;
pub const GSL_EMAXITER: c_int = 11;
pub const GSL_EZERODIV: c_int = 12;
pub const GSL_EBADTOL: c_int = 13;
pub const GSL_ETOL: c_int = 14;
pub const GSL_EUNDRFLW: c_int = 15;
pub const GSL_EOVRFLW: c_int = 16;
pub const GSL_ELOSS: c_int = 17;
pub const GSL_EROUND: c_int = 18;
pub const GSL_EBADLEN: c_int = 19;
pub const GSL_ENOTSQR: c_int = 20;
pub const GSL_ESING: c_int = 21;
pub const GSL_EDIVERGE: c_int = 22;
pub const GSL_EUNSUP: c_int = 23;
pub const GSL_EUNIMPL: c_int = 24;
pub const GSL_ECACHE: c_int = 25;
pub const GSL_ETABLE: c_int = 26;
pub const GSL_ENOPROG: c_int = 27;
pub const GSL_ENOPROGJ: c_int = 28;
pub const GSL_ETOLF: c_int = 29;
pub const GSL_ETOLX: c_int = 30;
pub const GSL_ETOLG: c_int = 31;
pub const GSL_EOF: c_int = 32;

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: c_double,
    pub err: c_double,
}

pub type GslModeT = c_uint;

pub fn c0sq(eta: c_double) -> c_double {
    let twopieta = 2.0 * PI * eta;
    if eta.abs() < 2.2204460492503131e-16 {
        1.0
    } else if twopieta > 7.0978271289338397e+02 {
        0.0
    } else {
        let mut scale = GslSfResult { val: 0.0, err: 0.0 };
        unsafe {
            gsl_sf_expm1_e(twopieta, &mut scale);
        }
        twopieta / scale.val
    }
}

pub fn cl_eta(l: c_double, eta: c_double, result: &mut GslSfResult) -> c_int {
    let mut ln1 = GslSfResult { val: 0.0, err: 0.0 };
    let mut ln2 = GslSfResult { val: 0.0, err: 0.0 };
    let mut sgn = 1.0;
    let mut arg_val;
    let mut arg_err;

    if (eta / (l + 1.0)).abs() < 2.2204460492503131e-16 {
        unsafe {
            gsl_sf_lngamma_e(l + 1.0, &mut ln1);
        }
    } else {
        let mut p1 = GslSfResult { val: 0.0, err: 0.0 };
        unsafe {
            gsl_sf_lngamma_complex_e(l + 1.0, eta, &mut ln1, &mut p1);
        }
    }

    unsafe {
        gsl_sf_lngamma_e(2.0 * (l + 1.0), &mut ln2);
    }

    if l < -1.0 {
        sgn = -sgn;
    }

    arg_val = l * 0.69314718055994530942 - 0.5 * eta * PI + ln1.val - ln2.val;
    arg_err = ln1.err + ln2.err;
    arg_err += 2.2204460492503131e-16 * (l * 0.69314718055994530942).abs() + (0.5 * eta * PI).abs();

    unsafe {
        gsl_sf_exp_err_e(arg_val, arg_err, result)
    }
}

#[no_mangle]
pub extern "C" fn gsl_sf_coulomb_CL_e(lam: c_double, eta: c_double, result: &mut GslSfResult) -> c_int {
    if lam <= -1.0 {
        result.val = f64::NAN;
        result.err = f64::NAN;
        unsafe {
            gsl_error(
                b"domain error\0".as_ptr() as *const i8,
                b"coulomb.c\0".as_ptr() as *const i8,
                110,
                GSL_EDOM,
            );
        }
        GSL_EDOM
    } else if lam.abs() < 2.2204460492503131e-16 {
        result.val = c0sq(eta).sqrt();
        result.err = 2.0 * 2.2204460492503131e-16 * result.val;
        GSL_SUCCESS
    } else {
        cl_eta(lam, eta, result)
    }
}

#[no_mangle]
pub extern "C" fn gsl_sf_coulomb_CL_array(
    lam_min: c_double,
    kmax: c_int,
    eta: c_double,
    cl: &mut [c_double],
) -> c_int {
    let mut cl_0 = GslSfResult { val: 0.0, err: 0.0 };
    gsl_sf_coulomb_CL_e(lam_min, eta, &mut cl_0);
    cl[0] = cl_0.val;

    for k in 1..=kmax {
        let l = lam_min + k as c_double;
        cl[k as usize] = cl[(k - 1) as usize] * (l.hypot(eta)) / (l * (2.0 * l + 1.0));
    }

    GSL_SUCCESS
}

// ... (其他函数类似地转换为Rust代码)

extern "C" {
    fn gsl_sf_expm1_e(x: c_double, result: *mut GslSfResult) -> c_int;
    fn gsl_sf_exp_err_e(x: c_double, dx: c_double, result: *mut GslSfResult) -> c_int;
    fn gsl_sf_lngamma_e(x: c_double, result: *mut GslSfResult) -> c_int;
    fn gsl_sf_lngamma_complex_e(
        zr: c_double,
        zi: c_double,
        lnr: *mut GslSfResult,
        arg: *mut GslSfResult,
    ) -> c_int;
    fn gsl_sf_psi_1piy_e(y: c_double, result: *mut GslSfResult) -> c_int;
    fn gsl_sf_airy_Ai_scaled_e(x: c_double, mode: GslModeT, result: *mut GslSfResult) -> c_int;
    fn gsl_sf_airy_Bi_scaled_e(x: c_double, mode: GslModeT, result: *mut GslSfResult) -> c_int;
    fn gsl_error(reason: *const i8, file: *const i8, line: c_int, gsl_errno: c_int);
}