use std::f64;
use std::f64::consts::SQRT_2;

#[derive(Debug, Copy, Clone)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GslError {
    Domain,
    Range,
    Memory,
    Invalid,
    Failed,
    Factor,
    Sanity,
    NoMem,
    BadFunc,
    Runaway,
    MaxIter,
    ZeroDiv,
    BadTol,
    Tol,
    Underflow,
    Overflow,
    Loss,
    Round,
    BadLen,
    NotSquare,
    Singular,
    Diverge,
    Unsupported,
    Unimplemented,
    Cache,
    Table,
    NoProgress,
    NoProgressJ,
    ToleranceF,
    ToleranceX,
    ToleranceG,
    Eof,
    Other(i32),
}

impl From<i32> for GslError {
    fn from(code: i32) -> Self {
        match code {
            1 => GslError::Domain,
            2 => GslError::Range,
            3 => GslError::Invalid,
            4 => GslError::Memory,
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
            29 => GslError::ToleranceF,
            30 => GslError::ToleranceX,
            31 => GslError::ToleranceG,
            32 => GslError::Eof,
            _ => GslError::Other(code),
        }
    }
}

pub fn bessel_inu_scaled_e(nu: f64, x: f64) -> Result<SfResult, GslError> {
    if x < 0.0 || nu < 0.0 {
        Err(GslError::Domain)
    } else if x * x < 10.0 * (nu + 1.0) {
        let ex = (-x).exp();
        let b = bessel_ij_taylor_e(nu, x, 1, 100, f64::EPSILON)?;
        let val = b.val * ex;
        let err = b.err * ex + 2.0 * f64::EPSILON * val.abs();
        Ok(SfResult { val, err })
    } else if 0.5 / (nu * nu + x * x) < 6.0554544523933429e-06 {
        bessel_inu_scaled_asymp_unif_e(nu, x)
    } else {
        let n = (nu + 0.5) as i32;
        let mu = nu - n as f64;
        
        let (k_mu, k_mup1, kp_mu) = if x < 2.0 {
            bessel_k_scaled_temme(mu, x)?
        } else {
            bessel_k_scaled_steed_temme_cf2(mu, x)?
        };
        
        let (k_nu, k_nup1) = recurrence_relation(mu, x, n, k_mu, k_mup1);
        
        let i_nu_ratio = bessel_i_cf1_ser(nu, x)?;
        
        let val = 1.0 / (x * (k_nup1 + i_nu_ratio * k_nu));
        let err = f64::EPSILON * (0.5 * n as f64 + 2.0) * val.abs();
        
        Ok(SfResult { val, err })
    }
}

pub fn bessel_inu_e(nu: f64, x: f64) -> Result<SfResult, GslError> {
    let b = bessel_inu_scaled_e(nu, x)?;
    let ex = x.exp();
    let val = b.val * ex;
    let err = b.err * ex + f64::EPSILON * x.abs() * val.abs();
    Ok(SfResult { val, err })
}

pub fn bessel_inu_scaled(nu: f64, x: f64) -> f64 {
    bessel_inu_scaled_e(nu, x).map(|r| r.val).unwrap_or(f64::NAN)
}

pub fn bessel_inu(nu: f64, x: f64) -> f64 {
    bessel_inu_e(nu, x).map(|r| r.val).unwrap_or(f64::NAN)
}

// Helper functions would need to be implemented similarly
fn bessel_ij_taylor_e(nu: f64, x: f64, sign: i32, kmax: i32, threshold: f64) -> Result<SfResult, GslError> {
    // Implementation would go here
    unimplemented!()
}

fn bessel_inu_scaled_asymp_unif_e(nu: f64, x: f64) -> Result<SfResult, GslError> {
    // Implementation would go here
    unimplemented!()
}

fn bessel_k_scaled_temme(nu: f64, x: f64) -> Result<(f64, f64, f64), GslError> {
    // Implementation would go here
    unimplemented!()
}

fn bessel_k_scaled_steed_temme_cf2(nu: f64, x: f64) -> Result<(f64, f64, f64), GslError> {
    // Implementation would go here
    unimplemented!()
}

fn bessel_i_cf1_ser(nu: f64, x: f64) -> Result<f64, GslError> {
    // Implementation would go here
    unimplemented!()
}

fn recurrence_relation(mu: f64, x: f64, n: i32, mut k_nu: f64, mut k_nup1: f64) -> (f64, f64) {
    let mut k_num1;
    for i in 0..n {
        k_num1 = k_nu;
        k_nu = k_nup1;
        k_nup1 = 2.0 * (mu + i as f64 + 1.0) / x * k_nu + k_num1;
    }
    (k_nu, k_nup1)
}