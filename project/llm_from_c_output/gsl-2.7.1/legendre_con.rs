use std::f64::consts::{PI, SQRT_2, FRAC_1_SQRT_2};
use std::f64::{EPSILON, MIN_POSITIVE, MAX};

const ROOT_2_OVER_PI: f64 = 0.797884560802865355879892;
const LOC_EPS: f64 = 1000.0 * EPSILON;
const RECURSE_LARGE: f64 = 1.0e-5 * MAX;
const RECURSE_SMALL: f64 = 1.0e+5 * MIN_POSITIVE;

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

pub enum SfError {
    DomainError,
    MaxIter,
    Other(&'static str),
}

impl From<&'static str> for SfError {
    fn from(s: &'static str) -> Self {
        SfError::Other(s)
    }
}

fn conicalp_negmu_xlt1_cf1(
    mu: f64,
    ell: i32,
    tau: f64,
    x: f64,
) -> Result<SfResult, SfError> {
    const MAXITER: i32 = 5000;
    let recur_big = MAX.sqrt();
    let mut n = 1;
    let xi = x / ((1.0 - x).sqrt() * (1.0 + x).sqrt());
    let mut anm2 = 1.0;
    let mut bnm2 = 0.0;
    let mut anm1 = 0.0;
    let mut bnm1 = 1.0;
    let a1 = 1.0;
    let b1 = 2.0 * (mu + ell as f64 + 1.0) * xi;
    let mut an = b1 * anm1 + a1 * anm2;
    let mut bn = b1 * bnm1 + a1 * bnm2;
    let mut fn_val = an / bn;

    while n < MAXITER {
        n += 1;
        anm2 = anm1;
        bnm2 = bnm1;
        anm1 = an;
        bnm1 = bn;
        let an_term = tau * tau + (mu - 0.5 + ell as f64 + n as f64).powi(2);
        let bn_term = 2.0 * (ell as f64 + mu + n as f64) * xi;
        an = bn_term * anm1 + an_term * anm2;
        bn = bn_term * bnm1 + an_term * bnm2;

        if an.abs() > recur_big || bn.abs() > recur_big {
            an /= recur_big;
            bn /= recur_big;
            anm1 /= recur_big;
            bnm1 /= recur_big;
            anm2 /= recur_big;
            bnm2 /= recur_big;
        }

        let old_fn = fn_val;
        fn_val = an / bn;
        let del = old_fn / fn_val;

        if (del - 1.0).abs() < 2.0 * EPSILON {
            break;
        }
    }

    let err = 4.0 * EPSILON * ((n as f64).sqrt() + 1.0) * fn_val.abs();

    if n >= MAXITER {
        Err(SfError::MaxIter)
    } else {
        Ok(SfResult::new(fn_val, err))
    }
}

fn conicalp_negmu_xgt1_cf1(
    mu: f64,
    ell: i32,
    tau: f64,
    x: f64,
) -> Result<SfResult, SfError> {
    const MAXK: i32 = 20000;
    let gamma = 1.0 - 1.0 / (x * x);
    let pre = ((x - 1.0).sqrt() * (x + 1.0).sqrt()) / (x * (2.0 * (ell as f64 + mu + 1.0)));
    let mut tk = 1.0;
    let mut sum = 1.0;
    let mut rhok = 0.0;
    let mut k = 1;

    while k < MAXK {
        let tlk = 2.0 * (ell as f64 + mu + k as f64);
        let l1k = ell as f64 + mu - 0.5 + 1.0 + k as f64;
        let ak = -(tau * tau + l1k * l1k) / (tlk * (tlk + 2.0)) * gamma;
        rhok = -ak * (1.0 + rhok) / (1.0 + ak * (1.0 + rhok));
        tk *= rhok;
        sum += tk;
        if (tk / sum).abs() < EPSILON {
            break;
        }
        k += 1;
    }

    let err = pre.abs() * tk.abs() + 2.0 * EPSILON * ((k as f64).sqrt() + 1.0) * (pre * sum).abs();

    if k >= MAXK {
        Err(SfError::MaxIter)
    } else {
        Ok(SfResult::new(pre * sum, err))
    }
}

fn olver_u1(beta2: f64, p: f64) -> f64 {
    (p - 1.0) / (24.0 * (1.0 + beta2)) * (3.0 + beta2 * (2.0 + 5.0 * p * (1.0 + p)))
}

