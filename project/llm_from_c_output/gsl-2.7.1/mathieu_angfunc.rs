use std::f64::consts::PI;
use std::f64::EPSILON as DBL_EPSILON;
use std::num::FpCategory;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug)]
pub struct MathieuWorkspace {
    pub aa: Vec<f64>,
    pub bb: Vec<f64>,
    pub size: usize,
}

impl MathieuWorkspace {
    pub fn new(size: usize) -> Self {
        Self {
            aa: vec![0.0; size],
            bb: vec![0.0; size],
            size,
        }
    }
}

pub const GSL_SF_MATHIEU_COEFF: usize = 100;
pub const GSL_SUCCESS: i32 = 0;
pub const GSL_EINVAL: i32 = 1;
pub const GSL_EDOM: i32 = 2;

pub fn gsl_sf_mathieu_ce_e(order: i32, qq: f64, zz: f64) -> Result<SfResult, i32> {
    let mut even_odd = 0;
    if order % 2 != 0 {
        even_odd = 1;
    }

    if qq == 0.0 {
        let mut norm = 1.0;
        if order == 0 {
            norm = 2.0.sqrt();
        }

        let fn_val = (order as f64 * zz).cos() / norm;
        let mut err = 2.0 * DBL_EPSILON;
        let factor = fn_val.abs();
        if factor > 1.0 {
            err *= factor;
        }

        return Ok(SfResult { val: fn_val, err });
    }

    let order = if order < 0 { -order } else { order };

    let aa = match gsl_sf_mathieu_a_e(order, qq) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let coeff = match gsl_sf_mathieu_a_coeff(order, qq, aa.val) {
        Ok(coeff) => coeff,
        Err(e) => return Err(e),
    };

    let (fn_val, norm) = if even_odd == 0 {
        let mut fn_val = 0.0;
        let mut norm = coeff[0] * coeff[0];
        for (ii, c) in coeff.iter().enumerate().take(GSL_SF_MATHIEU_COEFF) {
            fn_val += c * (2.0 * ii as f64 * zz).cos();
            norm += c * c;
        }
        (fn_val, norm)
    } else {
        let mut fn_val = 0.0;
        let mut norm = 0.0;
        for (ii, c) in coeff.iter().enumerate().take(GSL_SF_MATHIEU_COEFF) {
            fn_val += c * ((2.0 * ii as f64 + 1.0) * zz).cos();
            norm += c * c;
        }
        (fn_val, norm)
    };

    let norm = norm.sqrt();
    let fn_val = fn_val / norm;
    let mut err = 2.0 * DBL_EPSILON;
    let factor = fn_val.abs();
    if factor > 1.0 {
        err *= factor;
    }

    Ok(SfResult { val: fn_val, err })
}

pub fn gsl_sf_mathieu_se_e(order: i32, qq: f64, zz: f64) -> Result<SfResult, i32> {
    let mut even_odd = 0;
    if order % 2 != 0 {
        even_odd = 1;
    }

    if order == 0 {
        return Ok(SfResult { val: 0.0, err: 0.0 });
    }

    if qq == 0.0 {
        let norm = 1.0;
        let fn_val = (order as f64 * zz).sin();
        let mut err = 2.0 * DBL_EPSILON;
        let factor = fn_val.abs();
        if factor > 1.0 {
            err *= factor;
        }

        return Ok(SfResult { val: fn_val, err });
    }

    let order = if order < 0 { -order } else { order };

    let aa = match gsl_sf_mathieu_b_e(order, qq) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let coeff = match gsl_sf_mathieu_b_coeff(order, qq, aa.val) {
        Ok(coeff) => coeff,
        Err(e) => return Err(e),
    };

    let (fn_val, norm) = if even_odd == 0 {
        let mut fn_val = 0.0;
        let mut norm = 0.0;
        for (ii, c) in coeff.iter().enumerate().take(GSL_SF_MATHIEU_COEFF) {
            norm += c * c;
            fn_val += c * (2.0 * (ii + 1) as f64 * zz).sin();
        }
        (fn_val, norm)
    } else {
        let mut fn_val = 0.0;
        let mut norm = 0.0;
        for (ii, c) in coeff.iter().enumerate().take(GSL_SF_MATHIEU_COEFF) {
            norm += c * c;
            fn_val += c * ((2.0 * ii as f64 + 1.0) * zz).sin();
        }
        (fn_val, norm)
    };

    let norm = norm.sqrt();
    let fn_val = fn_val / norm;
    let mut err = 2.0 * DBL_EPSILON;
    let factor = fn_val.abs();
    if factor > 1.0 {
        err *= factor;
    }

    Ok(SfResult { val: fn_val, err })
}

