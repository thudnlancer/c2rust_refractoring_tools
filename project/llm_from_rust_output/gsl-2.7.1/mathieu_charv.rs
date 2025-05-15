use libc::{c_double, c_int, c_uint, c_ulong};
use std::f64::consts::PI;

const GSL_SUCCESS: c_int = 0;
const GSL_EFAILED: c_int = -1;
const GSL_EINVAL: c_int = 4;
const GSL_EIGEN_SORT_VAL_ASC: c_uint = 0;

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: c_double,
    pub err: c_double,
}

#[derive(Debug, Clone, Copy)]
pub struct GslVector {
    pub size: c_ulong,
    pub stride: c_ulong,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy)]
pub struct GslBlock {
    pub size: c_ulong,
    pub data: *mut c_double,
}

#[derive(Debug, Clone, Copy)]
pub struct GslMatrix {
    pub size1: c_ulong,
    pub size2: c_ulong,
    pub tda: c_ulong,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy)]
pub struct GslEigenSymmvWorkspace {
    pub size: c_ulong,
    pub d: *mut c_double,
    pub sd: *mut c_double,
    pub gc: *mut c_double,
    pub gs: *mut c_double,
}

#[derive(Debug, Clone, Copy)]
pub struct GslSfMathieuWorkspace {
    pub size: c_ulong,
    pub even_order: c_ulong,
    pub odd_order: c_ulong,
    pub extra_values: c_int,
    pub qa: c_double,
    pub qb: c_double,
    pub aa: *mut c_double,
    pub bb: *mut c_double,
    pub dd: *mut c_double,
    pub ee: *mut c_double,
    pub tt: *mut c_double,
    pub e2: *mut c_double,
    pub zz: *mut c_double,
    pub eval: *mut GslVector,
    pub evec: *mut GslMatrix,
    pub wmat: *mut GslEigenSymmvWorkspace,
}

fn ceer(order: c_int, qq: c_double, aa: c_double, nterms: c_int) -> c_double {
    let mut term = 0.0;
    let mut term1 = 0.0;
    
    if order == 0 {
        term = 0.0;
    } else {
        term = 2.0 * qq * qq / aa;
        if order != 2 {
            let n1 = order / 2 - 1;
            for ii in 0..n1 {
                term = qq * qq / (aa - 4.0 * ((ii + 1) as c_double).powi(2) - term);
            }
        }
    }
    
    term += (order * order) as c_double;
    
    for ii in 0..nterms {
        term1 = qq * qq / (aa - (order as c_double + 2.0 * (nterms - ii) as c_double).powi(2) - term1);
    }
    
    if order == 0 {
        term1 *= 2.0;
    }
    
    term + term1 - aa
}

fn ceor(order: c_int, qq: c_double, aa: c_double, nterms: c_int) -> c_double {
    let mut term = qq;
    let n1 = (order as c_double / 2.0 - 0.5) as c_int;
    
    for ii in 0..n1 {
        term = qq * qq / (aa - (2.0 * ii as c_double + 1.0).powi(2) - term);
    }
    
    term += (order * order) as c_double;
    
    let mut term1 = 0.0;
    for ii in 0..nterms {
        term1 = qq * qq / (aa - (order as c_double + 2.0 * (nterms - ii) as c_double).powi(2) - term1);
    }
    
    term + term1 - aa
}

fn seer(order: c_int, qq: c_double, aa: c_double, nterms: c_int) -> c_double {
    let mut term = 0.0;
    let n1 = order / 2 - 1;
    
    for ii in 0..n1 {
        term = qq * qq / (aa - 4.0 * ((ii + 1) as c_double).powi(2) - term);
    }
    
    term += (order * order) as c_double;
    
    let mut term1 = 0.0;
    for ii in 0..nterms {
        term1 = qq * qq / (aa - (order as c_double + 2.0 * (nterms - ii) as c_double).powi(2) - term1);
    }
    
    term + term1 - aa
}

fn seor(order: c_int, qq: c_double, aa: c_double, nterms: c_int) -> c_double {
    let mut term = -qq;
    let n1 = (order as c_double / 2.0 - 0.5) as c_int;
    
    for ii in 0..n1 {
        term = qq * qq / (aa - (2.0 * ii as c_double + 1.0).powi(2) - term);
    }
    
    term += (order * order) as c_double;
    
    let mut term1 = 0.0;
    for ii in 0..nterms {
        term1 = qq * qq / (aa - (order as c_double + 2.0 * (nterms - ii) as c_double).powi(2) - term1);
    }
    
    term + term1 - aa
}

