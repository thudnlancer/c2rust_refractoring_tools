use std::f64::consts::{PI, SQRT_2, FRAC_1_SQRT_2, FRAC_1_PI};
use std::cmp::{max, min};

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

const GSL_DBL_EPSILON: f64 = 2.2204460492503131e-16;
const GSL_DBL_MAX: f64 = 1.7976931348623157e+308;
const GSL_SF_DOUBLEFACT_NMAX: i32 = 301;
const GSL_EDOM: i32 = 1;
const GSL_SUCCESS: i32 = 0;
const GSL_EOVRFLW: i32 = 2;
const GSL_NEGINF: f64 = f64::NEG_INFINITY;
const GSL_POSINF: f64 = f64::INFINITY;

fn gsl_is_odd(n: i32) -> bool {
    n % 2 != 0
}

fn gsl_fcmp(x: f64, y: f64, epsilon: f64) -> i32 {
    if (x - y).abs() < epsilon {
        0
    } else if x < y {
        -1
    } else {
        1
    }
}

fn pow2(n: i32) -> f64 {
    2.0f64.powi(n)
}

fn rnd(x: f64) -> f64 {
    if x >= 0.0 {
        (x + 0.5).floor()
    } else {
        (x - 0.5).floor()
    }
}

fn domain_error(result: &mut SfResult) -> i32 {
    result.val = f64::NAN;
    result.err = f64::NAN;
    GSL_EDOM
}

fn gsl_sf_fact(n: i32) -> f64 {
    if n < 0 {
        f64::NAN
    } else if n <= 1 {
        1.0
    } else {
        let mut res = 1.0;
        for i in 2..=n {
            res *= i as f64;
        }
        res
    }
}

fn gsl_sf_choose(n: i32, m: i32) -> f64 {
    if m < 0 || m > n {
        0.0
    } else if m == 0 || m == n {
        1.0
    } else {
        let k = min(m, n - m);
        let mut res = 1.0;
        for i in 1..=k {
            res *= (n - k + i) as f64 / i as f64;
        }
        res
    }
}

fn gsl_sf_doublefact_e(n: i32, result: &mut SfResult) -> i32 {
    if n < -1 {
        domain_error(result)
    } else if n <= 0 {
        result.val = 1.0;
        result.err = 0.0;
        GSL_SUCCESS
    } else {
        let mut val = 1.0;
        let mut i = n;
        while i > 0 {
            val *= i as f64;
            i -= 2;
        }
        result.val = val;
        result.err = GSL_DBL_EPSILON * val.abs();
        GSL_SUCCESS
    }
}

pub fn gsl_sf_hermite_prob_e(n: i32, x: f64, result: &mut SfResult) -> i32 {
    if n < 0 {
        domain_error(result)
    } else if n == 0 {
        result.val = 1.0;
        result.err = 0.0;
        GSL_SUCCESS
    } else if n == 1 {
        result.val = x;
        result.err = 0.0;
        GSL_SUCCESS
    } else if x == 0.0 {
        if gsl_is_odd(n) {
            result.val = 0.0;
            result.err = 0.0;
            GSL_SUCCESS
        } else {
            let mut status = GSL_SUCCESS;
            
            if n - 1 > GSL_SF_DOUBLEFACT_NMAX {
                status = GSL_EOVRFLW;
                result.val = if gsl_is_odd(n / 2) { GSL_NEGINF } else { GSL_POSINF };
                result.err = GSL_POSINF;
            } else {
                gsl_sf_doublefact_e(n - 1, result);
                if gsl_is_odd(n / 2) {
                    result.val = -result.val;
                }
            }
            status
        }
    } else {
        let mut status = GSL_SUCCESS;
        let abs_x = x.abs();
        let thresh1 = if abs_x > 1.0 { 0.9 * GSL_DBL_MAX / abs_x } else { GSL_DBL_MAX };
        let thresh2 = 0.9 * GSL_DBL_MAX;

        let mut p_n0 = 1.0;
        let mut p_n1 = x;
        let mut p_n = p_n1;

        let mut e_n0 = GSL_DBL_EPSILON;
        let mut e_n1 = abs_x * GSL_DBL_EPSILON;
        let mut e_n = e_n1;

        for j in 1..n {
            if p_n1.abs() > thresh1 || p_n0.abs() > thresh2 / j as f64 {
                status = GSL_EOVRFLW;
                break;
            }

            p_n = x * p_n1 - j as f64 * p_n0;
            p_n0 = p_n1;
            p_n1 = p_n;

            e_n = abs_x * e_n1 + j as f64 * e_n0;
            e_n0 = e_n1;
            e_n1 = e_n;
        }

        result.val = p_n;
        result.err = e_n + result.val.abs() * GSL_DBL_EPSILON;
        status
    }
}

