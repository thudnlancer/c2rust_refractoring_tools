use std::f64::consts::{PI, E};
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

pub const GSL_DBL_EPSILON: f64 = f64::EPSILON;
pub const GSL_DBL_MIN: f64 = f64::MIN_POSITIVE;
pub const GSL_SQRT_DBL_MIN: f64 = GSL_DBL_MIN.sqrt();
pub const GSL_ROOT4_DBL_EPSILON: f64 = GSL_DBL_EPSILON.sqrt().sqrt();
pub const GSL_ROOT6_DBL_EPSILON: f64 = GSL_DBL_EPSILON.powf(1.0/6.0);
pub const BESSEL_J_SMALL: f64 = GSL_DBL_MIN / GSL_DBL_EPSILON;

#[derive(Debug, Clone, Copy)]
pub enum SfError {
    Domain,
    Underflow,
    MaxIter,
    Other,
}

pub type SfResultT<T> = Result<T, SfError>;

pub fn gsl_sf_bessel_j0_e(x: f64) -> SfResultT<SfResult> {
    let ax = x.abs();
    let mut result = SfResult { val: 0.0, err: 0.0 };

    if ax < 0.5 {
        let y = x * x;
        let c1 = -1.0 / 6.0;
        let c2 = 1.0 / 120.0;
        let c3 = -1.0 / 5040.0;
        let c4 = 1.0 / 362880.0;
        let c5 = -1.0 / 39916800.0;
        let c6 = 1.0 / 6227020800.0;
        result.val = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * c6)))));
    } else {
        result.val = x.sin() / x;
    }
    result.err = 2.0 * GSL_DBL_EPSILON * result.val.abs();
    Ok(result)
}

pub fn gsl_sf_bessel_j1_e(x: f64) -> SfResultT<SfResult> {
    let ax = x.abs();
    let mut result = SfResult { val: 0.0, err: 0.0 };

    if x == 0.0 {
        result.val = 0.0;
        result.err = 0.0;
        return Ok(result);
    } else if ax < 3.1 * GSL_DBL_MIN {
        return Err(SfError::Underflow);
    } else if ax < 0.25 {
        let y = x * x;
        let c1 = -1.0 / 10.0;
        let c2 = 1.0 / 280.0;
        let c3 = -1.0 / 15120.0;
        let c4 = 1.0 / 1330560.0;
        let c5 = -1.0 / 172972800.0;
        let sum = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * c5)));
        result.val = x / 3.0 * sum;
    } else {
        let cos_x = x.cos();
        let sin_x = x.sin();
        result.val = (sin_x / x - cos_x) / x;
        result.err = 2.0 * GSL_DBL_EPSILON * (sin_x / (x * x).abs() + cos_x / x.abs());
    }
    result.err += 2.0 * GSL_DBL_EPSILON * result.val.abs();
    Ok(result)
}

