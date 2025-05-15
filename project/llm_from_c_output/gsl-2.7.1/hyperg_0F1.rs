use std::f64::consts::{PI, EPSILON};
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

const LOC_EPS: f64 = 1000.0 * f64::EPSILON;
const LOG_DBL_MAX: f64 = f64::MAX.log2();

fn hyperg_0f1_bessel_i(nu: f64, x: f64) -> Result<SfResult, &'static str> {
    if x > LOG_DBL_MAX {
        return Err("overflow");
    }

    if nu < 0.0 {
        let anu = -nu;
        let s = 2.0 / PI * (anu * PI).sin();
        let ex = x.exp();
        
        let i_scaled = bessel_inu_scaled(anu, x)?;
        let k_scaled = bessel_knu_scaled(anu, x)?;
        
        let val = ex * i_scaled.val + s * (k_scaled.val / ex);
        let err = ex * i_scaled.err + (s * k_scaled.err / ex).abs();
        let err = err + (s * (k_scaled.val / ex)).abs() * f64::EPSILON * anu * PI;
        
        Ok(SfResult::new(val, err))
    } else {
        let ex = x.exp();
        let i_scaled = bessel_inu_scaled(nu, x)?;
        
        let val = ex * i_scaled.val;
        let err = ex * i_scaled.err + f64::EPSILON * val.abs();
        
        Ok(SfResult::new(val, err))
    }
}

fn hyperg_0f1_bessel_j(nu: f64, x: f64) -> Result<SfResult, &'static str> {
    if nu < 0.0 {
        let anu = -nu;
        let s = (anu * PI).sin();
        let c = (anu * PI).cos();
        
        let j = bessel_jnu(anu, x)?;
        let y = bessel_ynu(anu, x)?;
        
        let val = c * j.val - s * y.val;
        let err = (c * j.err).abs() + (s * y.err).abs();
        let err = err + (anu * PI).abs() * f64::EPSILON * (j.val + y.val).abs();
        
        Ok(SfResult::new(val, err))
    } else {
        bessel_jnu(nu, x)
    }
}

pub fn hyperg_0f1_e(c: f64, x: f64) -> Result<SfResult, &'static str> {
    let rintc = (c + 0.5).floor();
    let c_neg_integer = c < 0.0 && (c - rintc).abs() < LOC_EPS;

    if c == 0.0 || c_neg_integer {
        Err("domain error")
    } else if x < 0.0 {
        let (lg_c, sgn) = lngamma_sgn(c)?;
        let jcm1 = hyperg_0f1_bessel_j(c - 1.0, 2.0 * (-x).sqrt())?;
        
        if jcm1.val == 0.0 {
            Ok(SfResult::new(0.0, 0.0))
        } else {
            let tl = (-x).ln() * 0.5 * (1.0 - c);
            let ln_pre_val = lg_c.val + tl;
            let ln_pre_err = lg_c.err + 2.0 * f64::EPSILON * tl.abs();
            
            exp_mult_err(ln_pre_val, ln_pre_err, sgn * jcm1.val, jcm1.err)
        }
    } else if x == 0.0 {
        Ok(SfResult::new(1.0, 1.0))
    } else {
        let (lg_c, sgn) = lngamma_sgn(c)?;
        let icm1 = hyperg_0f1_bessel_i(c - 1.0, 2.0 * x.sqrt())?;
        
        if icm1.val == 0.0 {
            Ok(SfResult::new(0.0, 0.0))
        } else {
            let tl = x.ln() * 0.5 * (1.0 - c);
            let ln_pre_val = lg_c.val + tl;
            let ln_pre_err = lg_c.err + 2.0 * f64::EPSILON * tl.abs();
            
            exp_mult_err(ln_pre_val, ln_pre_err, sgn * icm1.val, icm1.err)
        }
    }
}

pub fn hyperg_0f1(c: f64, x: f64) -> f64 {
    match hyperg_0f1_e(c, x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

// Placeholder implementations for the required special functions
// These would need to be properly implemented or use an existing Rust crate

fn bessel_inu_scaled(nu: f64, x: f64) -> Result<SfResult, &'static str> {
    // Implementation needed
    unimplemented!()
}

fn bessel_knu_scaled(nu: f64, x: f64) -> Result<SfResult, &'static str> {
    // Implementation needed
    unimplemented!()
}

fn bessel_jnu(nu: f64, x: f64) -> Result<SfResult, &'static str> {
    // Implementation needed
    unimplemented!()
}

fn bessel_ynu(nu: f64, x: f64) -> Result<SfResult, &'static str> {
    // Implementation needed
    unimplemented!()
}

fn lngamma_sgn(x: f64) -> Result<(SfResult, f64), &'static str> {
    // Implementation needed
    unimplemented!()
}

fn exp_mult_err(ln_val: f64, ln_err: f64, val: f64, err: f64) -> Result<SfResult, &'static str> {
    // Implementation needed
    unimplemented!()
}