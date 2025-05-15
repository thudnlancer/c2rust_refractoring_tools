use std::f64::consts::E;
use std::f64::{EPSILON, MIN_POSITIVE};

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SfError {
    Domain,
    Overflow,
    Other,
}

impl SfResult {
    pub fn new(val: f64, err: f64) -> Self {
        Self { val, err }
    }
}

fn is_odd(n: i32) -> bool {
    n % 2 != 0
}

pub fn bessel_i0_scaled_e(x: f64) -> Result<SfResult, SfError> {
    // Implementation of gsl_sf_bessel_I0_scaled_e
    unimplemented!()
}

pub fn bessel_i1_scaled_e(x: f64) -> Result<SfResult, SfError> {
    // Implementation of gsl_sf_bessel_I1_scaled_e
    unimplemented!()
}

pub fn bessel_ij_taylor_e(n: f64, x: f64, mode: i32, max_iter: i32, eps: f64, result: &mut SfResult) -> Result<(), SfError> {
    // Implementation of gsl_sf_bessel_IJ_taylor_e
    unimplemented!()
}

pub fn bessel_i_cf1_ser(n: f64, x: f64, rat: &mut f64) -> Result<(), SfError> {
    // Implementation of gsl_sf_bessel_I_CF1_ser
    unimplemented!()
}

pub fn bessel_inu_scaled_asymp_unif_e(nu: f64, x: f64, result: &mut SfResult) -> Result<(), SfError> {
    // Implementation of gsl_sf_bessel_Inu_scaled_asymp_unif_e
    unimplemented!()
}

pub fn bessel_in_scaled_e(n_in: i32, x: f64, result: &mut SfResult) -> Result<(), SfError> {
    let ax = x.abs();
    let n = n_in.abs(); // I(-n, z) = I(n, z)

    match n {
        0 => {
            *result = bessel_i0_scaled_e(x)?;
            Ok(())
        }
        1 => {
            *result = bessel_i1_scaled_e(x)?;
            Ok(())
        }
        _ if x == 0.0 => {
            result.val = 0.0;
            result.err = 0.0;
            Ok(())
        }
        _ if x * x < 10.0 * (n as f64 + 1.0) / E => {
            let ex = (-ax).exp();
            let mut t = SfResult::new(0.0, 0.0);
            bessel_ij_taylor_e(n as f64, ax, 1, 50, EPSILON, &mut t)?;
            result.val = t.val * ex;
            result.err = t.err * ex;
            result.err += 2.0 * EPSILON * result.val.abs();
            if x < 0.0 && is_odd(n) {
                result.val = -result.val;
            }
            Ok(())
        }
        _ if n < 150 && ax < 1e7 => {
            let i0_scaled = bessel_i0_scaled_e(ax)?;
            let mut rat = 0.0;
            bessel_i_cf1_ser(n as f64, ax, &mut rat)?;
            let mut ikp1 = rat * MIN_POSITIVE.sqrt();
            let mut ik = MIN_POSITIVE.sqrt();
            let mut ikm1;
            for k in (1..=n).rev() {
                ikm1 = ikp1 + 2.0 * k as f64 / ax * ik;
                ikp1 = ik;
                ik = ikm1;
            }
            result.val = i0_scaled.val * (MIN_POSITIVE.sqrt() / ik);
            result.err = i0_scaled.err * (MIN_POSITIVE.sqrt() / ik);
            result.err += 2.0 * EPSILON * result.val.abs();
            if x < 0.0 && is_odd(n) {
                result.val = -result.val;
            }
            Ok(())
        }
        _ if (0.29 / (n * n) as f64).min(0.5 / (n * n + x * x) as f64) < 0.5 * EPSILON.cbrt() => {
            bessel_inu_scaled_asymp_unif_e(n as f64, ax, result)?;
            if x < 0.0 && is_odd(n) {
                result.val = -result.val;
            }
            Ok(())
        }
        _ => {
            let nhi = 2 + (1.2 / EPSILON.powf(1.0 / 6.0)) as i32;
            let mut r_ikp1 = SfResult::new(0.0, 0.0);
            let mut r_ik = SfResult::new(0.0, 0.0);
            bessel_inu_scaled_asymp_unif_e(nhi as f64 + 1.0, ax, &mut r_ikp1)?;
            bessel_inu_scaled_asymp_unif_e(nhi as f64, ax, &mut r_ik)?;
            let mut ikp1 = r_ikp1.val;
            let mut ik = r_ik.val;
            let mut ikm1;
            for k in (n + 1..=nhi).rev() {
                ikm1 = ikp1 + 2.0 * k as f64 / ax * ik;
                ikp1 = ik;
                ik = ikm1;
            }
            result.val = ik;
            result.err = ik * (r_ikp1.err / r_ikp1.val + r_ik.err / r_ik.val);
            if x < 0.0 && is_odd(n) {
                result.val = -result.val;
            }
            Ok(())
        }
    }
}

