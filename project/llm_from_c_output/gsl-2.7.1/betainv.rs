use std::f64::consts::E;
use std::f64;

use statrs::function::gamma::{ln_gamma, gamma};
use statrs::distribution::{Beta, ContinuousCDF};

const GSL_SQRT_DBL_EPSILON: f64 = 1.4901161193847656e-08;

#[derive(Debug, PartialEq)]
pub enum Error {
    DomainError(String),
    FailedConvergence(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::DomainError(msg) => write!(f, "Domain error: {}", msg),
            Error::FailedConvergence(msg) => write!(f, "Failed convergence: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

fn bisect(x: f64, p: f64, a: f64, b: f64, xtol: f64, ptol: f64) -> f64 {
    let mut x0 = 0.0;
    let mut x1 = 1.0;
    let mut x = x;

    while (x1 - x0).abs() > xtol {
        let px = Beta::new(a, b).unwrap().cdf(x);
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

pub fn beta_pinv(p: f64, a: f64, b: f64) -> Result<f64, Error> {
    if p < 0.0 || p > 1.0 {
        return Err(Error::DomainError("P must be in range 0 < P < 1".to_string()));
    }

    if a < 0.0 {
        return Err(Error::DomainError("a < 0".to_string()));
    }

    if b < 0.0 {
        return Err(Error::DomainError("b < 0".to_string()));
    }

    if p == 0.0 {
        return Ok(0.0);
    }

    if p == 1.0 {
        return Ok(1.0);
    }

    if p > 0.5 {
        return beta_qinv(1.0 - p, a, b);
    }

    let mean = a / (a + b);
    let x = if p < 0.1 {
        let lg_ab = ln_gamma(a + b);
        let lg_a = ln_gamma(a);
        let lg_b = ln_gamma(b);

        let lx = (a.ln() + lg_a + lg_b - lg_ab + p.ln()) / a;
        let mut x = if lx <= 0.0 {
            let x1 = E.powf(lx);
            x1 * (1.0 - x1).powf(-(b - 1.0) / a)
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

    let mut x = bisect(x, p, a, b, 0.01, 0.01);

    let mut n = 0;
    loop {
        let dp = p - Beta::new(a, b).unwrap().cdf(x);
        let phi = Beta::new(a, b).unwrap().pdf(x);

        if dp == 0.0 || n > 64 {
            break;
        }
        n += 1;

        let lambda = dp / (2.0 * (dp / x).abs().max(phi);

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

        if step0.abs() <= 1e-10 * x {
            break;
        }
    }

    let dp = p - Beta::new(a, b).unwrap().cdf(x);
    if dp.abs() > GSL_SQRT_DBL_EPSILON * p {
        Err(Error::FailedConvergence("inverse failed to converge".to_string()))
    } else {
        Ok(x)
    }
}

pub fn beta_qinv(q: f64, a: f64, b: f64) -> Result<f64, Error> {
    if q < 0.0 || q > 1.0 {
        return Err(Error::DomainError("Q must be inside range 0 < Q < 1".to_string()));
    }

    if a < 0.0 {
        return Err(Error::DomainError("a < 0".to_string()));
    }

    if b < 0.0 {
        return Err(Error::DomainError("b < 0".to_string()));
    }

    if q == 0.0 {
        return Ok(1.0);
    }

    if q == 1.0 {
        return Ok(0.0);
    }

    if q > 0.5 {
        beta_pinv(1.0 - q, a, b)
    } else {
        beta_pinv(q, b, a).map(|x| 1.0 - x)
    }
}