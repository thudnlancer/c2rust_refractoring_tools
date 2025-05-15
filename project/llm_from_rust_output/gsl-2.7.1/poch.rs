use std::f64::consts::PI;
use std::f64::{INFINITY, NAN};

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

const BERNOULLI: [f64; 21] = [
    0.0,
    0.083333333333333333333333333333333,
    -0.001388888888888888888888888888888,
    0.000033068783068783068783068783068,
    -0.000000826719576719576719576719576,
    0.000000020876756987868098979210903,
    -0.000000000528419013868749318484768,
    0.000000000013382536530684678832680,
    -0.000000000000338968029632258286683,
    0.000000000000008586062056277844564,
    -0.000000000000000217486869855806187,
    0.000000000000000005509002828360229,
    -0.000000000000000000139544646858125,
    0.000000000000000000003534707039629,
    -0.000000000000000000000089535174270,
    0.000000000000000000000002267952452,
    -0.000000000000000000000000057447243,
    0.000000000000000000000000001455172,
    -0.000000000000000000000000000036859,
    0.000000000000000000000000000000933,
    -0.000000000000000000000000000000023,
];

const SQTBIG: f64 = 1.0 / (2.0 * 1.41421356237309504880 * 1.73205080756887729352744634151 * 1.4916681462400413e-154);
const ALNEPS: f64 = -36.043653389117154 - 0.69314718055994530942;