pub fn bessel_in_scaled_array(nmin: i32, nmax: i32, x: f64, result_array: &mut [f64]) -> Result<(), SfError> {
    if nmax < nmin || nmin < 0 {
        for j in 0..=(nmax - nmin) as usize {
            result_array[j] = 0.0;
        }
        Err(SfError::Domain)
    } else if x == 0.0 {
        for j in 0..=(nmax - nmin) as usize {
            result_array[j] = 0.0;
        }
        if nmin == 0 {
            result_array[0] = 1.0;
        }
        Ok(())
    } else if nmax == 0 {
        let i0_scaled = bessel_i0_scaled_e(x)?;
        result_array[0] = i0_scaled.val;
        Ok(())
    } else {
        let ax = x.abs();
        let two_over_x = 2.0 / ax;

        let mut r_inp1 = SfResult::new(0.0, 0.0);
        let mut r_in = SfResult::new(0.0, 0.0);
        bessel_in_scaled_e(nmax + 1, ax, &mut r_inp1)?;
        bessel_in_scaled_e(nmax, ax, &mut r_in)?;
        let mut inp1 = r_inp1.val;
        let mut in_val = r_in.val;
        let mut inm1;

        for n in (nmin..=nmax).rev() {
            result_array[(n - nmin) as usize] = in_val;
            inm1 = inp1 + n as f64 * two_over_x * in_val;
            inp1 = in_val;
            in_val = inm1;
        }

        if x < 0.0 {
            for n in nmin..=nmax {
                if is_odd(n) {
                    result_array[(n - nmin) as usize] = -result_array[(n - nmin) as usize];
                }
            }
        }

        Ok(())
    }
}

pub fn bessel_in_e(n_in: i32, x: f64, result: &mut SfResult) -> Result<(), SfError> {
    let ax = x.abs();
    let n = n_in.abs();
    let mut in_scaled = SfResult::new(0.0, 0.0);
    bessel_in_scaled_e(n, ax, &mut in_scaled)?;

    if ax > f64::MAX.log2() - 1.0 {
        Err(SfError::Overflow)
    } else {
        let ex = ax.exp();
        result.val = ex * in_scaled.val;
        result.err = ex * in_scaled.err;
        result.err += ax * EPSILON * result.val.abs();
        if x < 0.0 && is_odd(n) {
            result.val = -result.val;
        }
        Ok(())
    }
}

pub fn bessel_in_array(nmin: i32, nmax: i32, x: f64, result_array: &mut [f64]) -> Result<(), SfError> {
    let ax = x.abs();

    if ax > f64::MAX.log2() - 1.0 {
        for j in 0..=(nmax - nmin) as usize {
            result_array[j] = 0.0;
        }
        Err(SfError::Overflow)
    } else {
        let eax = ax.exp();
        bessel_in_scaled_array(nmin, nmax, x, result_array)?;
        for j in 0..=(nmax - nmin) as usize {
            result_array[j] *= eax;
        }
        Ok(())
    }
}

pub fn bessel_in_scaled(n: i32, x: f64) -> Result<f64, SfError> {
    let mut result = SfResult::new(0.0, 0.0);
    bessel_in_scaled_e(n, x, &mut result)?;
    Ok(result.val)
}

pub fn bessel_in(n: i32, x: f64) -> Result<f64, SfError> {
    let mut result = SfResult::new(0.0, 0.0);
    bessel_in_e(n, x, &mut result)?;
    Ok(result.val)
}