use std::f64::consts;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

impl SfResult {
    pub fn new(val: f64, err: f64) -> Self {
        Self { val, err }
    }
}

const GSL_DBL_EPSILON: f64 = f64::EPSILON;

fn domain_error() -> Result<SfResult, &'static str> {
    Err("Domain error")
}

fn check_underflow(result: &mut SfResult) {
    if result.val == 0.0 {
        result.err = f64::MAX;
    }
}

fn r_norm(n: i32, l: i32, z: f64) -> Result<SfResult, &'static str> {
    let a = 2.0 * z / (n as f64);
    let pre = (a.powi(3) / (2.0 * n as f64)).sqrt();
    
    let ln_a = ln_fact(n + l)?;
    let ln_b = ln_fact(n - l - 1)?;
    
    let diff_val = 0.5 * (ln_b.val - ln_a.val);
    let diff_err = 0.5 * (ln_b.err + ln_a.err) + GSL_DBL_EPSILON * diff_val.abs();
    
    let ex = exp_err(diff_val, diff_err)?;
    
    let val = pre * ex.val;
    let err = pre * ex.err + 2.0 * GSL_DBL_EPSILON * val.abs();
    
    Ok(SfResult::new(val, err))
}

fn ln_fact(n: i32) -> Result<SfResult, &'static str> {
    if n < 0 {
        return domain_error();
    }
    let mut val = 0.0;
    for i in 1..=n {
        val += (i as f64).ln();
    }
    Ok(SfResult::new(val, GSL_DBL_EPSILON * val.abs()))
}

fn exp_err(x: f64, x_err: f64) -> Result<SfResult, &'static str> {
    let val = x.exp();
    let err = val * x_err + GSL_DBL_EPSILON * val.abs();
    Ok(SfResult::new(val, err))
}

fn pow_int(x: f64, n: i32) -> f64 {
    x.powi(n)
}

fn laguerre_n(n: i32, a: f64, x: f64) -> Result<SfResult, &'static str> {
    if n < 0 || a <= -1.0 {
        return domain_error();
    }
    
    let mut l0 = 1.0;
    let mut l1 = 1.0 + a - x;
    
    if n == 0 {
        return Ok(SfResult::new(l0, GSL_DBL_EPSILON * l0.abs()));
    } else if n == 1 {
        return Ok(SfResult::new(l1, GSL_DBL_EPSILON * l1.abs()));
    }
    
    for k in 2..=n {
        let lk = ((2 * k - 1 + a - x) * l1 - (k - 1 + a) * l0) / (k as f64);
        l0 = l1;
        l1 = lk;
    }
    
    Ok(SfResult::new(l1, GSL_DBL_EPSILON * l1.abs()))
}

pub fn hydrogenic_r_1_e(z: f64, r: f64) -> Result<SfResult, &'static str> {
    if z > 0.0 && r >= 0.0 {
        let a = 2.0 * z;
        let norm = a * z.sqrt();
        let ea = (-z * r).exp();
        let val = norm * ea;
        let err = 2.0 * GSL_DBL_EPSILON * val.abs() * (z * r).abs();
        let mut result = SfResult::new(val, err);
        check_underflow(&mut result);
        Ok(result)
    } else {
        domain_error()
    }
}

pub fn hydrogenic_r_e(n: i32, l: i32, z: f64, r: f64) -> Result<SfResult, &'static str> {
    if n < 1 || l > n - 1 || z <= 0.0 || r < 0.0 {
        return domain_error();
    }
    
    let a = 2.0 * z / (n as f64);
    let norm = r_norm(n, l, z)?;
    let rho = a * r;
    let ea = (-0.5 * rho).exp();
    let pp = pow_int(rho, l);
    
    let lag = laguerre_n(n - l - 1, 2.0 * l as f64 + 1.0, rho)?;
    
    let w_val = norm.val * ea * pp;
    let mut w_err = norm.err * ea * pp;
    w_err += norm.val * ((0.5 * rho + 1.0) * GSL_DBL_EPSILON) * ea * pp;
    w_err += norm.val * ea * ((l as f64 + 1.0) * GSL_DBL_EPSILON) * pp;
    
    let val = w_val * lag.val;
    let mut err = w_val * lag.err + w_err * lag.val.abs();
    err += 2.0 * GSL_DBL_EPSILON * val.abs();
    
    let mut result = SfResult::new(val, err);
    
    if (l == 0 || (r > 0.0 && l > 0)) && lag.val != 0.0 {
        check_underflow(&mut result);
    }
    
    Ok(result)
}

pub fn hydrogenic_r_1(z: f64, r: f64) -> Result<f64, &'static str> {
    hydrogenic_r_1_e(z, r).map(|res| res.val)
}

pub fn hydrogenic_r(n: i32, l: i32, z: f64, r: f64) -> Result<f64, &'static str> {
    hydrogenic_r_e(n, l, z, r).map(|res| res.val)
}