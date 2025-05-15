use std::f64::consts::{INFINITY, NAN};
use std::f64::{abs, log, sqrt};

#[derive(Debug, Copy, Clone)]
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

fn loc_max3(a: i32, b: i32, c: i32) -> i32 {
    let d = if a > b { a } else { b };
    if d > c { d } else { c }
}

fn loc_min3(a: i32, b: i32, c: i32) -> i32 {
    let d = if a < b { a } else { b };
    if d < c { d } else { c }
}

fn loc_min5(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let f = if a < b { a } else { b };
    let g = if c < d { c } else { d };
    let h = if f < g { f } else { g };
    if e < h { e } else { h }
}

fn delta(ta: i32, tb: i32, tc: i32) -> Result<GslSfResult, GslError> {
    let f1 = gsl_sf_fact(((ta + tb - tc) / 2) as u32)?;
    let f2 = gsl_sf_fact(((ta + tc - tb) / 2) as u32)?;
    let f3 = gsl_sf_fact(((tb + tc - ta) / 2) as u32)?;
    let f4 = gsl_sf_fact(((ta + tb + tc) / 2 + 1) as u32)?;

    let val = f1.val * f2.val * f3.val / f4.val;
    let err = 4.0 * 2.2204460492503131e-16 * abs(val);

    Ok(GslSfResult { val, err })
}

fn triangle_selection_fails(two_ja: i32, two_jb: i32, two_jc: i32) -> bool {
    two_jb < abs(two_ja - two_jc) || two_jb > two_ja + two_jc || (two_ja + two_jb + two_jc) % 2 != 0
}

fn m_selection_fails(
    two_ja: i32,
    two_jb: i32,
    two_jc: i32,
    two_ma: i32,
    two_mb: i32,
    two_mc: i32,
) -> bool {
    abs(two_ma) > two_ja
        || abs(two_mb) > two_jb
        || abs(two_mc) > two_jc
        || (two_ja + two_ma) % 2 != 0
        || (two_jb + two_mb) % 2 != 0
        || (two_jc + two_mc) % 2 != 0
        || two_ma + two_mb + two_mc != 0
}

pub fn gsl_sf_coupling_3j_e(
    two_ja: i32,
    two_jb: i32,
    two_jc: i32,
    two_ma: i32,
    two_mb: i32,
    two_mc: i32,
) -> Result<GslSfResult, GslError> {
    if two_ja < 0 || two_jb < 0 || two_jc < 0 {
        return Err(GslError::Domain);
    }

    if triangle_selection_fails(two_ja, two_jb, two_jc)
        || m_selection_fails(two_ja, two_jb, two_jc, two_ma, two_mb, two_mc)
    {
        return Ok(GslSfResult { val: 0.0, err: 0.0 });
    }

    if two_ma == 0 && two_mb == 0 && two_mc == 0 && (two_ja + two_jb + two_jc) % 4 == 2 {
        return Ok(GslSfResult { val: 0.0, err: 0.0 });
    }

    let jca = (-two_ja + two_jb + two_jc) / 2;
    let jcb = (two_ja - two_jb + two_jc) / 2;
    let jcc = (two_ja + two_jb - two_jc) / 2;
    let jmma = (two_ja - two_ma) / 2;
    let jmmb = (two_jb - two_mb) / 2;
    let jmmc = (two_jc - two_mc) / 2;
    let jpma = (two_ja + two_ma) / 2;
    let jpmb = (two_jb + two_mb) / 2;
    let jpmc = (two_jc + two_mc) / 2;
    let jsum = (two_ja + two_jb + two_jc) / 2;
    let kmin = loc_max3(0, jpmb - jmmc, jmma - jpmc);
    let kmax = loc_min3(jcc, jmma, jpmb);
    let mut sign = if (kmin - jpma + jmmb) % 2 != 0 { -1 } else { 1 };

    let mut sum_pos = 0.0;
    let mut sum_neg = 0.0;
    let mut sum_err = 0.0;

    let bcn1 = gsl_sf_lnchoose(two_ja as u32, jcc as u32)?;
    let bcn2 = gsl_sf_lnchoose(two_jb as u32, jcc as u32)?;
    let bcd1 = gsl_sf_lnchoose((jsum + 1) as u32, jcc as u32)?;
    let bcd2 = gsl_sf_lnchoose(two_ja as u32, jmma as u32)?;
    let bcd3 = gsl_sf_lnchoose(two_jb as u32, jmmb as u32)?;
    let bcd4 = gsl_sf_lnchoose(two_jc as u32, jpmc as u32)?;

    let lnorm_val = 0.5
        * (bcn1.val + bcn2.val - bcd1.val - bcd2.val - bcd3.val - bcd4.val
            - log((two_jc + 1) as f64));
    let lnorm_err = 0.5
        * (bcn1.err
            + bcn2.err
            + bcd1.err
            + bcd2.err
            + bcd3.err
            + bcd4.err
            + 2.2204460492503131e-16 * log((two_jc + 1) as f64));

    for k in kmin..=kmax {
        let bc1 = gsl_sf_lnchoose(jcc as u32, k as u32)?;
        let bc2 = gsl_sf_lnchoose(jcb as u32, (jmma - k) as u32)?;
        let bc3 = gsl_sf_lnchoose(jca as u32, (jpmb - k) as u32)?;

        let term = gsl_sf_exp_err(
            bc1.val + bc2.val + bc3.val + lnorm_val,
            bc1.err + bc2.err + bc3.err + lnorm_err,
        )?;

        if sign < 0 {
            sum_neg += term.val;
        } else {
            sum_pos += term.val;
        }
        sum_err += term.err;
        sign = -sign;
    }

    let val = sum_pos - sum_neg;
    let mut err = sum_err;
    err += 2.0 * 2.2204460492503131e-16 * (sum_pos + sum_neg);
    err += 2.0 * 2.2204460492503131e-16 * (kmax - kmin) as f64 * abs(val);

    Ok(GslSfResult { val, err })
}

// Additional helper functions would need to be implemented:
// gsl_sf_fact, gsl_sf_lnchoose, gsl_sf_exp_err, etc.
// These would follow similar patterns of safe Rust implementations