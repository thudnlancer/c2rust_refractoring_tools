use std::f64::consts::{PI, EULER};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, PartialEq)]
pub enum SfError {
    Overflow,
    Domain,
    Other,
}

impl SfResult {
    fn new(val: f64, err: f64) -> Self {
        SfResult { val, err }
    }
}

fn lnfact(n: u32) -> SfResult {
    if n == 0 {
        return SfResult::new(0.0, 0.0);
    }
    let mut res = 0.0;
    for i in 1..=n {
        res += (i as f64).ln();
    }
    SfResult::new(res, f64::EPSILON * res.abs())
}

fn psi_int(n: u32) -> SfResult {
    if n == 0 {
        return SfResult::new(f64::NEG_INFINITY, 0.0);
    }
    let mut res = -EULER;
    for k in 1..n {
        res += 1.0 / k as f64;
    }
    SfResult::new(res, f64::EPSILON * res.abs())
}

fn fact(n: u32) -> SfResult {
    if n > 170 {
        return SfResult::new(f64::INFINITY, 0.0);
    }
    let mut res = 1.0;
    for i in 1..=n {
        res *= i as f64;
    }
    SfResult::new(res, f64::EPSILON * res.abs())
}

fn bessel_y0(x: f64) -> SfResult {
    // Placeholder implementation - should be replaced with actual Y0 calculation
    SfResult::new(0.0, 0.0)
}

fn bessel_y1(x: f64) -> SfResult {
    // Placeholder implementation - should be replaced with actual Y1 calculation
    SfResult::new(0.0, 0.0)
}

fn bessel_ynu_asympx(nu: f64, x: f64) -> SfResult {
    // Placeholder implementation - should be replaced with actual asymptotic expansion
    SfResult::new(0.0, 0.0)
}

fn bessel_ynu_asymp_olver(nu: f64, x: f64) -> SfResult {
    // Placeholder implementation - should be replaced with actual Olver asymptotic expansion
    SfResult::new(0.0, 0.0)
}

fn bessel_yn_small_x(n: i32, x: f64) -> Result<SfResult, SfError> {
    if n < 1 {
        return Err(SfError::Domain);
    }
    
    let y = 0.25 * x * x;
    let ln_x_2 = (0.5 * x).ln();
    let ln_nm1_fact = lnfact((n - 1) as u32);
    
    let ln_pre1 = -(n as f64) * ln_x_2 + ln_nm1_fact.val;
    if ln_pre1 > f64::MAX.ln() - 3.0 {
        return Err(SfError::Overflow);
    }
    
    let mut sum1 = 1.0;
    let mut k_term = 1.0;
    for k in 1..n {
        k_term *= y / (k as f64 * (n - k) as f64);
        sum1 += k_term;
    }
    let term1 = -ln_pre1.exp() * sum1 / PI;
    
    let pre2 = -(n as f64 * ln_x_2).exp() / PI;
    let term2 = if pre2.abs() > 0.0 {
        const KMAX: i32 = 20;
        let psi_n = psi_int(n as u32);
        let mut npk_fact = fact(n as u32);
        let mut yk = 1.0;
        let mut k_fact = 1.0;
        let mut psi_kp1 = -EULER;
        let mut psi_npkp1 = psi_n.val + 1.0 / (n as f64);
        
        let mut sum2 = (psi_kp1 + psi_npkp1 - 2.0 * ln_x_2) / npk_fact.val;
        
        for k in 1..KMAX {
            psi_kp1 += 1.0 / k as f64;
            psi_npkp1 += 1.0 / (n + k) as f64;
            k_fact *= k as f64;
            npk_fact.val *= (n + k) as f64;
            yk *= -y;
            let k_term = yk * (psi_kp1 + psi_npkp1 - 2.0 * ln_x_2) / (k_fact * npk_fact.val);
            sum2 += k_term;
        }
        pre2 * sum2
    } else {
        0.0
    };
    
    let val = term1 + term2;
    let err = f64::EPSILON * (ln_pre1.abs() * term1.abs() + term2.abs());
    let err = err + 2.0 * f64::EPSILON * val.abs();
    
    Ok(SfResult::new(val, err))
}

pub fn bessel_yn_e(n: i32, x: f64) -> Result<SfResult, SfError> {
    let mut sign = 1;
    let mut n = n;
    
    if n < 0 {
        n = -n;
        if n % 2 != 0 {
            sign = -1;
        }
    }
    
    if n == 0 {
        let mut result = bessel_y0(x);
        result.val *= sign as f64;
        Ok(result)
    } else if n == 1 {
        let mut result = bessel_y1(x);
        result.val *= sign as f64;
        Ok(result)
    } else {
        if x <= 0.0 {
            return Err(SfError::Domain);
        }
        
        if x < 5.0 {
            let mut result = bessel_yn_small_x(n, x)?;
            result.val *= sign as f64;
            Ok(result)
        } else if (f64::EPSILON.powf(1.0/3.0) * x > ((n * n + 1) as f64) {
            let mut result = bessel_ynu_asympx(n as f64, x);
            result.val *= sign as f64;
            Ok(result)
        } else if n > 50 {
            let mut result = bessel_ynu_asymp_olver(n as f64, x);
            result.val *= sign as f64;
            Ok(result)
        } else {
            let two_over_x = 2.0 / x;
            let r_by = bessel_y1(x);
            let r_bym = bessel_y0(x);
            let mut bym = r_bym.val;
            let mut by = r_by.val;
            
            for j in 1..n {
                let byp = j as f64 * two_over_x * by - bym;
                bym = by;
                by = byp;
            }
            
            let val = sign as f64 * by;
            let err = val.abs() * (r_by.err.abs() / r_by.val.abs() + r_bym.err.abs() / r_bym.val.abs());
            let err = err + 2.0 * f64::EPSILON * val.abs();
            
            Ok(SfResult::new(val, err))
        }
    }
}

pub fn bessel_yn_array(nmin: i32, nmax: i32, x: f64) -> Result<Vec<f64>, SfError> {
    if nmin < 0 || nmax < nmin || x <= 0.0 {
        return Err(SfError::Domain);
    }
    
    let mut result_array = vec![0.0; (nmax - nmin + 1) as usize];
    
    let r_ynm1 = bessel_yn_e(nmin, x)?;
    let r_yn = bessel_yn_e(nmin + 1, x)?;
    
    let mut ynm1 = r_ynm1.val;
    let mut yn = r_yn.val;
    
    for n in nmin..=nmax {
        result_array[(n - nmin) as usize] = ynm1;
        let ynp1 = -ynm1 + 2.0 * n as f64 / x * yn;
        ynm1 = yn;
        yn = ynp1;
    }
    
    Ok(result_array)
}

pub fn bessel_yn(n: i32, x: f64) -> f64 {
    match bessel_yn_e(n, x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}