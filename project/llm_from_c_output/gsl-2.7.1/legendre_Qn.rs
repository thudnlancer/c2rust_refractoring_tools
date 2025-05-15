use std::f64::consts::{PI, SQRT_2};
use std::f64::{EPSILON, MIN_POSITIVE};

const GSL_DBL_EPSILON: f64 = EPSILON;
const GSL_DBL_MIN: f64 = MIN_POSITIVE;
const GSL_SQRT_DBL_MIN: f64 = MIN_POSITIVE.sqrt();
const GSL_ROOT4_DBL_EPSILON: f64 = GSL_DBL_EPSILON.sqrt().sqrt();
const GSL_ROOT6_DBL_EPSILON: f64 = GSL_DBL_EPSILON.powf(1.0/6.0);
const GSL_SQRT_DBL_MAX: f64 = f64::MAX.sqrt();

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug)]
pub enum SfError {
    DomainError,
    UnderflowError,
    MaxIteration,
    Other,
}

impl SfResult {
    fn new(val: f64, err: f64) -> Self {
        Self { val, err }
    }
}

fn legendre_q_cf1_xgt1(ell: i32, a: f64, b: f64, x: f64) -> Result<f64, SfError> {
    const MAXITER: i32 = 5000;
    let mut n = 1;
    let mut anm2 = 1.0;
    let mut bnm2 = 0.0;
    let mut anm1 = 0.0;
    let mut bnm1 = 1.0;
    let a1 = ell as f64 + 1.0 + a + b;
    let b1 = (2.0 * (ell as f64 + 1.0 + a) + 1.0) * x;
    let mut an = b1 * anm1 + a1 * anm2;
    let mut bn = b1 * bnm1 + a1 * bnm2;
    let mut fn_val = an / bn;

    while n < MAXITER {
        n += 1;
        anm2 = anm1;
        bnm2 = bnm1;
        anm1 = an;
        bnm1 = bn;
        let lna = ell as f64 + n as f64 + a;
        let an_coeff = b * b - lna * lna;
        let bn_coeff = (2.0 * lna + 1.0) * x;
        an = bn_coeff * anm1 + an_coeff * anm2;
        bn = bn_coeff * bnm1 + an_coeff * bnm2;

        if an.abs() > GSL_SQRT_DBL_MAX || bn.abs() > GSL_SQRT_DBL_MAX {
            an /= GSL_SQRT_DBL_MAX;
            bn /= GSL_SQRT_DBL_MAX;
            anm1 /= GSL_SQRT_DBL_MAX;
            bnm1 /= GSL_SQRT_DBL_MAX;
            anm2 /= GSL_SQRT_DBL_MAX;
            bnm2 /= GSL_SQRT_DBL_MAX;
        }

        let old_fn = fn_val;
        fn_val = an / bn;
        let del = old_fn / fn_val;

        if (del - 1.0).abs() < 4.0 * GSL_DBL_EPSILON {
            return Ok(fn_val);
        }
    }

    Err(SfError::MaxIteration)
}

fn legendre_ql_asymp_unif(ell: f64, x: f64) -> Result<SfResult, SfError> {
    if x < 1.0 {
        let u = ell + 0.5;
        let th = x.acos();
        let (b00, pre) = if th < GSL_ROOT4_DBL_EPSILON {
            ((1.0 + th * th / 15.0) / 24.0, 1.0 + th * th / 12.0)
        } else {
            let sin_th = (1.0 - x * x).sqrt();
            let cot_th = x / sin_th;
            ((1.0 - th * cot_th) / (8.0 * th * th), (th / sin_th).sqrt())
        };

        let y0 = bessel_y0(u * th)?;
        let y1 = bessel_y1(u * th)?;

        let sum = -0.5 * PI * (y0.val + th / u * y1.val * b00);
        let mut result = multiply(pre, sum)?;
        result.err += 0.5 * PI * pre.abs() * (y0.err + (th / u * b00).abs() * y1.err);
        result.err += GSL_DBL_EPSILON * result.val.abs();

        Ok(result)
    } else {
        let u = ell + 0.5;
        let xi = x.acosh();
        let (b00, pre) = if xi < GSL_ROOT4_DBL_EPSILON {
            ((1.0 - xi * xi / 15.0) / 24.0, 1.0 - xi * xi / 12.0)
        } else {
            let sinh_xi = (x * x - 1.0).sqrt();
            let coth_xi = x / sinh_xi;
            (-(1.0 - xi * coth_xi) / (8.0 * xi * xi), (xi / sinh_xi).sqrt())
        };

        let k0 = bessel_k0_scaled(u * xi)?;
        let k1 = bessel_k1_scaled(u * xi)?;

        let sum = k0.val - xi / u * k1.val * b00;
        let mut result = exp_mult(-u * xi, pre * sum)?;
        result.err = GSL_DBL_EPSILON * result.val.abs() * (u * xi).abs();
        result.err += 2.0 * GSL_DBL_EPSILON * result.val.abs();

        Ok(result)
    }
}

