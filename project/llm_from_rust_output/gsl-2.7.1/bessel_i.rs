use std::f64::consts::{PI, E};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

fn bessel_il_cf1(l: i32, x: f64, threshold: f64) -> Result<(f64, GslError), GslError> {
    let kmax = 2000;
    let mut tk = 1.0;
    let mut sum = 1.0;
    let mut rhok = 0.0;

    for k in 1..=kmax {
        let ak = x / (2.0 * l as f64 + 1.0 + 2.0 * k as f64) * 
                (x / (2.0 * l as f64 + 3.0 + 2.0 * k as f64));
        rhok = -ak * (1.0 + rhok) / (1.0 + ak * (1.0 + rhok));
        tk *= rhok;
        sum += tk;
        
        if (tk / sum).abs() < threshold {
            let ratio = x / (2.0 * l as f64 + 3.0) * sum;
            return Ok((ratio, GslError::Success));
        }
    }

    Err(GslError::MaxIter)
}

pub fn gsl_sf_bessel_i0_scaled_e(x: f64) -> Result<GslSfResult, GslError> {
    let ax = x.abs();
    if x == 0.0 {
        Ok(GslSfResult { val: 1.0, err: 0.0 })
    } else if ax < 0.2 {
        let eax = (-ax).exp();
        let y = ax * ax;
        let c1 = 1.0 / 6.0;
        let c2 = 1.0 / 120.0;
        let c3 = 1.0 / 5040.0;
        let c4 = 1.0 / 362880.0;
        let c5 = 1.0 / 39916800.0;
        let sum = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5))));
        let val = eax * sum;
        Ok(GslSfResult { val, err: 2.0 * f64::EPSILON * val })
    } else if ax < -0.5 * -36.043653389117154 {
        let val = (1.0 - (-2.0 * ax).exp()) / (2.0 * ax);
        Ok(GslSfResult { val, err: 2.0 * f64::EPSILON * val })
    } else {
        let val = 1.0 / (2.0 * ax);
        Ok(GslSfResult { val, err: 2.0 * f64::EPSILON * val })
    }
}

pub fn gsl_sf_bessel_i1_scaled_e(x: f64) -> Result<GslSfResult, GslError> {
    let ax = x.abs();
    if x == 0.0 {
        Ok(GslSfResult { val: 0.0, err: 0.0 })
    } else if ax < 3.0 * f64::MIN_POSITIVE {
        Err(GslError::Underflow)
    } else if ax < 0.25 {
        let eax = (-ax).exp();
        let y = x * x;
        let c1 = 1.0 / 10.0;
        let c2 = 1.0 / 280.0;
        let c3 = 1.0 / 15120.0;
        let c4 = 1.0 / 1330560.0;
        let c5 = 1.0 / 172972800.0;
        let sum = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5))));
        let val = eax * x / 3.0 * sum;
        Ok(GslSfResult { val, err: 2.0 * f64::EPSILON * val.abs() })
    } else {
        let ex = (-2.0 * ax).exp();
        let mut val = 0.5 * (ax * (1.0 + ex) - (1.0 - ex)) / (ax * ax);
        if x < 0.0 {
            val = -val;
        }
        Ok(GslSfResult { val, err: 2.0 * f64::EPSILON * val.abs() })
    }
}

pub fn gsl_sf_bessel_i2_scaled_e(x: f64) -> Result<GslSfResult, GslError> {
    let ax = x.abs();
    if x == 0.0 {
        Ok(GslSfResult { val: 0.0, err: 0.0 })
    } else if ax < 4.0 * 1.4916681462400413e-154 {
        Err(GslError::Underflow)
    } else if ax < 0.25 {
        let y = x * x;
        let c1 = 1.0 / 14.0;
        let c2 = 1.0 / 504.0;
        let c3 = 1.0 / 33264.0;
        let c4 = 1.0 / 3459456.0;
        let c5 = 1.0 / 518918400.0;
        let sum = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5)));
        let pre = (-ax).exp() * x * x / 15.0;
        let val = pre * sum;
        Ok(GslSfResult { val, err: 2.0 * f64::EPSILON * val.abs() })
    } else {
        let ex = (-2.0 * ax).exp();
        let x2 = x * x;
        let val = 0.5 * ((3.0 + x2) * (1.0 - ex) - 3.0 * ax * (1.0 + ex)) / (ax * ax * ax);
        Ok(GslSfResult { val, err: 2.0 * f64::EPSILON * val.abs() })
    }
}

pub fn gsl_sf_bessel_il_scaled_e(l: i32, x: f64) -> Result<GslSfResult, GslError> {
    let sgn = if x < 0.0 && l & 1 != 0 { -1.0 } else { 1.0 };
    let ax = x.abs();
    
    if l < 0 {
        Err(GslError::Domain)
    } else if x == 0.0 {
        let val = if l == 0 { 1.0 } else { 0.0 };
        Ok(GslSfResult { val, err: 0.0 })
    } else if l == 0 {
        gsl_sf_bessel_i0_scaled_e(x)
    } else if l == 1 {
        gsl_sf_bessel_i1_scaled_e(x)
    } else if l == 2 {
        gsl_sf_bessel_i2_scaled_e(x)
    } else if x * x < 10.0 * (l as f64 + 1.5) / E {
        // Taylor series implementation would go here
        unimplemented!()
    } else if l < 150 {
        let i0_scaled = gsl_sf_bessel_i0_scaled_e(ax)?;
        let (rat, _) = bessel_il_cf1(l, ax, f64::EPSILON)?;
        
        let mut iellp1 = rat * 1.4916681462400413e-154;
        let mut iell = 1.4916681462400413e-154;
        let mut iellm1;
        
        for ell in (1..=l).rev() {
            iellm1 = iellp1 + (2 * ell + 1) as f64 / x * iell;
            iellp1 = iell;
            iell = iellm1;
        }
        
        let val = sgn * i0_scaled.val * (1.4916681462400413e-154 / iell);
        let err = i0_scaled.err * (1.4916681462400413e-154 / iell) + 2.0 * f64::EPSILON * val.abs();
        Ok(GslSfResult { val, err })
    } else {
        // Asymptotic uniform approximation would go here
        unimplemented!()
    }
}

// Additional helper functions and array versions would follow similar patterns