fn olver_u2(beta2: f64, p: f64) -> f64 {
    let beta4 = beta2 * beta2;
    let p2 = p * p;
    let poly1 = 4.0 * beta4 + 84.0 * beta2 - 63.0;
    let poly2 = 16.0 * beta4 + 90.0 * beta2 - 81.0;
    let poly3 = beta2 * p2 * (97.0 * beta2 - 432.0 + 77.0 * p * (beta2 - 6.0) - 385.0 * beta2 * p2 * (1.0 + p));
    (1.0 - p) / (1152.0 * (1.0 + beta2)) * (poly1 + poly2 + poly3)
}

fn olver_b0_xi(mu: f64, xi: f64) -> f64 {
    (1.0 - 4.0 * mu * mu) / (8.0 * xi) * (1.0 / xi.tanh() - 1.0 / xi)
}

fn olver_a1_xi(mu: f64, xi: f64, x: f64) -> f64 {
    let b = olver_b0_xi(mu, xi);
    let psi = if (x - 1.0).abs() < EPSILON.powf(0.25) {
        let y = x - 1.0;
        let s = -1.0 / 3.0 + y * (2.0 / 15.0 - y * (61.0 / 945.0 - 452.0 / 14175.0 * y));
        (4.0 * mu * mu - 1.0) / 16.0 * s
    } else {
        (4.0 * mu * mu - 1.0) / 16.0 * (1.0 / (x * x - 1.0) - 1.0 / (xi * xi))
    };
    0.5 * xi * xi * b * b + (mu + 0.5) * b - psi + mu / 6.0 * (0.25 - mu * mu)
}

fn olver_b0_th(mu: f64, theta: f64) -> f64 {
    -(1.0 - 4.0 * mu * mu) / (8.0 * theta) * (1.0 / theta.tan() - 1.0 / theta)
}

fn olver_a1_th(mu: f64, theta: f64, x: f64) -> f64 {
    let b = olver_b0_th(mu, theta);
    let psi = if (x - 1.0).abs() < EPSILON.powf(0.25) {
        let y = 1.0 - x;
        let s = -1.0 / 3.0 + y * (2.0 / 15.0 - y * (61.0 / 945.0 - 452.0 / 14175.0 * y));
        (4.0 * mu * mu - 1.0) / 16.0 * s
    } else {
        (4.0 * mu * mu - 1.0) / 16.0 * (1.0 / (x * x - 1.0) + 1.0 / (theta * theta))
    };
    -0.5 * theta * theta * b * b + (mu + 0.5) * b - psi + mu / 6.0 * (0.25 - mu * mu)
}

fn conicalp_hyperg_large_x(
    mu: f64,
    tau: f64,
    y: f64,
) -> Result<(f64, f64), SfError> {
    const KMAX: i32 = 1000;
    let re_a = 0.25 - 0.5 * mu;
    let re_b = 0.75 - 0.5 * mu;
    let re_c = 1.0;
    let im_a = -0.5 * tau;
    let im_b = -0.5 * tau;
    let im_c = -tau;

    let mut re_sum = 1.0;
    let mut im_sum = 0.0;
    let mut re_term = 1.0;
    let mut im_term = 0.0;
    let mut k = 1;

    while k <= KMAX {
        let re_ak = re_a + k as f64 - 1.0;
        let re_bk = re_b + k as f64 - 1.0;
        let re_ck = re_c + k as f64 - 1.0;
        let im_ak = im_a;
        let im_bk = im_b;
        let im_ck = im_c;
        let den = re_ck * re_ck + im_ck * im_ck;
        let re_multiplier = ((re_ak * re_bk - im_ak * im_bk) * re_ck + im_ck * (im_ak * re_bk + re_ak * im_bk)) / den;
        let im_multiplier = ((im_ak * re_bk + re_ak * im_bk) * re_ck - im_ck * (re_ak * re_bk - im_ak * im_bk)) / den;
        let re_tmp = re_multiplier * re_term - im_multiplier * im_term;
        let im_tmp = im_multiplier * re_term + re_multiplier * im_term;
        let asum = re_sum.abs() + im_sum.abs();
        re_term = y / k as f64 * re_tmp;
        im_term = y / k as f64 * im_tmp;
        if (re_term / asum).abs() < EPSILON && (im_term / asum).abs() < EPSILON {
            break;
        }
        re_sum += re_term;
        im_sum += im_term;
        k += 1;
    }

    if k == KMAX {
        Err(SfError::MaxIter)
    } else {
        Ok((re_sum, im_sum))
    }
}

