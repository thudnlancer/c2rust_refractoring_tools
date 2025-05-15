use std::f64::consts::{PI, SQRT_2};
use std::f64::{INFINITY, NAN};

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

impl GslSfResult {
    pub fn new(val: f64, err: f64) -> Self {
        Self { val, err }
    }
}

fn gsl_pow_3(x: f64) -> f64 {
    x * x * x
}

fn gamma_inc_D(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    if a < 10.0 {
        let lg = gsl_sf_lngamma(a + 1.0)?;
        let lnr = a * x.ln() - x - lg.val;
        let val = lnr.exp();
        let err = 2.0 * f64::EPSILON * (lnr.abs() + 1.0) * val.abs();
        Ok(GslSfResult::new(val, err))
    } else {
        let (ln_term, gstar) = if x < 0.5 * a {
            let u = x / a;
            let ln_u = u.ln();
            let ln_term = GslSfResult::new(ln_u - u + 1.0, (ln_u.abs() + u.abs() + 1.0) * f64::EPSILON);
            let gstar = gsl_sf_gammastar(a)?;
            (ln_term, gstar)
        } else {
            let mu = (x - a) / a;
            let ln_term = gsl_sf_log_1plusx_mx(mu)?;
            let gstar = gsl_sf_gammastar(a)?;
            (ln_term, gstar)
        };

        let term1 = (a * ln_term.val).exp() / (2.0 * PI * a).sqrt();
        let val = term1 / gstar.val;
        let mut err = 2.0 * f64::EPSILON * (a * ln_term.val).abs() * val.abs();
        err += a * ln_term.err * val.abs();
        err += gstar.err / gstar.val.abs() * val.abs();
        Ok(GslSfResult::new(val, err))
    }
}

fn gamma_inc_P_series(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    const NMAX: i32 = 10000;
    let D = gamma_inc_D(a, x)?;

    if x > 0.995 * a && a > 1e5 {
        let cf_res = gsl_sf_exprel_n_CF(a, x)?;
        let val = D.val * cf_res.val;
        let err = D.val.abs() * cf_res.err + D.err.abs() * cf_res.val;
        return Ok(GslSfResult::new(val, err));
    }

    if x > a + NMAX as f64 {
        return Err(GslError::MaxIter);
    }

    let nlow = if x > a { (x - a) as i32 } else { 0 };
    let mut sum = 1.0;
    let mut term = 1.0;
    let mut n = 1;

    while n < nlow {
        term *= x / (a + n as f64);
        sum += term;
        n += 1;
    }

    while n < NMAX {
        term *= x / (a + n as f64);
        sum += term;
        if (term / sum).abs() < f64::EPSILON {
            break;
        }
        n += 1;
    }

    let tnp1 = x / (a + n as f64) * term;
    let remainder = tnp1 / (1.0 - x / (a + n as f64 + 1.0));
    let val = D.val * sum;
    let mut err = D.err * sum.abs() + D.val.abs() * remainder;
    err += (1.0 + n as f64) * f64::EPSILON * val.abs();

    if n == NMAX && (remainder / sum).abs() > 1.49e-8 {
        Err(GslError::MaxIter)
    } else {
        Ok(GslSfResult::new(val, err))
    }
}

fn gamma_inc_Q_large_x(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    const NMAX: i32 = 5000;
    let D = gamma_inc_D(a, x)?;
    let mut sum = 1.0;
    let mut term = 1.0;
    let mut last = 1.0;
    let mut n = 1;

    while n < NMAX {
        term *= (a - n as f64) / x;
        if (term / last).abs() > 1.0 {
            break;
        }
        if (term / sum).abs() < f64::EPSILON {
            break;
        }
        sum += term;
        last = term;
        n += 1;
    }

    let val = D.val * (a / x) * sum;
    let mut err = D.err * (a / x * sum).abs();
    err += 2.0 * f64::EPSILON * val.abs();

    if n == NMAX {
        Err(GslError::MaxIter)
    } else {
        Ok(GslSfResult::new(val, err))
    }
}

