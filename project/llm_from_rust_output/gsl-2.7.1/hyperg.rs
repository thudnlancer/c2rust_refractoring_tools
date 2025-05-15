use std::f64::{NAN, INFINITY, EPSILON, MAX};
use std::cmp::max;

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GslError {
    Success,
    Failure,
    Continue,
    Domain,
    Range,
    // ... other error variants
    MaxIter,
    Overflow,
    Failed,
}

fn gsl_max_dbl(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn gsl_sf_hyperg_1f1_series_e(a: f64, b: f64, x: f64) -> Result<GslSfResult, GslError> {
    let mut an = a;
    let mut bn = b;
    let mut n = 1.0;
    let mut del = 1.0;
    let mut abs_del = 1.0;
    let mut max_abs_del = 1.0;
    let mut sum_val = 1.0;
    let mut sum_err = 0.0;

    while abs_del / sum_val.abs() > 0.25 * EPSILON {
        if bn == 0.0 {
            return Err(GslError::Domain);
        }
        if an == 0.0 {
            let err = sum_err + 2.0 * EPSILON * n * sum_val.abs();
            return Ok(GslSfResult {
                val: sum_val,
                err,
            });
        }
        if n > 10000.0 {
            return Err(GslError::Failed);
        }

        let u = x * (an / (bn * n));
        let abs_u = u.abs();

        if abs_u > 1.0 && max_abs_del > MAX / abs_u {
            return Err(GslError::Overflow);
        }

        del *= u;
        sum_val += del;
        if sum_val.abs() > 1.0e-5 * MAX {
            return Err(GslError::Overflow);
        }

        abs_del = del.abs();
        max_abs_del = gsl_max_dbl(abs_del, max_abs_del);
        sum_err += 2.0 * EPSILON * abs_del;
        an += 1.0;
        bn += 1.0;
        n += 1.0;
    }

    let err = sum_err + abs_del + 2.0 * EPSILON * n * sum_val.abs();
    Ok(GslSfResult {
        val: sum_val,
        err,
    })
}

fn gsl_sf_hyperg_1f1_large_b_e(a: f64, b: f64, x: f64) -> Result<GslSfResult, GslError> {
    if (x / b).abs() < 1.0 {
        let u = x / b;
        let v = 1.0 / (1.0 - u);
        let pre = v.powf(a);
        let uv = u * v;
        let uv2 = uv * uv;
        let t1 = a * (a + 1.0) / (2.0 * b) * uv2;
        let t2a = a * (a + 1.0) / (24.0 * b * b) * uv2;
        let t2b = 12.0 + 16.0 * (a + 2.0) * uv + 3.0 * (a + 2.0) * (a + 3.0) * uv2;
        let t2 = t2a * t2b;
        let val = pre * (1.0 - t1 + t2);
        let err = pre * EPSILON * (1.0 + t1.abs() + t2.abs()) + 2.0 * EPSILON * val.abs();
        Ok(GslSfResult { val, err })
    } else {
        Err(GslError::Domain)
    }
}

fn gsl_sf_hyperg_u_large_b_e(
    a: f64,
    b: f64,
    x: f64,
) -> Result<(GslSfResult, f64), GslError> {
    let n = b.floor();
    let eps = b - n;

    if eps.abs() < 1.4901161193847656e-08 {
        let (lnpre_val, lnpre_err, m) = if b > 1.0 {
            let tmp = (1.0 - b) * x.ln();
            let lg_bm1 = gsl_sf_lngamma_e(b - 1.0)?;
            let lg_a = gsl_sf_lngamma_e(a)?;
            let lnpre_val = tmp + x + lg_bm1.val - lg_a.val;
            let lnpre_err = lg_bm1.err + lg_a.err + EPSILON * (x.abs() + tmp.abs());
            let m = gsl_sf_hyperg_1f1_large_b_e(1.0 - a, 2.0 - b, -x)?;
            (lnpre_val, lnpre_err, m)
        } else {
            let lg_1mb = gsl_sf_lngamma_e(1.0 - b)?;
            let lg_1pamb = gsl_sf_lngamma_e(1.0 + a - b)?;
            let lnpre_val = lg_1mb.val - lg_1pamb.val;
            let lnpre_err = lg_1mb.err + lg_1pamb.err;
            let m = gsl_sf_hyperg_1f1_large_b_e(a, b, x)?;
            (lnpre_val, lnpre_err, m)
        };

        if lnpre_val > 7.0978271289338397e+02 - 10.0 {
            Err(GslError::Overflow)
        } else {
            let epre = gsl_sf_exp_err_e(lnpre_val, lnpre_err)?;
            let val = epre.val * m.val;
            let err = epre.val * m.err + epre.err * m.val.abs() + 2.0 * EPSILON * val.abs();
            Ok((GslSfResult { val, err }, 0.0))
        }
    } else {
        let omb_lnx = (1.0 - b) * x.ln();
        let (lg_1mb_0, sgn_1mb) = gsl_sf_lngamma_sgn_e(1.0 - b)?;
        let (lg_1pamb_0, sgn_1pamb) = gsl_sf_lngamma_sgn_e(1.0 + a - b)?;
        let (lg_bm1_0, sgn_bm1) = gsl_sf_lngamma_sgn_e(b - 1.0)?;
        let (lg_a_0, sgn_a) = gsl_sf_lngamma_sgn_e(a)?;
        
        let m1 = gsl_sf_hyperg_1f1_large_b_e(a, b, x)?;
        let m2 = gsl_sf_hyperg_1f1_large_b_e(1.0 - a, 2.0 - b, x)?;
        
        let lnpre1_val = lg_1mb_0.val - lg_1pamb_0.val;
        let lnpre1_err = lg_1mb_0.err + lg_1pamb_0.err;
        let lnpre2_val = lg_bm1_0.val - lg_a_0.val - omb_lnx - x;
        let lnpre2_err = lg_bm1_0.err + lg_a_0.err + EPSILON * (omb_lnx.abs() + x.abs());
        let sgpre1 = sgn_1mb * sgn_1pamb;
        let sgpre2 = sgn_bm1 * sgn_a;

        if lnpre1_val > 7.0978271289338397e+02 - 10.0 || lnpre2_val > 7.0978271289338397e+02 - 10.0 {
            let max_lnpre_val = lnpre1_val.max(lnpre2_val);
            let max_lnpre_err = lnpre1_err.max(lnpre2_err);
            let lp1 = lnpre1_val - max_lnpre_val;
            let lp2 = lnpre2_val - max_lnpre_val;
            let t1 = sgpre1 * lp1.exp();
            let t2 = sgpre2 * lp2.exp();
            let val = t1 * m1.val + t2 * m2.val;
            let err = t1.abs() * m1.err + t2.abs() * m2.err 
                + EPSILON * max_lnpre_err.exp() * (t1.abs() * m1.val.abs() + t2.abs() * m2.val.abs())
                + 2.0 * EPSILON * val.abs();
            Ok((GslSfResult { val, err }, max_lnpre_val))
        } else {
            let t1 = sgpre1 * lnpre1_val.exp();
            let t2 = sgpre2 * lnpre2_val.exp();
            let val = t1 * m1.val + t2 * m2.val;
            let err = t1.abs() * m1.err + t2.abs() * m2.err
                + EPSILON * (lnpre1_err.exp() * t1.abs() * m1.val.abs() + lnpre2_err.exp() * t2.abs() * m2.val.abs())
                + 2.0 * EPSILON * val.abs();
            Ok((GslSfResult { val, err }, 0.0))
        }
    }
}

fn gsl_sf_hyperg_2f0_series_e(
    a: f64,
    b: f64,
    x: f64,
    n_trunc: Option<usize>,
) -> Result<GslSfResult, GslError> {
    const MAXITER: usize = 2000;
    let mut an = a;
    let mut bn = b;
    let mut n = 1.0;
    let mut sum = 1.0;
    let mut del = 1.0;
    let mut abs_del = 1.0;
    let mut max_abs_del = 1.0;
    let mut last_abs_del = 1.0;

    while abs_del / sum.abs() > EPSILON && n < MAXITER as f64 {
        let u = an * (bn / n * x);
        let abs_u = u.abs();

        if abs_u > 1.0 && max_abs_del > MAX / abs_u {
            return Err(GslError::Overflow);
        }

        del *= u;
        sum += del;
        abs_del = del.abs();

        if abs_del > last_abs_del {
            break;
        }

        last_abs_del = abs_del;
        max_abs_del = max_abs_del.max(abs_del);
        an += 1.0;
        bn += 1.0;
        n += 1.0;

        if an == 0.0 || bn == 0.0 {
            break;
        }

        if let Some(trunc) = n_trunc {
            if n >= trunc as f64 {
                break;
            }
        }
    }

    if n >= MAXITER as f64 {
        Err(GslError::MaxIter)
    } else {
        Ok(GslSfResult {
            val: sum,
            err: EPSILON * n + abs_del,
        })
    }
}

// Helper functions would need to be implemented:
// fn gsl_sf_lngamma_e(x: f64) -> Result<GslSfResult, GslError>
// fn gsl_sf_lngamma_sgn_e(x: f64) -> Result<(GslSfResult, f64), GslError>
// fn gsl_sf_exp_err_e(x: f64, dx: f64) -> Result<GslSfResult, GslError>