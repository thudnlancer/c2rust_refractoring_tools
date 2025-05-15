use std::f64::consts::PI;
use std::f64::{NAN, INFINITY};

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GslMode {
    PrecSingle = 0,
    PrecDouble = 1,
    PrecApprox = 2,
}

fn gsl_mode_prec(mode: GslMode) -> GslMode {
    match mode {
        GslMode::PrecSingle | GslMode::PrecDouble | GslMode::PrecApprox => mode,
    }
}

fn max3(x: f64, y: f64, z: f64) -> f64 {
    x.max(y).max(z)
}

fn max4(x: f64, y: f64, z: f64, w: f64) -> f64 {
    x.max(y).max(z).max(w)
}

pub fn gsl_sf_ellint_rc_e(x: f64, y: f64, mode: GslMode) -> Result<GslSfResult, GslError> {
    let lolim = 5.0 * f64::MIN_POSITIVE;
    let uplim = 0.2 * f64::MAX;
    let goal = gsl_mode_prec(mode);
    let errtol = if goal == GslMode::PrecSingle { 0.001 } else { 0.03 };
    let prec = match goal {
        GslMode::PrecSingle => f64::EPSILON,
        GslMode::PrecDouble => f64::EPSILON * 0.5,
        GslMode::PrecApprox => f64::EPSILON * 0.1,
    };
    let nmax = 10000;

    if x < 0.0 || y < 0.0 || x + y < lolim {
        Err(GslError::Domain)
    } else if x.max(y) < uplim {
        let c1 = 1.0 / 7.0;
        let c2 = 9.0 / 22.0;
        let mut xn = x;
        let mut yn = y;
        let mut n = 0;

        loop {
            let mu = (xn + yn + yn) / 3.0;
            let sn = (yn + mu) / mu - 2.0;
            if sn.abs() < errtol {
                break;
            }
            let lamda = 2.0 * xn.sqrt() * yn.sqrt() + yn;
            xn = (xn + lamda) * 0.25;
            yn = (yn + lamda) * 0.25;
            n += 1;
            if n == nmax {
                return Err(GslError::MaxIter);
            }
        }

        let mu = (xn + yn + yn) / 3.0;
        let sn = (yn + mu) / mu - 2.0;
        let s = sn * sn * (0.3 + sn * (c1 + sn * (0.375 + sn * c2)));
        let val = (1.0 + s) / mu.sqrt();
        let err = prec * val.abs();
        Ok(GslSfResult { val, err })
    } else {
        Err(GslError::Domain)
    }
}

// Similar safe implementations for other functions:
// gsl_sf_ellint_rd_e, gsl_sf_ellint_rf_e, gsl_sf_ellint_rj_e,
// gsl_sf_ellint_f_e, gsl_sf_ellint_e_e, gsl_sf_ellint_p_e,
// gsl_sf_ellint_d_e, gsl_sf_ellint_kcomp_e, gsl_sf_ellint_ecomp_e,
// gsl_sf_ellint_pcomp_e, gsl_sf_ellint_dcomp_e

// And their simple wrappers:
pub fn gsl_sf_ellint_rc(x: f64, y: f64, mode: GslMode) -> f64 {
    gsl_sf_ellint_rc_e(x, y, mode).unwrap().val
}

// Similar wrappers for other functions...