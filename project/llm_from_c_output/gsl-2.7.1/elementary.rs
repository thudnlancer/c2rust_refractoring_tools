use std::f64;
use std::cmp::{min, max};

/// Represents the result of a special function calculation with value and error estimate
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

/// Error codes for special functions
pub mod error {
    pub const SUCCESS: i32 = 0;
    pub const OVERFLOW: i32 = 1;
}

const DBL_EPSILON: f64 = f64::EPSILON;
const DBL_MAX: f64 = f64::MAX;
const SQRT_DBL_MAX: f64 = 1.3407807929942596e154; // sqrt(f64::MAX)

/// Multiply two numbers with error estimation
pub fn multiply_e(x: f64, y: f64) -> Result<SfResult, i32> {
    let ax = x.abs();
    let ay = y.abs();

    if x == 0.0 || y == 0.0 {
        Ok(SfResult::new(0.0, 0.0))
    } else if (ax <= 1.0 && ay >= 1.0) || (ay <= 1.0 && ax >= 1.0) {
        let val = x * y;
        let err = 2.0 * DBL_EPSILON * val.abs();
        Ok(SfResult::new(val, err))
    } else {
        let f = 1.0 - 2.0 * DBL_EPSILON;
        let min = min(ax, ay);
        let max = max(ax, ay);
        
        if max < 0.9 * SQRT_DBL_MAX || min < (f * DBL_MAX) / max {
            let val = x * y;
            let err = 2.0 * DBL_EPSILON * val.abs();
            let result = SfResult::new(val, err);
            check_underflow(&result)?;
            Ok(result)
        } else {
            Err(error::OVERFLOW)
        }
    }
}

/// Multiply two numbers with separate error terms
pub fn multiply_err_e(x: f64, dx: f64, y: f64, dy: f64) -> Result<SfResult, i32> {
    let mut result = multiply_e(x, y)?;
    result.err += (dx * y).abs() + (dy * x).abs();
    Ok(result)
}

/// Simple multiplication wrapper
pub fn multiply(x: f64, y: f64) -> f64 {
    match multiply_e(x, y) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

/// Check for underflow in result
fn check_underflow(result: &SfResult) -> Result<(), i32> {
    if result.val.abs() < f64::MIN_POSITIVE {
        Err(error::UNDERFLOW)
    } else {
        Ok(())
    }
}