pub fn gsl_sf_bessel_j2_e(x: f64) -> SfResultT<SfResult> {
    let ax = x.abs();
    let mut result = SfResult { val: 0.0, err: 0.0 };

    if x == 0.0 {
        result.val = 0.0;
        result.err = 0.0;
        return Ok(result);
    } else if ax < 4.0 * GSL_SQRT_DBL_MIN {
        return Err(SfError::Underflow);
    } else if ax < 1.3 {
        let y = x * x;
        let c1 = -1.0 / 14.0;
        let c2 = 1.0 / 504.0;
        let c3 = -1.0 / 33264.0;
        let c4 = 1.0 / 3459456.0;
        let c5 = -1.0 / 518918400.0;
        let c6 = 1.0 / 105859353600.0;
        let c7 = -1.0 / 28158588057600.0;
        let c8 = 1.0 / 9461285587353600.0;
        let c9 = -1.0 / 3916972233164390400.0;
        let sum = 1.0 + y * (c1 + y * (c2 + y * (c3 + y * (c4 + y * (c5 + y * (c6 + y * (c7 + y * (c8 + y * c9)))))));
        result.val = y / 15.0 * sum;
    } else {
        let cos_x = x.cos();
        let sin_x = x.sin();
        let f = 3.0 / (x * x) - 1.0;
        result.val = (f * sin_x - 3.0 * cos_x / x) / x;
        result.err = 2.0 * GSL_DBL_EPSILON * (f * sin_x / x.abs() + 3.0 * cos_x / (x * x).abs());
    }
    result.err += 2.0 * GSL_DBL_EPSILON * result.val.abs();
    Ok(result)
}

pub fn gsl_sf_bessel_jl_e(l: i32, x: f64) -> SfResultT<SfResult> {
    if l < 0 || x < 0.0 {
        Err(SfError::Domain)
    } else if x == 0.0 {
        Ok(SfResult {
            val: if l > 0 { 0.0 } else { 1.0 },
            err: 0.0,
        })
    } else if l == 0 {
        gsl_sf_bessel_j0_e(x)
    } else if l == 1 {
        gsl_sf_bessel_j1_e(x)
    } else if l == 2 {
        gsl_sf_bessel_j2_e(x)
    } else if x * x < 10.0 * (l as f64 + 0.5) / E {
        let b = gsl_sf_bessel_IJ_taylor_e(l as f64 + 0.5, x, -1, 50, GSL_DBL_EPSILON)?;
        let pre = (0.5 * PI / x).sqrt();
        Ok(SfResult {
            val: pre * b.val,
            err: pre * b.err + 2.0 * GSL_DBL_EPSILON * (pre * b.val).abs(),
        })
    } else if GSL_ROOT4_DBL_EPSILON * x > (l * l + l + 1) as f64 {
        let b = gsl_sf_bessel_Jnu_asympx_e(l as f64 + 0.5, x)?;
        let pre = (0.5 * PI / x).sqrt();
        Ok(SfResult {
            val: pre * b.val,
            err: 2.0 * GSL_DBL_EPSILON * (pre * b.val).abs() + pre * b.err,
        })
    } else if l as f64 > 1.0 / GSL_ROOT6_DBL_EPSILON {
        let b = gsl_sf_bessel_Jnu_asymp_Olver_e(l as f64 + 0.5, x)?;
        let pre = (0.5 * PI / x).sqrt();
        Ok(SfResult {
            val: pre * b.val,
            err: 2.0 * GSL_DBL_EPSILON * (pre * b.val).abs() + pre * b.err,
        })
    } else if x > 1000.0 && x > (l * l) as f64 {
        let b = gsl_sf_bessel_Jnu_asympx_e(l as f64 + 0.5, x)?;
        let pre = (0.5 * PI / x).sqrt();
        Ok(SfResult {
            val: pre * b.val,
            err: 2.0 * GSL_DBL_EPSILON * (pre * b.val).abs() + pre * b.err,
        })
    } else {
        let (ratio, sgn) = gsl_sf_bessel_J_CF1(l as f64 + 0.5, x)?;
        let mut jellp1 = BESSEL_J_SMALL * ratio;
        let mut jell = BESSEL_J_SMALL;
        let mut jellm1;
        
        for ell in (1..=l).rev() {
            jellm1 = -jellp1 + (2 * ell + 1) as f64 / x * jell;
            jellp1 = jell;
            jell = jellm1;
        }

        if jell.abs() > jellp1.abs() {
            let j0_result = gsl_sf_bessel_j0_e(x)?;
            let pre = BESSEL_J_SMALL / jell;
            Ok(SfResult {
                val: j0_result.val * pre,
                err: j0_result.err * pre.abs() + 4.0 * GSL_DBL_EPSILON * (0.5 * l as f64 + 1.0) * (j0_result.val * pre).abs(),
            })
        } else {
            let j1_result = gsl_sf_bessel_j1_e(x)?;
            let pre = BESSEL_J_SMALL / jellp1;
            Ok(SfResult {
                val: j1_result.val * pre,
                err: j1_result.err * pre.abs() + 4.0 * GSL_DBL_EPSILON * (0.5 * l as f64 + 1.0) * (j1_result.val * pre).abs(),
            })
        }
    }
}

pub fn gsl_sf_bessel_jl_array(lmax: i32, x: f64) -> SfResultT<Vec<f64>> {
    if lmax < 0 || x < 0.0 {
        Err(SfError::Domain)
    } else if x == 0.0 {
        let mut result = vec![0.0; (lmax + 1) as usize];
        result[0] = 1.0;
        Ok(result)
    } else {
        let r_jellp1 = gsl_sf_bessel_jl_e(lmax + 1, x)?;
        let r_jell = gsl_sf_bessel_jl_e(lmax, x)?;
        let mut jellp1 = r_jellp1.val;
        let mut jell = r_jell.val;
        let mut jellm1;
        
        let mut result = vec![0.0; (lmax + 1) as usize];
        result[lmax as usize] = jell;
        
        for ell in (1..=lmax).rev() {
            jellm1 = -jellp1 + (2 * ell + 1) as f64 / x * jell;
            jellp1 = jell;
            jell = jellm1;
            result[(ell - 1) as usize] = jellm1;
        }
        
        Ok(result)
    }
}

// Helper functions (stubs for now - would need full implementations)
fn gsl_sf_bessel_IJ_taylor_e(nu: f64, x: f64, sign: i32, maxiter: i32, eps: f64) -> SfResultT<SfResult> {
    unimplemented!()
}

fn gsl_sf_bessel_Jnu_asympx_e(nu: f64, x: f64) -> SfResultT<SfResult> {
    unimplemented!()
}

fn gsl_sf_bessel_Jnu_asymp_Olver_e(nu: f64, x: f64) -> SfResultT<SfResult> {
    unimplemented!()
}

fn gsl_sf_bessel_J_CF1(nu: f64, x: f64) -> SfResultT<(f64, f64)> {
    unimplemented!()
}

// Simple evaluation functions
pub fn gsl_sf_bessel_j0(x: f64) -> f64 {
    gsl_sf_bessel_j0_e(x).unwrap().val
}

pub fn gsl_sf_bessel_j1(x: f64) -> f64 {
    gsl_sf_bessel_j1_e(x).unwrap().val
}

pub fn gsl_sf_bessel_j2(x: f64) -> f64 {
    gsl_sf_bessel_j2_e(x).unwrap().val
}

pub fn gsl_sf_bessel_jl(l: i32, x: f64) -> f64 {
    gsl_sf_bessel_jl_e(l, x).unwrap().val
}