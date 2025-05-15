use libc::{c_double, c_int};
use std::f64::consts;

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: c_double,
    pub err: c_double,
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

impl From<c_int> for GslError {
    fn from(code: c_int) -> Self {
        match code {
            0 => GslError::Success,
            -1 => GslError::Failure,
            -2 => GslError::Continue,
            1 => GslError::Domain,
            2 => GslError::Range,
            3 => GslError::Fault,
            4 => GslError::Invalid,
            5 => GslError::Failed,
            6 => GslError::Factor,
            7 => GslError::Sanity,
            8 => GslError::NoMem,
            9 => GslError::BadFunc,
            10 => GslError::Runaway,
            11 => GslError::MaxIter,
            12 => GslError::ZeroDiv,
            13 => GslError::BadTol,
            14 => GslError::Tol,
            15 => GslError::Underflow,
            16 => GslError::Overflow,
            17 => GslError::Loss,
            18 => GslError::Round,
            19 => GslError::BadLen,
            20 => GslError::NotSquare,
            21 => GslError::Singular,
            22 => GslError::Diverge,
            23 => GslError::Unsupported,
            24 => GslError::Unimplemented,
            25 => GslError::Cache,
            26 => GslError::Table,
            27 => GslError::NoProgress,
            28 => GslError::NoProgressJ,
            29 => GslError::TolF,
            30 => GslError::TolX,
            31 => GslError::TolG,
            32 => GslError::Eof,
            _ => GslError::Invalid,
        }
    }
}

pub fn gsl_sf_bessel_jn_e(n: c_int, x: c_double) -> Result<GslSfResult, GslError> {
    let mut sign = 1;
    let mut n = n;
    let mut x = x;

    if n < 0 {
        n = -n;
        if n & 1 != 0 {
            sign = -sign;
        }
    }

    if x < 0.0 {
        x = -x;
        if n & 1 != 0 {
            sign = -sign;
        }
    }

    if n == 0 {
        let b0 = gsl_sf_bessel_j0_e(x)?;
        Ok(GslSfResult {
            val: sign as c_double * b0.val,
            err: b0.err,
        })
    } else if n == 1 {
        let b1 = gsl_sf_bessel_j1_e(x)?;
        Ok(GslSfResult {
            val: sign as c_double * b1.val,
            err: b1.err,
        })
    } else if x == 0.0 {
        Ok(GslSfResult { val: 0.0, err: 0.0 })
    } else if x * x < 10.0 * (n as c_double + 1.0) * 7.4009597974140505e-04 {
        let mut b = GslSfResult { val: 0.0, err: 0.0 };
        let status = gsl_sf_bessel_ij_taylor_e(
            n as c_double,
            x,
            -1,
            50,
            2.2204460492503131e-16,
            &mut b,
        )?;
        let val = sign as c_double * b.val;
        let err = b.err + 2.2204460492503131e-16 * val.abs();
        Ok(GslSfResult { val, err })
    } else if 1.2207031250000000e-04 * x > (n * n) as c_double + 1.0 {
        let mut result = GslSfResult { val: 0.0, err: 0.0 };
        gsl_sf_bessel_jnu_asympx_e(n as c_double, x, &mut result)?;
        result.val *= sign as c_double;
        Ok(result)
    } else if n > 50 {
        let mut result = GslSfResult { val: 0.0, err: 0.0 };
        gsl_sf_bessel_jnu_asymp_olver_e(n as c_double, x, &mut result)?;
        result.val *= sign as c_double;
        Ok(result)
    } else if x > 1000.0 {
        let mut result = GslSfResult { val: 0.0, err: 0.0 };
        gsl_sf_bessel_jnu_asympx_e(n as c_double, x, &mut result)?;
        result.val *= sign as c_double;
        Ok(result)
    } else {
        let (ratio, sgn) = gsl_sf_bessel_j_cf1(n as c_double, x)?;
        let mut jkp1 = 1.4916681462400413e-154 * ratio;
        let mut jk = 1.4916681462400413e-154;
        let mut jkm1;

        let mut k = n;
        while k > 0 {
            jkm1 = 2.0 * k as c_double / x * jk - jkp1;
            jkp1 = jk;
            jk = jkm1;
            k -= 1;
        }

        let (ans, err) = if jkp1.abs() > jk.abs() {
            let b1 = gsl_sf_bessel_j1_e(x)?;
            (
                b1.val / jkp1 * 1.4916681462400413e-154,
                b1.err / jkp1 * 1.4916681462400413e-154,
            )
        } else {
            let b0 = gsl_sf_bessel_j0_e(x)?;
            (
                b0.val / jk * 1.4916681462400413e-154,
                b0.err / jk * 1.4916681462400413e-154,
            )
        };

        Ok(GslSfResult {
            val: sign as c_double * ans,
            err: err.abs(),
        })
    }
}

pub fn gsl_sf_bessel_jn(n: c_int, x: c_double) -> Result<c_double, GslError> {
    let result = gsl_sf_bessel_jn_e(n, x)?;
    Ok(result.val)
}

// Helper functions would need to be implemented similarly
// These are placeholders for the actual safe implementations
fn gsl_sf_bessel_j0_e(x: c_double) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

fn gsl_sf_bessel_j1_e(x: c_double) -> Result<GslSfResult, GslError> {
    unimplemented!()
}

fn gsl_sf_bessel_ij_taylor_e(
    nu: c_double,
    x: c_double,
    sign: c_int,
    kmax: c_int,
    threshold: c_double,
    result: &mut GslSfResult,
) -> Result<(), GslError> {
    unimplemented!()
}

fn gsl_sf_bessel_jnu_asympx_e(
    nu: c_double,
    x: c_double,
    result: &mut GslSfResult,
) -> Result<(), GslError> {
    unimplemented!()
}

fn gsl_sf_bessel_jnu_asymp_olver_e(
    nu: c_double,
    x: c_double,
    result: &mut GslSfResult,
) -> Result<(), GslError> {
    unimplemented!()
}

fn gsl_sf_bessel_j_cf1(
    nu: c_double,
    x: c_double,
) -> Result<(c_double, c_double), GslError> {
    unimplemented!()
}