use std::f64;
use std::f64::consts::LN_10;

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

const RECUR_BIG: f64 = 1.3407807929942596e+154;
const MAXITER: i32 = 5000;
const GSL_DBL_EPSILON: f64 = 2.2204460492503131e-16;
const LOG10: f64 = 2.30258509299404568402;

fn gsl_max(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn exprel_n_cf(n: f64, x: f64) -> Result<GslSfResult, GslError> {
    let mut anm2 = 1.0;
    let mut bnm2 = 0.0;
    let mut anm1 = 0.0;
    let mut bnm1 = 1.0;
    let a1 = 1.0;
    let b1 = 1.0;
    let mut a2 = -x;
    let mut b2 = n + 1.0;

    let mut an = b1 * anm1 + a1 * anm2;
    let mut bn = b1 * bnm1 + a1 * bnm2;
    
    anm2 = anm1;
    bnm2 = bnm1;
    anm1 = an;
    bnm1 = bn;
    
    an = b2 * anm1 + a2 * anm2;
    bn = b2 * bnm1 + a2 * bnm2;
    
    let mut fn_val = an / bn;
    let mut n_iter = 2;

    while n_iter < MAXITER {
        let old_fn = fn_val;
        n_iter += 1;
        
        anm2 = anm1;
        bnm2 = bnm1;
        anm1 = an;
        bnm1 = bn;
        
        let an_term = if n_iter % 2 != 0 {
            ((n_iter - 1) / 2) as f64 * x
        } else {
            -(n + (n_iter / 2) as f64 - 1.0) * x
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

    if n_iter == MAXITER {
        Err(GslError::MaxIter)
    } else {
        Ok(GslSfResult {
            val: fn_val,
            err: 4.0 * (n_iter as f64 + 1.0) * GSL_DBL_EPSILON * fn_val.abs(),
        })
    }
}

pub fn gsl_sf_exp_e(x: f64) -> Result<GslSfResult, GslError> {
    if x > 7.0978271289338397e+02 {
        Err(GslError::Overflow)
    } else if x < -7.0839641853226408e+02 {
        Ok(GslSfResult {
            val: 0.0,
            err: 2.2250738585072014e-308,
        })
    } else {
        let val = x.exp();
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    }
}

pub fn gsl_sf_exp_e10_e(x: f64) -> Result<GslSfResultE10, GslError> {
    if x > (i32::MAX - 1) as f64 {
        Err(GslError::Overflow)
    } else if x < (i32::MIN + 1) as f64 {
        Ok(GslSfResultE10 {
            val: 0.0,
            err: 2.2250738585072014e-308,
            e10: 0,
        })
    } else {
        let n = if x > 7.0978271289338397e+02 || x < -7.0839641853226408e+02 {
            (x / LOG10).floor() as i32
        } else {
            0
        };
        let val = (x - n as f64 * LOG10).exp();
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
    } else if x < 0.5 * 7.0978271289338397e+02
        && x > 0.5 * -7.0839641853226408e+02
        && ay < 0.8 * 1.3407807929942596e+154
        && ay > 1.2 * 1.4916681462400413e-154
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
        if lnr > 7.0978271289338397e+02 - 0.01 {
            Err(GslError::Overflow)
        } else if lnr < -7.0839641853226408e+02 + 0.01 {
            Ok(GslSfResult {
                val: 0.0,
                err: 2.2250738585072014e-308,
            })
        } else {
            let sy = if y >= 0.0 { 1.0 } else { -1.0 };
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

pub fn gsl_sf_exp_mult_e10_e(x: f64, y: f64) -> Result<GslSfResultE10, GslError> {
    let ay = y.abs();
    if y == 0.0 {
        Ok(GslSfResultE10 {
            val: 0.0,
            err: 0.0,
            e10: 0,
        })
    } else if x < 0.5 * 7.0978271289338397e+02
        && x > 0.5 * -7.0839641853226408e+02
        && ay < 0.8 * 1.3407807929942596e+154
        && ay > 1.2 * 1.4916681462400413e-154
    {
        let ex = x.exp();
        let val = y * ex;
        Ok(GslSfResultE10 {
            val,
            err: (2.0 + x.abs()) * GSL_DBL_EPSILON * val.abs(),
            e10: 0,
        })
    } else {
        let ly = ay.ln();
        let l10_val = (x + ly) / LOG10;
        if l10_val > (i32::MAX - 1) as f64 {
            Err(GslError::Overflow)
        } else if l10_val < (i32::MIN + 1) as f64 {
            Ok(GslSfResultE10 {
                val: 0.0,
                err: 2.2250738585072014e-308,
                e10: 0,
            })
        } else {
            let sy = if y >= 0.0 { 1.0 } else { -1.0 };
            let n = l10_val.floor() as i32;
            let arg_val = (l10_val - n as f64) * LOG10;
            let arg_err = 2.0 * GSL_DBL_EPSILON
                * (x.abs() + ly.abs() + LOG10 * (n as f64).abs());
            let val = sy * arg_val.exp();
            let mut err = arg_err * val.abs();
            err += 2.0 * GSL_DBL_EPSILON * val.abs();
            Ok(GslSfResultE10 { val, err, e10: n })
        }
    }
}

pub fn gsl_sf_expm1_e(x: f64) -> Result<GslSfResult, GslError> {
    let cut = 0.002;
    if x < -7.0839641853226408e+02 {
        Ok(GslSfResult {
            val: -1.0,
            err: GSL_DBL_EPSILON,
        })
    } else if x < -cut {
        let val = x.exp() - 1.0;
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else if x < cut {
        let val = x * (1.0 + 0.5 * x * (1.0 + x / 3.0 * (1.0 + 0.25 * x * (1.0 + 0.2 * x))));
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else if x < 7.0978271289338397e+02 {
        let val = x.exp() - 1.0;
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else {
        Err(GslError::Overflow)
    }
}

pub fn gsl_sf_exprel_e(x: f64) -> Result<GslSfResult, GslError> {
    let cut = 0.002;
    if x < -7.0839641853226408e+02 {
        Ok(GslSfResult {
            val: -1.0 / x,
            err: GSL_DBL_EPSILON * (-1.0 / x).abs(),
        })
    } else if x < -cut {
        let val = (x.exp() - 1.0) / x;
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else if x < cut {
        let val = 1.0 + 0.5 * x * (1.0 + x / 3.0 * (1.0 + 0.25 * x * (1.0 + 0.2 * x)));
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else if x < 7.0978271289338397e+02 {
        let val = (x.exp() - 1.0) / x;
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else {
        Err(GslError::Overflow)
    }
}

pub fn gsl_sf_exprel_2_e(x: f64) -> Result<GslSfResult, GslError> {
    let cut = 0.002;
    if x < -7.0839641853226408e+02 {
        let val = -2.0 / x * (1.0 + 1.0 / x);
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else if x < -cut {
        let val = 2.0 * (x.exp() - 1.0 - x) / (x * x);
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else if x < cut {
        let val = 1.0 + x / 3.0 * (1.0 + 0.25 * x * (1.0 + 0.2 * x * (1.0 + x / 6.0)));
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else if x < 7.0978271289338397e+02 {
        let val = 2.0 * (x.exp() - 1.0 - x) / (x * x);
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else {
        Err(GslError::Overflow)
    }
}

pub fn gsl_sf_exprel_n_e(n: i32, x: f64) -> Result<GslSfResult, GslError> {
    if n < 0 {
        Err(GslError::Domain)
    } else if x == 0.0 {
        Ok(GslSfResult { val: 1.0, err: 0.0 })
    } else if x.abs() < 6.0554544523933429e-06 * n as f64 {
        let val = 1.0 + x / (n + 1) as f64 * (1.0 + x / (n + 2) as f64);
        Ok(GslSfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON,
        })
    } else {
        match n {
            0 => gsl_sf_exp_e(x),
            1 => gsl_sf_exprel_e(x),
            2 => gsl_sf_exprel_2_e(x),
            _ => {
                if x > n as f64
                    && -x + n as f64 * (1.0 + (x / n as f64).ln())
                        < -3.6043653389117154e+01
                {
                    let lnf_n = lnfact(n as u32)?;
                    let lnterm = n as f64 * x.ln();
                    let lnr_val = x + lnf_n.val - lnterm;
                    let lnr_err = GSL_DBL_EPSILON * (x.abs() + lnf_n.val.abs() + lnterm.abs());
                    let lnr_err = lnr_err + lnf_n.err;
                    gsl_sf_exp_err_e(lnr_val, lnr_err)
                } else if x > n as f64 {
                    let ln_x = x.ln();
                    let lnf_n = lnfact(n as u32)?;
                    let lg_n = lnf_n.val - (n as f64).ln();
                    let lnpre_val = x + lnf_n.val - n as f64 * ln_x;
                    let lnpre_err = GSL_DBL_EPSILON
                        * (x.abs() + lnf_n.val.abs() + (n as f64 * ln_x).abs());
                    let lnpre_err = lnpre_err + lnf_n.err;
                    
                    if lnpre_val < 7.0978271289338397e+02 - 5.0 {
                        let pre = gsl_sf_exp_err_e(lnpre_val, lnpre_err)?;
                        let ln_big_g_ratio_pre = -x + (n - 1) as