use std::f64::consts::PI;

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

const GSL_DBL_EPSILON: f64 = 2.2204460492503131e-16;
const ZETA2: f64 = PI * PI / 6.0;
const MAGIC_SPLIT_VALUE: f64 = 0.732;

fn dilog_series_1(x: f64) -> Result<GslSfResult, GslError> {
    const KMAX: i32 = 1000;
    let mut sum = x;
    let mut term = x;
    
    for k in 2..KMAX {
        let rk = (k as f64 - 1.0) / k as f64;
        term *= x * rk * rk;
        sum += term;
        
        if (term / sum).abs() < GSL_DBL_EPSILON {
            let err = 2.0 * term.abs() + 2.0 * GSL_DBL_EPSILON * sum.abs();
            return Ok(GslSfResult { val: sum, err });
        }
    }
    
    Err(GslError::MaxIter)
}

fn series_2(r: f64) -> GslSfResult {
    const KMAX: i32 = 100;
    let mut rk = r;
    let mut sum = 0.5 * r;
    
    for k in 2..10 {
        rk *= r;
        sum += rk / ((k * k) as f64 * (k as f64 + 1.0));
    }
    
    for k in 10..KMAX {
        rk *= r;
        let ds = rk / ((k * k) as f64 * (k as f64 + 1.0));
        sum += ds;
        
        if (ds / sum).abs() < 0.5 * GSL_DBL_EPSILON {
            break;
        }
    }
    
    let err = 2.0 * KMAX as f64 * GSL_DBL_EPSILON * sum.abs();
    GslSfResult { val: sum, err }
}

fn dilog_series_2(x: f64) -> Result<GslSfResult, GslError> {
    let mut result = series_2(x);
    
    let t = if x > 0.01 {
        (1.0 - x) * (1.0 - x).ln() / x
    } else {
        let t68 = 1.0/6.0 + x * (1.0/7.0 + x * 1.0/8.0);
        let t38 = 1.0/3.0 + x * (1.0/4.0 + x * (1.0/5.0 + x * t68));
        (x - 1.0) * (1.0 + x * (0.5 + x * t38))
    };
    
    result.val += 1.0 + t;
    result.err += 2.0 * GSL_DBL_EPSILON * t.abs();
    Ok(result)
}

fn dilog_xge0(x: f64) -> Result<GslSfResult, GslError> {
    if x > 2.0 {
        let ser = dilog_series_2(1.0 / x)?;
        let log_x = x.ln();
        let t1 = PI * PI / 3.0;
        let t2 = ser.val;
        let t3 = 0.5 * log_x * log_x;
        
        let val = t1 - t2 - t3;
        let err = GSL_DBL_EPSILON * log_x.abs() + ser.err;
        let err = err + GSL_DBL_EPSILON * (t1.abs() + t2.abs() + t3.abs());
        let err = err + 2.0 * GSL_DBL_EPSILON * val.abs();
        
        Ok(GslSfResult { val, err })
    } else if x > 1.01 {
        let ser = dilog_series_2(1.0 - 1.0 / x)?;
        let log_x = x.ln();
        let log_term = log_x * ((1.0 - 1.0 / x).ln() + 0.5 * log_x);
        
        let t1 = PI * PI / 6.0;
        let t2 = ser.val;
        let t3 = log_term;
        
        let val = t1 + t2 - t3;
        let err = GSL_DBL_EPSILON * log_x.abs() + ser.err;
        let err = err + GSL_DBL_EPSILON * (t1.abs() + t2.abs() + t3.abs());
        let err = err + 2.0 * GSL_DBL_EPSILON * val.abs();
        
        Ok(GslSfResult { val, err })
    } else if x > 1.0 {
        let eps = x - 1.0;
        let lne = eps.ln();
        
        let c0 = PI * PI / 6.0;
        let c1 = 1.0 - lne;
        let c2 = -(1.0 - 2.0 * lne) / 4.0;
        let c3 = (1.0 - 3.0 * lne) / 9.0;
        let c4 = -(1.0 - 4.0 * lne) / 16.0;
        let c5 = (1.0 - 5.0 * lne) / 25.0;
        let c6 = -(1.0 - 6.0 * lne) / 36.0;
        let c7 = (1.0 - 7.0 * lne) / 49.0;
        let c8 = -(1.0 - 8.0 * lne) / 64.0;
        
        let val = c0 + eps * (c1 + eps * (c2 + eps * (c3 + eps * (c4 + eps * (c5 + eps * (c6 + eps * (c7 + eps * c8)))));
        let err = 2.0 * GSL_DBL_EPSILON * val.abs();
        
        Ok(GslSfResult { val, err })
    } else if x == 1.0 {
        let val = PI * PI / 6.0;
        let err = 2.0 * GSL_DBL_EPSILON * val;
        Ok(GslSfResult { val, err })
    } else if x > 0.5 {
        let ser = dilog_series_2(1.0 - x)?;
        let log_x = x.ln();
        
        let t1 = PI * PI / 6.0;
        let t2 = ser.val;
        let t3 = log_x * (1.0 - x).ln();
        
        let val = t1 - t2 - t3;
        let err = GSL_DBL_EPSILON * log_x.abs() + ser.err;
        let err = err + GSL_DBL_EPSILON * (t1.abs() + t2.abs() + t3.abs());
        let err = err + 2.0 * GSL_DBL_EPSILON * val.abs();
        
        Ok(GslSfResult { val, err })
    } else if x > 0.25 {
        dilog_series_2(x)
    } else if x > 0.0 {
        dilog_series_1(x)
    } else {
        Ok(GslSfResult { val: 0.0, err: 0.0 })
    }
}

pub fn gsl_sf_dilog_e(x: f64) -> Result<GslSfResult, GslError> {
    if x >= 0.0 {
        dilog_xge0(x)
    } else {
        let d1 = dilog_xge0(-x)?;
        let d2 = dilog_xge0(x * x)?;
        
        let val = -d1.val + 0.5 * d2.val;
        let err = d1.err + 0.5 * d2.err;
        let err = err + 2.0 * GSL_DBL_EPSILON * val.abs();
        
        Ok(GslSfResult { val, err })
    }
}

pub fn gsl_sf_dilog(x: f64) -> f64 {
    match gsl_sf_dilog_e(x) {
        Ok(result) => result.val,
        Err(_) => {
            // Handle error appropriately
            0.0
        }
    }
}