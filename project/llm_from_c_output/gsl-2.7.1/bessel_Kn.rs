use std::f64::consts::{E, EULER};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

impl SfResult {
    pub fn new(val: f64, err: f64) -> Self {
        Self { val, err }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SfError {
    Domain,
    Overflow,
    Other,
}

pub fn bessel_kn_scaled_small_x(n: i32, x: f64) -> Result<SfResult, SfError> {
    let mut y = 0.25 * x * x;
    let ln_x_2 = (0.5 * x).ln();
    let ex = x.exp();
    let ln_nm1_fact = ln_fact((n - 1) as u32);
    
    let ln_pre1 = -(n as f64) * ln_x_2 + ln_nm1_fact;
    if ln_pre1 > f64::MAX.ln() - 3.0 {
        return Err(SfError::Overflow);
    }

    let mut sum1 = 1.0;
    let mut k_term = 1.0;
    for k in 1..n {
        k_term *= -y / (k as f64 * (n - k) as f64;
        sum1 += k_term;
    }
    let term1 = 0.5 * ln_pre1.exp() * sum1;

    let pre2 = 0.5 * (n as f64 * ln_x_2).exp();
    let term2 = if pre2 > 0.0 {
        const KMAX: i32 = 20;
        let psi_n = psi_int(n);
        let mut npk_fact = fact(n as u32) as f64;
        let mut yk = 1.0;
        let mut k_fact = 1.0;
        let mut psi_kp1 = -EULER;
        let mut psi_npkp1 = psi_n + 1.0 / (n as f64);
        let mut sum2 = (psi_kp1 + psi_npkp1 - 2.0 * ln_x_2) / npk_fact;
        
        for k in 1..KMAX {
            psi_kp1 += 1.0 / (k as f64);
            psi_npkp1 += 1.0 / ((n + k) as f64);
            k_fact *= k as f64;
            npk_fact *= (n + k) as f64;
            yk *= y;
            let k_term = yk * (psi_kp1 + psi_npkp1 - 2.0 * ln_x_2) / (k_fact * npk_fact);
            sum2 += k_term;
        }
        (if n % 2 != 0 { -1.0 } else { 1.0 }) * pre2 * sum2
    } else {
        0.0
    };

    let val = ex * (term1 + term2);
    let err = ex * f64::EPSILON * (ln_pre1.abs() * term1.abs() + term2.abs());
    let err = err + 2.0 * f64::EPSILON * val.abs();

    Ok(SfResult::new(val, err))
}

pub fn bessel_kn_scaled(n: i32, x: f64) -> Result<SfResult, SfError> {
    let n = n.abs(); // K(-n, z) = K(n, z)

    if x <= 0.0 {
        Err(SfError::Domain)
    } else if n == 0 {
        bessel_k0_scaled(x)
    } else if n == 1 {
        bessel_k1_scaled(x)
    } else if x <= 5.0 {
        bessel_kn_scaled_small_x(n, x)
    } else if f64::EPSILON.powf(1.0/3.0) * x > 0.25 * ((n * n + 1) as f64) {
        bessel_knu_scaled_asympx(n as f64, x)
    } else if f64::min(0.29 / (n * n) as f64, 0.5 / ((n * n) as f64 + x * x)) < f64::EPSILON.powf(1.0/3.0) {
        bessel_knu_scaled_asymp_unif(n as f64, x)
    } else {
        // Upward recurrence
        let two_over_x = 2.0 / x;
        let r_b_jm1 = bessel_k0_scaled(x)?;
        let r_b_j = bessel_k1_scaled(x)?;
        let mut b_jm1 = r_b_jm1.val;
        let mut b_j = r_b_j.val;
        
        for j in 1..n {
            let b_jp1 = b_jm1 + (j as f64) * two_over_x * b_j;
            b_jm1 = b_j;
            b_j = b_jp1;
        }
        
        let val = b_j;
        let err = (n as f64) * (val.abs() * (r_b_jm1.err / r_b_jm1.val.abs() + r_b_j.err / r_b_j.val.abs()));
        let err = err + 2.0 * f64::EPSILON * val.abs();

        Ok(SfResult::new(val, err))
    }
}

pub fn bessel_kn(n: i32, x: f64) -> Result<SfResult, SfError> {
    let mut result = bessel_kn_scaled(n, x)?;
    let ex = (-x).exp();
    result.val *= ex;
    result.err *= ex;
    result.err += x * f64::EPSILON * result.val.abs();
    Ok(result)
}

pub fn bessel_kn_scaled_array(nmin: i32, nmax: i32, x: f64) -> Result<Vec<f64>, SfError> {
    if nmin < 0 || nmax < nmin || x <= 0.0 {
        Err(SfError::Domain)
    } else if nmax == 0 {
        let b = bessel_k0_scaled(x)?;
        Ok(vec![b.val])
    } else {
        let two_over_x = 2.0 / x;
        let mut r_knm1 = bessel_kn_scaled(nmin, x)?;
        let mut r_kn = bessel_kn_scaled(nmin + 1, x)?;
        let mut knm1 = r_knm1.val;
        let mut kn = r_kn.val;
        let mut result = Vec::with_capacity((nmax - nmin + 1) as usize);
        
        for n in nmin..=nmax {
            if knm1 < f64::MAX {
                result.push(knm1);
                let knp1 = knm1 + (n as f64) * two_over_x * kn;
                knm1 = kn;
                kn = knp1;
            } else {
                return Err(SfError::Overflow);
            }
        }
        
        Ok(result)
    }
}

pub fn bessel_kn_array(nmin: i32, nmax: i32, x: f64) -> Result<Vec<f64>, SfError> {
    let mut result = bessel_kn_scaled_array(nmin, nmax, x)?;
    let ex = (-x).exp();
    for val in result.iter_mut() {
        *val *= ex;
    }
    Ok(result)
}

// Helper functions (to be implemented)
fn ln_fact(n: u32) -> f64 {
    // Implementation of ln(n!)
    unimplemented!()
}

fn psi_int(n: i32) -> f64 {
    // Implementation of digamma function for integer n
    unimplemented!()
}

fn fact(n: u32) -> u64 {
    // Implementation of factorial
    unimplemented!()
}

fn bessel_k0_scaled(x: f64) -> Result<SfResult, SfError> {
    // Implementation of scaled K0
    unimplemented!()
}

fn bessel_k1_scaled(x: f64) -> Result<SfResult, SfError> {
    // Implementation of scaled K1
    unimplemented!()
}

fn bessel_knu_scaled_asympx(nu: f64, x: f64) -> Result<SfResult, SfError> {
    // Implementation of asymptotic expansion for large x
    unimplemented!()
}

fn bessel_knu_scaled_asymp_unif(nu: f64, x: f64) -> Result<SfResult, SfError> {
    // Implementation of uniform asymptotic expansion
    unimplemented!()
}