pub fn gsl_sf_mathieu_ce_array(
    nmin: i32,
    nmax: i32,
    qq: f64,
    zz: f64,
    work: &mut MathieuWorkspace,
) -> Result<Vec<f64>, i32> {
    if work.size < nmax as usize {
        return Err(GSL_EINVAL);
    }

    if nmin < 0 || nmax < nmin {
        return Err(GSL_EDOM);
    }

    let mut result_array = vec![0.0; (nmax - nmin + 1) as usize];
    gsl_sf_mathieu_a_array(0, nmax, qq, work);

    for (ii, order) in (nmin..=nmax).enumerate() {
        let mut even_odd = 0;
        if order % 2 != 0 {
            even_odd = 1;
        }

        if qq == 0.0 {
            let mut norm = 1.0;
            if order == 0 {
                norm = 2.0.sqrt();
            }
            result_array[ii] = (order as f64 * zz).cos() / norm;
            continue;
        }

        let coeff = match gsl_sf_mathieu_a_coeff(order, qq, work.aa[order as usize]) {
            Ok(coeff) => coeff,
            Err(e) => return Err(e),
        };

        let (fn_val, norm) = if even_odd == 0 {
            let mut fn_val = 0.0;
            let mut norm = coeff[0] * coeff[0];
            for (jj, c) in coeff.iter().enumerate().take(GSL_SF_MATHIEU_COEFF) {
                fn_val += c * (2.0 * jj as f64 * zz).cos();
                norm += c * c;
            }
            (fn_val, norm)
        } else {
            let mut fn_val = 0.0;
            let mut norm = 0.0;
            for (jj, c) in coeff.iter().enumerate().take(GSL_SF_MATHIEU_COEFF) {
                fn_val += c * ((2.0 * jj as f64 + 1.0) * zz).cos();
                norm += c * c;
            }
            (fn_val, norm)
        };

        let norm = norm.sqrt();
        result_array[ii] = fn_val / norm;
    }

    Ok(result_array)
}

pub fn gsl_sf_mathieu_se_array(
    nmin: i32,
    nmax: i32,
    qq: f64,
    zz: f64,
    work: &mut MathieuWorkspace,
) -> Result<Vec<f64>, i32> {
    if work.size < nmax as usize {
        return Err(GSL_EINVAL);
    }

    if nmin < 0 || nmax < nmin {
        return Err(GSL_EDOM);
    }

    let mut result_array = vec![0.0; (nmax - nmin + 1) as usize];
    gsl_sf_mathieu_b_array(0, nmax, qq, work);

    for (ii, order) in (nmin..=nmax).enumerate() {
        let mut even_odd = 0;
        if order % 2 != 0 {
            even_odd = 1;
        }

        if order == 0 {
            result_array[ii] = 0.0;
            continue;
        }

        if qq == 0.0 {
            result_array[ii] = (order as f64 * zz).sin();
            continue;
        }

        let coeff = match gsl_sf_mathieu_b_coeff(order, qq, work.bb[order as usize]) {
            Ok(coeff) => coeff,
            Err(e) => return Err(e),
        };

        let (fn_val, norm) = if even_odd == 0 {
            let mut fn_val = 0.0;
            let mut norm = 0.0;
            for (jj, c) in coeff.iter().enumerate().take(GSL_SF_MATHIEU_COEFF) {
                fn_val += c * (2.0 * (jj + 1) as f64 * zz).sin();
                norm += c * c;
            }
            (fn_val, norm)
        } else {
            let mut fn_val = 0.0;
            let mut norm = 0.0;
            for (jj, c) in coeff.iter().enumerate().take(GSL_SF_MATHIEU_COEFF) {
                fn_val += c * ((2.0 * jj as f64 + 1.0) * zz).sin();
                norm += c * c;
            }
            (fn_val, norm)
        };

        let norm = norm.sqrt();
        result_array[ii] = fn_val / norm;
    }

    Ok(result_array)
}

pub fn gsl_sf_mathieu_ce(order: i32, qq: f64, zz: f64) -> f64 {
    match gsl_sf_mathieu_ce_e(order, qq, zz) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

pub fn gsl_sf_mathieu_se(order: i32, qq: f64, zz: f64) -> f64 {
    match gsl_sf_mathieu_se_e(order, qq, zz) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}

// Placeholder implementations for the missing functions
pub fn gsl_sf_mathieu_a_e(_order: i32, _qq: f64) -> Result<SfResult, i32> {
    unimplemented!()
}

pub fn gsl_sf_mathieu_b_e(_order: i32, _qq: f64) -> Result<SfResult, i32> {
    unimplemented!()
}

pub fn gsl_sf_mathieu_a_coeff(_order: i32, _qq: f64, _aa: f64) -> Result<[f64; GSL_SF_MATHIEU_COEFF], i32> {
    unimplemented!()
}

pub fn gsl_sf_mathieu_b_coeff(_order: i32, _qq: f64, _bb: f64) -> Result<[f64; GSL_SF_MATHIEU_COEFF], i32> {
    unimplemented!()
}

pub fn gsl_sf_mathieu_a_array(_nmin: i32, _nmax: i32, _qq: f64, _work: &mut MathieuWorkspace) {
    unimplemented!()
}

pub fn gsl_sf_mathieu_b_array(_nmin: i32, _nmax: i32, _qq: f64, _work: &mut MathieuWorkspace) {
    unimplemented!()
}