pub fn gsl_sf_hermite_prob(n: i32, x: f64) -> f64 {
    let mut result = SfResult::new(0.0, 0.0);
    gsl_sf_hermite_prob_e(n, x, &mut result);
    result.val
}

pub fn gsl_sf_hermite_prob_deriv_e(m: i32, n: i32, x: f64, result: &mut SfResult) -> i32 {
    if n < 0 || m < 0 {
        domain_error(result)
    } else if n < m {
        result.val = 0.0;
        result.err = 0.0;
        GSL_SUCCESS
    } else {
        let f = gsl_sf_choose(n, m) * gsl_sf_fact(m);
        let mut he = SfResult::new(0.0, 0.0);
        let status = gsl_sf_hermite_prob_e(n - m, x, &mut he);
        
        if status == GSL_SUCCESS {
            result.val = he.val * f;
            result.err = he.err * f + GSL_DBL_EPSILON * result.val.abs();
        } else {
            result.val = he.val;
            result.err = GSL_POSINF;
        }
        status
    }
}

pub fn gsl_sf_hermite_prob_deriv(m: i32, n: i32, x: f64) -> f64 {
    let mut result = SfResult::new(0.0, 0.0);
    gsl_sf_hermite_prob_deriv_e(m, n, x, &mut result);
    result.val
}

pub fn gsl_sf_hermite_e(n: i32, x: f64, result: &mut SfResult) -> i32 {
    if n < 0 {
        domain_error(result)
    } else if n == 0 {
        result.val = 1.0;
        result.err = 0.0;
        GSL_SUCCESS
    } else if n == 1 {
        result.val = 2.0 * x;
        result.err = 0.0;
        GSL_SUCCESS
    } else if x == 0.0 {
        if gsl_is_odd(n) {
            result.val = 0.0;
            result.err = 0.0;
            GSL_SUCCESS
        } else {
            let mut status = GSL_SUCCESS;
            let m = n >> 1;
            
            if n - 1 > GSL_SF_DOUBLEFACT_NMAX {
                status = GSL_EOVRFLW;
                result.val = if gsl_is_odd(m) { GSL_NEGINF } else { GSL_POSINF };
                result.err = GSL_POSINF;
            } else {
                let f = 2.0f64.powi(m);
                gsl_sf_doublefact_e(n - 1, result);
                
                if result.val > 0.9 * GSL_DBL_MAX / f {
                    status = GSL_EOVRFLW;
                    result.val = if gsl_is_odd(m) { GSL_NEGINF } else { GSL_POSINF };
                    result.err = GSL_POSINF;
                } else {
                    result.val *= f;
                    result.err *= f;
                    if gsl_is_odd(m) {
                        result.val = -result.val;
                    }
                }
            }
            status
        }
    } else {
        let mut status = GSL_SUCCESS;
        let two_x = 2.0 * x;
        let abs_two_x = two_x.abs();
        let thresh1 = if abs_two_x > 1.0 { 0.9 * GSL_DBL_MAX / abs_two_x } else { GSL_DBL_MAX };
        let thresh2 = 0.9 * GSL_DBL_MAX / 2.0;

        let mut p_n0 = 1.0;
        let mut p_n1 = two_x;
        let mut p_n = p_n1;

        let mut e_n0 = GSL_DBL_EPSILON;
        let mut e_n1 = 2.0 * x.abs() * GSL_DBL_EPSILON;
        let mut e_n = e_n1;

        for j in 1..n {
            if p_n1.abs() > thresh1 || p_n0.abs() > thresh2 / j as f64 {
                status = GSL_EOVRFLW;
                break;
            }

            p_n = two_x * p_n1 - 2.0 * j as f64 * p_n0;
            p_n0 = p_n1;
            p_n1 = p_n;

            e_n = 2.0 * (x.abs() * e_n1 + j as f64 * e_n0);
            e_n0 = e_n1;
            e_n1 = e_n;
        }

        result.val = p_n;
        result.err = e_n + result.val.abs() * GSL_DBL_EPSILON;
        status
    }
}

