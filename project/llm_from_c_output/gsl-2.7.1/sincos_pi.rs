use std::f64::consts::PI;

/// Any double precision number bigger than this is automatically an even integer.
const TWOBIG: f64 = 2.0 / f64::EPSILON;

/// Result type for special function calculations
#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

impl SfResult {
    pub fn new() -> Self {
        SfResult { val: 0.0, err: 0.0 }
    }
}

/// Routine computing sin(pi*x) using a Taylor expansion around the origin and otherwise the library function.
fn sin_pi_taylor(x: f64) -> SfResult {
    let mut result = SfResult::new();
    
    if 16.0 * x.abs() < 1.0 {
        let y = PI * x;
        let a = y * y;
        result.val = y * (1.0 - a * (1.0 - a * (1.0 - a * (1.0 - a * (1.0 - a / 110.0) / 72.0) / 42.0) / 20.0) / 6.0);
    } else {
        result.val = (PI * x).sin();
    }
    
    result.err = f64::EPSILON * result.val.abs();
    result
}

/// Routine computing cos(pi*x) using a Taylor expansion around the origin and otherwise the library function.
fn cos_pi_taylor(x: f64) -> SfResult {
    let mut result = SfResult::new();
    
    if 20.0 * x.abs() < 1.0 {
        let y = PI * x;
        let a = y * y;
        result.val = 1.0 - 0.5 * a * (1.0 - a * (1.0 - a * (1.0 - a * (1.0 - a / 90.0) / 56.0) / 30.0) / 12.0);
    } else {
        result.val = (PI * x).cos();
    }
    
    result.err = f64::EPSILON * result.val.abs();
    result
}

/// Compute sin(πx) with error handling
pub fn gsl_sf_sin_pi_e(x: f64) -> Result<SfResult, ()> {
    let mut result = SfResult::new();
    let (fracx, intx) = x.modf();
    
    if fracx == 0.0 {
        return Ok(result);
    }
    if intx.abs() >= TWOBIG {
        return Ok(result);
    }

    let q = if intx >= i64::MIN as f64 && intx <= i64::MAX as f64 {
        intx as i64
    } else {
        intx.rem_euclid(2.0) as i64
    };
    let mut sign = if q % 2 != 0 { -1 } else { 1 };

    if fracx.abs() == 0.5 {
        if fracx < 0.0 {
            sign = -sign;
        }
        result.val = if sign != 1 { -1.0 } else { 1.0 };
        return Ok(result);
    }

    let mut fracx = fracx;
    if fracx.abs() > 0.5 {
        sign = -sign;
        fracx = if fracx > 0.0 { fracx - 1.0 } else { fracx + 1.0 };
    }

    let status = if fracx > 0.25 {
        result = cos_pi_taylor(fracx - 0.5);
        Ok(())
    } else if fracx < -0.25 {
        result = cos_pi_taylor(fracx + 0.5);
        sign = -sign;
        Ok(())
    } else {
        result = sin_pi_taylor(fracx);
        Ok(())
    };

    if sign != 1 {
        result.val = -result.val;
    }
    status.map(|_| result)
}

/// Compute cos(πx) with error handling
pub fn gsl_sf_cos_pi_e(x: f64) -> Result<SfResult, ()> {
    let mut result = SfResult::new();
    let (fracx, intx) = x.modf();
    
    if fracx.abs() == 0.5 {
        return Ok(result);
    }
    if intx.abs() >= TWOBIG {
        result.val = 1.0;
        return Ok(result);
    }

    let q = if intx >= i64::MIN as f64 && intx <= i64::MAX as f64 {
        intx as i64
    } else {
        intx.rem_euclid(2.0) as i64
    };
    let mut sign = if q % 2 != 0 { -1 } else { 1 };

    if fracx == 0.0 {
        result.val = if sign != 1 { -1.0 } else { 1.0 };
        return Ok(result);
    }

    let mut fracx = fracx;
    if fracx.abs() > 0.5 {
        sign = -sign;
        fracx = if fracx > 0.0 { fracx - 1.0 } else { fracx + 1.0 };
    }

    let status = if fracx > 0.25 {
        result = sin_pi_taylor(fracx - 0.5);
        sign = -sign;
        Ok(())
    } else if fracx < -0.25 {
        result = sin_pi_taylor(fracx + 0.5);
        Ok(())
    } else {
        result = cos_pi_taylor(fracx);
        Ok(())
    };

    if sign != 1 {
        result.val = -result.val;
    }
    status.map(|_| result)
}

/// Compute sin(πx) - simple version
pub fn gsl_sf_sin_pi(x: f64) -> f64 {
    gsl_sf_sin_pi_e(x).unwrap().val
}

/// Compute cos(πx) - simple version
pub fn gsl_sf_cos_pi(x: f64) -> f64 {
    gsl_sf_cos_pi_e(x).unwrap().val
}