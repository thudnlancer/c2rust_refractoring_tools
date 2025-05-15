use std::f64::consts::{LN_2, PI};
use std::f64;

const BERN: [f64; 21] = [
    0.0, // no element 0
    0.083333333333333333333333333333333,
    -0.001388888888888888888888888888888,
    0.000033068783068783068783068783068,
    -0.000000826719576719576719576719576,
    0.000000020876756987868098979210090,
    -0.000000000528419013868749318484768,
    0.000000000013382536530684678832869,
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

const SQRT2: f64 = 1.41421356237309504880168872420969808;
const SQRT3: f64 = 1.73205080756887729352744634150587237;
const GAMMA_XMAX: f64 = 171.0; // Approximate value where gamma overflows
const DBL_EPSILON: f64 = 2.2204460492503131e-16;
const DBL_MIN: f64 = 2.2250738585072014e-308;
const LOG_DBL_EPSILON: f64 = -36.043653389117156;
const LOG_DBL_MAX: f64 = 709.78271289338397;

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

#[derive(Debug)]
pub enum SfError {
    DomainError,
    OverflowError,
    UnderflowError,
    SanityCheckFailed,
    OtherError,
}

pub type SfResultT<T> = Result<T, SfError>;

fn pochrel_smallx(a: f64, x: f64) -> SfResultT<SfResult> {
    let sqrtbig = 1.0 / (2.0 * SQRT2 * SQRT3 * DBL_MIN.sqrt());
    let alneps = LOG_DBL_EPSILON - LN_2;

    if x == 0.0 {
        sf_psi_e(a)
    } else {
        let (bp, incr) = if a < -0.5 {
            (1.0 - a - x, (11.0 - (1.0 - a - x)) as usize)
        } else {
            (a, if a < 10.0 { (11.0 - a) as usize } else { 0 })
        };
        let b = bp + incr as f64;

        let var = b + 0.5 * (x - 1.0);
        let alnvar = var.ln();
        let q = x * alnvar;

        let mut poly1 = 0.0;

        if var < sqrtbig {
            let nterms = ((-0.5 * alneps / alnvar) as usize + 1;
            let var2 = (1.0 / var) / var;
            let rho = 0.5 * (x + 1.0);
            let mut term = var2;

            let mut gbern = vec![0.0; 24];
            gbern[1] = 1.0;
            gbern[2] = -rho / 12.0;
            poly1 = gbern[2] * term;

            if nterms > 20 {
                return Err(SfError::SanityCheckFailed);
            }

            for k in 2..=nterms {
                let mut gbk = 0.0;
                for j in 1..=k {
                    gbk += BERN[k - j + 1] * gbern[j];
                }
                gbern[k + 1] = -rho * gbk / k as f64;

                term *= (2.0 * k as f64 - 2.0 - x) * (2.0 * k as f64 - 1.0 - x) * var2;
                poly1 += gbern[k + 1] * term;
            }
        }

        let dexprl = sf_expm1_e(q)?;
        let dexprl_val = dexprl.val / q;
        poly1 *= x - 1.0;
        let mut dpoch1 = dexprl_val * (alnvar + q * poly1) + poly1;

        for i in (0..incr).rev() {
            let binv = 1.0 / (bp + i as f64);
            dpoch1 = (dpoch1 - binv) / (1.0 + x * binv);
        }

        if bp == a {
            Ok(SfResult::new(
                dpoch1,
                2.0 * DBL_EPSILON * (incr as f64 + 1.0) * dpoch1.abs(),
            ))
        } else {
            let sinpxx = (PI * x).sin() / x;
            let sinpx2 = (0.5 * PI * x).sin();
            let t1 = sinpxx / (PI * b).tan();
            let t2 = 2.0 * sinpx2 * (sinpx2 / x);
            let trig = t1 - t2;
            let val = dpoch1 * (1.0 + x * trig) + trig;
            let err = (dpoch1.abs() * x + 1.0) * DBL_EPSILON * (t1.abs() + t2.abs())
                + 2.0 * DBL_EPSILON * (incr as f64 + 1.0) * val.abs();
            Ok(SfResult::new(val, err))
        }
    }
}

fn lnpoch_pos(a: f64, x: f64) -> SfResultT<SfResult> {
    let absx = x.abs();

    if absx > 0.1 * a || absx * a.max(2.0).ln() > 0.1 {
        if a < GAMMA_XMAX && a + x < GAMMA_XMAX {
            let g1 = sf_gammainv_e(a)?;
            let g2 = sf_gammainv_e(a + x)?;
            let val = -(g2.val / g1.val).ln();
            let err = g1.err / g1.val.abs() + g2.err / g2.val.abs();
            Ok(SfResult::new(val, err + 2.0 * DBL_EPSILON * val.abs()))
        } else {
            let lg1 = sf_lngamma_e(a)?;
            let lg2 = sf_lngamma_e(a + x)?;
            let val = lg2.val - lg1.val;
            Ok(SfResult::new(val, lg2.err + lg1.err + 2.0 * DBL_EPSILON * val.abs()))
        }
    } else if absx < 0.1 * a && a > 15.0 {
        let eps = x / a;
        let den = 1.0 + eps;
        let d3 = den.powi(3);
        let d5 = den.powi(5);
        let d7 = den.powi(7);
        let c1 = -eps / den;
        let c3 = -eps * (3.0 + eps * (3.0 + eps)) / d3;
        let c5 = -eps * (5.0 + eps * (10.0 + eps * (10.0 + eps * (5.0 + eps)))) / d5;
        let c7 = -eps * (7.0 + eps * (21.0 + eps * (35.0 + eps * (35.0 + eps * (21.0 + eps * (7.0 + eps)))))) / d7;
        let p8 = (1.0 + eps).powi(8);
        let c8 = 1.0 / p8 - 1.0;
        let c9 = 1.0 / (p8 * (1.0 + eps)) - 1.0;
        let a4 = a.powi(4);
        let a6 = a.powi(6);
        let ser_1 = c1 + c3 / (30.0 * a * a) + c5 / (105.0 * a4) + c7 / (140.0 * a6);
        let ser_2 = c8 / (99.0 * a6 * a * a) - 691.0 / 360360.0 * c9 / (a6 * a4);
        let ser = (ser_1 + ser_2) / (12.0 * a);

        let term1 = x * (a / std::f64::consts::E).ln();
        let ln_1peps = sf_log_1plusx_e(eps)?;
        let term2 = (x + a - 0.5) * ln_1peps.val;

        let val = term1 + term2 + ser;
        let err = DBL_EPSILON * term1.abs()
            + (x + a - 0.5).abs() * ln_1peps.err
            + ln_1peps.val.abs() * DBL_EPSILON * (x.abs() + a.abs() + 0.5)
            + 2.0 * DBL_EPSILON * val.abs();
        Ok(SfResult::new(val, err))
    } else {
        let poch_rel = pochrel_smallx(a, x)?;
        let eps = x * poch_rel.val;
        let result = sf_log_1plusx_e(eps)?;
        let err = 2.0 * (x * poch_rel.err / (1.0 + eps)).abs() + 2.0 * DBL_EPSILON * result.val.abs();
        Ok(SfResult::new(result.val, err))
    }
}

pub fn sf_lnpoch_e(a: f64, x: f64) -> SfResultT<SfResult> {
    if a <= 0.0 || a + x <= 0.0 {
        Err(SfError::DomainError)
    } else if x == 0.0 {
        Ok(SfResult::new(0.0, 0.0))
    } else {
        lnpoch_pos(a, x)
    }
}

pub fn sf_lnpoch_sgn_e(a: f64, x: f64) -> SfResultT<(SfResult, f64)> {
    if x == 0.0 {
        Ok((SfResult::new(0.0, 0.0), 1.0))
    } else if a > 0.0 && a + x > 0.0 {
        Ok((lnpoch_pos(a, x)?, 1.0))
    } else if a <= 0.0 && a == a.floor() {
        if a + x < 0.0 && x == x.floor() {
            let result_pos = lnpoch_pos(-a, -x)?;
            let f = (a / (a + x)).ln();
            let s = if x % 2.0 == 0.0 { 1.0 } else { -1.0 };
            let val = f - result_pos.val;
            let err = result_pos.err + 2.0 * DBL_EPSILON * f.abs();
            Ok((SfResult::new(val, err), s))
        } else if a + x == 0.0 {
            let (result, sgn) = sf_lngamma_sgn_e(-a + 1.0)?;
            let s = if (-a) % 2.0 == 0.0 { 1.0 } else { -1.0 };
            Ok((result, sgn * s))
        } else {
            Ok((SfResult::new(f64::NEG_INFINITY, 0.0), 1.0))
        }
    } else if a < 0.0 && a + x < 0.0 {
        let sin_1 = (PI * (1.0 - a)).sin();
        let sin_2 = (PI * (1.0 - a - x)).sin();
        if sin_1 == 0.0 || sin_2 == 0.0 {
            Err(SfError::DomainError)
        } else {
            let lnp_pos = lnpoch_pos(1.0 - a, -x)?;
            let lnterm = (sin_1 / sin_2).abs().ln();
            let val = lnterm - lnp_pos.val;
            let err = lnp_pos.err
                + 2.0 * DBL_EPSILON * (1.0 - a).abs() + (1.0 - a - x).abs() * lnterm.abs()
                + 2.0 * DBL_EPSILON * val.abs();
            let sgn = (sin_1 * sin_2).signum();
            Ok((SfResult::new(val, err), sgn))
        }
    } else {
        let (lg_apn, s_apn) = sf_lngamma_sgn_e(a + x)?;
        let (lg_a, s_a) = sf_lngamma_sgn_e(a)?;
        let val = lg_apn.val - lg_a.val;
        let err = lg_apn.err + lg_a.err + 2.0 * DBL_EPSILON * val.abs();
        let sgn = s_a * s_apn;
        Ok((SfResult::new(val, err), sgn))
    }
}

pub fn sf_poch_e(a: f64, x: f64) -> SfResultT<SfResult> {
    if x == 0.0 {
        Ok(SfResult::new(1.0, 0.0))
    } else {
        let (lnpoch, sgn) = sf_lnpoch_sgn_e(a, x)?;
        if lnpoch.val == f64::NEG_INFINITY {
            Ok(SfResult::new(0.0, 0.0))
        } else {
            let result = sf_exp_err_e(lnpoch.val, lnpoch.err)?;
            let val = result.val * sgn;
            let err = result.err + 2.0 * DBL_EPSILON * val.abs();
            Ok(SfResult::new(val, err))
        }
    }
}

pub fn sf_pochrel_e(a: f64, x: f64) -> SfResultT<SfResult> {
    let absx = x.abs();
    let absa = a.abs();

    if absx > 0.1 * absa || absx * absa.max(2.0).ln() > 0.1 {
        let (lnpoch, sgn) = sf_lnpoch_sgn_e(a, x)?;
        if lnpoch.val > LOG_DBL_MAX {
            Err(SfError::OverflowError)
        } else {
            let el = lnpoch.val.exp();
            let val = (sgn * el - 1.0) / x;
            let err = val.abs() * (lnpoch.err + 2.0 * DBL_EPSILON)
                + 2.0 * DBL_EPSILON * (sgn * el).abs() + 1.0) / x.abs();
            Ok(SfResult::new(val, err))
        }
    } else {
        pochrel_smallx(a, x)
    }
}

// Helper functions that would be implemented elsewhere in the Rust library
fn sf_psi_e(_a: f64) -> SfResultT<SfResult> {
    unimplemented!()
}

fn sf_expm1_e(_q: f64) -> SfResultT<SfResult> {
    unimplemented!()
}

fn sf_gammainv_e(_a: f64) -> SfResultT<SfResult> {
    unimplemented!()
}

fn sf_lngamma_e(_a: f64) -> SfResultT<SfResult> {
    unimplemented!()
}

fn sf_log_1plusx_e(_eps: f64) -> SfResultT<SfResult> {
    unimplemented!()
}

fn sf_lngamma_sgn_e(_a: f64) -> SfResultT<(SfResult, f64)> {
    unimplemented!()
}

fn sf_exp_err_e(_val: f64, _err: f64) -> SfResultT<SfResult> {
    unimplemented!()
}