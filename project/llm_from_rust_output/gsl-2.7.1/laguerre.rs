use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

fn laguerre_large_n(n: i32, alpha: f64, x: f64) -> Result<GslSfResult, GslError> {
    let a = -f64::from(n);
    let b = alpha + 1.0;
    let eta = 2.0 * b - 4.0 * a;
    let cos2th = x / eta;
    let sin2th = 1.0 - cos2th;
    let eps = (cos2th.sqrt()).asin();
    let pre_h = 0.25 * PI * PI * eta * eta * cos2th * sin2th;

    let lg_b = gsl_sf_lngamma_e(b + f64::from(n))?;
    let lnfact = gsl_sf_lnfact_e(n as u32)?;

    let pre_term1 = 0.5 * (1.0 - b) * (0.25 * x * eta).ln();
    let pre_term2 = 0.25 * pre_h.ln();
    let lnpre_val = lg_b.val - lnfact.val + 0.5 * x + pre_term1 - pre_term2;
    let lnpre_err = lg_b.err + lnfact.err + 2.2204460492503131e-16 * (pre_term1.abs() + pre_term2.abs());

    let phi1 = 0.25 * eta * (2.0 * eps + (2.0 * eps).sin());
    let ser_term1 = -phi1.sin();
    let a1 = 1.0 / 12.0 * (5.0 / (4.0 * sin2th) + (3.0 * b * b - 6.0 * b + 2.0) * sin2th - 1.0);
    let ser_term2 = -a1 * phi1.cos() / (0.25 * eta * (2.0 * eps).sin());
    let ser_val = ser_term1 + ser_term2;
    let ser_err = ser_term2 * ser_term2 + 2.2204460492503131e-16 * (ser_term1.abs() + ser_term2.abs());

    let mut result = gsl_sf_exp_mult_err_e(lnpre_val, lnpre_err, ser_val, ser_err)?;
    result.err += 2.0 * 1.4901161193847656e-08 * result.val.abs();
    Ok(result)
}

fn laguerre_n_cp(n: i32, a: f64, x: f64) -> Result<GslSfResult, GslError> {
    let lnfact = gsl_sf_lnfact_e(n as u32)?;
    let (lg1, s1) = gsl_sf_lngamma_sgn_e(a + 1.0 + f64::from(n))?;
    let (lg2, s2) = gsl_sf_lngamma_sgn_e(a + 1.0)?;

    let mut poly_1f1_val = 1.0;
    let mut poly_1f1_err = 0.0;

    let lnpre_val = lg1.val - lg2.val - lnfact.val;
    let lnpre_err = lg1.err + lg2.err + lnfact.err + 2.0 * 2.2204460492503131e-16 * lnpre_val.abs();

    for k in (0..=n-1).rev() {
        let t = f64::from(-n + k) / (a + 1.0 + f64::from(k)) * (x / f64::from(k + 1));
        let r = t + 1.0 / poly_1f1_val;
        if r > 0.9 * f64::MAX / poly_1f1_val {
            return Err(GslError::Overflow);
        }
        poly_1f1_val = 1.0 + t * poly_1f1_val;
        poly_1f1_err += 2.2204460492503131e-16 + t.abs() * poly_1f1_err;
    }

    let mut result = gsl_sf_exp_mult_err_e(lnpre_val, lnpre_err, poly_1f1_val, poly_1f1_err)?;
    result.err += 2.0 * 2.2204460492503131e-16 * result.val.abs();
    Ok(result)
}

fn laguerre_n_poly_safe(n: i32, a: f64, x: f64) -> Result<GslSfResult, GslError> {
    let b = a + 1.0;
    let mx = -x;
    let tc_sgn = if x < 0.0 {
        1.0
    } else if n & 1 != 0 {
        -1.0
    } else {
        1.0
    };

    let tc = gsl_sf_taylorcoeff_e(n, x.abs())?;
    let mut term = tc.val * tc_sgn;
    let mut sum_val = term;
    let mut sum_err = tc.err;

    for k in (0..=n-1).rev() {
        term *= (b + f64::from(k)) / f64::from(n - k) * (f64::from(k) + 1.0) / mx;
        sum_val += term;
        sum_err += 4.0 * 2.2204460492503131e-16 * term.abs();
    }

    let mut result = GslSfResult {
        val: sum_val,
        err: sum_err + 2.0 * 2.2204460492503131e-16 * sum_val.abs(),
    };
    Ok(result)
}

pub fn gsl_sf_laguerre_1_e(a: f64, x: f64) -> GslSfResult {
    GslSfResult {
        val: 1.0 + a - x,
        err: 2.0 * 2.2204460492503131e-16 * (1.0 + a.abs() + x.abs()),
    }
}

