use std::f64::consts::{EULER, PI, SQRT_2};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

const GSL_DBL_EPSILON: f64 = f64::EPSILON;
const GSL_SQRT_DBL_EPSILON: f64 = f64::EPSILON.sqrt();
const GSL_ROOT5_DBL_EPSILON: f64 = f64::EPSILON.powf(0.2);

fn gamma_inc_d(a: f64, x: f64) -> Result<SfResult, &'static str> {
    if a < 10.0 {
        let lg = lngamma(a + 1.0)?;
        let lnr = a * x.ln() - x - lg.val;
        let val = lnr.exp();
        let err = 2.0 * GSL_DBL_EPSILON * (lnr.abs() + 1.0) * val.abs();
        Ok(SfResult { val, err })
    } else {
        let gstar = gammastar(a)?;
        let ln_term = if x < 0.5 * a {
            let u = x / a;
            let ln_u = u.ln();
            SfResult {
                val: ln_u - u + 1.0,
                err: (ln_u.abs() + u.abs() + 1.0) * GSL_DBL_EPSILON,
            }
        } else {
            let mu = (x - a) / a;
            let mut res = log_1plusx_mx(mu)?;
            res.err += GSL_DBL_EPSILON * mu.abs();
            res
        };

        let term1 = (a * ln_term.val).exp() / (2.0 * PI * a).sqrt();
        let val = term1 / gstar.val;
        let mut err = 2.0 * GSL_DBL_EPSILON * ((a * ln_term.val).abs() + 1.0) * val.abs();
        err += a.abs() * ln_term.err * val.abs();
        err += gstar.err / gstar.val.abs() * val.abs();
        Ok(SfResult { val, err })
    }
}

fn gamma_inc_p_series(a: f64, x: f64) -> Result<SfResult, &'static str> {
    const NMAX: usize = 10000;

    let d = gamma_inc_d(a, x)?;

    if x > 0.995 * a && a > 1e5 {
        let cf_res = exprel_n_cf(a, x)?;
        let val = d.val * cf_res.val;
        let err = d.val.abs() * cf_res.err + d.err.abs() * cf_res.val.abs();
        return Ok(SfResult { val, err });
    }

    if x > (a + NMAX as f64) {
        return Err("gamma_inc_P_series x>>a exceeds range");
    }

    let mut sum = 1.0;
    let mut term = 1.0;
    let nlow = if x > a { (x - a) as usize } else { 0 };

    for n in 1..nlow {
        term *= x / (a + n as f64);
        sum += term;
    }

    let mut n = nlow;
    while n < NMAX {
        term *= x / (a + n as f64);
        sum += term;
        if (term / sum).abs() < GSL_DBL_EPSILON {
            break;
        }
        n += 1;
    }

    let tnp1 = (x / (a + n as f64)) * term;
    let remainder = tnp1 / (1.0 - x / (a + n as f64 + 1.0));

    let val = d.val * sum;
    let mut err = d.err * sum.abs() + d.val.abs() * remainder;
    err += (1.0 + n as f64) * GSL_DBL_EPSILON * val.abs();

    if n == NMAX && (remainder / sum).abs() > GSL_SQRT_DBL_EPSILON {
        Err("gamma_inc_P_series failed to converge")
    } else {
        Ok(SfResult { val, err })
    }
}

fn gamma_inc_q_large_x(a: f64, x: f64) -> Result<SfResult, &'static str> {
    const NMAX: usize = 5000;

    let d = gamma_inc_d(a, x)?;

    let mut sum = 1.0;
    let mut term = 1.0;
    let mut last = 1.0;
    let mut n = 1;

    while n < NMAX {
        term *= (a - n as f64) / x;
        if (term / last).abs() > 1.0 {
            break;
        }
        if (term / sum).abs() < GSL_DBL_EPSILON {
            break;
        }
        sum += term;
        last = term;
        n += 1;
    }

    let val = d.val * (a / x) * sum;
    let mut err = d.err * (a / x * sum).abs();
    err += 2.0 * GSL_DBL_EPSILON * val.abs();

    if n == NMAX {
        Err("error in large x asymptotic")
    } else {
        Ok(SfResult { val, err })
    }
}

