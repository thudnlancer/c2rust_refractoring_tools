use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

fn gsl_error(reason: &str, file: &str, line: i32, gsl_errno: GslError) {
    eprintln!("GSL error: {} at {}:{} (code: {:?})", reason, file, line, gsl_errno);
}

fn bessel_yl_small_x(l: i32, x: f64) -> Result<GslSfResult, GslError> {
    let num_fact = match gsl_sf_doublefact_e((2 * l - 1) as u32) {
        Ok(res) => res,
        Err(_) => {
            return Err(GslError::Overflow);
        }
    };
    
    let den = match gsl_sf_pow_int(x, l + 1) {
        val if val == 0.0 => {
            return Err(GslError::Overflow);
        },
        val => val,
    };

    let lmax = 200;
    let t = -0.5 * x * x;
    let mut sum = 1.0;
    let mut t_coeff = 1.0;
    let mut t_power = 1.0;

    for i in 1..=lmax {
        t_coeff /= (i * (2 * (i - l) - 1)) as f64;
        t_power *= t;
        let delta = t_power * t_coeff;
        sum += delta;
        
        if (delta / sum).abs() < 0.5 * f64::EPSILON {
            break;
        }
    }

    let val = -num_fact.val / den * sum;
    Ok(GslSfResult {
        val,
        err: f64::EPSILON * val.abs(),
    })
}

pub fn gsl_sf_bessel_y0_e(x: f64) -> Result<GslSfResult, GslError> {
    if x <= 0.0 {
        gsl_error("domain error", "bessel_y.c", 82, GslError::Domain);
        return Ok(GslSfResult { val: NAN, err: NAN });
    }

    if x < f64::MIN_POSITIVE {
        gsl_error("overflow", "bessel_y.c", 85, GslError::Overflow);
        return Ok(GslSfResult { val: INFINITY, err: INFINITY });
    }

    let cos_result = gsl_sf_cos_e(x)?;
    let val = -cos_result.val / x;
    let err = cos_result.err / x.abs() + 2.0 * f64::EPSILON * val.abs();
    
    Ok(GslSfResult { val, err })
}

pub fn gsl_sf_bessel_y1_e(x: f64) -> Result<GslSfResult, GslError> {
    if x <= 0.0 {
        gsl_error("domain error", "bessel_y.c", 103, GslError::Domain);
        return Ok(GslSfResult { val: NAN, err: NAN });
    }

    if x < 1.0 / 1.3407807929942596e154 {
        gsl_error("overflow", "bessel_y.c", 106, GslError::Overflow);
        return Ok(GslSfResult { val: INFINITY, err: INFINITY });
    }

    if x < 0.25 {
        let y = x * x;
        let sum = 1.0 + y * (0.5 + y * (-0.125 + y * (1.0/144.0 + y * (-1.0/5760.0 + y * (1.0/403200.0 + y * (-1.0/43545600.0))))));
        let val = -sum / y;
        return Ok(GslSfResult {
            val,
            err: f64::EPSILON * val.abs(),
        });
    }

    let cos_result = gsl_sf_cos_e(x)?;
    let sin_result = gsl_sf_sin_e(x)?;
    let cx = cos_result.val;
    let sx = sin_result.val;
    
    let val = -(cx / x + sx) / x;
    let err = (cos_result.err / x.abs() + sin_result.err) / x.abs() 
        + f64::EPSILON * (sx.abs() / x.abs() + cx.abs() / (x * x).abs());
    
    Ok(GslSfResult { val, err })
}

pub fn gsl_sf_bessel_y2_e(x: f64) -> Result<GslSfResult, GslError> {
    if x <= 0.0 {
        gsl_error("domain error", "bessel_y.c", 141, GslError::Domain);
        return Ok(GslSfResult { val: NAN, err: NAN });
    }

    if x < 1.0 / 5.6438030941222897e102 {
        gsl_error("overflow", "bessel_y.c", 144, GslError::Overflow);
        return Ok(GslSfResult { val: INFINITY, err: INFINITY });
    }

    if x < 0.5 {
        let y = x * x;
        let sum = 1.0 + y * (1.0/6.0 + y * (1.0/24.0 + y * (-1.0/144.0 + y * (1.0/3456.0 + y * (-1.0/172800.0 + y * (1.0/14515200.0 + y * (-1.0/1828915200.0)))))));
        let val = -3.0 / (x * x * x) * sum;
        return Ok(GslSfResult {
            val,
            err: f64::EPSILON * val.abs(),
        });
    }

    let cos_result = gsl_sf_cos_e(x)?;
    let sin_result = gsl_sf_sin_e(x)?;
    let sx = sin_result.val;
    let cx = cos_result.val;
    
    let a = 3.0 / (x * x);
    let val = (1.0 - a) / x * cx - a * sx;
    let err = cos_result.err * ((1.0 - a) / x).abs() + sin_result.err * a.abs()
        + f64::EPSILON * (cx.abs() / x.abs() + sx.abs() / (x * x).abs());
    
    Ok(GslSfResult { val, err })
}

