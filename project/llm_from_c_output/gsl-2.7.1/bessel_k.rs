use std::f64::consts::{PI, SQRT_2};
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

pub const DBL_EPSILON: f64 = f64::EPSILON;
pub const DBL_MAX: f64 = f64::MAX;
pub const GSL_EDOM: i32 = 1;
pub const GSL_SUCCESS: i32 = 0;

fn is_odd(n: i32) -> bool {
    n % 2 != 0
}

fn pow_int(x: f64, n: i32) -> f64 {
    x.powi(n)
}

fn doublefact_e(n: u32) -> Result<f64, i32> {
    if n == 0 || n == 1 {
        return Ok(1.0);
    }
    let mut result = 1.0;
    for i in (n % 2 + 1..=n).step_by(2) {
        result *= i as f64;
        if !result.is_finite() {
            return Err(GSL_EDOM);
        }
    }
    Ok(result)
}

fn bessel_il_scaled_e(l: i32, x: f64) -> Result<SfResult, i32> {
    // Placeholder implementation - would need actual Bessel I function
    Err(GSL_EDOM)
}

fn bessel_kl_scaled_small_x(l: i32, x: f64) -> Result<SfResult, i32> {
    let den = pow_int(x, l + 1);
    let num_fact = match doublefact_e((2 * l - 1) as u32) {
        Ok(val) => val,
        Err(_) => return Err(GSL_EDOM),
    };

    if den == 0.0 {
        return Err(GSL_EDOM);
    }

    let lmax = 50;
    let sgn = if is_odd(l) { -1.0 } else { 1.0 };
    let ex = x.exp();
    let t = 0.5 * x * x;
    let mut sum = 1.0;
    let mut t_coeff = 1.0;
    let mut t_power = 1.0;

    for i in 1..lmax {
        t_coeff /= (i as f64) * (2.0 * (i as f64 - l as f64) - 1.0);
        t_power *= t;
        let delta = t_power * t_coeff;
        sum += delta;
        if (delta / sum).abs() < DBL_EPSILON {
            break;
        }
    }

    let ipos_term = match bessel_il_scaled_e(l, x) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let ineg_term = sgn * num_fact / den * sum;
    let mut result_val = -sgn * 0.5 * PI * (ex * ipos_term.val - ineg_term);
    result_val *= ex;
    Ok(SfResult::new(result_val, 2.0 * DBL_EPSILON * result_val.abs()))
}

pub fn bessel_k0_scaled_e(x: f64) -> Result<SfResult, i32> {
    if x <= 0.0 {
        Err(GSL_EDOM)
    } else {
        let val = PI / (2.0 * x);
        Ok(SfResult::new(val, 2.0 * DBL_EPSILON * val.abs()))
    }
}

pub fn bessel_k1_scaled_e(x: f64) -> Result<SfResult, i32> {
    if x <= 0.0 {
        Err(GSL_EDOM)
    } else if x < (PI.sqrt() + 1.0) / (SQRT_2 * DBL_MAX.sqrt()) {
        Err(GSL_EDOM)
    } else {
        let val = PI / (2.0 * x) * (1.0 + 1.0 / x);
        Ok(SfResult::new(val, 2.0 * DBL_EPSILON * val.abs()))
    }
}

pub fn bessel_k2_scaled_e(x: f64) -> Result<SfResult, i32> {
    if x <= 0.0 {
        Err(GSL_EDOM)
    } else if x < 2.0 / DBL_MAX.cbrt() {
        Err(GSL_EDOM)
    } else {
        let val = PI / (2.0 * x) * (1.0 + 3.0 / x * (1.0 + 1.0 / x));
        Ok(SfResult::new(val, 2.0 * DBL_EPSILON * val.abs()))
    }
}

pub fn bessel_kl_scaled_e(l: i32, x: f64) -> Result<SfResult, i32> {
    if l < 0 || x <= 0.0 {
        Err(GSL_EDOM)
    } else if l == 0 {
        bessel_k0_scaled_e(x)
    } else if l == 1 {
        bessel_k1_scaled_e(x)
    } else if l == 2 {
        bessel_k2_scaled_e(x)
    } else if x < 3.0 {
        bessel_kl_scaled_small_x(l, x)
    } else {
        // Placeholder for other cases - would need additional Bessel functions
        Err(GSL_EDOM)
    }
}

pub fn bessel_kl_scaled_array(lmax: i32, x: f64) -> Result<Vec<f64>, i32> {
    if lmax < 0 || x <= 0.0 {
        Err(GSL_EDOM)
    } else if lmax == 0 {
        let result = bessel_k0_scaled_e(x)?;
        Ok(vec![result.val])
    } else {
        let mut result_array = Vec::with_capacity((lmax + 1) as usize);
        let r_kellm1 = bessel_k0_scaled_e(x)?;
        let r_kell = bessel_k1_scaled_e(x)?;
        let mut kell = r_kell.val;
        let mut kellm1 = r_kellm1.val;
        result_array.push(kellm1);
        result_array.push(kell);

        for ell in 1..lmax {
            let kellp1 = (2 * ell + 1) as f64 / x * kell + kellm1;
            result_array.push(kellp1);
            kellm1 = kell;
            kell = kellp1;
        }

        Ok(result_array)
    }
}

pub fn bessel_k0_scaled(x: f64) -> f64 {
    bessel_k0_scaled_e(x).unwrap().val
}

pub fn bessel_k1_scaled(x: f64) -> f64 {
    bessel_k1_scaled_e(x).unwrap().val
}

pub fn bessel_k2_scaled(x: f64) -> f64 {
    bessel_k2_scaled_e(x).unwrap().val
}

pub fn bessel_kl_scaled(l: i32, x: f64) -> f64 {
    bessel_kl_scaled_e(l, x).unwrap().val
}