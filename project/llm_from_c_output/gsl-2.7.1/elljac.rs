use std::f64::consts::PI;
use std::f64::EPSILON as DBL_EPSILON;

#[derive(Debug, PartialEq)]
pub enum EllJacError {
    DomainError,
    MaxIterations,
}

pub fn gsl_sf_elljac_e(u: f64, m: f64) -> Result<(f64, f64, f64), EllJacError> {
    if m.abs() > 1.0 {
        Err(EllJacError::DomainError)
    } else if m.abs() < 2.0 * DBL_EPSILON {
        Ok((u.sin(), u.cos(), 1.0))
    } else if (m - 1.0).abs() < 2.0 * DBL_EPSILON {
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
        let mut status = Ok(());
        
        while (mu[n] - nu[n]).abs() > 4.0 * DBL_EPSILON * (mu[n] + nu[n]).abs() {
            if n >= N - 1 {
                status = Err(EllJacError::MaxIterations);
                break;
            }
            mu[n+1] = 0.5 * (mu[n] + nu[n]);
            nu[n+1] = (mu[n] * nu[n]).sqrt();
            n += 1;
        }
        
        let sin_umu = (u * mu[n]).sin();
        let cos_umu = (u * mu[n]).cos();
        
        let (sn, cn, dn) = if sin_umu.abs() < cos_umu.abs() {
            let t = sin_umu / cos_umu;
            
            c[n] = mu[n] * t;
            d[n] = 1.0;
            
            let mut current_n = n;
            while current_n > 0 {
                current_n -= 1;
                c[current_n] = d[current_n+1] * c[current_n+1];
                let r = (c[current_n+1] * c[current_n+1]) / mu[current_n+1];
                d[current_n] = (r + nu[current_n]) / (r + mu[current_n]);
            }
            
            let dn = (1.0 - m).sqrt() / d[0];
            let cn_sign = if cos_umu >= 0.0 { 1.0 } else { -1.0 };
            let cn = dn * cn_sign / (1.0 + c[0]*c[0]).sqrt();
            let sn = cn * c[0] / (1.0 - m).sqrt();
            
            (sn, cn, dn)
        } else {
            let t = cos_umu / sin_umu;
            
            c[n] = mu[n] * t;
            d[n] = 1.0;
            
            let mut current_n = n;
            while current_n > 0 {
                current_n -= 1;
                c[current_n] = d[current_n+1] * c[current_n+1];
                let r = (c[current_n+1] * c[current_n+1]) / mu[current_n+1];
                d[current_n] = (r + nu[current_n]) / (r + mu[current_n]);
            }
            
            let dn = d[0];
            let sn_sign = if sin_umu >= 0.0 { 1.0 } else { -1.0 };
            let sn = sn_sign / (1.0 + c[0]*c[0]).sqrt();
            let cn = c[0] * sn;
            
            (sn, cn, dn)
        };
        
        status.map(|_| (sn, cn, dn))
    }
}