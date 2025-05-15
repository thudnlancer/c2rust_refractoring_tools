use std::f64::{self, NAN};

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GslError {
    Domain,
    Success,
    Other(i32),
}

impl GslError {
    fn code(&self) -> i32 {
        match self {
            GslError::Domain => 1,
            GslError::Success => 0,
            GslError::Other(code) => *code,
        }
    }
}

pub fn hyperg_2f0_e(a: f64, b: f64, x: f64) -> Result<GslSfResult, GslError> {
    if x < 0.0 {
        let pre = (-1.0 / x).powf(a);
        let U = hyperg_u_e(a, 1.0 + a - b, -1.0 / x)?;
        let val = pre * U.val;
        let err = 2.2204460492503131e-16 * val.abs() + pre * U.err;
        Ok(GslSfResult { val, err })
    } else if x == 0.0 {
        Ok(GslSfResult { val: 1.0, err: 0.0 })
    } else {
        Err(GslError::Domain)
    }
}

pub fn hyperg_2f0(a: f64, b: f64, x: f64) -> f64 {
    match hyperg_2f0_e(a, b, x) {
        Ok(result) => result.val,
        Err(e) => {
            // In a real implementation, you might want to log this error
            NAN
        }
    }
}

// Placeholder for the actual hyperg_U_e implementation
// This would need to be implemented or wrapped from the GSL library
fn hyperg_u_e(a: f64, b: f64, x: f64) -> Result<GslSfResult, GslError> {
    // This is a placeholder - in a real implementation you would:
    // 1. Call the actual GSL function through FFI (would require unsafe)
    // 2. Or reimplement the function in pure Rust
    // For now, we'll just return a dummy value
    Ok(GslSfResult { val: 0.0, err: 0.0 })
}