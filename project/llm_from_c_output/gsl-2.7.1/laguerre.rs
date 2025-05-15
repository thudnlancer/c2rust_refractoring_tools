use std::f64::consts::PI;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

impl SfResult {
    pub fn new(val: f64, err: f64) -> Self {
        SfResult { val, err }
    }
}

pub fn gsl_sf_laguerre_1_e(a: f64, x: f64) -> SfResult {
    SfResult::new(1.0 + a - x, 2.0 * f64::EPSILON * (1.0 + a.abs() + x.abs()))
}

pub fn gsl_sf_laguerre_2_e(a: f64, x: f64) -> SfResult {
    if a == -2.0 {
        SfResult::new(0.5 * x * x, 2.0 * f64::EPSILON * (0.5 * x * x).abs())
    } else {
        let c0 = 0.5 * (2.0 + a) * (1.0 + a);
        let c1 = -(2.0 + a);
        let c2 = -0.5 / (2.0 + a);
        let val = c0 + c1 * x * (1.0 + c2 * x);
        let err = 2.0 * f64::EPSILON * (c0.abs() + 2.0 * (c1 * x).abs() * (1.0 + 2.0 * (c2 * x).abs()));
        SfResult::new(val, err + 2.0 * f64::EPSILON * val.abs())
    }
}

pub fn gsl_sf_laguerre_3_e(a: f64, x: f64) -> SfResult {
    if a == -2.0 {
        let x2_6 = x * x / 6.0;
        let val = x2_6 * (3.0 - x);
        let err = x2_6 * (3.0 + x.abs()) * 2.0 * f64::EPSILON;
        SfResult::new(val, err + 2.0 * f64::EPSILON * val.abs())
    } else if a == -3.0 {
        SfResult::new(-x * x / 6.0, 2.0 * f64::EPSILON * (x * x / 6.0).abs())
    } else {
        let c0 = (3.0 + a) * (2.0 + a) * (1.0 + a) / 6.0;
        let c1 = -c0 * 3.0 / (1.0 + a);
        let c2 = -1.0 / (2.0 + a);
        let c3 = -1.0 / (3.0 * (3.0 + a));
        let val = c0 + c1 * x * (1.0 + c2 * x * (1.0 + c3 * x));
        let mut err = 1.0 + 2.0 * (c3 * x).abs();
        err = 1.0 + 2.0 * (c2 * x).abs() * err;
        err = 2.0 * f64::EPSILON * (c0.abs() + 2.0 * (c1 * x).abs() * err);
        SfResult::new(val, err + 2.0 * f64::EPSILON * val.abs())
    }
}

fn laguerre_large_n(n: i32, alpha: f64, x: f64) -> Result<SfResult, &'static str> {
    let a = -n as f64;
    let b = alpha + 1.0;
    let eta = 2.0 * b - 4.0 * a;
    let cos2th = x / eta;
    let sin2th = 1.0 - cos2th;
    let eps = (cos2th.sqrt()).asin();
    let pre_h = 0.25 * PI * PI * eta * eta * cos2th * sin2th;

    let lg_b = ln_gamma(b + n as f64)?;
    let lnfact = ln_fact(n as u64)?;
    
    let pre_term1 = 0.5 * (1.0 - b) * (0.25 * x * eta).ln();
    let pre_term2 = 0.25 * pre_h.ln();
    let lnpre_val = lg_b.val - lnfact.val + 0.5 * x + pre_term1 - pre_term2;
    let lnpre_err = lg_b.err + lnfact.err + f64::EPSILON * (pre_term1.abs() + pre_term2.abs());

    let phi1 = 0.25 * eta * (2.0 * eps + (2.0 * eps).sin());
    let ser_term1 = -phi1.sin();

    let a1 = (1.0 / 12.0) * (5.0 / (4.0 * sin2th) + (3.0 * b * b - 6.0 * b + 2.0) * sin2th - 1.0);
    let ser_term2 = -a1 * phi1.cos() / (0.25 * eta * (2.0 * eps).sin());

    let ser_val = ser_term1 + ser_term2;
    let ser_err = ser_term2 * ser_term2 + f64::EPSILON * (ser_term1.abs() + ser_term2.abs());
    
    let result = exp_mult_err(lnpre_val, lnpre_err, ser_val, ser_err)?;
    Ok(SfResult::new(result.val, result.err + 2.0 * f64::EPSILON.sqrt() * result.val.abs()))
}

