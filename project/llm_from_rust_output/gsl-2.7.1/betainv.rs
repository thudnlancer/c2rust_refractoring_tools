use std::f64::consts::NAN;
use std::f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

fn bisect(x: f64, p: f64, a: f64, b: f64, xtol: f64, ptol: f64) -> f64 {
    let mut x0 = 0.0;
    let mut x1 = 1.0;
    let mut x = x;
    
    while (x1 - x0).abs() > xtol {
        let px = gsl_cdf_beta_p(x, a, b);
        if (px - p).abs() < ptol {
            return x;
        } else if px < p {
            x0 = x;
        } else if px > p {
            x1 = x;
        }
        x = 0.5 * (x0 + x1);
    }
    x
}

pub fn gsl_cdf_beta_pinv(p: f64, a: f64, b: f64) -> Result<f64, GslError> {
    if p < 0.0 || p > 1.0 {
        return Err(GslError::Domain);
    }
    if a < 0.0 {
        return Err(GslError::Domain);
    }
    if b < 0.0 {
        return Err(GslError::Domain);
    }

    if p == 0.0 {
        return Ok(0.0);
    }
    if p == 1.0 {
        return Ok(1.0);
    }
    if p > 0.5 {
        return Ok(gsl_cdf_beta_qinv(1.0 - p, a, b)?);
    }

    let mean = a / (a + b);
    let x = if p < 0.1 {
        let lg_ab = gsl_sf_lngamma(a + b);
        let lg_a = gsl_sf_lngamma(a);
        let lg_b = gsl_sf_lngamma(b);
        let lx = (a.ln() + lg_a + lg_b - lg_ab + p.ln()) / a;
        let mut x = if lx <= 0.0 {
            let x = lx.exp();
            x * (1.0 - x).powf(-(b - 1.0) / a)
        } else {
            mean
        };
        if x > mean {
            mean
        } else {
            x
        }
    } else {
        mean
    };

    let x = bisect(x, p, a, b, 0.01, 0.01);
    let mut x = x;
    let mut n = 0;
    let mut dp = 0.0;

    loop {
        dp = p - gsl_cdf_beta_p(x, a, b);
        let phi = gsl_ran_beta_pdf(x, a, b);
        if dp == 0.0 || n > 64 {
            break;
        }
        n += 1;

        let lambda = dp / (2.0 * (dp / x).abs().max(phi));
        let step0 = lambda;
        let step1 = -((a - 1.0) / x - (b - 1.0) / (1.0 - x)) * lambda * lambda / 2.0;
        let mut step = step0;
        if step1.abs() < step0.abs() {
            step += step1;
        } else {
            step *= 2.0 * (step0 / step1).abs();
        }

        if x + step > 0.0 && x + step < 1.0 {
            x += step;
        } else {
            x = x.sqrt() * mean.sqrt();
        }

        if !(step0.abs() > 1e-10 * x) {
            break;
        }
    }

    if dp.abs() > 1.4901161193847656e-08 * p {
        Err(GslError::Failed)
    } else {
        Ok(x)
    }
}

pub fn gsl_cdf_beta_qinv(q: f64, a: f64, b: f64) -> Result<f64, GslError> {
    if q < 0.0 || q > 1.0 {
        return Err(GslError::Domain);
    }
    if a < 0.0 {
        return Err(GslError::Domain);
    }
    if b < 0.0 {
        return Err(GslError::Domain);
    }

    if q == 0.0 {
        return Ok(1.0);
    }
    if q == 1.0 {
        return Ok(0.0);
    }

    if q > 0.5 {
        gsl_cdf_beta_pinv(1.0 - q, a, b)
    } else {
        Ok(1.0 - gsl_cdf_beta_pinv(q, b, a)?)
    }
}

// These functions would be implemented elsewhere in safe Rust or wrapped from FFI
fn gsl_cdf_beta_p(x: f64, a: f64, b: f64) -> f64 {
    // Implementation or FFI wrapper
    unsafe { gsl_cdf_beta_P(x, a, b) }
}

fn gsl_ran_beta_pdf(x: f64, a: f64, b: f64) -> f64 {
    // Implementation or FFI wrapper
    unsafe { gsl_ran_beta_pdf(x, a, b) }
}

fn gsl_sf_lngamma(x: f64) -> f64 {
    // Implementation or FFI wrapper
    unsafe { gsl_sf_lngamma(x) }
}

// External C functions would be declared in an unsafe block
extern "C" {
    fn gsl_cdf_beta_P(x: f64, a: f64, b: f64) -> f64;
    fn gsl_ran_beta_pdf(x: f64, a: f64, b: f64) -> f64;
    fn gsl_sf_lngamma(x: f64) -> f64;
}