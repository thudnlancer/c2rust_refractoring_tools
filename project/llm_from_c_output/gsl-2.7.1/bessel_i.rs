use std::f64::consts::{E, PI};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

const GSL_DBL_EPSILON: f64 = f64::EPSILON;
const GSL_SQRT_DBL_MIN: f64 = f64::MIN.sqrt();
const GSL_LOG_DBL_EPSILON: f64 = f64::EPSILON.ln();
const GSL_ROOT3_DBL_EPSILON: f64 = f64::EPSILON.cbrt();
const GSL_ROOT6_DBL_EPSILON: f64 = f64::EPSILON.powf(1.0 / 6.0);
const GSL_DBL_MIN: f64 = f64::MIN;

#[derive(Debug)]
pub enum SfError {
    DomainError,
    UnderflowError,
    MaxIteration,
    Other(&'static str),
}

type SfResultT<T> = Result<T, SfError>;

fn bessel_il_cf1(l: i32, x: f64, threshold: f64) -> SfResultT<f64> {
    const KMAX: i32 = 2000;
    let mut tk = 1.0;
    let mut sum = 1.0;
    let mut rhok = 0.0;

    for k in 1..=KMAX {
        let ak = (x / (2.0 * l as f64 + 1.0 + 2.0 * k as f64)) * (x / (2.0 * l as f64 + 3.0 + 2.0 * k as f64));
        rhok = -ak * (1.0 + rhok) / (1.0 + ak * (1.0 + rhok));
        tk *= rhok;
        sum += tk;
        if (tk / sum).abs() < threshold {
            return Ok(x / (2.0 * l as f64 + 3.0) * sum);
        }
    }

    Err(SfError::MaxIteration)
}

pub fn bessel_i0_scaled_e(x: f64) -> SfResultT<SfResult> {
    let ax = x.abs();
    let mut result = SfResult { val: 0.0, err: 0.0 };

    if x == 0.0 {
        result.val = 1.0;
        result.err = 0.0;
        Ok(result)
    } else if ax < 0.2 {
        let eax = (-ax).exp();
        let y = ax * ax;
        let c1 = 1.0 / 6.0;
        let c2 = 1.0 / 120.0;
        let c3 = 1.0 / 5040.0;
        let c4 = 1.0 / 362880.0;
        let c5 = 1.0 / 39916800.0;
        let sum = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5))));
        result.val = eax * sum;
        result.err = 2.0 * GSL_DBL_EPSILON * result.val;
        Ok(result)
    } else if ax < -0.5 * GSL_LOG_DBL_EPSILON {
        result.val = (1.0 - (-2.0 * ax).exp()) / (2.0 * ax);
        result.err = 2.0 * GSL_DBL_EPSILON * result.val;
        Ok(result)
    } else {
        result.val = 1.0 / (2.0 * ax);
        result.err = 2.0 * GSL_DBL_EPSILON * result.val;
        Ok(result)
    }
}

pub fn bessel_i1_scaled_e(x: f64) -> SfResultT<SfResult> {
    let ax = x.abs();
    let mut result = SfResult { val: 0.0, err: 0.0 };

    if x == 0.0 {
        result.val = 0.0;
        result.err = 0.0;
        Ok(result)
    } else if ax < 3.0 * GSL_DBL_MIN {
        Err(SfError::UnderflowError)
    } else if ax < 0.25 {
        let eax = (-ax).exp();
        let y = x * x;
        let c1 = 1.0 / 10.0;
        let c2 = 1.0 / 280.0;
        let c3 = 1.0 / 15120.0;
        let c4 = 1.0 / 1330560.0;
        let c5 = 1.0 / 172972800.0;
        let sum = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5))));
        result.val = eax * x / 3.0 * sum;
        result.err = 2.0 * GSL_DBL_EPSILON * result.val.abs();
        Ok(result)
    } else {
        let ex = (-2.0 * ax).exp();
        result.val = 0.5 * (ax * (1.0 + ex) - (1.0 - ex)) / (ax * ax);
        result.err = 2.0 * GSL_DBL_EPSILON * result.val.abs();
        if x < 0.0 {
            result.val = -result.val;
        }
        Ok(result)
    }
}