pub fn gsl_sf_laguerre_2_e(a: f64, x: f64) -> GslSfResult {
    if a == -2.0 {
        GslSfResult {
            val: 0.5 * x * x,
            err: 2.0 * 2.2204460492503131e-16 * (0.5 * x * x).abs(),
        }
    } else {
        let c0 = 0.5 * (2.0 + a) * (1.0 + a);
        let c1 = -(2.0 + a);
        let c2 = -0.5 / (2.0 + a);
        let val = c0 + c1 * x * (1.0 + c2 * x);
        GslSfResult {
            val,
            err: 2.0 * 2.2204460492503131e-16 * (c0.abs() + 2.0 * (c1 * x).abs() * (1.0 + 2.0 * (c2 * x).abs()))
                + 2.0 * 2.2204460492503131e-16 * val.abs(),
        }
    }
}

pub fn gsl_sf_laguerre_3_e(a: f64, x: f64) -> GslSfResult {
    if a == -2.0 {
        let x2_6 = x * x / 6.0;
        let val = x2_6 * (3.0 - x);
        GslSfResult {
            val,
            err: x2_6 * (3.0 + x.abs()) * 2.0 * 2.2204460492503131e-16 + 2.0 * 2.2204460492503131e-16 * val.abs(),
        }
    } else if a == -3.0 {
        GslSfResult {
            val: -x * x / 6.0,
            err: 2.0 * 2.2204460492503131e-16 * (x * x / 6.0).abs(),
        }
    } else {
        let c0 = (3.0 + a) * (2.0 + a) * (1.0 + a) / 6.0;
        let c1 = -c0 * 3.0 / (1.0 + a);
        let c2 = -1.0 / (2.0 + a);
        let c3 = -1.0 / (3.0 * (3.0 + a));
        let val = c0 + c1 * x * (1.0 + c2 * x * (1.0 + c3 * x));
        let mut err = 1.0 + 2.0 * (c3 * x).abs();
        err = 1.0 + 2.0 * (c2 * x).abs() * err;
        err = 2.0 * 2.2204460492503131e-16 * (c0.abs() + 2.0 * (c1 * x).abs() * err);
        GslSfResult {
            val,
            err: err + 2.0 * 2.2204460492503131e-16 * val.abs(),
        }
    }
}

pub fn gsl_sf_laguerre_n_e(n: i32, a: f64, x: f64) -> Result<GslSfResult, GslError> {
    if n < 0 {
        Err(GslError::Domain)
    } else if n == 0 {
        Ok(GslSfResult { val: 1.0, err: 0.0 })
    } else if n == 1 {
        Ok(gsl_sf_laguerre_1_e(a, x))
    } else if x == 0.0 {
        let mut product = a + 1.0;
        for k in 2..=n {
            product *= (a + f64::from(k)) / f64::from(k);
        }
        Ok(GslSfResult {
            val: product,
            err: 2.0 * (f64::from(n) + 1.0) * 2.2204460492503131e-16 * product.abs() + 2.2204460492503131e-16,
        })
    } else if x < 0.0 && a > -1.0 {
        laguerre_n_cp(n, a, x)
    } else if n < 5 || (x > 0.0 && a < f64::from(-n - 1)) {
        laguerre_n_cp(n, a, x).or_else(|_| laguerre_n_poly_safe(n, a, x))
    } else if f64::from(n) > 1.0e7 && x > 0.0 && a > -1.0 && x < 2.0 * (a + 1.0) + 4.0 * f64::from(n) {
        laguerre_large_n(n, a, x)
    } else if a >= 0.0 || (x > 0.0 && a < f64::from(-n - 1)) {
        let lg2 = gsl_sf_laguerre_2_e(a, x);
        let mut lkm1 = 1.0 + a - x;
        let mut lk = lg2.val;
        let mut lkp1;
        for k in 2..n {
            lkp1 = (-(f64::from(k) + a) * lkm1 + (2.0 * f64::from(k) + a + 1.0 - x) * lk) / (f64::from(k) + 1.0);
            lkm1 = lk;
            lk = lkp1;
        }
        Ok(GslSfResult {
            val: lk,
            err: (lg2.err / lg2.val.abs() + 2.2204460492503131e-16) * f64::from(n) * lk.abs(),
        })
    } else {
        laguerre_n_poly_safe(n, a, x)
    }
}

// Helper functions (mock implementations - would need actual implementations)
fn gsl_sf_lngamma_e(x: f64) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

fn gsl_sf_lngamma_sgn_e(x: f64) -> Result<(GslSfResult, f64), GslError> {
    unimplemented!()
}

fn gsl_sf_lnfact_e(n: u32) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

fn gsl_sf_taylorcoeff_e(n: i32, x: f64) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

fn gsl_sf_exp_mult_err_e(x: f64, dx: f64, y: f64, dy: f64) -> Result<GslSfResult, GslError> {
    unimplemented!()
}