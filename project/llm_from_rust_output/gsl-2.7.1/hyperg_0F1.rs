use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDOM = 1,
    ERANGE = 2,
    EFAULT = 3,
    EINVAL = 4,
    EFAILED = 5,
    EFACTOR = 6,
    ESANITY = 7,
    ENOMEM = 8,
    EBADFUNC = 9,
    ERUNAWAY = 10,
    EMAXITER = 11,
    EZERODIV = 12,
    EBADTOL = 13,
    ETOL = 14,
    EUNDRFLW = 15,
    EOVRFLW = 16,
    ELOSS = 17,
    EROUND = 18,
    EBADLEN = 19,
    ENOTSQR = 20,
    ESING = 21,
    EDIVERGE = 22,
    EUNSUP = 23,
    EUNIMPL = 24,
    ECACHE = 25,
    ETABLE = 26,
    ENOPROG = 27,
    ENOPROGJ = 28,
    ETOLF = 29,
    ETOLX = 30,
    ETOLG = 31,
    EOF = 32,
}

fn hyperg_0f1_bessel_i(nu: f64, x: f64) -> Result<GslSfResult, GslError> {
    if x > 7.0978271289338397e+02 {
        return Err(GslError::EOVRFLW);
    }

    if nu < 0.0 {
        let anu = -nu;
        let s = 2.0 / PI * (anu * PI).sin();
        let ex = x.exp();
        let i_result = gsl_sf_bessel_inu_scaled_e(anu, x)?;
        let k_result = gsl_sf_bessel_knu_scaled_e(anu, x)?;

        let val = ex * i_result.val + s * (k_result.val / ex);
        let err = ex * i_result.err + (s * k_result.err / ex).abs();
        let err = err + (s * (k_result.val / ex)).abs() * 2.2204460492503131e-16 * anu * PI;

        Ok(GslSfResult { val, err })
    } else {
        let ex = x.exp();
        let i_result = gsl_sf_bessel_inu_scaled_e(nu, x)?;
        let val = ex * i_result.val;
        let err = ex * i_result.err + 2.2204460492503131e-16 * val.abs();
        Ok(GslSfResult { val, err })
    }
}

fn hyperg_0f1_bessel_j(nu: f64, x: f64) -> Result<GslSfResult, GslError> {
    if nu < 0.0 {
        let anu = -nu;
        let s = (anu * PI).sin();
        let c = (anu * PI).cos();
        let j_result = gsl_sf_bessel_jnu_e(anu, x)?;
        let y_result = gsl_sf_bessel_ynu_e(anu, x)?;

        let val = c * j_result.val - s * y_result.val;
        let err = (c * j_result.err).abs() + (s * y_result.err).abs();
        let err = err + (anu * PI).abs() * 2.2204460492503131e-16 * (j_result.val + y_result.val).abs();

        Ok(GslSfResult { val, err })
    } else {
        gsl_sf_bessel_jnu_e(nu, x)
    }
}

pub fn gsl_sf_hyperg_0f1_e(c: f64, x: f64) -> Result<GslSfResult, GslError> {
    let rintc = (c + 0.5).floor();
    let c_neg_integer = c < 0.0 && (c - rintc).abs() < 1000.0 * 2.2204460492503131e-16;

    if c == 0.0 || c_neg_integer {
        Err(GslError::EDOM)
    } else if x < 0.0 {
        let lg_c = gsl_sf_lngamma_sgn_e(c)?;
        let jcm1 = hyperg_0f1_bessel_j(c - 1.0, 2.0 * (-x).sqrt())?;

        if jcm1.val == 0.0 {
            Ok(GslSfResult { val: 0.0, err: 0.0 })
        } else {
            let tl = (-x).ln() * 0.5 * (1.0 - c);
            let ln_pre_val = lg_c.val + tl;
            let ln_pre_err = lg_c.err + 2.0 * 2.2204460492503131e-16 * tl.abs();
            gsl_sf_exp_mult_err_e(ln_pre_val, ln_pre_err, lg_c.sgn * jcm1.val, jcm1.err)
        }
    } else if x == 0.0 {
        Ok(GslSfResult { val: 1.0, err: 1.0 })
    } else {
        let lg_c = gsl_sf_lngamma_sgn_e(c)?;
        let icm1 = hyperg_0f1_bessel_i(c - 1.0, 2.0 * x.sqrt())?;

        if icm1.val == 0.0 {
            Ok(GslSfResult { val: 0.0, err: 0.0 })
        } else {
            let tl = x.ln() * 0.5 * (1.0 - c);
            let ln_pre_val = lg_c.val + tl;
            let ln_pre_err = lg_c.err + 2.0 * 2.2204460492503131e-16 * tl.abs();
            gsl_sf_exp_mult_err_e(ln_pre_val, ln_pre_err, lg_c.sgn * icm1.val, icm1.err)
        }
    }
}

pub fn gsl_sf_hyperg_0f1(c: f64, x: f64) -> f64 {
    match gsl_sf_hyperg_0f1_e(c, x) {
        Ok(result) => result.val,
        Err(_) => NAN,
    }
}

// Placeholder implementations for GSL functions
fn gsl_sf_bessel_inu_scaled_e(_nu: f64, _x: f64) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

fn gsl_sf_bessel_knu_scaled_e(_nu: f64, _x: f64) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

fn gsl_sf_bessel_jnu_e(_nu: f64, _x: f64) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

fn gsl_sf_bessel_ynu_e(_nu: f64, _x: f64) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

fn gsl_sf_lngamma_sgn_e(_x: f64) -> Result<LngammaResult, GslError> {
    unimplemented!()
}

fn gsl_sf_exp_mult_err_e(_x: f64, _dx: f64, _y: f64, _dy: f64) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

#[derive(Debug)]
struct LngammaResult {
    val: f64,
    err: f64,
    sgn: f64,
}