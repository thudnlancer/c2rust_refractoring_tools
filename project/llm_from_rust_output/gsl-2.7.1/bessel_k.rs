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

fn gsl_sf_pow_int(x: f64, n: i32) -> f64 {
    x.powi(n)
}

fn gsl_sf_doublefact_e(n: u32) -> Result<GslSfResult, GslError> {
    // Implementation of double factorial would go here
    unimplemented!()
}

fn gsl_sf_bessel_il_scaled_e(l: i32, x: f64) -> Result<GslSfResult, GslError> {
    // Implementation would go here
    unimplemented!()
}

fn gsl_sf_bessel_knu_scaled_asympx_e(nu: f64, x: f64) -> Result<GslSfResult, GslError> {
    // Implementation would go here
    unimplemented!()
}

fn gsl_sf_bessel_knu_scaled_asymp_unif_e(nu: f64, x: f64) -> Result<GslSfResult, GslError> {
    // Implementation would go here
    unimplemented!()
}

fn bessel_kl_scaled_small_x(l: i32, x: f64) -> Result<GslSfResult, GslError> {
    let num_fact = match gsl_sf_doublefact_e((2 * l - 1) as u32) {
        Ok(r) => r,
        Err(_) => {
            return Err(GslError::Overflow);
        }
    };

    let den = gsl_sf_pow_int(x, l + 1);
    if den == 0.0 {
        return Err(GslError::Overflow);
    }

    let lmax = 50;
    let sgn = if l & 1 != 0 { -1.0 } else { 1.0 };
    let ex = x.exp();
    let t = 0.5 * x * x;
    let mut sum = 1.0;
    let mut t_coeff = 1.0;
    let mut t_power = 1.0;

    for i in 1..lmax {
        t_coeff /= (i * (2 * (i - l) - 1) as f64;
        t_power *= t;
        let delta = t_power * t_coeff;
        sum += delta;
        if (delta / sum).abs() < 2.2204460492503131e-16 {
            break;
        }
    }

    let ipos_term = gsl_sf_bessel_il_scaled_e(l, x)?;
    let ineg_term = sgn * num_fact.val / den * sum;
    let mut result_val = -sgn * 0.5 * PI * (ex * ipos_term.val - ineg_term);
    result_val *= ex;
    let result_err = 2.0 * 2.2204460492503131e-16 * result_val.abs();

    Ok(GslSfResult {
        val: result_val,
        err: result_err,
    })
}

pub fn gsl_sf_bessel_k0_scaled_e(x: f64) -> Result<GslSfResult, GslError> {
    if x <= 0.0 {
        Err(GslError::Domain)
    } else {
        let val = PI / (2.0 * x);
        let err = 2.0 * 2.2204460492503131e-16 * val.abs();
        if val.abs() < 2.2250738585072014e-308 {
            Err(GslError::Underflow)
        } else {
            Ok(GslSfResult { val, err })
        }
    }
}

pub fn gsl_sf_bessel_k1_scaled_e(x: f64) -> Result<GslSfResult, GslError> {
    if x <= 0.0 {
        Err(GslError::Domain)
    } else if x < (1.77245385090551602729816748334 + 1.0) / (1.41421356237309504880 * 1.3407807929942596e+154) {
        Err(GslError::Overflow)
    } else {
        let val = PI / (2.0 * x) * (1.0 + 1.0 / x);
        let err = 2.0 * 2.2204460492503131e-16 * val.abs();
        if val.abs() < 2.2250738585072014e-308 {
            Err(GslError::Underflow)
        } else {
            Ok(GslSfResult { val, err })
        }
    }
}

pub fn gsl_sf_bessel_k2_scaled_e(x: f64) -> Result<GslSfResult, GslError> {
    if x <= 0.0 {
        Err(GslError::Domain)
    } else if x < 2.0 / 5.6438030941222897e+102 {
        Err(GslError::Overflow)
    } else {
        let val = PI / (2.0 * x) * (1.0 + 3.0 / x * (1.0 + 1.0 / x));
        let err = 2.0 * 2.2204460492503131e-16 * val.abs();
        if val.abs() < 2.2250738585072014e-308 {
            Err(GslError::Underflow)
        } else {
            Ok(GslSfResult { val, err })
        }
    }
}

pub fn gsl_sf_bessel_kl_scaled_e(l: i32, x: f64) -> Result<GslSfResult, GslError> {
    if l < 0 || x <= 0.0 {
        Err(GslError::Domain)
    } else {
        match l {
            0 => gsl_sf_bessel_k0_scaled_e(x),
            1 => gsl_sf_bessel_k1_scaled_e(x),
            2 => gsl_sf_bessel_k2_scaled_e(x),
            _ => {
                if x < 3.0 {
                    bessel_kl_scaled_small_x(l, x)
                } else if 6.0554544523933429e-06 * x > (l * l + l + 1) as f64 {
                    let mut result = gsl_sf_bessel_knu_scaled_asympx_e(l as f64 + 0.5, x)?;
                    let pre = (0.5 * PI / x).sqrt();
                    result.val *= pre;
                    result.err *= pre;
                    Ok(result)
                } else {
                    let threshold = (0.29 / ((l * l) as f64 + 1.0))
                        .min(0.5 / ((l * l) as f64 + 1.0 + x * x));
                    if threshold < 6.0554544523933429e-06 {
                        let mut result = gsl_sf_bessel_knu_scaled_asymp_unif_e(l as f64 + 0.5, x)?;
                        let pre = (0.5 * PI / x).sqrt();
                        result.val *= pre;
                        result.err *= pre;
                        Ok(result)
                    } else {
                        let mut r_bk = gsl_sf_bessel_k1_scaled_e(x)?;
                        let mut r_bkm = gsl_sf_bessel_k0_scaled_e(x)?;
                        let mut bk = r_bk.val;
                        let mut bkm = r_bkm.val;
                        
                        for j in 1..l {
                            let bkp = (2 * j + 1) as f64 / x * bk + bkm;
                            bkm = bk;
                            bk = bkp;
                        }
                        
                        let val = bk;
                        let err = bk.abs() * (r_bk.err / r_bk.val).abs() + (r_bkm.err / r_bkm.val).abs();
                        let err = err + 2.0 * 2.2204460492503131e-16 * val.abs();
                        
                        Ok(GslSfResult { val, err })
                    }
                }
            }
        }
    }
}

pub fn gsl_sf_bessel_k0_scaled(x: f64) -> f64 {
    gsl_sf_bessel_k0_scaled_e(x).unwrap().val
}

pub fn gsl_sf_bessel_k1_scaled(x: f64) -> f64 {
    gsl_sf_bessel_k1_scaled_e(x).unwrap().val
}

pub fn gsl_sf_bessel_k2_scaled(x: f64) -> f64 {
    gsl_sf_bessel_k2_scaled_e(x).unwrap().val
}

pub fn gsl_sf_bessel_kl_scaled(l: i32, x: f64) -> f64 {
    gsl_sf_bessel_kl_scaled_e(l, x).unwrap().val
}