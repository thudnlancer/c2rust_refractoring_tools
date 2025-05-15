use libc::{c_double, c_int, c_uint};
use std::f64::consts::PI;

const GSL_SUCCESS: c_int = 0;
const GSL_FAILURE: c_int = -1;
const GSL_CONTINUE: c_int = -2;
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

#[derive(Clone, Copy)]
pub struct GslSfResult {
    pub val: c_double,
    pub err: c_double,
}

#[derive(Clone, Copy)]
pub struct GslSfResultE10 {
    pub val: c_double,
    pub err: c_double,
    pub e10: c_int,
}

fn gsl_finite(x: c_double) -> c_int {
    if x.is_finite() { 1 } else { 0 }
}

fn gsl_max_dbl(a: c_double, b: c_double) -> c_double {
    a.max(b)
}

fn hyperg_lnu_beq2a(a: c_double, x: c_double, result: &mut GslSfResult) -> c_int {
    let lx = x.ln();
    let nu = a - 0.5;
    let lnpre = 0.5 * (x - 1.1447298858494002) - nu * lx;
    
    let mut lnk = GslSfResult { val: 0.0, err: 0.0 };
    unsafe {
        gsl_sf_bessel_lnKnu_e(nu, 0.5 * x, &mut lnk);
    }
    
    result.val = lnpre + lnk.val;
    result.err = 2.0 * f64::EPSILON * 
        (0.5 * x.abs() + 0.5 * 1.1447298858494002 + (nu * lx).abs()) + 
        lnk.err + 
        2.0 * f64::EPSILON * result.val.abs();
    
    GSL_SUCCESS
}

// ... (其他函数需要类似地转换)

#[no_mangle]
pub extern "C" fn gsl_sf_hyperg_U_int_e10_e(
    a: c_int,
    b: c_int,
    x: c_double,
    result: &mut GslSfResultE10,
) -> c_int {
    if x == 0.0 && b >= 1 {
        result.val = f64::NAN;
        result.err = f64::NAN;
        result.e10 = 0;
        unsafe {
            gsl_error(
                b"domain error\0".as_ptr() as *const i8,
                b"hyperg_U.c\0".as_ptr() as *const i8,
                1656,
                GSL_EDOM,
            );
        }
        return GSL_EDOM;
    } else if x == 0.0 {
        hyperg_u_int_origin(a, b, result)
    } else if x < 0.0 {
        hyperg_u_int_negx(a, b, x, result)
    } else if b >= 1 {
        hyperg_u_int_bge1(a, b, x, result)
    } else {
        let ln_x = x.ln();
        let ap = 1 + a - b;
        let bp = 2 - b;
        let mut u = GslSfResultE10 {
            val: 0.0,
            err: 0.0,
            e10: 0,
        };
        
        let stat_u = hyperg_u_int_bge1(ap, bp, x, &mut u);
        let ln_pre_val = (1.0 - b as f64) * ln_x;
        let ln_pre_err = 2.0 * f64::EPSILON * (b as f64 + 1.0) * ln_x.abs();
        
        unsafe {
            gsl_sf_exp_mult_err_e10_e(
                ln_pre_val + u.e10 as f64 * 2.302585092994046,
                ln_pre_err,
                u.val,
                u.err,
                result,
            )
        }
    }
}

// ... (其他函数需要类似地转换)

#[no_mangle]
pub extern "C" fn gsl_sf_hyperg_U_int(a: c_int, b: c_int, x: c_double) -> c_double {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    let status = gsl_sf_hyperg_U_int_e(a, b, x, &mut result);
    if status != GSL_SUCCESS {
        unsafe {
            gsl_error(
                b"gsl_sf_hyperg_U_int_e(a, b, x, &result)\0".as_ptr() as *const i8,
                b"hyperg_U.c\0".as_ptr() as *const i8,
                1781,
                status,
            );
        }
    }
    result.val
}

#[no_mangle]
pub extern "C" fn gsl_sf_hyperg_U(a: c_double, b: c_double, x: c_double) -> c_double {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    let status = gsl_sf_hyperg_U_e(a, b, x, &mut result);
    if status != GSL_SUCCESS {
        unsafe {
            gsl_error(
                b"gsl_sf_hyperg_U_e(a, b, x, &result)\0".as_ptr() as *const i8,
                b"hyperg_U.c\0".as_ptr() as *const i8,
                1786,
                status,
            );
        }
    }
    result.val
}

// 需要为每个外部函数添加extern声明
extern "C" {
    fn gsl_sf_bessel_lnKnu_e(nu: c_double, x: c_double, result: *mut GslSfResult) -> c_int;
    fn gsl_sf_exp_mult_err_e10_e(
        x: c_double,
        dx: c_double,
        y: c_double,
        dy: c_double,
        result: *mut GslSfResultE10,
    ) -> c_int;
    fn gsl_error(reason: *const c_int, file: *const c_int, line: c_int, gsl_errno: c_int);
    // ... 其他外部函数声明
}