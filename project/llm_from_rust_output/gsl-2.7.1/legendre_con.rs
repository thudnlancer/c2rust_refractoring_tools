use std::f64::consts::{PI, FRAC_1_SQRT_2, SQRT_2};
use std::f64::{NAN, INFINITY};

const GSL_SUCCESS: i32 = 0;
const GSL_EDOM: i32 = 1;
const GSL_EMAXITER: i32 = 11;

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[inline]
fn gsl_max(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn conicalp_negmu_xlt1_cf1(
    mu: f64,
    ell: i32,
    tau: f64,
    x: f64,
    result: &mut GslSfResult,
) -> i32 {
    const RECUR_BIG: f64 = 1.3407807929942596e+154;
    const MAXITER: i32 = 5000;
    
    let xi = x / ((1.0 - x).sqrt() * (1.0 + x).sqrt());
    let mut anm2 = 1.0;
    let mut bnm2 = 0.0;
    let mut anm1 = 0.0;
    let mut bnm1 = 1.0;
    let a1 = 1.0;
    let b1 = 2.0 * (mu + ell as f64 + 1.0) * xi;
    let mut an = b1 * anm1 + a1 * anm2;
    let mut bn = b1 * bnm1 + a1 * bnm2;
    let mut fn_val = an / bn;
    
    for n in 1..MAXITER {
        let old_fn = fn_val;
        anm2 = anm1;
        bnm2 = bnm1;
        anm1 = an;
        bnm1 = bn;
        
        let an_term = tau.powi(2) + (mu - 0.5 + ell as f64 + n as f64).powi(2);
        let bn_term = 2.0 * (ell as f64 + mu + n as f64) * xi;
        
        an = bn_term * anm1 + an_term * anm2;
        bn = bn_term * bnm1 + an_term * bnm2;
        
        if an.abs() > RECUR_BIG || bn.abs() > RECUR_BIG {
            an /= RECUR_BIG;
            bn /= RECUR_BIG;
            anm1 /= RECUR_BIG;
            bnm1 /= RECUR_BIG;
            anm2 /= RECUR_BIG;
            bnm2 /= RECUR_BIG;
        }
        
        fn_val = an / bn;
        let del = old_fn / fn_val;
        
        if (del - 1.0).abs() < 2.0 * f64::EPSILON {
            break;
        }
    }
    
    result.val = fn_val;
    result.err = 4.0 * f64::EPSILON * (MAXITER as f64.sqrt() + 1.0) * fn_val.abs();
    
    if MAXITER >= MAXITER {
        // gsl_error(...)
        GSL_EMAXITER
    } else {
        GSL_SUCCESS
    }
}

// 其他函数类似地进行转换...

pub fn gsl_sf_conicalP_0(lambda: f64, x: f64) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    let status = gsl_sf_conicalP_0_e(lambda, x, &mut result);
    if status != GSL_SUCCESS {
        // 处理错误
        return NAN;
    }
    result.val
}

// 类似地实现其他公共接口函数...

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conicalP_0() {
        let res = gsl_sf_conicalP_0(1.0, 0.5);
        assert!((res - 1.0).abs() < 1e-10);
    }
}