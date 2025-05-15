use std::f64;

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GslError {
    Success,
    Failure,
    Continue,
    Domain,
    Range,
    Fault,
    Invalid,
    Failed,
    Factor,
    Sanity,
    NoMem,
    BadFunc,
    Runaway,
    MaxIter,
    ZeroDiv,
    BadTol,
    Tol,
    Underflow,
    Overflow,
    Loss,
    Round,
    BadLen,
    NotSquare,
    Singular,
    Diverge,
    Unsupported,
    Unimplemented,
    Cache,
    Table,
    NoProgress,
    NoProgressJ,
    TolF,
    TolX,
    TolG,
    Eof,
}

impl GslError {
    fn to_int(&self) -> i32 {
        match self {
            GslError::Success => 0,
            GslError::Failure => -1,
            GslError::Continue => -2,
            GslError::Domain => 1,
            GslError::Range => 2,
            GslError::Fault => 3,
            GslError::Invalid => 4,
            GslError::Failed => 5,
            GslError::Factor => 6,
            GslError::Sanity => 7,
            GslError::NoMem => 8,
            GslError::BadFunc => 9,
            GslError::Runaway => 10,
            GslError::MaxIter => 11,
            GslError::ZeroDiv => 12,
            GslError::BadTol => 13,
            GslError::Tol => 14,
            GslError::Underflow => 15,
            GslError::Overflow => 16,
            GslError::Loss => 17,
            GslError::Round => 18,
            GslError::BadLen => 19,
            GslError::NotSquare => 20,
            GslError::Singular => 21,
            GslError::Diverge => 22,
            GslError::Unsupported => 23,
            GslError::Unimplemented => 24,
            GslError::Cache => 25,
            GslError::Table => 26,
            GslError::NoProgress => 27,
            GslError::NoProgressJ => 28,
            GslError::TolF => 29,
            GslError::TolX => 30,
            GslError::TolG => 31,
            GslError::Eof => 32,
        }
    }
}

pub fn gsl_sf_pow_int_e(x: f64, mut n: i32) -> Result<GslSfResult, GslError> {
    let mut value = 1.0;
    let mut count = 0;
    let mut x = x;

    if n < 0 {
        n = -n;
        if x == 0.0 {
            let u = 1.0 / x;
            let val = if n % 2 != 0 { u } else { u * u };
            return Err(GslError::Overflow);
        }
        x = 1.0 / x;
    }

    let mut m = n;
    while m != 0 {
        if m & 1 != 0 {
            value *= x;
        }
        m >>= 1;
        x *= x;
        count += 1;
    }

    Ok(GslSfResult {
        val: value,
        err: 2.0 * f64::EPSILON * (count as f64 + 1.0) * value.abs(),
    })
}

pub fn gsl_sf_pow_int(x: f64, n: i32) -> f64 {
    match gsl_sf_pow_int_e(x, n) {
        Ok(result) => result.val,
        Err(e) => {
            // In a real implementation, you might want to log this error
            match e {
                GslError::Overflow => f64::INFINITY,
                _ => f64::NAN,
            }
        }
    }
}