pub fn legendre_q0_e(x: f64) -> Result<SfResult, SfError> {
    if x <= -1.0 || x == 1.0 {
        Err(SfError::DomainError)
    } else if x * x < GSL_ROOT6_DBL_EPSILON {
        let y = x * x;
        let series = 1.0 + y * (1.0/3.0 + y * (1.0/5.0 + y * (1.0/7.0 + y * (1.0/9.0 + y * 1.0/11.0))));
        Ok(SfResult::new(x * series, 2.0 * GSL_DBL_EPSILON * x.abs()))
    } else if x < 1.0 {
        let val = 0.5 * ((1.0 + x) / (1.0 - x)).ln();
        Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else if x < 10.0 {
        let val = 0.5 * ((x + 1.0) / (x - 1.0)).ln();
        Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else if x * GSL_DBL_MIN < 2.0 {
        let y = 1.0 / (x * x);
        let val = (1.0 / x) * (1.0 + y * (1.0/3.0 + y * (1.0/5.0 + y * (1.0/7.0 + y * (1.0/9.0 + y * (1.0/11.0 + y * (1.0/13.0 + y * 1.0/15.0)))))));
        Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else {
        Err(SfError::UnderflowError)
    }
}

pub fn legendre_q1_e(x: f64) -> Result<SfResult, SfError> {
    if x <= -1.0 || x == 1.0 {
        Err(SfError::DomainError)
    } else if x * x < GSL_ROOT6_DBL_EPSILON {
        let y = x * x;
        let series = 1.0 + y * (1.0/3.0 + y * (1.0/5.0 + y * (1.0/7.0 + y * (1.0/9.0 + y * 1.0/11.0))));
        let val = x * x * series - 1.0;
        Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else if x < 1.0 {
        let val = 0.5 * x * ((1.0 + x) / (1.0 - x)).ln() - 1.0;
        Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else if x < 6.0 {
        let val = 0.5 * x * ((x + 1.0) / (x - 1.0)).ln() - 1.0;
        Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else if x * GSL_SQRT_DBL_MIN < 0.99 / SQRT_2 {
        let y = 1.0 / (x * x);
        let sum = 1.0 + y * (3.0/5.0 + y * (3.0/7.0 + y * (3.0/9.0 + y * (3.0/11.0 + y * (3.0/13.0 + y * (3.0/15.0 + y * (3.0/17.0 + y * 3.0/19.0)))))));
        let val = sum / (3.0 * x * x);
        Ok(SfResult::new(val, 2.0 * GSL_DBL_EPSILON * val.abs()))
    } else {
        Err(SfError::UnderflowError)
    }
}

pub fn legendre_ql_e(l: i32, x: f64) -> Result<SfResult, SfError> {
    if x <= -1.0 || x == 1.0 || l < 0 {
        Err(SfError::DomainError)
    } else if l == 0 {
        legendre_q0_e(x)
    } else if l == 1 {
        legendre_q1_e(x)
    } else if l > 100000 {
        legendre_ql_asymp_unif(l as f64, x)
    } else if x < 1.0 {
        let q0 = legendre_q0_e(x)?;
        let q1 = legendre_q1_e(x)?;
        let mut qellm1 = q0.val;
        let mut qell = q1.val;
        for ell in 1..l {
            let qellp1 = (x * (2.0 * ell as f64 + 1.0) * qell - ell as f64 * qellm1) / (ell as f64 + 1.0);
            qellm1 = qell;
            qell = qellp1;
        }
        Ok(SfResult::new(qell, GSL_DBL_EPSILON * l as f64 * qell.abs()))
    } else {
        let rat = legendre_q_cf1_xgt1(l, 0.0, 0.0, x)?;
        let mut qellp1 = rat * GSL_SQRT_DBL_MIN;
        let mut qell = GSL_SQRT_DBL_MIN;
        for ell in (1..=l).rev() {
            let qellm1 = (x * (2.0 * ell as f64 + 1.0) * qell - (ell as f64 + 1.0) * qellp1) / ell as f64;
            qellp1 = qell;
            qell = qellm1;
        }

        if qell.abs() > qellp1.abs() {
            let q0 = legendre_q0_e(x)?;
            let val = GSL_SQRT_DBL_MIN * q0.val / qell;
            Ok(SfResult::new(val, l as f64 * GSL_DBL_EPSILON * val.abs()))
        } else {
            let q1 = legendre_q1_e(x)?;
            let val = GSL_SQRT_DBL_MIN * q1.val / qellp1;
            Ok(SfResult::new(val, l as f64 * GSL_DBL_EPSILON * val.abs()))
        }
    }
}

// Placeholder implementations for Bessel functions and other utilities
fn bessel_y0(x: f64) -> Result<SfResult, SfError> {
    // Implementation would use a Bessel function library
    unimplemented!()
}

fn bessel_y1(x: f64) -> Result<SfResult, SfError> {
    // Implementation would use a Bessel function library
    unimplemented!()
}

fn bessel_k0_scaled(x: f64) -> Result<SfResult, SfError> {
    // Implementation would use a Bessel function library
    unimplemented!()
}

fn bessel_k1_scaled(x: f64) -> Result<SfResult, SfError> {
    // Implementation would use a Bessel function library
    unimplemented!()
}

fn multiply(a: f64, b: f64) -> Result<SfResult, SfError> {
    Ok(SfResult::new(a * b, 2.0 * GSL_DBL_EPSILON * (a * b).abs()))
}

fn exp_mult(x: f64, y: f64) -> Result<SfResult, SfError> {
    Ok(SfResult::new((-x).exp() * y, 2.0 * GSL_DBL_EPSILON * ((-x).exp() * y).abs()))
}