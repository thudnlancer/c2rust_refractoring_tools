use std::f64::consts::{PI, EULER};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct Result {
    pub val: f64,
    pub err: f64,
}

impl Result {
    pub fn new(val: f64, err: f64) -> Self {
        Self { val, err }
    }
}

const DBL_EPSILON: f64 = f64::EPSILON;
const DBL_MAX: f64 = f64::MAX;
const LOG_DBL_MAX: f64 = f64::MAX.log10();
const LOG_DBL_MIN: f64 = f64::MIN_POSITIVE.log10();
const SQRT_DBL_EPSILON: f64 = f64::EPSILON.sqrt();

fn c0sq(eta: f64) -> f64 {
    let twopieta = 2.0 * PI * eta;

    if eta.abs() < DBL_EPSILON {
        1.0
    } else if twopieta > LOG_DBL_MAX {
        0.0
    } else {
        let scale = (twopieta).exp_m1();
        twopieta / scale
    }
}

fn cleta(l: f64, eta: f64) -> Result {
    let mut ln1 = Result::new(0.0, 0.0);
    let mut ln2 = Result::new(0.0, 0.0);
    let mut sgn = 1.0;

    if (eta / (l + 1.0)).abs() < DBL_EPSILON {
        ln1 = lngamma(l + 1.0);
    } else {
        let (ln_val, _) = lngamma_complex(l + 1.0, eta);
        ln1 = Result::new(ln_val, 0.0);
    }

    ln2 = lngamma(2.0 * (l + 1.0));
    if l < -1.0 {
        sgn = -sgn;
    }

    let arg_val = l * f64::consts::LN_2 - 0.5 * eta * PI + ln1.val - ln2.val;
    let arg_err = ln1.err + ln2.err + DBL_EPSILON * (l * f64::consts::LN_2).abs() + (0.5 * eta * PI).abs();
    exp_err(arg_val, arg_err)
}

pub fn coulomb_cl_e(lam: f64, eta: f64) -> Result {
    if lam <= -1.0 {
        Result::new(0.0, 0.0)
    } else if lam.abs() < DBL_EPSILON {
        let val = c0sq(eta).sqrt();
        Result::new(val, 2.0 * DBL_EPSILON * val)
    } else {
        cleta(lam, eta)
    }
}

pub fn coulomb_cl_array(lam_min: f64, kmax: i32, eta: f64, cl: &mut [f64]) {
    let cl_0 = coulomb_cl_e(lam_min, eta);
    cl[0] = cl_0.val;

    for k in 1..=kmax {
        let l = lam_min + k as f64;
        cl[k as usize] = cl[(k - 1) as usize] * (l.hypot(eta)) / (l * (2.0 * l + 1.0));
    }
}

fn coulomb_connection(lam: f64, eta: f64, cos_phi: &mut f64, sin_phi: &mut f64) -> Result<(), &'static str> {
    if eta > -LOG_DBL_MIN / 2.0 * PI - 1.0 {
        *cos_phi = 1.0;
        *sin_phi = 0.0;
        Err("underflow")
    } else if eta > -LOG_DBL_EPSILON / (4.0 * PI) {
        let eps = 2.0 * (-2.0 * PI * eta).exp();
        let tpl = (PI * lam).tan();
        let dth = eps * tpl / (tpl * tpl + 1.0);
        *cos_phi = -1.0 + 0.5 * dth * dth;
        *sin_phi = -dth;
        Ok(())
    } else {
        let x = (PI * eta).tanh() / (PI * lam).tan();
        let phi = -x.atan() - (lam + 0.5) * PI;
        *cos_phi = phi.cos();
        *sin_phi = phi.sin();
        Ok(())
    }
}

fn coulomb_fg_series(lam: f64, eta: f64, x: f64, f: &mut Result, g: &mut Result) -> Result<(), &'static str> {
    const MAX_ITER: usize = 800;
    let clam_a = cleta(lam, eta);
    let clam_b = cleta(-lam - 1.0, eta);
    let tlp1 = 2.0 * lam + 1.0;
    let pow_x = x.powf(lam);
    let mut cos_phi_lam = 0.0;
    let mut sin_phi_lam = 0.0;

    let mut ua_mm2 = 1.0;
    let mut ua_mm1 = x * eta / (lam + 1.0);
    let mut ub_mm2 = 1.0;
    let mut ub_mm1 = -x * eta / lam;
    let mut a_sum = ua_mm2 + ua_mm1;
    let mut b_sum = ub_mm2 + ub_mm1;
    let mut a_abs_del_prev = a_sum.abs();
    let mut b_abs_del_prev = b_sum.abs();

    coulomb_connection(lam, eta, &mut cos_phi_lam, &mut sin_phi_lam)?;

    if sin_phi_lam == 0.0 {
        f.val = 0.0;
        f.err = 0.0;
        return Err("overflow");
    }

    let mut m = 2;
    while m < MAX_ITER {
        let ua_m = x * (2.0 * eta * ua_mm1 - x * ua_mm2) / (m as f64 * (m as f64 + tlp1));
        let ub_m = x * (2.0 * eta * ub_mm1 - x * ub_mm2) / (m as f64 * (m as f64 - tlp1));
        a_sum += ua_m;
        b_sum += ub_m;
        let abs_da = ua_m.abs();
        let abs_db = ub_m.abs();

        if m > 15 {
            let max_abs_da = abs_da.max(a_abs_del_prev);
            let max_abs_db = abs_db.max(b_abs_del_prev);
            let abs_a = a_sum.abs();
            let abs_b = b_sum.abs();
            if max_abs_da / (max_abs_da + abs_a) < 4.0 * DBL_EPSILON &&
               max_abs_db / (max_abs_db + abs_b) < 4.0 * DBL_EPSILON {
                break;
            }
        }

        a_abs_del_prev = abs_da;
        b_abs_del_prev = abs_db;
        ua_mm2 = ua_mm1;
        ua_mm1 = ua_m;
        ub_mm2 = ub_mm1;
        ub_mm1 = ub_m;
        m += 1;
    }

    let fa = Result::new(a_sum * clam_a.val * pow_x * x, 
                        a_sum.abs() * clam_a.err * pow_x * x + 2.0 * DBL_EPSILON * (a_sum * clam_a.val * pow_x * x).abs());
    let fb = Result::new(b_sum * clam_b.val / pow_x, 
                        b_sum.abs() * clam_b.err / pow_x + 2.0 * DBL_EPSILON * (b_sum * clam_b.val / pow_x).abs());

    f.val = fa.val;
    f.err = fa.err;
    g.val = (fa.val * cos_phi_lam - fb.val) / sin_phi_lam;
    g.err = (fa.err * cos_phi_lam.abs() + fb.err) / sin_phi_lam.abs();

    if m >= MAX_ITER {
        Err("max iterations")
    } else {
        Ok(())
    }
}

// Note: Additional helper functions like lngamma, lngamma_complex, exp_err would need to be implemented
// These are placeholders for the actual implementations which would be more complex
fn lngamma(x: f64) -> Result {
    Result::new(x.ln_gamma().0, 0.0)
}

fn lngamma_complex(a: f64, b: f64) -> (f64, f64) {
    (a.ln(), b.ln())
}

fn exp_err(x: f64, err: f64) -> Result {
    let val = x.exp();
    Result::new(val, val * err)
}