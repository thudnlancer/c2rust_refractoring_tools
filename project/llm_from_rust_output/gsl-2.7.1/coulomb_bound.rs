use std::f64::consts;
use std::f64;

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, PartialEq)]
pub enum GslError {
    Success,
    Domain,
    Underflow,
    // Add other error variants as needed
}

fn r_norm(n: i32, l: i32, z: f64) -> Result<GslSfResult, GslError> {
    let a = 2.0 * z / n as f64;
    let pre = (a.powi(3) / (2.0 * n as f64)).sqrt();
    
    let ln_a = ln_fact((n + l) as u32)?;
    let ln_b = ln_fact((n - l - 1) as u32)?;
    
    let diff_val = 0.5 * (ln_b.val - ln_a.val);
    let diff_err = 0.5 * (ln_b.err + ln_a.err) + f64::EPSILON * diff_val.abs();
    
    let ex = exp_err(diff_val, diff_err)?;
    
    let val = pre * ex.val;
    let err = pre * ex.err + 2.0 * f64::EPSILON * val.abs();
    
    Ok(GslSfResult { val, err })
}

pub fn hydrogenic_r_1(z: f64, r: f64) -> Result<GslSfResult, GslError> {
    if z <= 0.0 || r < 0.0 {
        Err(GslError::Domain)
    } else {
        let a = 2.0 * z;
        let norm = a * z.sqrt();
        let ea = (-z * r).exp();
        
        let val = norm * ea;
        let err = 2.0 * f64::EPSILON * val.abs() * (z * r).abs();
        
        if val.abs() < f64::MIN_POSITIVE {
            Err(GslError::Underflow)
        } else {
            Ok(GslSfResult { val, err })
        }
    }
}

pub fn hydrogenic_r(n: i32, l: i32, z: f64, r: f64) -> Result<GslSfResult, GslError> {
    if n < 1 || l > n - 1 || z <= 0.0 || r < 0.0 {
        Err(GslError::Domain)
    } else {
        let a = 2.0 * z / n as f64;
        let norm = r_norm(n, l, z)?;
        let rho = a * r;
        let ea = (-0.5 * rho).exp();
        let pp = rho.powi(l);
        let lag = laguerre_n(n - l - 1, (2 * l + 1) as f64, rho)?;
        
        let w_val = norm.val * ea * pp;
        let mut w_err = norm.err * ea * pp;
        w_err += norm.val * ((0.5 * rho + 1.0) * f64::EPSILON) * ea * pp;
        w_err += norm.val * ea * ((l as f64 + 1.0) * f64::EPSILON) * pp;
        
        let val = w_val * lag.val;
        let mut err = w_val * lag.err + w_err * lag.val.abs();
        err += 2.0 * f64::EPSILON * val.abs();
        
        if (l == 0 || (r > 0.0 && l > 0)) && lag.val != 0.0 {
            if val.abs() < f64::MIN_POSITIVE {
                Err(GslError::Underflow)
            } else {
                Ok(GslSfResult { val, err })
            }
        } else {
            Ok(GslSfResult { val, err })
        }
    }
}

// Helper functions would need to be implemented:
fn ln_fact(n: u32) -> Result<GslSfResult, GslError> {
    // Implement factorial logarithm calculation
    unimplemented!()
}

fn exp_err(x: f64, dx: f64) -> Result<GslSfResult, GslError> {
    // Implement exponential with error propagation
    unimplemented!()
}

fn laguerre_n(n: i32, a: f64, x: f64) -> Result<GslSfResult, GslError> {
    // Implement Laguerre polynomial calculation
    unimplemented!()
}

// Convenience functions
pub fn hydrogenic_r_1_val(z: f64, r: f64) -> f64 {
    hydrogenic_r_1(z, r).map(|r| r.val).unwrap_or(f64::NAN)
}

pub fn hydrogenic_r_val(n: i32, l: i32, z: f64, r: f64) -> f64 {
    hydrogenic_r(n, l, z, r).map(|r| r.val).unwrap_or(f64::NAN)
}