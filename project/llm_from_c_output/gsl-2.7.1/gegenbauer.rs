use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

pub const GSL_SUCCESS: i32 = 0;
pub const GSL_EDOM: i32 = 1;
pub const GSL_DBL_EPSILON: f64 = std::f64::EPSILON;

pub fn gsl_sf_gegenpoly_1_e(lambda: f64, x: f64) -> Result<SfResult, i32> {
    if lambda == 0.0 {
        Ok(SfResult {
            val: 2.0 * x,
            err: 2.0 * GSL_DBL_EPSILON * (2.0 * x).abs(),
        })
    } else {
        Ok(SfResult {
            val: 2.0 * lambda * x,
            err: 4.0 * GSL_DBL_EPSILON * (2.0 * lambda * x).abs(),
        })
    }
}

pub fn gsl_sf_gegenpoly_2_e(lambda: f64, x: f64) -> Result<SfResult, i32> {
    if lambda == 0.0 {
        let txx = 2.0 * x * x;
        let val = -1.0 + txx;
        Ok(SfResult {
            val,
            err: 2.0 * GSL_DBL_EPSILON * txx.abs() + 2.0 * GSL_DBL_EPSILON * val.abs(),
        })
    } else {
        let val = lambda * (-1.0 + 2.0 * (1.0 + lambda) * x * x);
        Ok(SfResult {
            val,
            err: GSL_DBL_EPSILON * (2.0 * val.abs() + lambda.abs()),
        })
    }
}

pub fn gsl_sf_gegenpoly_3_e(lambda: f64, x: f64) -> Result<SfResult, i32> {
    if lambda == 0.0 {
        let val = x * (-2.0 + 4.0 / 3.0 * x * x);
        Ok(SfResult {
            val,
            err: GSL_DBL_EPSILON * (2.0 * val.abs() + x.abs()),
        })
    } else {
        let c = 4.0 + lambda * (6.0 + 2.0 * lambda);
        let val = 2.0 * lambda * x * (-1.0 - lambda + c * x * x / 3.0);
        Ok(SfResult {
            val,
            err: GSL_DBL_EPSILON * (2.0 * val.abs() + (lambda * x).abs()),
        })
    }
}

pub fn gsl_sf_gegenpoly_n_e(n: i32, lambda: f64, x: f64) -> Result<SfResult, i32> {
    if lambda <= -0.5 || n < 0 {
        Err(GSL_EDOM)
    } else if n == 0 {
        Ok(SfResult { val: 1.0, err: 0.0 })
    } else if n == 1 {
        gsl_sf_gegenpoly_1_e(lambda, x)
    } else if n == 2 {
        gsl_sf_gegenpoly_2_e(lambda, x)
    } else if n == 3 {
        gsl_sf_gegenpoly_3_e(lambda, x)
    } else {
        if lambda == 0.0 && (x >= -1.0 && x <= 1.0) {
            let z = n as f64 * x.acos();
            let val = 2.0 * (z).cos() / n as f64;
            Ok(SfResult {
                val,
                err: 2.0 * GSL_DBL_EPSILON * (z * val).abs(),
            })
        } else {
            let g2 = gsl_sf_gegenpoly_2_e(lambda, x)?;
            let g3 = gsl_sf_gegenpoly_3_e(lambda, x)?;
            let mut gkm2 = g2.val;
            let mut gkm1 = g3.val;
            let mut gk = 0.0;
            for k in 4..=n {
                let kf = k as f64;
                gk = (2.0 * (kf + lambda - 1.0) * x * gkm1 - (kf + 2.0 * lambda - 2.0) * gkm2) / kf;
                gkm2 = gkm1;
                gkm1 = gk;
            }
            Ok(SfResult {
                val: gk,
                err: 2.0 * GSL_DBL_EPSILON * 0.5 * n as f64 * gk.abs(),
            })
        }
    }
}

pub fn gsl_sf_gegenpoly_array(nmax: i32, lambda: f64, x: f64) -> Result<Vec<f64>, i32> {
    if lambda <= -0.5 || nmax < 0 {
        return Err(GSL_EDOM);
    }

    let mut result_array = vec![0.0; (nmax + 1) as usize];
    result_array[0] = 1.0;
    if nmax == 0 {
        return Ok(result_array);
    }

    if lambda == 0.0 {
        result_array[1] = 2.0 * x;
    } else {
        result_array[1] = 2.0 * lambda * x;
    }

    for k in 2..=nmax {
        let kf = k as f64;
        let term1 = 2.0 * (kf + lambda - 1.0) * x * result_array[(k - 1) as usize];
        let term2 = (kf + 2.0 * lambda - 2.0) * result_array[(k - 2) as usize];
        result_array[k as usize] = (term1 - term2) / kf;
    }

    Ok(result_array)
}

pub fn gsl_sf_gegenpoly_1(lambda: f64, x: f64) -> f64 {
    gsl_sf_gegenpoly_1_e(lambda, x).unwrap().val
}

pub fn gsl_sf_gegenpoly_2(lambda: f64, x: f64) -> f64 {
    gsl_sf_gegenpoly_2_e(lambda, x).unwrap().val
}

pub fn gsl_sf_gegenpoly_3(lambda: f64, x: f64) -> f64 {
    gsl_sf_gegenpoly_3_e(lambda, x).unwrap().val
}

pub fn gsl_sf_gegenpoly_n(n: i32, lambda: f64, x: f64) -> f64 {
    gsl_sf_gegenpoly_n_e(n, lambda, x).unwrap().val
}