pub fn gsl_sf_bessel_yl_e(l: i32, x: f64) -> Result<GslSfResult, GslError> {
    if l < 0 || x <= 0.0 {
        gsl_error("domain error", "bessel_y.c", 181, GslError::Domain);
        return Ok(GslSfResult { val: NAN, err: NAN });
    }

    match l {
        0 => gsl_sf_bessel_y0_e(x),
        1 => gsl_sf_bessel_y1_e(x),
        2 => gsl_sf_bessel_y2_e(x),
        _ => {
            if x < 3.0 {
                bessel_yl_small_x(l, x)
            } else if 6.0554544523933429e-6 * x > (l * l + l) as f64 + 1.0 {
                let mut result = gsl_sf_bessel_Ynu_asympx_e(l as f64 + 0.5, x)?;
                let pre = (0.5 * PI / x).sqrt();
                result.val *= pre;
                result.err *= pre;
                Ok(result)
            } else if l > 40 {
                let mut result = gsl_sf_bessel_Ynu_asymp_Olver_e(l as f64 + 0.5, x)?;
                let pre = (0.5 * PI / x).sqrt();
                result.val *= pre;
                result.err *= pre;
                Ok(result)
            } else {
                let mut bym = gsl_sf_bessel_y0_e(x)?.val;
                let mut by = gsl_sf_bessel_y1_e(x)?.val;
                
                for j in 1..l {
                    let byp = (2 * j + 1) as f64 / x * by - bym;
                    bym = by;
                    by = byp;
                }
                
                Ok(GslSfResult {
                    val: by,
                    err: by.abs() * (f64::EPSILON + (by - bym).abs() / by.abs()),
                })
            }
        }
    }
}

pub fn gsl_sf_bessel_yl_array(lmax: i32, x: f64) -> Result<Vec<f64>, GslError> {
    if lmax < 0 || x <= 0.0 {
        gsl_error("error", "bessel_y.c", 237, GslError::Domain);
        return Err(GslError::Domain);
    }

    let mut result = Vec::with_capacity((lmax + 1) as usize);
    
    if lmax == 0 {
        result.push(gsl_sf_bessel_y0_e(x)?.val);
        return Ok(result);
    }

    let mut yellm1 = gsl_sf_bessel_y0_e(x)?.val;
    let mut yell = gsl_sf_bessel_y1_e(x)?.val;
    
    result.push(yellm1);
    result.push(yell);

    for ell in 1..lmax {
        let yellp1 = (2 * ell + 1) as f64 / x * yell - yellm1;
        result.push(yellp1);
        yellm1 = yell;
        yell = yellp1;
    }

    Ok(result)
}

// Placeholder implementations for GSL functions we need to wrap
fn gsl_sf_doublefact_e(n: u32) -> Result<GslSfResult, GslError> {
    // Implement double factorial function
    unimplemented!()
}

fn gsl_sf_pow_int(x: f64, n: i32) -> f64 {
    x.powi(n)
}

fn gsl_sf_cos_e(x: f64) -> Result<GslSfResult, GslError> {
    Ok(GslSfResult {
        val: x.cos(),
        err: f64::EPSILON * x.cos().abs(),
    })
}

fn gsl_sf_sin_e(x: f64) -> Result<GslSfResult, GslError> {
    Ok(GslSfResult {
        val: x.sin(),
        err: f64::EPSILON * x.sin().abs(),
    })
}

fn gsl_sf_bessel_Ynu_asympx_e(nu: f64, x: f64) -> Result<GslSfResult, GslError> {
    // Implement asymptotic expansion for Bessel Y
    unimplemented!()
}

fn gsl_sf_bessel_Ynu_asymp_Olver_e(nu: f64, x: f64) -> Result<GslSfResult, GslError> {
    // Implement Olver's asymptotic expansion for Bessel Y
    unimplemented!()
}