pub fn gsl_sf_hermite(n: i32, x: f64) -> f64 {
    let mut result = SfResult::new(0.0, 0.0);
    gsl_sf_hermite_e(n, x, &mut result);
    result.val
}

pub fn gsl_sf_hermite_deriv_e(m: i32, n: i32, x: f64, result: &mut SfResult) -> i32 {
    if n < 0 || m < 0 {
        domain_error(result)
    } else if n < m {
        result.val = 0.0;
        result.err = 0.0;
        GSL_SUCCESS
    } else {
        let f = gsl_sf_choose(n, m) * gsl_sf_fact(m) * pow2(m);
        let mut h = SfResult::new(0.0, 0.0);
        let status = gsl_sf_hermite_e(n - m, x, &mut h);
        
        if status == GSL_SUCCESS {
            result.val = h.val * f;
            result.err = h.err * f + GSL_DBL_EPSILON * result.val.abs();
        } else {
            result.val = h.val;
            result.err = GSL_POSINF;
        }
        status
    }
}

pub fn gsl_sf_hermite_deriv(m: i32, n: i32, x: f64) -> f64 {
    let mut result = SfResult::new(0.0, 0.0);
    gsl_sf_hermite_deriv_e(m, n, x, &mut result);
    result.val
}

pub fn gsl_sf_hermite_func_e(n: i32, x: f64, result: &mut SfResult) -> i32 {
    if n < 0 {
        domain_error(result)
    } else if x == 0.0 {
        if gsl_is_odd(n) {
            result.val = 0.0;
            result.err = 0.0;
            GSL_SUCCESS
        } else {
            let mut f = if gsl_is_odd(n / 2) { -1.0 } else { 1.0 };
            
            for j in (1..n).step_by(2) {
                f *= (j as f64 / (j + 1) as f64).sqrt();
            }
            
            result.val = f / PI.sqrt();
            result.err = GSL_DBL_EPSILON * result.val.abs();
            GSL_SUCCESS
        }
    } else if n == 0 {
        result.val = (-0.5 * x * x).exp() / PI.sqrt();
        result.err = GSL_DBL_EPSILON * result.val.abs();
        GSL_SUCCESS
    } else if n == 1 {
        result.val = SQRT_2 * x * (-0.5 * x * x).exp() / PI.sqrt();
        result.err = GSL_DBL_EPSILON * result.val.abs();
        GSL_SUCCESS
    } else {
        let mut hi2 = 1.0 / PI.sqrt();
        let mut hi1 = SQRT_2 * x * hi2;
        let mut hi = 0.0;
        let mut sum_log_scale = 0.0;

        for i in 2..=n {
            hi = (2.0 / i as f64).sqrt() * x * hi1 - ((i - 1) as f64 / i as f64).sqrt() * hi2;
            hi2 = hi1;
            hi1 = hi;

            let abshi = hi.abs();
            if abshi > 1.0 {
                let log_scale = rnd(abshi.ln());
                let scale = (-log_scale).exp();

                hi *= scale;
                hi1 *= scale;
                hi2 *= scale;
                sum_log_scale += log_scale;
            }
        }

        result.val = hi * (-0.5 * x * x + sum_log_scale).exp();
        result.err = n as f64 * GSL_DBL_EPSILON * result.val.abs();
        GSL_SUCCESS
    }
}

pub fn gsl_sf_hermite_func(n: i32, x: f64) -> f64 {
    let mut result = SfResult::new(0.0, 0.0);
    gsl_sf_hermite_func_e(n, x, &mut result);
    result.val
}

// ... (remaining functions follow the same pattern of conversion)