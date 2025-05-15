use std::f64::consts::LN_10;
use std::f64::consts::LN_2;
use std::f64::{NAN, INFINITY};

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct SfResultE10 {
    pub val: f64,
    pub err: f64,
    pub e10: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

fn gsl_error(reason: &str, file: &str, line: i32, errno: GslError) {
    eprintln!("GSL error: {} at {}:{} (code {})", reason, file, line, errno as i32);
}

fn gsl_sf_exp_mult_err_e(x: f64, dx: f64, y: f64, dy: f64) -> Result<SfResult, GslError> {
    let val = (x * y).exp();
    let err = val * (dx * dx + dy * dy).sqrt();
    Ok(SfResult { val, err })
}

fn gsl_sf_result_smash_e(re: &SfResultE10) -> Result<SfResult, GslError> {
    let factor = 10.0f64.powi(re.e10);
    Ok(SfResult {
        val: re.val * factor,
        err: re.err * factor,
    })
}

fn gsl_sf_lngamma_e(x: f64) -> Result<SfResult, GslError> {
    let val = x.ln_gamma().0;
    let err = 2.0 * f64::EPSILON * val.abs();
    Ok(SfResult { val, err })
}

fn gsl_sf_bessel_K0_scaled_e(x: f64) -> Result<SfResult, GslError> {
    if x <= 0.0 {
        gsl_error("domain error", "bessel_Knu.c", 116, GslError::Domain);
        Ok(SfResult { val: NAN, err: NAN })
    } else {
        // Implementation would call actual Bessel K0 function
        unimplemented!()
    }
}

pub fn gsl_sf_bessel_Knu_scaled_e(nu: f64, x: f64) -> Result<SfResult, GslError> {
    if x <= 0.0 || nu < 0.0 {
        gsl_error("domain error", "bessel_Knu.c", 42, GslError::Domain);
        Ok(SfResult { val: NAN, err: NAN })
    } else {
        let result_e10 = gsl_sf_bessel_Knu_scaled_e10_e(nu, x)?;
        gsl_sf_result_smash_e(&result_e10)
    }
}

pub fn gsl_sf_bessel_Knu_scaled_e10_e(nu: f64, x: f64) -> Result<SfResultE10, GslError> {
    if x <= 0.0 || nu < 0.0 {
        gsl_error("domain error", "bessel_Knu.c", 58, GslError::Domain);
        Ok(SfResultE10 {
            val: NAN,
            err: NAN,
            e10: 0,
        })
    } else {
        let n = (nu + 0.5) as i32;
        let mu = nu - n as f64;
        let (k_mu, k_mup1, kp_mu) = if x < 2.0 {
            gsl_sf_bessel_K_scaled_temme(mu, x)?
        } else {
            gsl_sf_bessel_K_scaled_steed_temme_cf2(mu, x)?
        };

        let mut k_nu = k_mu;
        let mut k_nup1 = k_mup1;
        let mut e10 = 0;

        for i in 0..n {
            let k_num1 = k_nu;
            k_nu = k_nup1;
            
            if k_nu.abs() > 1.3407807929942596e154 {
                let p = (k_nu.abs().ln() / LN_10).floor();
                let factor = 10.0f64.powf(p);
                k_nu /= factor;
                e10 += p as i32;
            }

            k_nup1 = 2.0 * (mu + i as f64 + 1.0) / x * k_nu + k_num1;
        }

        Ok(SfResultE10 {
            val: k_nu,
            err: 2.0 * f64::EPSILON * (n as f64 + 4.0) * k_nu.abs(),
            e10,
        })
    }
}

fn gsl_sf_bessel_K_scaled_temme(nu: f64, x: f64) -> Result<(f64, f64, f64), GslError> {
    // Implementation would call actual Temme method
    unimplemented!()
}

fn gsl_sf_bessel_K_scaled_steed_temme_cf2(nu: f64, x: f64) -> Result<(f64, f64, f64), GslError> {
    // Implementation would call actual Steed-Temme CF2 method
    unimplemented!()
}

pub fn gsl_sf_bessel_Knu_e(nu: f64, x: f64) -> Result<SfResult, GslError> {
    let b = gsl_sf_bessel_Knu_scaled_e(nu, x)?;
    gsl_sf_exp_mult_err_e(-x, 0.0, b.val, b.err)
}

pub fn gsl_sf_bessel_lnKnu_e(nu: f64, x: f64) -> Result<SfResult, GslError> {
    if x <= 0.0 || nu < 0.0 {
        gsl_error("domain error", "bessel_Knu.c", 116, GslError::Domain);
        Ok(SfResult { val: NAN, err: NAN })
    } else if nu == 0.0 {
        let k_scaled = gsl_sf_bessel_K0_scaled_e(x)?;
        let val = -x + k_scaled.val.abs().ln();
        let err = f64::EPSILON * x.abs() + (k_scaled.err / k_scaled.val).abs();
        Ok(SfResult {
            val,
            err: err + f64::EPSILON * val.abs(),
        })
    } else if x < 2.0 && nu > 1.0 {
        let lg_nu = gsl_sf_lngamma_e(nu)?;
        let ln_bound = -LN_2 - nu * (0.5 * x).ln() + lg_nu.val;
        
        if ln_bound > 709.78271289338397 - 20.0 {
            let xi = 0.25 * x * x;
            let mut sum = 1.0 - xi / (nu - 1.0);
            if nu > 2.0 {
                sum += xi / (nu - 1.0) * (xi / (nu - 2.0));
            }
            let val = ln_bound + sum.ln();
            Ok(SfResult {
                val,
                err: lg_nu.err + 2.0 * f64::EPSILON * val.abs(),
            })
        } else {
            let k_scaled_e10 = gsl_sf_bessel_Knu_scaled_e10_e(nu, x)?;
            let val = -x + k_scaled_e10.val.abs().ln() + k_scaled_e10.e10 as f64 * LN_10;
            let err = f64::EPSILON * x.abs() + (k_scaled_e10.err / k_scaled_e10.val).abs();
            Ok(SfResult {
                val,
                err: err + f64::EPSILON * val.abs(),
            })
        }
    } else {
        let k_scaled_e10 = gsl_sf_bessel_Knu_scaled_e10_e(nu, x)?;
        let val = -x + k_scaled_e10.val.abs().ln() + k_scaled_e10.e10 as f64 * LN_10;
        let err = f64::EPSILON * x.abs() + (k_scaled_e10.err / k_scaled_e10.val).abs();
        Ok(SfResult {
            val,
            err: err + f64::EPSILON * val.abs(),
        })
    }
}

pub fn gsl_sf_bessel_Knu_scaled(nu: f64, x: f64) -> f64 {
    gsl_sf_bessel_Knu_scaled_e(nu, x).unwrap().val
}

pub fn gsl_sf_bessel_Knu(nu: f64, x: f64) -> f64 {
    gsl_sf_bessel_Knu_e(nu, x).unwrap().val
}

pub fn gsl_sf_bessel_lnKnu(nu: f64, x: f64) -> f64 {
    gsl_sf_bessel_lnKnu_e(nu, x).unwrap().val
}