use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    eprintln!("GSL error: {} at {}:{} - {:?}", reason, file, line, errno);
}

fn legendre_h3d_lnnorm(ell: i32, lambda: f64) -> Result<f64, GslError> {
    let abs_lam = lambda.abs();
    if abs_lam == 0.0 {
        gsl_error("error", "legendre_H3d.c", 52, GslError::Domain);
        Err(GslError::Domain)
    } else if lambda > (ell as f64 + 1.0) / 6.0554544523933429e-06 {
        let rat = (ell as f64 + 1.0) / lambda;
        let ln_lam2ell2 = 2.0 * lambda.ln() + (1.0 + rat * rat).ln();
        let lg_corrected = -2.0 * (ell as f64 + 1.0)
            + 1.14472988584940017414342735135
            + (ell as f64 + 0.5) * ln_lam2ell2
            + 1.0 / (288.0 * lambda * lambda);
        let angle_terms = lambda * 2.0 * rat * (1.0 - rat * rat / 3.0);
        Ok(abs_lam.ln() + lg_corrected + angle_terms - 1.14472988584940017414342735135)
    } else {
        let lg_r = gsl_sf_lngamma_complex(ell as f64 + 1.0, lambda)?;
        let ln_sinh = gsl_sf_lnsinh(PI * abs_lam)?;
        Ok(abs_lam.ln() + ln_sinh.val + 2.0 * lg_r.val - 1.14472988584940017414342735135)
    }
}

fn legendre_h3d_series(ell: i32, lambda: f64, eta: f64) -> Result<GslSfResult, GslError> {
    const NMAX: i32 = 5000;
    let shheta = (0.5 * eta).sinh();
    let ln_zp1 = 0.69314718055994530942 + (1.0 + shheta * shheta).ln();
    let ln_zm1 = 0.69314718055994530942 + 2.0 * shheta.ln();
    let zeta = -shheta * shheta;
    
    let lg_lp32 = gsl_sf_lngamma(ell as f64 + 1.5)?;
    let lnsheta = gsl_sf_lnsinh(eta)?;
    let ln_n = legendre_h3d_lnnorm(ell, lambda)?;
    
    let lnprepow = 0.5 * (ell as f64 + 0.5) * (ln_zm1 - ln_zp1);
    let lnpre_val = lnprepow + 0.5 * (ln_n + 1.14472988584940017414342735135 - 0.69314718055994530942 - lnsheta.val) 
        - lg_lp32.val - lambda.abs().ln();
    let mut lnpre_err = lnsheta.err + lg_lp32.err + 2.2204460492503131e-16 * lnpre_val.abs();
    lnpre_err += 2.0 * 2.2204460492503131e-16 * (ln_n.abs() + 1.14472988584940017414342735135 + 0.69314718055994530942);
    lnpre_err += 2.0 * 2.2204460492503131e-16 * (0.5 * (ell as f64 + 0.5) * (ln_zm1.abs() + ln_zp1.abs()));
    
    let mut term = 1.0;
    let mut sum = 1.0;
    let mut sum_err = 0.0;
    
    for n in 1..NMAX {
        let a_r = n as f64 - 0.5;
        term *= (a_r * a_r + lambda * lambda) * zeta / ((ell + n) as f64 + 0.5) / n as f64;
        sum += term;
        sum_err += 2.0 * 2.2204460492503131e-16 * term.abs();
        
        if (term / sum).abs() < 2.0 * 2.2204460492503131e-16 {
            break;
        }
    }
    
    let result = gsl_sf_exp_mult_err(lnpre_val, lnpre_err, sum, term.abs() + sum_err)?;
    
    if result.val.is_nan() || result.err.is_nan() {
        Err(GslError::MaxIter)
    } else {
        Ok(result)
    }
}

fn legendre_h3d_cf1_ser(ell: i32, lambda: f64, coth_eta: f64) -> Result<GslSfResult, GslError> {
    const MAXK: i32 = 20000;
    let pre = (lambda * lambda + (ell as f64 + 1.0).powi(2)).sqrt() 
        / ((2.0 * ell as f64 + 3.0) * coth_eta);
    
    let mut tk = 1.0;
    let mut sum = 1.0;
    let mut rhok = 0.0;
    let mut sum_err = 0.0;
    
    for k in 1..MAXK {
        let tlk = 2.0 * ell as f64 + 1.0 + 2.0 * k as f64;
        let l1k = ell as f64 + 1.0 + k as f64;
        let ak = -(lambda * lambda + l1k * l1k) / (tlk * (tlk + 2.0) * coth_eta * coth_eta);
        rhok = -ak * (1.0 + rhok) / (1.0 + ak * (1.0 + rhok));
        tk *= rhok;
        sum += tk;
        sum_err += 2.0 * 2.2204460492503131e-16 * k as f64 * tk.abs();
        
        if (tk / sum).abs() < 2.2204460492503131e-16 {
            break;
        }
    }
    
    let mut result = GslSfResult {
        val: pre * sum,
        err: (pre * tk).abs(),
    };
    result.err += (pre * sum_err).abs();
    result.err += 4.0 * 2.2204460492503131e-16 * result.val.abs();
    
    if result.val.is_nan() || result.err.is_nan() {
        Err(GslError::MaxIter)
    } else {
        Ok(result)
    }
}

pub fn gsl_sf_legendre_h3d_0(lambda: f64, eta: f64) -> Result<GslSfResult, GslError> {
    if eta < 0.0 {
        gsl_error("domain error", "legendre_H3d.c", 253, GslError::Domain);
        Ok(GslSfResult { val: NAN, err: NAN })
    } else if eta == 0.0 || lambda == 0.0 {
        Ok(GslSfResult { val: 1.0, err: 0.0 })
    } else {
        let lam_eta = lambda * eta;
        let s = gsl_sf_sin_err(lam_eta, 2.0 * 2.2204460492503131e-16 * lam_eta.abs())?;
        
        if eta > -0.5 * -3.6043653389117154e+01 {
            let f = 2.0 / lambda * (-eta).exp();
            Ok(GslSfResult {
                val: f * s.val,
                err: (f * s.val).abs() * (eta.abs() + 1.0) * 2.2204460492503131e-16 + f.abs() * s.err 
                    + 2.0 * 2.2204460492503131e-16 * (f * s.val).abs(),
            })
        } else {
            let f_0 = 1.0 / (lambda * eta.sinh());
            Ok(GslSfResult {
                val: f_0 * s.val,
                err: (f_0 * s.val).abs() * (eta.abs() + 1.0) * 2.2204460492503131e-16 + f_0.abs() * s.err 
                    + 2.0 * 2.2204460492503131e-16 * (f_0 * s.val).abs(),
            })
        }
    }
}

// Additional helper functions would need to be implemented:
// gsl_sf_lngamma_complex, gsl_sf_lnsinh, gsl_sf_lngamma, 
// gsl_sf_sin_err, gsl_sf_exp_mult_err, etc.
// These would follow similar patterns of safe Rust implementations.