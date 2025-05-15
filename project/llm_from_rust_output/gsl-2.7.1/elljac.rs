use std::f64::consts::*;
use std::f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GslError {
    Success = 0,
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

pub fn gsl_sf_elljac_e(u: f64, m: f64) -> Result<(f64, f64, f64), GslError> {
    if m.abs() > 1.0 {
        Err(GslError::Domain)
    } else if m.abs() < 2.0 * f64::EPSILON {
        Ok((u.sin(), u.cos(), 1.0))
    } else if (m - 1.0).abs() < 2.0 * f64::EPSILON {
        let sn = u.tanh();
        let cn = 1.0 / u.cosh();
        Ok((sn, cn, cn))
    } else {
        const N: usize = 16;
        let mut mu = [0.0; N];
        let mut nu = [0.0; N];
        let mut c = [0.0; N];
        let mut d = [0.0; N];
        
        mu[0] = 1.0;
        nu[0] = (1.0 - m).sqrt();
        
        let mut n = 0;
        while (mu[n] - nu[n]).abs() > 4.0 * f64::EPSILON * (mu[n] + nu[n]).abs() {
            if n >= N - 1 {
                return Err(GslError::MaxIter);
            }
            mu[n + 1] = 0.5 * (mu[n] + nu[n]);
            nu[n + 1] = (mu[n] * nu[n]).sqrt();
            n += 1;
        }
        
        let sin_umu = (u * mu[n]).sin();
        let cos_umu = (u * mu[n]).cos();
        
        if sin_umu.abs() < cos_umu.abs() {
            let t = sin_umu / cos_umu;
            c[n] = mu[n] * t;
            d[n] = 1.0;
            
            while n > 0 {
                n -= 1;
                c[n] = d[n + 1] * c[n + 1];
                let r = c[n + 1].powi(2) / mu[n + 1];
                d[n] = (r + nu[n]) / (r + mu[n]);
            }
            
            let dn = (1.0 - m).sqrt() / d[n];
            let cn = dn * cos_umu.signum() / (1.0 + c[n].powi(2)).sqrt();
            let sn = cn * c[n] / (1.0 - m).sqrt();
            
            Ok((sn, cn, dn))
        } else {
            let t = cos_umu / sin_umu;
            c[n] = mu[n] * t;
            d[n] = 1.0;
            
            while n > 0 {
                n -= 1;
                c[n] = d[n + 1] * c[n + 1];
                let r = c[n + 1].powi(2) / mu[n + 1];
                d[n] = (r + nu[n]) / (r + mu[n]);
            }
            
            let dn = d[n];
            let sn = sin_umu.signum() / (1.0 + c[n].powi(2)).sqrt();
            let cn = c[n] * sn;
            
            Ok((sn, cn, dn))
        }
    }
}