pub fn bessel_i2_scaled_e(x: f64) -> SfResultT<SfResult> {
    let ax = x.abs();
    let mut result = SfResult { val: 0.0, err: 0.0 };

    if x == 0.0 {
        result.val = 0.0;
        result.err = 0.0;
        Ok(result)
    } else if ax < 4.0 * GSL_SQRT_DBL_MIN {
        Err(SfError::UnderflowError)
    } else if ax < 0.25 {
        let y = x * x;
        let c1 = 1.0 / 14.0;
        let c2 = 1.0 / 504.0;
        let c3 = 1.0 / 33264.0;
        let c4 = 1.0 / 3459456.0;
        let c5 = 1.0 / 518918400.0;
        let sum = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5))));
        let pre = (-ax).exp() * x * x / 15.0;
        result.val = pre * sum;
        result.err = 2.0 * GSL_DBL_EPSILON * result.val.abs();
        Ok(result)
    } else {
        let ex = (-2.0 * ax).exp();
        let x2 = x * x;
        result.val = 0.5 * ((3.0 + x2) * (1.0 - ex) - 3.0 * ax * (1.0 + ex)) / (ax * ax * ax);
        result.err = 2.0 * GSL_DBL_EPSILON * result.val.abs();
        Ok(result)
    }
}

pub fn bessel_il_scaled_e(l: i32, x: f64) -> SfResultT<SfResult> {
    let mut sgn = 1.0;
    let mut ax = x.abs();

    if x < 0.0 {
        sgn = if l % 2 == 1 { -1.0 } else { 1.0 };
        ax = -x;
    }

    if l < 0 {
        Err(SfError::DomainError)
    } else if x == 0.0 {
        let mut result = SfResult { val: 0.0, err: 0.0 };
        result.val = if l == 0 { 1.0 } else { 0.0 };
        Ok(result)
    } else if l == 0 {
        let il = bessel_i0_scaled_e(x)?;
        Ok(SfResult {
            val: sgn * il.val,
            err: il.err,
        })
    } else if l == 1 {
        let il = bessel_i1_scaled_e(x)?;
        Ok(SfResult {
            val: sgn * il.val,
            err: il.err,
        })
    } else if l == 2 {
        let il = bessel_i2_scaled_e(x)?;
        Ok(SfResult {
            val: sgn * il.val,
            err: il.err,
        })
    } else if x * x < 10.0 * (l as f64 + 1.5) / E {
        // Note: bessel_IJ_taylor_e implementation is required here
        unimplemented!("bessel_IJ_taylor_e not implemented");
    } else if l < 150 {
        let i0_scaled = bessel_i0_scaled_e(ax)?;
        let rat = bessel_il_cf1(l, ax, GSL_DBL_EPSILON)?;
        let mut iellp1 = rat * GSL_SQRT_DBL_MIN;
        let mut iell = GSL_SQRT_DBL_MIN;
        let mut iellm1;
        for ell in (1..=l).rev() {
            iellm1 = iellp1 + (2 * ell + 1) as f64 / x * iell;
            iellp1 = iell;
            iell = iellm1;
        }
        let val = sgn * i0_scaled.val * (GSL_SQRT_DBL_MIN / iell);
        let err = i0_scaled.err * (GSL_SQRT_DBL_MIN / iell) + 2.0 * GSL_DBL_EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else if f64::min(0.29 / (l as f64 * l as f64 + 1.0), 0.5 / (l as f64 * l as f64 + 1.0 + x * x)) < 0.5 * GSL_ROOT3_DBL_EPSILON {
        // Note: bessel_Inu_scaled_asymp_unif_e implementation is required here
        unimplemented!("bessel_Inu_scaled_asymp_unif_e not implemented");
    } else {
        // Note: bessel_Inu_scaled_asymp_unif_e implementation is required here
        unimplemented!("bessel_Inu_scaled_asymp_unif_e not implemented");
    }
}

pub fn bessel_il_scaled_array(lmax: i32, x: f64) -> SfResultT<Vec<f64>> {
    let mut result_array = vec![0.0; (lmax + 1) as usize];

    if x == 0.0 {
        result_array[0] = 1.0;
        for ell in 1..=lmax as usize {
            result_array[ell] = 0.0;
        }
        Ok(result_array)
    } else {
        let r_iellp1 = bessel_il_scaled_e(lmax + 1, x)?;
        let r_iell = bessel_il_scaled_e(lmax, x)?;
        let mut iellp1 = r_iellp1.val;
        let mut iell = r_iell.val;
        let mut iellm1;

        result_array[lmax as usize] = iell;
        for ell in (1..=lmax).rev() {
            iellm1 = iellp1 + (2 * ell + 1) as f64 / x * iell;
            iellp1 = iell;
            iell = iellm1;
            result_array[(ell - 1) as usize] = iellm1;
        }
        Ok(result_array)
    }
}

// Helper functions for direct evaluation (without error handling)
pub fn bessel_i0_scaled(x: f64) -> f64 {
    bessel_i0_scaled_e(x).unwrap().val
}

pub fn bessel_i1_scaled(x: f64) -> f64 {
    bessel_i1_scaled_e(x).unwrap().val
}

pub fn bessel_i2_scaled(x: f64) -> f64 {
    bessel_i2_scaled_e(x).unwrap().val
}

pub fn bessel_il_scaled(l: i32, x: f64) -> f64 {
    bessel_il_scaled_e(l, x).unwrap().val
}