use std::f64::consts::{PI, LN_2};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

impl SfResult {
    pub fn new(val: f64, err: f64) -> Self {
        SfResult { val, err }
    }
}

const GSL_DBL_EPSILON: f64 = 2.2204460492503131e-16;
const GSL_ROOT3_DBL_EPSILON: f64 = 6.0554544523933429e-6;
const GSL_ROOT5_DBL_EPSILON: f64 = 7.4009597974140505e-4;
const GSL_LOG_DBL_MAX: f64 = 709.78271289338397;
const GSL_SQRT_DBL_MIN: f64 = 1.4916681462400413e-154;
const GSL_SQRT_DBL_MAX: f64 = 1.3407807929942596e154;
const M_LNPI: f64 = 1.1442227999201618;
const M_LN2: f64 = 0.69314718055994529;

#[derive(Debug)]
pub enum SfError {
    Domain,
    Overflow,
    MaxIter,
    Other,
}

fn legendre_h3d_lnnorm(ell: i32, lambda: f64) -> Result<SfResult, SfError> {
    let abs_lam = lambda.abs();

    if abs_lam == 0.0 {
        Err(SfError::Domain)
    } else if lambda > (ell as f64 + 1.0) / GSL_ROOT3_DBL_EPSILON {
        let rat = (ell as f64 + 1.0) / lambda;
        let ln_lam2ell2 = 2.0 * lambda.ln() + (1.0 + rat * rat).ln();
        let lg_corrected = -2.0 * (ell as f64 + 1.0) + M_LNPI 
            + (ell as f64 + 0.5) * ln_lam2ell2 
            + 1.0 / (288.0 * lambda * lambda);
        let angle_terms = lambda * 2.0 * rat * (1.0 - rat * rat / 3.0);
        let val = abs_lam.ln() + lg_corrected + angle_terms - M_LNPI;
        Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else {
        let lg_r = ln_gamma_complex(ell as f64 + 1.0, lambda)?;
        let ln_sinh = ln_sinh(PI * abs_lam)?;
        let val = abs_lam.ln() + ln_sinh.val + 2.0 * lg_r.val - M_LNPI;
        Ok(SfResult::new(val, ln_sinh.err + 2.0 * lg_r.err + 2.0 * GSL_DBL_EPSILON * val.abs()))
    }
}

fn legendre_h3d_series(ell: i32, lambda: f64, eta: f64) -> Result<SfResult, SfError> {
    const NMAX: usize = 5000;
    let shheta = (0.5 * eta).sinh();
    let ln_zp1 = LN_2 + (1.0 + shheta * shheta).ln();
    let ln_zm1 = LN_2 + 2.0 * shheta.ln();
    let zeta = -shheta * shheta;
    
    let lg_lp32 = ln_gamma(ell as f64 + 1.5)?;
    let lnsheta = ln_sinh(eta)?;
    let ln_n = legendre_h3d_lnnorm(ell, lambda)?;
    
    let lnprepow = 0.5 * (ell as f64 + 0.5) * (ln_zm1 - ln_zp1);
    let lnpre_val = lnprepow + 0.5 * (ln_n.val + M_LNPI - LN_2 - lnsheta.val) 
        - lg_lp32.val - lambda.abs().ln();
    
    let mut lnpre_err = lnsheta.err + lg_lp32.err + GSL_DBL_EPSILON * lnpre_val.abs();
    lnpre_err += 2.0 * GSL_DBL_EPSILON * (ln_n.val.abs() + M_LNPI + LN_2);
    lnpre_err += 2.0 * GSL_DBL_EPSILON * (0.5 * (ell as f64 + 0.5) * (ln_zm1.abs() + ln_zp1.abs()));
    
    let mut term = 1.0;
    let mut sum = 1.0;
    let mut sum_err = 0.0;
    
    for n in 1..NMAX {
        let a_r = n as f64 - 0.5;
        term *= (a_r * a_r + lambda * lambda) * zeta / (ell as f64 + n as f64 + 0.5) / n as f64;
        sum += term;
        sum_err += 2.0 * GSL_DBL_EPSILON * term.abs();
        if (term / sum).abs() < 2.0 * GSL_DBL_EPSILON {
            break;
        }
    }
    
    let result = exp_mult_err(lnpre_val, lnpre_err, sum, term.abs() + sum_err)?;
    Ok(result)
}

fn legendre_h3d_cf1_ser(ell: i32, lambda: f64, coth_eta: f64) -> Result<SfResult, SfError> {
    const MAXK: usize = 20000;
    let pre = (lambda.powi(2) + (ell as f64 + 1.0).powi(2)).sqrt() 
        / ((2.0 * ell as f64 + 3.0) * coth_eta);
    
    let mut tk = 1.0;
    let mut sum = 1.0;
    let mut rhok = 0.0;
    let mut sum_err = 0.0;
    
    for k in 1..MAXK {
        let tlk = 2.0 * ell as f64 + 1.0 + 2.0 * k as f64;
        let l1k = ell as f64 + 1.0 + k as f64;
        let ak = -(lambda.powi(2) + l1k.powi(2)) 
            / (tlk * (tlk + 2.0) * coth_eta.powi(2));
        rhok = -ak * (1.0 + rhok) / (1.0 + ak * (1.0 + rhok));
        tk *= rhok;
        sum += tk;
        sum_err += 2.0 * GSL_DBL_EPSILON * k as f64 * tk.abs();
        if (tk / sum).abs() < GSL_DBL_EPSILON {
            break;
        }
    }
    
    let val = pre * sum;
    let mut err = pre * tk.abs();
    err += pre * sum_err;
    err += 4.0 * GSL_DBL_EPSILON * val.abs();
    
    Ok(SfResult::new(val, err))
}

pub fn legendre_h3d_0(lambda: f64, eta: f64) -> Result<SfResult, SfError> {
    if eta < 0.0 {
        Err(SfError::Domain)
    } else if eta == 0.0 || lambda == 0.0 {
        Ok(SfResult::new(1.0, 0.0))
    } else {
        let lam_eta = lambda * eta;
        let s = sin_err(lam_eta, 2.0 * GSL_DBL_EPSILON * lam_eta.abs())?;
        
        if eta > -0.5 * GSL_LOG_DBL_MAX {
            let f = 2.0 / lambda * (-eta).exp();
            let val = f * s.val;
            let mut err = (f * s.val).abs() * (eta.abs() + 1.0) * GSL_DBL_EPSILON;
            err += f.abs() * s.err;
            err += 2.0 * GSL_DBL_EPSILON * val.abs();
            Ok(SfResult::new(val, err))
        } else {
            let f = 1.0 / (lambda * eta.sinh());
            let val = f * s.val;
            let mut err = (f * s.val).abs() * (eta.abs() + 1.0) * GSL_DBL_EPSILON;
            err += f.abs() * s.err;
            err += 2.0 * GSL_DBL_EPSILON * val.abs();
            Ok(SfResult::new(val, err))
        }
    }
}

// Helper functions implementations
fn ln_gamma_complex(a: f64, b: f64) -> Result<SfResult, SfError> {
    // Simplified implementation - in practice would use complex log gamma
    let r = (a.powi(2) + b.powi(2)).sqrt();
    let theta = b.atan2(a);
    Ok(SfResult::new(r.ln(), GSL_DBL_EPSILON * r.abs()))
}

fn ln_sinh(x: f64) -> Result<SfResult, SfError> {
    let val = x.sinh().ln();
    Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
}

fn ln_gamma(x: f64) -> Result<SfResult, SfError> {
    let val = x.ln_gamma().0;
    Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
}

fn sin_err(x: f64, dx: f64) -> Result<SfResult, SfError> {
    let val = x.sin();
    let err = (x.cos() * dx).abs() + 2.0 * GSL_DBL_EPSILON * val.abs();
    Ok(SfResult::new(val, err))
}

fn exp_mult_err(lnx: f64, dlnx: f64, y: f64, dy: f64) -> Result<SfResult, SfError> {
    let x = lnx.exp();
    let val = x * y;
    let err = val * (dlnx + dy / y.abs()) + 2.0 * GSL_DBL_EPSILON * val.abs();
    Ok(SfResult::new(val, err))
}

// Note: This is a partial implementation. The full implementation would include:
// - All the remaining functions from the C code
// - More accurate implementations of special functions
// - Proper error handling for edge cases
// - Complete test coverage