fn gamma_inc_q_asymp_unif(a: f64, x: f64) -> Result<SfResult, &'static str> {
    let rta = a.sqrt();
    let eps = (x - a) / a;

    let mut ln_term = log_1plusx_mx(eps)?;
    let eta = eps.signum() * (-2.0 * ln_term.val).sqrt();

    let erfc_val = erfc(eta * rta / SQRT_2);

    let (c0, c1) = if eps.abs() < GSL_ROOT5_DBL_EPSILON {
        (
            -1.0 / 3.0
                + eps * (1.0 / 12.0 - eps * (23.0 / 540.0 - eps * (353.0 / 12960.0 - eps * 589.0 / 30240.0))),
            -1.0 / 540.0 - eps / 288.0,
        )
    } else {
        let rt_term = (-2.0 * ln_term.val / (eps * eps)).sqrt();
        let lam = x / a;
        (
            (1.0 - 1.0 / rt_term) / eps,
            -(eta.powi(3) * (lam * lam + 10.0 * lam + 1.0) - 12.0 * eps.powi(3))
                / (12.0 * eta.powi(3) * eps.powi(3),
        )
    };

    let r = (-0.5 * a * eta * eta).exp() / (SQRT_2 * PI.sqrt() * rta) * (c0 + c1 / a);
    let val = 0.5 * erfc_val + r;
    let mut err = GSL_DBL_EPSILON * (r * 0.5 * a * eta * eta).abs();
    err += 0.5 * 2.0 * GSL_DBL_EPSILON * erfc_val.abs();
    err += 2.0 * GSL_DBL_EPSILON * val.abs();

    Ok(SfResult { val, err })
}

fn gamma_inc_f_cf(a: f64, x: f64) -> Result<SfResult, &'static str> {
    const NMAX: usize = 5000;
    const SMALL: f64 = GSL_DBL_EPSILON.powi(3);

    let mut hn = 1.0;
    let mut cn = 1.0 / SMALL;
    let mut dn = 1.0;
    let mut n = 2;

    while n < NMAX {
        let an = if n % 2 == 1 {
            0.5 * (n as f64 - 1.0) / x
        } else {
            (0.5 * n as f64 - a) / x
        };

        dn = 1.0 + an * dn;
        if dn.abs() < SMALL {
            dn = SMALL;
        }
        cn = 1.0 + an / cn;
        if cn.abs() < SMALL {
            cn = SMALL;
        }
        dn = 1.0 / dn;
        let delta = cn * dn;
        hn *= delta;
        if (delta - 1.0).abs() < GSL_DBL_EPSILON {
            break;
        }
        n += 1;
    }

    let val = hn;
    let mut err = 2.0 * GSL_DBL_EPSILON * hn.abs();
    err += GSL_DBL_EPSILON * (2.0 + 0.5 * n as f64) * val.abs();

    if n == NMAX {
        Err("error in CF for F(a,x)")
    } else {
        Ok(SfResult { val, err })
    }
}

fn gamma_inc_q_cf(a: f64, x: f64) -> Result<SfResult, &'static str> {
    let d = gamma_inc_d(a, x)?;
    let f = gamma_inc_f_cf(a, x)?;

    let val = d.val * (a / x) * f.val;
    let err = d.err * (a / x * f.val).abs() + d.val.abs() * a / x * f.err;

    Ok(SfResult { val, err })
}

