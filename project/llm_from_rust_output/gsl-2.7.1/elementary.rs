use std::f64;

#[derive(Debug, Copy, Clone)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
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

fn gsl_coerce_double(x: f64) -> f64 {
    x
}

fn gsl_error(reason: &str, file: &str, line: i32, gsl_errno: GslError) {
    eprintln!("GSL Error: {} at {}:{} - {:?}", reason, file, line, gsl_errno);
}

fn min_dbl(a: f64, b: f64) -> f64 {
    a.min(b)
}

fn max_dbl(a: f64, b: f64) -> f64 {
    a.max(b)
}

pub fn multiply_e(x: f64, y: f64) -> Result<SfResult, GslError> {
    let ax = x.abs();
    let ay = y.abs();
    
    if x == 0.0 || y == 0.0 {
        Ok(SfResult { val: 0.0, err: 0.0 })
    } else if (ax <= 1.0 && ay >= 1.0) || (ay <= 1.0 && ax >= 1.0) {
        let val = x * y;
        Ok(SfResult {
            val,
            err: 2.0 * f64::EPSILON * val.abs(),
        })
    } else {
        let f = 1.0 - 2.0 * f64::EPSILON;
        let min = min_dbl(x.abs(), y.abs());
        let max = max_dbl(x.abs(), y.abs());
        
        if max < 0.9 * f64::MAX.sqrt() || min < f * f64::MAX / max {
            let val = gsl_coerce_double(x * y);
            let result = SfResult {
                val,
                err: 2.0 * f64::EPSILON * val.abs(),
            };
            
            if result.val.abs() < f64::MIN_POSITIVE {
                gsl_error("underflow", "elementary.c", 57, GslError::Underflow);
                Err(GslError::Underflow)
            } else {
                Ok(result)
            }
        } else {
            gsl_error("overflow", "elementary.c", 61, GslError::Overflow);
            Err(GslError::Overflow)
        }
    }
}

pub fn multiply_err_e(x: f64, dx: f64, y: f64, dy: f64) -> Result<SfResult, GslError> {
    let mut result = multiply_e(x, y)?;
    result.err += (dx * y).abs() + (dy * x).abs();
    Ok(result)
}

pub fn multiply(x: f64, y: f64) -> f64 {
    match multiply_e(x, y) {
        Ok(result) => result.val,
        Err(e) => {
            gsl_error(
                "gsl_sf_multiply_e(x, y, &result)",
                "elementary.c",
                84,
                e,
            );
            0.0
        }
    }
}