fn laguerre_n_cp(n: i32, a: f64, x: f64) -> Result<SfResult, &'static str> {
    let lnfact = ln_fact(n as u64)?;
    let (lg1, s1) = ln_gamma_sgn(a + 1.0 + n as f64)?;
    let (lg2, s2) = ln_gamma_sgn(a + 1.0)?;
    
    let mut poly_1f1_val = 1.0;
    let mut poly_1f1_err = 0.0;

    let lnpre_val = (lg1.val - lg2.val) - lnfact.val;
    let lnpre_err = lg1.err + lg2.err + lnfact.err + 2.0 * f64::EPSILON * lnpre_val.abs();

    for k in (0..n).rev() {
        let t = (-(n as f64) + k as f64) / (a + 1.0 + k as f64) * (x / (k as f64 + 1.0));
        let r = t + 1.0 / poly_1f1_val;
        if r > 0.9 * f64::MAX / poly_1f1_val {
            return Err("Overflow in laguerre_n_cp");
        } else {
            poly_1f1_val = 1.0 + t * poly_1f1_val;
            poly_1f1_err += f64::EPSILON + t.abs() * poly_1f1_err;
        }
    }

    let result = exp_mult_err(lnpre_val, lnpre_err, poly_1f1_val, poly_1f1_err)?;
    Ok(result)
}

fn laguerre_n_poly_safe(n: i32, a: f64, x: f64) -> Result<SfResult, &'static str> {
    let b = a + 1.0;
    let mx = -x;
    let tc_sgn = if x < 0.0 { 1.0 } else { if n % 2 == 1 { -1.0 } else { 1.0 } };
    let tc = taylor_coeff(n, x.abs())?;

    if tc.val.is_infinite() {
        Ok(SfResult::new(0.0, 0.0)) // FIXME: should be Inf
    } else {
        let mut term = tc.val * tc_sgn;
        let mut sum_val = term;
        let mut sum_err = tc.err;

        for k in (0..n).rev() {
            term *= ((b + k as f64) / (n as f64 - k as f64)) * (k as f64 + 1.0) / mx;
            sum_val += term;
            sum_err += 4.0 * f64::EPSILON * term.abs();
        }

        Ok(SfResult::new(sum_val, sum_err + 2.0 * f64::EPSILON * sum_val.abs()))
    }
}

pub fn gsl_sf_laguerre_n_e(n: i32, a: f64, x: f64) -> Result<SfResult, &'static str> {
    if n < 0 {
        Err("Domain error: n must be non-negative")
    } else if n == 0 {
        Ok(SfResult::new(1.0, 0.0))
    } else if n == 1 {
        Ok(gsl_sf_laguerre_1_e(a, x))
    } else if x == 0.0 {
        let mut product = a + 1.0;
        for k in 2..=n {
            product *= (a + k as f64) / k as f64;
        }
        Ok(SfResult::new(
            product,
            2.0 * (n as f64 + 1.0) * f64::EPSILON * product.abs() + f64::EPSILON,
        ))
    } else if x < 0.0 && a > -1.0 {
        laguerre_n_cp(n, a, x)
    } else if n < 5 || (x > 0.0 && a < -n as f64 - 1.0) {
        laguerre_n_cp(n, a, x).or_else(|_| laguerre_n_poly_safe(n, a, x))
    } else if n > 1_000_000 && x > 0.0 && a > -1.0 && x < 2.0 * (a + 1.0) + 4.0 * n as f64 {
        laguerre_large_n(n, a, x)
    } else if a >= 0.0 || (x > 0.0 && a < -n as f64 - 1.0) {
        let lg2 = gsl_sf_laguerre_2_e(a, x);
        let mut lkm1 = 1.0 + a - x;
        let mut lk = lg2.val;
        let mut lkp1;

        for k in 2..n {
            lkp1 = (-(k as f64 + a) * lkm1 + (2.0 * k as f64 + a + 1.0 - x) * lk) / (k as f64 + 1.0);
            lkm1 = lk;
            lk = lkp1;
        }

        Ok(SfResult::new(
            lk,
            (lg2.err / lg2.val.abs() + f64::EPSILON) * n as f64 * lk.abs(),
        ))
    } else {
        laguerre_n_poly_safe(n, a, x)
    }
}

// Helper functions (implementations omitted for brevity)
fn ln_gamma(x: f64) -> Result<SfResult, &'static str> {
    // Implementation of log gamma function
    unimplemented!()
}

fn ln_fact(n: u64) -> Result<SfResult, &'static str> {
    // Implementation of log factorial
    unimplemented!()
}

fn ln_gamma_sgn(x: f64) -> Result<(SfResult, f64), &'static str> {
    // Implementation of log gamma with sign
    unimplemented!()
}

fn exp_mult_err(ln_val: f64, ln_err: f64, val: f64, err: f64) -> Result<SfResult, &'static str> {
    // Implementation of exponential with error propagation
    unimplemented!()
}

fn taylor_coeff(n: i32, x: f64) -> Result<SfResult, &'static str> {
    // Implementation of Taylor coefficient
    unimplemented!()
}