use std::f64::consts::{PI, SQRT_2};
use std::f64::{NAN, INFINITY};

#[derive(Debug, Copy, Clone)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GslError {
    Domain,
    Other(i32),
}

impl SfResult {
    pub fn new(val: f64, err: f64) -> Self {
        Self { val, err }
    }
}

fn is_neg_int(x: f64) -> bool {
    x < 0.0 && x.floor() == x
}

pub fn lnbeta_sgn_e(x: f64, y: f64) -> Result<(SfResult, f64), GslError> {
    if x == 0.0 || y == 0.0 {
        return Err(GslError::Domain);
    } else if is_neg_int(x) || is_neg_int(y) {
        return Err(GslError::Domain);
    }

    if x > 0.0 && y > 0.0 {
        let max = x.max(y);
        let min = x.min(y);
        let rat = min / max;

        if rat < 0.2 {
            let gsx = gammastar_e(x)?;
            let gsy = gammastar_e(y)?;
            let gsxy = gammastar_e(x + y)?;
            let lnopr = log_1plusx_e(rat)?;

            let lnpre_val = (gsx.val * gsy.val / gsxy.val * SQRT_2 * PI.sqrt()).ln();
            let lnpre_err = gsx.err / gsx.val + gsy.err / gsy.val + gsxy.err / gsxy.val;

            let t1 = min * rat.ln();
            let t2 = 0.5 * min.ln();
            let t3 = (x + y - 0.5) * lnopr.val;

            let lnpow_val = t1 - t2 - t3;
            let lnpow_err = f64::EPSILON * (t1.abs() + t2.abs() + t3.abs());
            let lnpow_err = lnpow_err + (x + y - 0.5).abs() * lnopr.err;

            let val = lnpre_val + lnpow_val;
            let err = lnpre_err + lnpow_err + 2.0 * f64::EPSILON * val.abs();

            return Ok((SfResult::new(val, err), 1.0));
        }
    }

    let (lgx, sgx) = lngamma_sgn_e(x)?;
    let (lgy, sgy) = lngamma_sgn_e(y)?;
    let (lgxy, sgxy) = lngamma_sgn_e(x + y)?;

    let sgn = sgx * sgy * sgxy;
    let val = lgx.val + lgy.val - lgxy.val;
    let err = lgx.err + lgy.err + lgxy.err;
    let err = err + 2.0 * f64::EPSILON * (lgx.val.abs() + lgy.val.abs() + lgxy.val.abs());
    let err = err + 2.0 * f64::EPSILON * val.abs();

    Ok((SfResult::new(val, err), sgn))
}

pub fn lnbeta_e(x: f64, y: f64) -> Result<SfResult, GslError> {
    let (result, sgn) = lnbeta_sgn_e(x, y)?;
    if sgn == -1.0 {
        Err(GslError::Domain)
    } else {
        Ok(result)
    }
}

pub fn beta_e(x: f64, y: f64) -> Result<SfResult, GslError> {
    if x > 0.0 && y > 0.0 && x < 50.0 && y < 50.0 {
        let gx = gamma_e(x)?;
        let gy = gamma_e(y)?;
        let gxy = gamma_e(x + y)?;

        let val = gx.val * gy.val / gxy.val;
        let mut err = gx.err * (gy.val / gxy.val).abs();
        err += gy.err * (gx.val / gxy.val).abs();
        err += (gx.val * gy.val / (gxy.val * gxy.val)).abs() * gxy.err;
        err += 2.0 * f64::EPSILON * val.abs();

        Ok(SfResult::new(val, err))
    } else if is_neg_int(x) || is_neg_int(y) {
        Err(GslError::Domain)
    } else if is_neg_int(x + y) {
        Ok(SfResult::new(0.0, 0.0))
    } else {
        let (lb, sgn) = lnbeta_sgn_e(x, y)?;
        let result = exp_err_e(lb.val, lb.err)?;
        Ok(SfResult::new(result.val * sgn, result.err))
    }
}

pub fn lnbeta(x: f64, y: f64) -> f64 {
    match lnbeta_e(x, y) {
        Ok(result) => result.val,
        Err(_) => NAN,
    }
}

pub fn beta(x: f64, y: f64) -> f64 {
    match beta_e(x, y) {
        Ok(result) => result.val,
        Err(_) => NAN,
    }
}

// Placeholder for GSL functions that would need to be implemented in Rust
fn gammastar_e(x: f64) -> Result<SfResult, GslError> {
    unimplemented!()
}

fn log_1plusx_e(x: f64) -> Result<SfResult, GslError> {
    unimplemented!()
}

fn lngamma_sgn_e(x: f64) -> Result<(SfResult, f64), GslError> {
    unimplemented!()
}

fn gamma_e(x: f64) -> Result<SfResult, GslError> {
    unimplemented!()
}

fn exp_err_e(x: f64, dx: f64) -> Result<SfResult, GslError> {
    unimplemented!()
}