fn gamma_inc_Q_asymp_unif(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    let rta = a.sqrt();
    let eps = (x - a) / a;
    let ln_term = gsl_sf_log_1plusx_mx(eps)?;
    let eta = if eps >= 0.0 { 1.0 } else { -1.0 } * (-2.0 * ln_term.val).sqrt();
    let erfc = gsl_sf_erfc(eta * rta / SQRT_2)?;

    let (c0, c1) = if eps.abs() < 7.4e-4 {
        let c0 = -1.0 / 3.0 
            + eps * (1.0 / 12.0 
                - eps * (23.0 / 540.0 
                    - eps * (353.0 / 12960.0 - eps * 589.0 / 30240.0)));
        let c1 = -1.0 / 540.0 - eps / 288.0;
        (c0, c1)
    } else {
        let rt_term = (-2.0 * ln_term.val / (eps * eps)).sqrt();
        let lam = x / a;
        let c0 = (1.0 - 1.0 / rt_term) / eps;
        let c1 = -(eta.powi(3) * (lam.powi(2) + 10.0 * lam + 1.0)
            - 12.0 * eps.powi(3)) / (12.0 * eta.powi(3) * eps.powi(3));
        (c0, c1)
    };

    let R = (-0.5 * a * eta.powi(2)).exp() 
        / (SQRT_2 * (2.0 * PI).sqrt() * rta) 
        * (c0 + c1 / a);
    let val = 0.5 * erfc.val + R;
    let mut err = f64::EPSILON * (R * 0.5 * a * eta.powi(2)).abs() + 0.5 * erfc.err;
    err += 2.0 * f64::EPSILON * val.abs();
    Ok(GslSfResult::new(val, err))
}

fn gamma_inc_F_CF(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    const NMAX: i32 = 5000;
    const SMALL: f64 = f64::EPSILON.powi(3);
    let mut hn = 1.0;
    let mut Cn = 1.0 / SMALL;
    let mut Dn = 1.0;
    let mut n = 2;

    while n < NMAX {
        let an = if n % 2 != 0 {
            0.5 * (n - 1) as f64 / x
        } else {
            (0.5 * n as f64 - a) / x
        };

        Dn = 1.0 + an * Dn;
        if Dn.abs() < SMALL { Dn = SMALL; }
        Cn = 1.0 + an / Cn;
        if Cn.abs() < SMALL { Cn = SMALL; }
        Dn = 1.0 / Dn;
        let delta = Cn * Dn;
        hn *= delta;

        if (delta - 1.0).abs() < f64::EPSILON {
            break;
        }
        n += 1;
    }

    let val = hn;
    let mut err = 2.0 * f64::EPSILON * hn.abs();
    err += f64::EPSILON * (2.0 + 0.5 * n as f64) * val.abs();

    if n == NMAX {
        Err(GslError::MaxIter)
    } else {
        Ok(GslSfResult::new(val, err))
    }
}

fn gamma_inc_Q_CF(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    let D = gamma_inc_D(a, x)?;
    let F = gamma_inc_F_CF(a, x)?;
    let val = D.val * (a / x) * F.val;
    let err = D.err * (a / x * F.val).abs() + D.val.abs() * (a / x * F.err);
    Ok(GslSfResult::new(val, err))
}