fn gamma_inc_q_series(a: f64, x: f64) -> Result<SfResult, &'static str> {
    const NMAX: usize = 5000;
    const PG21: f64 = -2.404113806319188570799476;
    let lnx = x.ln();
    let el = EULER + lnx;

    let term1 = a * (-el
        + a * (PI * PI / 12.0 - 0.5 * el * el
            + a * (el * (PI * PI / 12.0 - el * el / 6.0) + PG21 / 6.0
                + a * (-0.04166666666666666667
                    * (-1.758243446661483480 + lnx)
                    * (-0.764428657272716373 + lnx)
                    * (0.723980571623507657 + lnx)
                    * (4.107554191916823640 + lnx)
                    + a * (-0.0083333333333333333
                        * (-2.06563396085715900 + lnx)
                        * (-1.28459889470864700 + lnx)
                        * (-0.27583535756454143 + lnx)
                        * (1.33677371336239618 + lnx)
                        * (5.17537282427561550 + lnx)
                        + a * (-0.0013888888888888889
                            * (-2.30814336454783200 + lnx)
                            * (-1.65846557706987300 + lnx)
                            * (-0.88768082560020400 + lnx)
                            * (0.17043847751371778 + lnx)
                            * (1.92135970115863890 + lnx)
                            * (6.22578557795474900 + lnx)
                            + a * (-0.00019841269841269841
                                * (-2.5078657901291800 + lnx)
                                * (-1.9478900888958200 + lnx)
                                * (-1.3194837322612730 + lnx)
                                * (-0.5281322700249279 + lnx)
                                * (0.5913834939078759 + lnx)
                                * (2.4876819633378140 + lnx)
                                * (7.2648160783762400 + lnx)
                                + a * (-0.00002480158730158730
                                    * (-2.677341544966400 + lnx)
                                    * (-2.182810448271700 + lnx)
                                    * (-1.649350342277400 + lnx)
                                    * (-1.014099048290790 + lnx)
                                    * (-0.191366955370652 + lnx)
                                    * (0.995403817918724 + lnx)
                                    * (3.041323283529310 + lnx)
                                    * (8.295966556941250 + lnx)
                                    + a * (-2.75573192239859e-6
                                        * (-2.8243487670469080 + lnx)
                                        * (-2.3798494322701120 + lnx)
                                        * (-1.9143674728689960 + lnx)
                                        * (-1.3814529102920370 + lnx)
                                        * (-0.7294312810261694 + lnx)
                                        * (0.1299079285269565 + lnx)
                                        * (1.3873333251885240 + lnx)
                                        * (3.5857258865210760 + lnx)
                                        * (9.3214237073814600 + lnx)
                                        + a * (-2.75573192239859e-7
                                            * (-2.9540329644556910 + lnx)
                                            * (-2.5491366926991850 + lnx)
                                            * (-2.1348279229279880 + lnx)
                                            * (-1.6741881076349450 + lnx)
                                            * (-1.1325949616098420 + lnx)
                                            * (-0.4590034650618494 + lnx)
                                            * (0.4399352987435699 + lnx)
                                            * (1.7702236517651670 + lnx)
                                            * (4.1231539047474080 + lnx)
                                            * (10.342627908148680 + lnx))))))))))));

    let mut sum = 1.0;
    let mut term = 1.0;
    let mut n = 1;
    let mut stat_sum = Ok(());

    while n < NMAX {
        term *= -x / (n as f64 + 1.0);
        sum += (a + 1.0) / (a + n as f64 + 1.0) * term;
        if (term / sum).abs() < GSL_DBL_EPSILON {
            break;
        }
        n += 1;
    }

    if n == NMAX {
        stat_sum = Err("max iterations reached");
    }

    let term2 = (1.0 - term1) * a / (a + 1.0) * x * sum;
    let val = term1 + term2;
    let err = GSL_DBL_EPSILON * (term1.abs() + 2.0 * term2.abs());
    let err = err + 2.0 * GSL_DBL_EPSILON * val.abs();

    stat_sum.map(|_| SfResult { val, err })
}

fn gamma_inc_series(a: f64, x: f64) -> Result<SfResult, &'static str> {
    let q = gamma_inc_q_series(a, x)?;
    let g = gamma(a)?;
    let val = q.val * g.val;
    let err = q.val.abs() * g.err + q.err.abs() * g.val.abs();
    let err = err + 2.0 * GSL_DBL_EPSILON * val.abs();
    Ok(SfResult { val, err })
}

fn gamma_inc_a_gt_0(a: f64, x: f64) -> Result<SfResult, &'static str> {
    let q = gamma_inc_q(a, x)?;
    let g = gamma(a)?;
    let val = g.val * q.val;
    let err = g.val.abs() * q.err + g.err.abs() * q.val.abs();
    let err = err + 2.0 * GSL_DBL_EPSILON * val.abs();
    Ok(SfResult { val, err })
}

fn gamma_inc_cf(a: f64, x: f64) -> Result<SfResult, &'static str> {
    let f = gamma_inc_f_cf(a, x)?;
    let am1lgx = (a - 1.0) * x.ln();
    let pre = exp_err(am1lgx - x, GSL_DBL_EPSILON * am1lgx.abs())?;
    let val = f.val * pre.val;
    let err = f.err.abs() * pre.val.abs() + f.val.abs() * pre.err.abs();
    let err = err + 2.0 * GSL_DBL_EPSILON * val.abs();
    Ok(SfResult { val, err })
}

fn gamma_inc(a: f64, x: f64) -> Result<SfResult, &'static str> {
    if x < 0.0 {
        Err("domain error")
    } else if x == 0.0 {
        gamma(a)
    } else if a == 0.0 {
        expint_e1(x)
    } else if a > 0.0 {
        gamma_inc_a_gt_0(a, x)
    } else if x > 0.25 {
        gamma_inc_cf(a, x)
    } else if a.abs() < 0.5 {
        gamma_inc_series(a, x)
    } else {
        let fa = a.floor();
        let da = a - fa;

        let g_da = if da > 0.0 {
            gamma_inc_a_gt_0(da, x)
        } else {
            expint_e1(x)
        }?;

        let mut alpha = da;
        let mut gax = g_da.val;

        while alpha > a {
            let shift = (-x + (alpha - 1.0) * x.ln()).exp();
            gax = (gax - shift) / (alpha - 1.0