fn asymptotic(order: c_int, qq: c_double) -> c_double {
    let nn = (2 * order + 1) as c_double;
    let n2 = nn * nn;
    let n4 = n2 * n2;
    let n6 = n4 * n2;
    let hh = 2.0 * qq.sqrt();
    let ah = 16.0 * hh;
    let ah2 = ah * ah;
    let ah3 = ah2 * ah;
    let ah4 = ah3 * ah;
    let ah5 = ah4 * ah;
    
    let mut asymp = -2.0 * qq + nn * hh - 0.125 * (n2 + 1.0);
    asymp -= 0.25 * nn * (n2 + 3.0) / ah;
    asymp -= 0.25 * (5.0 * n4 + 34.0 * n2 + 9.0) / ah2;
    asymp -= 0.25 * nn * (33.0 * n4 + 410.0 * n2 + 405.0) / ah3;
    asymp -= (63.0 * n6 + 1260.0 * n4 + 2943.0 * n2 + 486.0) / ah4;
    asymp -= nn * (527.0 * n6 + 15617.0 * n4 + 69001.0 * n2 + 41607.0) / ah5;
    
    asymp
}

fn solve_cubic(c2: c_double, c1: c_double, c0: c_double) -> c_double {
    let q = (3.0 * c1 - c2 * c2) / 9.0;
    let r = (9.0 * c2 * c1 - 27.0 * c0 - 2.0 * c2.powi(3)) / 54.0;
    let w = q.powi(3) + r * r;
    
    if w >= 0.0 {
        let t1 = r + w.sqrt();
        let s = t1.abs() / t1 * t1.abs().powf(1.0 / 3.0);
        let t1 = r - w.sqrt();
        let t = t1.abs() / t1 * t1.abs().powf(1.0 / 3.0);
        s + t - c2 / 3.0
    } else {
        let theta = (r / (-q.powi(3)).sqrt()).acos();
        2.0 * (-q).sqrt() * cos((theta + 4.0 * PI) / 3.0)
    }
}

pub fn gsl_sf_mathieu_a_e(order: c_int, qq: c_double, result: &mut GslSfResult) -> c_int {
    let mut even_odd = 0;
    if order % 2 != 0 {
        even_odd = 1;
    }
    
    if qq == 0.0 {
        result.val = (order * order) as c_double;
        result.err = 0.0;
        return GSL_SUCCESS;
    }
    
    if order < 0 {
        return gsl_sf_mathieu_a_e(-order, qq, result);
    }
    
    if qq < 0.0 {
        if even_odd == 0 {
            return gsl_sf_mathieu_a_e(order, -qq, result);
        } else {
            return gsl_sf_mathieu_b_e(order, -qq, result);
        }
    }
    
    let aa_approx = approx_c(order, qq);
    let mut aa = aa_approx;
    let aa_orig = aa;
    let mut counter = 0;
    let maxcount = 1000;
    let mut dir = 0;
    let mut da = 0.025;
    
    while counter < maxcount {
        let a1 = aa + 0.001;
        let mut fa1 = if even_odd == 0 {
            ceer(order, qq, a1, 50)
        } else {
            ceor(order, qq, a1, 50)
        };
        
        let mut ii = 0;
        let mut dela;
        loop {
            let fa = if even_odd == 0 {
                ceer(order, qq, aa, 50)
            } else {
                ceor(order, qq, aa, 50)
            };
            
            let a2 = a1;
            if fa == fa1 {
                result.err = 2.2204460492503131e-16;
                break;
            } else {
                aa -= (aa - a2) / (fa - fa1) * fa;
                dela = (aa - a2).abs();
                if dela < 2.2204460492503131e-16 {
                    result.err = 2.2204460492503131e-16;
                    break;
                } else if ii > 40 {
                    result.err = dela;
                    break;
                } else {
                    fa1 = fa;
                    ii += 1;
                }
            }
        }
        
        if !((aa - aa_orig).abs() > 3.0 + 0.01 * order as c_double * aa_orig.abs()
            || (order > 10 && (aa - aa_orig).abs() > 1.5 * order as c_double))
        {
            break;
        }
        
        counter += 1;
        if counter == maxcount {
            result.err = (aa - aa_orig).abs();
            break;
        } else {
            if aa > aa_orig {
                if dir == 1 {
                    da /= 2.0;
                }
                dir = -1;
            } else {
                if dir == -1 {
                    da /= 2.0;
                }
                dir = 1;
            }
            aa = aa_approx + dir as c_double * da * counter as c_double;
        }
    }
    
    result.val = aa;
    if counter == maxcount {
        return GSL_EFAILED;
    }
    GSL_SUCCESS
}

// Similar implementations for other functions like gsl_sf_mathieu_b_e, 
// gsl_sf_mathieu_a_array, gsl_sf_mathieu_b_array would follow the same pattern
// of converting the C code to safe Rust while maintaining the same functionality

// Helper functions like approx_c, approx_s would also be implemented similarly