fn gamma_inc_Q_series(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    const NMAX: i32 = 5000;
    let pg21 = -2.404113806319188570799476;
    let lnx = x.ln();
    let el = 0.57721566490153286060651209008 + lnx;
    
    let c1 = -el;
    let c2 = PI.powi(2) / 12.0 - 0.5 * el.powi(2);
    let c3 = el * (PI.powi(2) / 12.0 - el.powi(2) / 6.0) + pg21 / 6.0;
    let c4 = -0.04166666666666666667 
        * (-1.758243446661483480 + lnx) 
        * (-0.764428657272716373 + lnx) 
        * (0.723980571623507657 + lnx) 
        * (4.107554191916823640 + lnx);
    let c5 = -0.0083333333333333333 
        * (-2.06563396085715900 + lnx) 
        * (-1.28459889470864700 + lnx) 
        * (-0.27583535756454143 + lnx) 
        * (1.33677371336239618 + lnx) 
        * (5.17537282427561550 + lnx);
    let c6 = -0.0013888888888888889 
        * (-2.30814336454783200 + lnx) 
        * (-1.65846557706987300 + lnx) 
        * (-0.88768082560020400 + lnx) 
        * (0.17043847751371778 + lnx) 
        * (1.92135970115863890 + lnx) 
        * (6.22578557795474900 + lnx);
    let c7 = -0.00019841269841269841 
        * (-2.5078657901291800 + lnx) 
        * (-1.9478900888958200 + lnx) 
        * (-1.3194837322612730 + lnx) 
        * (-0.5281322700249279 + lnx) 
        * (0.5913834939078759 + lnx) 
        * (2.4876819633378140 + lnx) 
        * (7.2648160783762400 + lnx);
    let c8 = -0.00002480158730158730 
        * (-2.677341544966400 + lnx) 
        * (-2.182810448271700 + lnx) 
        * (-1.649350342277400 + lnx) 
        * (-1.014099048290790 + lnx) 
        * (-0.191366955370652 + lnx) 
        * (0.995403817918724 + lnx) 
        * (3.041323283529310 + lnx) 
        * (8.295966556941250 + lnx);
    let c9 = -2.75573192239859e-6 
        * (-2.8243487670469080 + lnx) 
        * (-2.3798494322701120 + lnx) 
        * (-1.9143674728689960 + lnx) 
        * (-1.3814529102920370 + lnx) 
        * (-0.7294312810261694 + lnx) 
        * (0.1299079285269565 + lnx) 
        * (1.3873333251885240 + lnx) 
        * (3.5857258865210760 + lnx) 
        * (9.3214237073814600 + lnx);
    let c10 = -2.75573192239859e-7 
        * (-2.9540329644556910 + lnx) 
        * (-2.5491366926991850 + lnx) 
        * (-2.1348279229279880 + lnx) 
        * (-1.6741881076349450 + lnx) 
        * (-1.1325949616098420 + lnx) 
        * (-0.4590034650618494 + lnx) 
        * (0.4399352987435699 + lnx) 
        * (1.7702236517651670 + lnx) 
        * (4.1231539047474080 + lnx) 
        * (10.342627908148680 + lnx);

    let term1 = a * (c1 + a * (c2 + a * (c3 + a * (c4 + a * (c5 + a * (c6 + a * (c7 + a * (c8 + a * (c9 + a * c10)))))))));
    
    let mut sum = 1.0;
    let mut t = 1.0;
    let mut n = 1;
    
    while n < NMAX {
        t *= -x / (n as f64 + 1.0);
        sum += (a + 1.0) / (a + n as f64 + 1.0) * t;
        if (t / sum).abs() < f64::EPSILON {
            break;
        }
        n += 1;
    }

    let term2 = (1.0 - term1) * a / (a + 1.0) * x * sum;
    let val = term1 + term2;
    let err = f64::EPSILON * (term1.abs() + 2.0 * term2.abs());
    let err = err + 2.0 * f64::EPSILON * val.abs();

    if n == NMAX {
        Err(GslError::MaxIter)
    } else {
        Ok(GslSfResult::new(val, err))
    }
}

fn gamma_inc_series(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    let Q = gamma_inc_Q_series(a, x)?;
    let G = gsl_sf_gamma(a)?;
    let val = Q.val * G.val;
    let err = Q.val.abs() * G.err + Q.err.abs() * G.val;
    let err = err + 2.0 * f64::EPSILON * val.abs();
    Ok(GslSfResult::new(val, err))
}

fn gamma_inc_a_gt_0(a: f64, x: f64) -> Result<GslSfResult, GslError> {
    let Q = gsl_sf_gamma_inc_Q(a, x)?;
    let G = gsl_sf_gamma(a)?;
    let val = G.val * Q.val;
    let err = G.val.abs() * Q.err + G.err.abs() * Q.val;
    let err = err + 2.