fn gsl_max(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn pochrel_smallx(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    if x == 0.0 {
        return gsl_sf_psi_e(a);
    }

    let bp = if a < -0.5 { 1.0 - a - x } else { a };
    let incr = if bp < 10.0 { (11.0 - bp) as i32 } else { 0 };
    let b = bp + incr as f64;

    let var = b + 0.5 * (x - 1.0);
    let alnvar = var.ln();
    let q = x * alnvar;
    let mut poly1 = 0.0;

    if var < SQTBIG {
        let nterms = (-0.5 * ALNEPS / alnvar + 1.0) as i32;
        let var2 = 1.0 / (var * var);
        let rho = 0.5 * (x + 1.0);
        let mut term = var2;
        let mut gbern = [0.0; 24];
        gbern[1] = 1.0;
        gbern[2] = -rho / 12.0;
        poly1 = gbern[2] * term;

        if nterms > 20 {
            return Err(GslError::Sanity);
        }

        for k in 2..=nterms {
            let mut gbk = 0.0;
            for j in 1..=k {
                gbk += BERNOULLI[(k - j + 1) as usize] * gbern[j as usize];
            }
            gbern[(k + 1) as usize] = -rho * gbk / k as f64;
            term *= ((2 * k - 2) as f64 - x) * ((2 * k - 1) as f64 - x) * var2;
            poly1 += gbern[(k + 1) as usize] * term;
        }
    }

    let dexprl = gsl_sf_expm1_e(q)?;
    let dexprl_val = dexprl.val / q;
    poly1 *= x - 1.0;
    let mut dpoch1 = dexprl_val * (alnvar + q * poly1) + poly1;

    for i in (0..incr).rev() {
        let binv = 1.0 / (bp + i as f64);
        dpoch1 = (dpoch1 - binv) / (1.0 + x * binv);
    }

    if bp == a {
        Ok(GslSfResult {
            val: dpoch1,
            err: 2.0 * f64::EPSILON * (incr as f64 + 1.0).abs() * dpoch1.abs(),
        })
    } else {
        let sinpxx = (PI * x).sin() / x;
        let sinpx2 = (0.5 * PI * x).sin();
        let t1 = sinpxx / (PI * b).tan();
        let t2 = 2.0 * sinpx2 * (sinpx2 / x);
        let trig = t1 - t2;
        let val = dpoch1 * (1.0 + x * trig) + trig;
        let err = (dpoch1.abs() * x.abs() + 1.0) * f64::EPSILON * (t1.abs() + t2.abs())
            + 2.0 * f64::EPSILON * (incr as f64 + 1.0).abs() * val.abs();
        Ok(GslSfResult { val, err })
    }
}

fn lnpoch_pos(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    let absx = x.abs();
    if absx > 0.1 * a || absx * gsl_max(a, 2.0).ln() > 0.1 {
        if a < 171.0 && a + x < 171.0 {
            let g1 = gsl_sf_gammainv_e(a)?;
            let g2 = gsl_sf_gammainv_e(a + x)?;
            let val = -(g2.val / g1.val).ln();
            let err = g1.err / g1.val.abs() + g2.err / g2.val.abs() + 2.0 * f64::EPSILON * val.abs();
            Ok(GslSfResult { val, err })
        } else {
            let lg1 = gsl_sf_lngamma_e(a)?;
            let lg2 = gsl_sf_lngamma_e(a + x)?;
            let val = lg2.val - lg1.val;
            let err = lg2.err + lg1.err + 2.0 * f64::EPSILON * val.abs();
            Ok(GslSfResult { val, err })
        }
    } else if absx < 0.1 * a && a > 15.0 {
        let eps = x / a;
        let den = 1.0 + eps;
        let d3 = den * den * den;
        let d5 = d3 * den * den;
        let d7 = d5 * den * den;
        let c1 = -eps / den;
        let c3 = -eps * (3.0 + eps * (3.0 + eps)) / d3;
        let c5 = -eps * (5.0 + eps * (10.0 + eps * (10.0 + eps * (5.0 + eps)))) / d5;
        let c7 = -eps * (7.0 + eps * (21.0 + eps * (35.0 + eps * (35.0 + eps * (21.0 + eps * (7.0 + eps)))))) / d7;
        let p8 = (1.0 + eps).powi(8);
        let c8 = 1.0 / p8 - 1.0;
        let c9 = 1.0 / (p8 * (1.0 + eps)) - 1.0;
        let a4 = a * a * a * a;
        let a6 = a4 * a * a;
        let ser_1 = c1 + c3 / (30.0 * a * a) + c5 / (105.0 * a4) + c7 / (140.0 * a6);
        let ser_2 = c8 / (99.0 * a6 * a * a) - 691.0 / 360360.0 * c9 / (a6 * a4);
        let ser = (ser_1 + ser_2) / (12.0 * a);
        let term1 = x * (a / 2.7182818284590452354).ln();
        let ln_1peps = gsl_sf_log_1plusx_e(eps)?;
        let term2 = (x + a - 0.5) * ln_1peps.val;
        let val = term1 + term2 + ser;
        let err = f64::EPSILON * term1.abs()
            + (x + a - 0.5).abs() * ln_1peps.err
            + ln_1peps.val.abs() * f64::EPSILON * (x.abs() + a.abs() + 0.5)
            + 2.0 * f64::EPSILON * val.abs();
        Ok(GslSfResult { val, err })
    } else {
        let poch_rel = pochrel_smallx(a, x)?;
        let eps_0 = x * poch_rel.val;
        let result = gsl_sf_log_1plusx_e(eps_0)?;
        let err = 2.0 * (x * poch_rel.err / (1.0 + eps_0)).abs() + 2.0 * f64::EPSILON * result.val.abs();
        Ok(GslSfResult { val: result.val, err })
    }
}

pub fn gsl_sf_lnpoch_e(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    if a <= 0.0 || a + x <= 0.0 {
        Err(GslError::Domain)
    } else if x == 0.0 {
        Ok(GslSfResult { val: 0.0, err: 0.0 })
    } else {
        lnpoch_pos(a, x)
    }
}

pub fn gsl_sf_lnpoch_sgn_e(a: f64, x: f64) -> Result<(GslSfResult, f64), GslError> {
    if x == 0.0 {
        Ok((GslSfResult { val: 0.0, err: 0.0 }, 1.0))
    } else if a > 0.0 && a + x > 0.0 {
        Ok((lnpoch_pos(a, x)?, 1.0))
    } else if a <= 0.0 && a == a.floor() {
        if a + x < 0.0 && x == x.floor() {
            let result_pos = lnpoch_pos(-a, -x)?;
            let f = (a / (a + x)).ln();
            let s = if x % 2.0 == 0.0 { 1.0 } else { -1.0 };
            let val = f - result_pos.val;
            let err = result_pos.err + 2.0 * f64::EPSILON * f.abs();
            Ok((GslSfResult { val, err }, s))
        } else if a + x == 0.0 {
            let (result, sgn) = gsl_sf_lngamma_sgn_e(-a + 1.0)?;
            let s = if (-a) % 2.0 == 0.0 { 1.0 } else { -1.0 };
            Ok((result, sgn * s))
        } else {
            Ok((GslSfResult { val: -INFINITY, err: 0.0 }, 1.0))
        }
    } else if a < 0.0 && a + x < 0.0 {
        let sin_1 = (PI * (1.0 - a)).sin();
        let sin_2 = (PI * (1.0 - a - x)).sin();
        if sin_1 == 0.0 || sin_2 == 0.0 {
            Err(GslError::Domain)
        } else {
            let lnp_pos = lnpoch_pos(1.0 - a, -x)?;
            let lnterm = (sin_1.abs() / sin_2.abs()).ln();
            let val = lnterm - lnp_pos.val;
            let err = lnp_pos.err
                + 2.0 * f64::EPSILON * (1.0 - a).abs() * (1.0 - a - x).abs() * lnterm.abs()
                + 2.0 * f64::EPSILON * val.abs();
            let sgn = if sin_1 * sin_2 >= 0.0 { 1.0 } else { -1.0 };
            Ok((GslSfResult { val, err }, sgn))
        }
    } else {
        let (lg_apn, s_apn) = gsl_sf_lngamma_sgn_e(a + x)?;
        let (lg_a, s_a) = gsl_sf_lngamma_sgn_e(a)?;
        let val = lg_apn.val - lg_a.val;
        let err = lg_apn.err + lg_a.err + 2.0 * f64::EPSILON * val.abs();
        Ok((GslSfResult { val, err }, s_a * s_apn))
    }
}

pub fn gsl_sf_poch_e(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    if x == 0.0 {
        Ok(GslSfResult { val: 1.0, err: 0.0 })
    } else {
        let (lnpoch, sgn) = gsl_sf_lnpoch_sgn_e(a, x)?;
        if lnpoch.val == -INFINITY {
            Ok(GslSfResult { val: 0.0, err: 0.0 })
        } else {
            let result = gsl_sf_exp_err_e(lnpoch.val, lnpoch.err)?;
            let val = result.val * sgn;
            let err = result.err + 2.0 * f64::EPSILON * val.abs();
            Ok(GslSfResult { val, err })
        }
    }
}

pub fn gsl_sf_pochrel_e(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    let absx = x.abs();
    let absa = a.abs();
    if absx > 0.1 * absa || absx * gsl_max(absa, 2.0).ln() > 0.1 {
        let (lnpoch, sgn) = gsl_sf_lnpoch_sgn_e(a, x)?;
        if lnpoch.val > 709.78271289338397 {
            Err(GslError::Overflow)
        } else {
            let el = lnpoch.val.exp();
            let val = (sgn * el - 1.0) / x;
            let err = val.abs() * (lnpoch.err + 2.0 * f64::EPSILON)
                + 2.0 * f64::EPSILON * (sgn * el).abs() / x.abs();
            Ok(GslSfResult { val, err })
        }
    } else {
        pochrel_smallx(a, x)
    }
}

// Helper functions that would be implemented elsewhere in the GSL bindings
fn gsl_sf_psi_e(_a: f64) -> Result<GslSfResult, GslError> { unimplemented!() }
fn gsl_sf_expm1_e(_x: f64) -> Result<GslSfResult, GslError> { unimplemented!() }
fn gsl_sf_log_1plusx_e(_x: f64) -> Result<GslSfResult, GslError> { unimplemented!() }
fn gsl_sf_gammainv_e(_x: f64) -> Result<GslSfResult, GslError> { unimplemented!() }
fn gsl_sf_lngamma_e(_x: f64) -> Result<GslSfResult, GslError> { unimplemented!() }
fn gsl_sf_lngamma_sgn_e(_x: f64) -> Result<(GslSfResult, f64), GslError> { unimplemented!() }
fn gsl_sf_exp