fn conicalp_xlt1_hyperg_a(mu: f64, tau: f64, x: f64) -> Result<SfResult, SfError> {
    let x2 = x * x;
    let err_amp = 1.0 + 1.0 / (EPSILON + (1.0 - x.abs()).abs());
    let pre_val = PI.sqrt() / (0.5 * (1.0 - x2).sqrt()).powf(mu);
    let pre_err = err_amp * EPSILON * (mu.abs() + 1.0) * pre_val.abs();

    let (f1_re, f1_im) = hyperg_2f1_conj(0.25 - 0.5 * mu, 0.5 * tau, 0.5, x2)?;
    let (f2_re, f2_im) = hyperg_2f1_conj(0.75 - 0.5 * mu, 0.5 * tau, 1.5, x2)?;

    let (ln_g1, arg_g1) = lngamma_complex(0.75 - 0.5 * mu, -0.5 * tau)?;
    let (ln_g2, arg_g2) = lngamma_complex(0.25 - 0.5 * mu, -0.5 * tau)?;

    let pre1 = (-2.0 * ln_g1).exp();
    let pre2 = -2.0 * x * (-2.0 * ln_g2).exp();

    let t1_val = pre1 * f1_re;
    let t1_err = pre1 * (f1_re.abs() + f1_im.abs()) + pre1 * (f1_re.abs() + f1_im.abs()) * EPSILON;
    let t2_val = pre2 * f2_re;
    let t2_err = pre2.abs() * (f2_re.abs() + f2_im.abs()) + pre2 * (f2_re.abs() + f2_im.abs()) * EPSILON;

    let val = pre_val * (t1_val + t2_val);
    let err = pre_val * (t1_err + t2_err) + pre_err * (t1_val + t2_val).abs() + 2.0 * EPSILON * val.abs();

    Ok(SfResult::new(val, err))
}

// Placeholder for unimplemented functions
fn hyperg_2f1_conj(a: f64, b: f64, c: f64, x: f64) -> Result<(f64, f64), SfError> {
    // Implement hypergeometric function 2F1 with complex conjugate parameters
    Err(SfError::Other("Not implemented"))
}

fn lngamma_complex(a: f64, b: f64) -> Result<(f64, f64), SfError> {
    // Implement complex log gamma function
    Err(SfError::Other("Not implemented"))
}

// Main conical functions
pub fn gsl_sf_conicalp_0(lambda: f64, x: f64) -> Result<SfResult, SfError> {
    if x <= -1.0 {
        Err(SfError::DomainError)
    } else if x == 1.0 {
        Ok(SfResult::new(1.0, 0.0))
    } else if lambda == 0.0 {
        if x < 1.0 {
            let th = x.acos();
            let s = (0.5 * th).sin();
            let k = ellint_kcomp(s)?;
            Ok(SfResult::new(2.0 / PI * k.val, 2.0 / PI * k.err + 2.0 * EPSILON * (2.0 / PI * k.val).abs()))
        } else {
            let xi = x.acosh();
            let c = (0.5 * xi).cosh();
            let t = (0.5 * xi).tanh();
            let k = ellint_kcomp(t)?;
            Ok(SfResult::new(2.0 / PI / c * k.val, 2.0 / PI / c * k.err + 2.0 * EPSILON * (2.0 / PI / c * k.val).abs()))
        }
    } else if (x <= 0.0 && lambda < 1000.0) || (x < 0.1 && lambda < 17.0) || (x < 0.2 && lambda < 5.0) {
        conicalp_xlt1_hyperg_a(0.0, lambda, x)
    } else if (x <= 0.2 && lambda < 17.0) || (x <= 1.5 && lambda < 20.0) {
        hyperg_2f1_conj_renorm(0.5, lambda, 1.0, (1.0 - x) / 2.0)
    } else if 1.5 < x && lambda < x.max(20.0) {
        let (p, lm) = conicalp_large_x(0.0, lambda, x)?;
        exp_mult_err(lm, 2.0 * EPSILON * lm.abs(), p.val, p.err)
    } else {
        let (v0, v1) = conicalp_0_v(x, lambda)?;
        if x < 1.0 {
            let th = x.acos();
            let sth = (1.0 - x * x).sqrt();
            let i0 = bessel_i0_scaled(th * lambda)?;
            let i1 = bessel_i1_scaled(th * lambda)?;
            let bessterm = v0 * i0.val + v1 * i1.val;
            let besserr = v0.abs() * i0.err + v1.abs() * i1.err;
            let arg1 = th * lambda;
            let sqts = (th / sth).sqrt();
            exp_mult_err(arg1, 4.0 * EPSILON * arg1.abs(), sqts * bessterm, sqts * besserr)
        } else {
            let sh = (x - 1.0).sqrt() * (x + 1.0).sqrt();
            let xi = (x + sh).ln();
            let j0 = bessel_j0(xi * lambda