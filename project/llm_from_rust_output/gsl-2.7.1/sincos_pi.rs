use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
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
    // ... other error variants
}

fn sin_pi_taylor(x: f64) -> GslSfResult {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    
    if (16.0 * x.abs()) < 1.0 {
        let y = PI * x;
        let a = y * y;
        result.val = y * (1.0 - a * (1.0 - a * (1.0 - a * (1.0 - a * (1.0 - a / 110.0) / 72.0) / 42.0) / 20.0) / 6.0);
    } else {
        result.val = (PI * x).sin();
    }
    
    result.err = 2.2204460492503131e-16 * result.val.abs();
    result
}

fn cos_pi_taylor(x: f64) -> GslSfResult {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    
    if (20.0 * x.abs()) < 1.0 {
        let y = PI * x;
        let a = y * y;
        result.val = 1.0 - 0.5 * a * (1.0 - a * (1.0 - a * (1.0 - a * (1.0 - a / 90.0) / 56.0) / 30.0) / 12.0;
    } else {
        result.val = (PI * x).cos();
    }
    
    result.err = 2.2204460492503131e-16 * result.val.abs();
    result
}

pub fn gsl_sf_sin_pi_e(x: f64) -> Result<GslSfResult, GslError> {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    let int_part = x.trunc();
    let frac_part = x.fract();
    
    if frac_part == 0.0 {
        return Ok(result);
    }
    
    if int_part.abs() >= 2.0 / 2.2204460492503131e-16 {
        return Ok(result);
    }
    
    let q = if int_part >= (i64::MIN as f64) && int_part <= (i64::MAX as f64) {
        int_part as i64
    } else {
        int_part.rem_euclid(2.0) as i64
    };
    
    let mut sign = if q % 2 != 0 { -1 } else { 1 };
    
    if frac_part.abs() == 0.5 {
        if frac_part < 0.0 {
            sign = -sign;
        }
        result.val = if sign != 1 { -1.0 } else { 1.0 };
        return Ok(result);
    }
    
    let mut adjusted_frac = frac_part;
    if frac_part.abs() > 0.5 {
        sign = -sign;
        adjusted_frac = if frac_part > 0.0 { frac_part - 1.0 } else { frac_part + 1.0 };
    }
    
    let status_result = if adjusted_frac > 0.25 {
        cos_pi_taylor(adjusted_frac - 0.5)
    } else if adjusted_frac < -0.25 {
        let res = cos_pi_taylor(adjusted_frac + 0.5);
        sign = -sign;
        res
    } else {
        sin_pi_taylor(adjusted_frac)
    };
    
    result = status_result;
    if sign != 1 {
        result.val = -result.val;
    }
    
    Ok(result)
}

pub fn gsl_sf_cos_pi_e(x: f64) -> Result<GslSfResult, GslError> {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    let int_part = x.trunc();
    let frac_part = x.fract();
    
    if frac_part.abs() == 0.5 {
        return Ok(result);
    }
    
    if int_part.abs() >= 2.0 / 2.2204460492503131e-16 {
        result.val = 1.0;
        return Ok(result);
    }
    
    let q = if int_part >= (i64::MIN as f64) && int_part <= (i64::MAX as f64) {
        int_part as i64
    } else {
        int_part.rem_euclid(2.0) as i64
    };
    
    let mut sign = if q % 2 != 0 { -1 } else { 1 };
    
    if frac_part == 0.0 {
        result.val = if sign != 1 { -1.0 } else { 1.0 };
        return Ok(result);
    }
    
    let mut adjusted_frac = frac_part;
    if frac_part.abs() > 0.5 {
        sign = -sign;
        adjusted_frac = if frac_part > 0.0 { frac_part - 1.0 } else { frac_part + 1.0 };
    }
    
    let status_result = if adjusted_frac > 0.25 {
        let res = sin_pi_taylor(adjusted_frac - 0.5);
        sign = -sign;
        res
    } else if adjusted_frac < -0.25 {
        sin_pi_taylor(adjusted_frac + 0.5)
    } else {
        cos_pi_taylor(adjusted_frac)
    };
    
    result = status_result;
    if sign != 1 {
        result.val = -result.val;
    }
    
    Ok(result)
}

pub fn gsl_sf_sin_pi(x: f64) -> f64 {
    match gsl_sf_sin_pi_e(x) {
        Ok(result) => result.val,
        Err(_) => {
            // Handle error appropriately
            0.0
        }
    }
}

pub fn gsl_sf_cos_pi(x: f64) -> f64 {
    match gsl_sf_cos_pi_e(x) {
        Ok(result) => result.val,
        Err(_) => {
            // Handle error appropriately
            1.0
        }
    }
}