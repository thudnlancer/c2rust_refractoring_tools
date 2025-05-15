use std::f64;
use std::i32;

const GSL_DBL_EPSILON: f64 = 2.2204460492503131e-16;
const GSL_LOG_DBL_MAX: f64 = 709.78271289338397;
const GSL_LOG_DBL_MIN: f64 = -708.39641853226408;
const GSL_SQRT_DBL_MAX: f64 = 1.3407807929942596e+154;
const GSL_SQRT_DBL_MIN: f64 = 1.4916681462400413e-154;
const GSL_ROOT3_DBL_EPSILON: f64 = 6.0554544523933429e-6;
const M_LN10: f64 = 2.30258509299404568402;

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct GslSfResultE10 {
    pub val: f64,
    pub err: f64,
    pub e10: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Domain,
    Overflow,
    Underflow,
    MaxIter,
    Success,
}

fn is_odd(n: i32) -> bool {
    n % 2 != 0
}

fn exprel_n_cf(n: f64, x: f64) -> Result<GslSfResult, GslError> {
    const RECUR_BIG: f64 = 1.3407807929942596e+154;
    const MAXITER: i32 = 5000;

    let mut n_iter = 1;
    let mut anm2 = 1.0;
    let mut bnm2 = 0.0;
    let mut anm1 = 0.0;
    let mut bnm1 = 1.0;
    let a1 = 1.0;
    let b1 = 1.0;
    let a2 = -x;
    let b2 = n + 1.0;

    let mut an = b1 * anm1 + a1 * anm2;
    let mut bn = b1 * bnm1 + a1 * bnm2;

    n_iter += 1;
    anm2 = anm1;
    bnm2 = bnm1;
    anm1 = an;
    bnm1 = bn;
    an = b2 * anm1 + a2 * anm2;
    bn = b2 * bnm1 + a2 * bnm2;

    let mut fn_val = an / bn;

    while n_iter < MAXITER {
        let old_fn = fn_val;
        n_iter += 1;
        anm2 = anm1;
        bnm2 = bnm1;
        anm1 = an;
        bnm1 = bn;

        let an_term = if is_odd(n_iter) {
            ((n_iter - 1) as f64 / 2.0) * x
        } else {
            -(n + (n_iter as f64 / 2.0) - 1.0) * x
        };

        let bn_term = n + n_iter as f64 - 1.0;
        an = bn_term * anm1 + an_term * anm2;
        bn = bn_term * bnm1 + an_term * bnm2;

        if an.abs() > RECUR_BIG || bn.abs() > RECUR_BIG {
            an /= RECUR_BIG;
            bn /= RECUR_BIG;
            anm1 /= RECUR_BIG;
            bnm1 /= RECUR_BIG;
            anm2 /= RECUR_BIG;
            bnm2 /= RECUR_BIG;
        }

        fn_val = an / bn;
        let del = old_fn / fn_val;

        if (del - 1.0).abs() < 2.0 * GSL_DBL_EPSILON {
            break;
        }
    }

    let result = GslSfResult {
        val: fn_val,
        err: 4.0 * (n_iter as f64 + 1.0) * GSL_DBL_EPSILON * fn_val.abs(),
    };

    if n_iter == MAXITER {
        Err(GslError::MaxIter)
    } else {
        Ok(result)
    }
}

pub fn gsl_sf_exp_e(x: f64) -> Result<GslSfResult, GslError> {
    if x > GSL_LOG_DBL_MAX {
        Err(GslError::Overflow)
    } else if x < GSL_LOG_DBL_MIN {
        Err(GslError::Underflow)
    } else {
        let val = x.exp();
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    }
}

pub fn gsl_sf_exp_e10_e(x: f64) -> Result<GslSfResultE10, GslError> {
    if x > i32::MAX as f64 - 1.0 {
        Err(GslError::Overflow)
    } else if x < i32::MIN as f64 + 1.0 {
        Err(GslError::Underflow)
    } else {
        let n = if x > GSL_LOG_DBL_MAX || x < GSL_LOG_DBL_MIN {
            (x / M_LN10).floor() as i32
        } else {
            0
        };
        let val = (x - n as f64 * M_LN10).exp();
        Ok(GslSfResultE10 {
            val,
            err: 2.0 * (x.abs() + 1.0) * GSL_DBL_EPSILON * val.abs(),
            e10: n,
        })
    }
}

pub fn gsl_sf_exp_mult_e(x: f64, y: f64) -> Result<GslSfResult, GslError> {
    let ay = y.abs();

    if y == 0.0 {
        Ok(GslSfResult { val: 0.0, err: 0.0 })
    } else if (x < 0.5 * GSL_LOG_DBL_MAX && x > 0.5 * GSL_LOG_DBL_MIN)
        && (ay < 0.8 * GSL_SQRT_DBL_MAX && ay > 1.2 * GSL_SQRT_DBL_MIN)
    {
        let ex = x.exp();
        let val = y * ex;
        Ok(GslSfResult {
            val,
            err: (2.0 + x.abs()) * GSL_DBL_EPSILON * val.abs(),
        })
    } else {
        let ly = ay.ln();
        let lnr = x + ly;

        if lnr > GSL_LOG_DBL_MAX - 0.01 {
            Err(GslError::Overflow)
        } else if lnr < GSL_LOG_DBL_MIN + 0.01 {
            Err(GslError::Underflow)
        } else {
            let sy = y.signum();
            let m = x.floor();
            let n = ly.floor();
            let a = x - m;
            let b = ly - n;
            let berr = 2.0 * GSL_DBL_EPSILON * (ly.abs() + n.abs());
            let val = sy * (m + n).exp() * (a + b).exp();
            let mut err = berr * val.abs();
            err += 2.0 * GSL_DBL_EPSILON * (m + n + 1.0) * val.abs();
            Ok(GslSfResult { val, err })
        }
    }
}

// ... (其他函数类似实现)

pub fn gsl_sf_expm1_e(x: f64) -> Result<GslSfResult, GslError> {
    const CUT: f64 = 0.002;

    if x < GSL_LOG_DBL_MIN {
        Ok(GslSfResult {
            val: -1.0,
            err: GSL_DBL_EPSILON,
        })
    } else if x < -CUT {
        let val = x.exp() - 1.0;
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else if x < CUT {
        let val = x * (1.0 + 0.5 * x * (1.0 + x / 3.0 * (1.0 + 0.25 * x * (1.0 + 0.2 * x))));
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else if x < GSL_LOG_DBL_MAX {
        let val = x.exp() - 1.0;
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else {
        Err(GslError::Overflow)
    }
}

// ... (继续实现剩余函数)

pub fn gsl_sf_exprel_n_e(n: i32, x: f64) -> Result<GslSfResult, GslError> {
    if n < 0 {
        Err(GslError::Domain)
    } else if x == 0.0 {
        Ok(GslSfResult { val: 1.0, err: 0.0 })
    } else if x.abs() < GSL_ROOT3_DBL_EPSILON * n as f64 {
        let val = 1.0 + x / (n + 1) as f64 * (1.0 + x / (n + 2) as f64);
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON,
        })
    } else if n == 0 {
        gsl_sf_exp_e(x)
    } else if n == 1 {
        gsl_sf_exprel_e(x)
    } else if n == 2 {
        gsl_sf_exprel_2_e(x)
    } else {
        if x > n as f64 && (-x + n as f64 * (1.0 + (x / n as f64).ln()) < GSL_DBL_EPSILON.ln()) {
            // ... (实现剩余逻辑)
            unimplemented!()
        } else {
            unimplemented!()
        }
    }
}