use std::f64::consts::PI;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MathieuError {
    InvalidOrder,
    FailedCalculation,
    InvalidRange,
    InternalError,
}

impl fmt::Display for MathieuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MathieuError::InvalidOrder => write!(f, "Undefined order for Mathieu function"),
            MathieuError::FailedCalculation => write!(f, "Wrong characteristic Mathieu value"),
            MathieuError::InvalidRange => write!(f, "Invalid range [order_min, order_max]"),
            MathieuError::InternalError => write!(f, "Internal error in tridiagonal Mathieu matrix"),
        }
    }
}

impl Error for MathieuError {}

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

pub struct MathieuWorkspace {
    even_order: usize,
    odd_order: usize,
    extra_values: usize,
    size: usize,
    tt: Vec<f64>,
    dd: Vec<f64>,
    ee: Vec<f64>,
    e2: Vec<f64>,
    zz: Vec<f64>,
    aa: Vec<f64>,
    bb: Vec<f64>,
    eval: Vec<f64>,
    evec: Vec<Vec<f64>>,
}

impl MathieuWorkspace {
    pub fn new(size: usize) -> Self {
        let even_order = size + 1;
        let odd_order = size + 1;
        let extra_values = 1;
        
        MathieuWorkspace {
            even_order,
            odd_order,
            extra_values,
            size,
            tt: vec![0.0; 3 * even_order],
            dd: vec![0.0; even_order],
            ee: vec![0.0; even_order],
            e2: vec![0.0; even_order],
            zz: vec![0.0; even_order * even_order],
            aa: vec![0.0; size + 1],
            bb: vec![0.0; size + 1],
            eval: vec![0.0; even_order],
            evec: vec![vec![0.0; even_order]; even_order],
        }
    }
}

fn solve_cubic(c2: f64, c1: f64, c0: f64) -> f64 {
    let qq = (3.0 * c1 - c2 * c2) / 9.0;
    let rr = (9.0 * c2 * c1 - 27.0 * c0 - 2.0 * c2 * c2 * c2) / 54.0;
    let ww = qq * qq * qq + rr * rr;
    
    if ww >= 0.0 {
        let t1 = rr + ww.sqrt();
        let ss = t1.abs() / t1 * t1.abs().powf(1.0 / 3.0);
        let t1 = rr - ww.sqrt();
        let tt = t1.abs() / t1 * t1.abs().powf(1.0 / 3.0);
        ss + tt - c2 / 3.0
    } else {
        let theta = (rr / (-qq * qq * qq).sqrt()).acos();
        2.0 * (-qq).sqrt() * ((theta + 4.0 * PI) / 3.0).cos()
    }
}

fn asymptotic(order: i32, qq: f64) -> f64 {
    let nn = 2.0 * order as f64 + 1.0;
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

fn approx_c(order: i32, qq: f64) -> Result<f64, MathieuError> {
    if order < 0 {
        return Err(MathieuError::InvalidOrder);
    }
    
    match order {
        0 => {
            if qq <= 4.0 {
                Ok(2.0 - (4.0 + 2.0 * qq * qq).sqrt())
            } else {
                Ok(asymptotic(order, qq))
            }
        }
        1 => {
            if qq <= 4.0 {
                Ok(5.0 + 0.5 * (qq - (5.0 * qq * qq - 16.0 * qq + 64.0).sqrt()))
            } else {
                Ok(asymptotic(order, qq))
            }
        }
        2 => {
            if qq <= 3.0 {
                let c2 = -8.0;
                let c1 = -48.0 - 3.0 * qq * qq;
                let c0 = 20.0 * qq * qq;
                let approx = solve_cubic(c2, c1, c0);
                
                if approx < 0.0 && qq.sqrt() > 0.1 * order as f64 {
                    Ok(asymptotic(order - 1, qq))
                } else {
                    Ok((order * order) as f64 + approx.abs())
                }
            } else {
                Ok(asymptotic(order, qq))
            }
        }
        3 => {
            if qq <= 6.25 {
                let c2 = -qq - 8.0;
                let c1 = 16.0 * qq - 128.0 - 2.0 * qq * qq;
                let c0 = qq * qq * (qq + 8.0);
                let approx = solve_cubic(c2, c1, c0);
                
                if approx < 0.0 && qq.sqrt() > 0.1 * order as f64 {
                    Ok(asymptotic(order - 1, qq))
                } else {
                    Ok((order * order) as f64 + approx.abs())
                }
            } else {
                Ok(asymptotic(order, qq))
            }
        }
        _ => {
            if order < 70 {
                if 1.7 * order as f64 > 2.0 * qq.sqrt() {
                    let n2 = (order * order) as f64;
                    let n22 = ((n2 - 1.0) * (n2 - 1.0)) as f64;
                    let q2 = qq * qq;
                    let q4 = q2 * q2;
                    
                    let mut approx = n2 + 0.5 * q2 / (n2 - 1.0);
                    approx += (5.0 * n2 + 7.0) * q4 / (32.0 * n22 * (n2 - 1.0) * (n2 - 4.0));
                    approx += (9.0 * n2 * n2 + 58.0 * n2 + 29.0) * q4 * q2 /
                        (64.0 * n22 * n22 * (n2 - 1.0) * (n2 - 4.0) * (n2 - 9.0));
                    
                    if 1.4 * order as f64 < 2.0 * qq.sqrt() {
                        approx += asymptotic(order, qq);
                        approx *= 0.5;
                    }
                    Ok(approx)
                } else {
                    Ok(asymptotic(order, qq))
                }
            } else {
                Ok((order * order) as f64)
            }
        }
    }
}

fn approx_s(order: i32, qq: f64) -> Result<f64, MathieuError> {
    if order < 1 {
        return Err(MathieuError::InvalidOrder);
    }
    
    match order {
        1 => {
            if qq <= 4.0 {
                Ok(5.0 - 0.5 * (qq + (5.0 * qq * qq + 16.0 * qq + 64.0).sqrt()))
            } else {
                Ok(asymptotic(order - 1, qq))
            }
        }
        2 => {
            if qq <= 5.0 {
                Ok(10.0 - (36.0 + qq * qq).sqrt())
            } else {
                Ok(asymptotic(order - 1, qq))
            }
        }
        3 => {
            if qq <= 6.25 {
                let c2 = qq - 8.0;
                let c1 = -128.0 - 16.0 * qq - 2.0 * qq * qq;
                let c0 = qq * qq * (8.0 - qq);
                let approx = solve_cubic(c2, c1, c0);
                
                if approx < 0.0 && qq.sqrt() > 0.1 * order as f64 {
                    Ok(asymptotic(order - 1, qq))
                } else {
                    Ok((order * order) as f64 + approx.abs())
                }
            } else {
                Ok(asymptotic(order - 1, qq))
            }
        }
        _ => {
            if order < 70 {
                if 1.7 * order as f64 > 2.0 * qq.sqrt() {
                    let n2 = (order * order) as f64;
                    let n22 = ((n2 - 1.0) * (n2 - 1.0)) as f64;
                    let q2 = qq * qq;
                    let q4 = q2 * q2;
                    
                    let mut approx = n2 + 0.5 * q2 / (n2 - 1.0);
                    approx += (5.0 * n2 + 7.0) * q4 / (32.0 * n22 * (n2 - 1.0) * (n2 - 4.0));
                    approx += (9.0 * n2 * n2 + 58.0 * n2 + 29.0) * q4 * q2 /
                        (64.0 * n22 * n22 * (n2 - 1.0) * (n2 - 4.0) * (n2 - 9.0));
                    
                    if 1.4 * order as f64 < 2.0 * qq.sqrt() {
                        approx += asymptotic(order - 1, qq);
                        approx *= 0.5;
                    }
                    Ok(approx)
                } else {
                    Ok(asymptotic(order - 1, qq))
                }
            } else {
                Ok((order * order) as f64)
            }
        }
    }
}

fn ceer(order: i32, qq: f64, aa: f64, nterms: i32) -> f64 {
    let mut term = if order == 0 {
        0.0
    } else {
        let mut t = 2.0 * qq * qq / aa;
        
        if order != 2 {
            let n1 = order / 2 - 1;
            
            for ii in 0..n1 {
                t = qq * qq / (aa - 4.0 * (ii + 1) as f64 * (ii + 1) as f64 - t);
            }
        }
        t
    };
    
    term += (order * order) as f64;
    
    let mut term1 = 0.0;
    for ii in 0..nterms {
        term1 = qq * qq / (aa - (order + 2 * (nterms - ii)) as f64 * 
            (order + 2 * (nterms - ii)) as f64 - term1);
    }
    
    if order == 0 {
        term1 *= 2.0;
    }
    
    term + term1 - aa
}

fn ceor(order: i32, qq: f64, aa: f64, nterms: i32) -> f64 {
    let mut term = qq;
    let n1 = (order as f64 / 2.0 - 0.5) as i32;
    
    for ii in 0..n1 {
        term = qq * qq / (aa - (2 * ii + 1) as f64 * (2 * ii + 1) as f64 - term);
    }
    
    term += (order * order) as f64;
    
    let mut term1 = 0.0;
    for ii in 0..nterms {
        term1 = qq * qq / (aa - (order + 2 * (nterms - ii)) as f64 * 
            (order + 2 * (nterms - ii)) as f64 - term1);
    }
    
    term + term1 - aa
}

fn seer(order: i32, qq: f64, aa: f64, nterms: i32) -> f64 {
    let mut term = 0.0;
    let n1 = order / 2 - 1;
    
    for ii in 0..n1 {
        term = qq * qq / (aa - 4 * (ii + 1) as f64 * (ii + 1) as f64 - term);
    }
    
    term += (order * order) as f64;
    
    let mut term1 = 0.0;
    for ii in 0..nterms {
        term1 = qq * qq / (aa - (order + 2 * (nterms - ii)) as f64 * 
            (order + 2 * (nterms - ii)) as f64 - term1);
    }
    
    term + term1 - aa
}

fn seor(order: i32, qq: f64, aa: f64, nterms: i32) -> f64 {
    let mut term = -qq;
    let n1 = (order as f64 / 2.0 - 0.5) as i32;
    
    for ii in 0..n1 {
        term = qq * qq / (aa - (2 * ii + 1) as f64 * (2 * ii + 1) as f64 - term);
    }
    
    term += (order * order) as f64;
    
    let mut term1 = 0.0;
    for ii in 0..nterms {
        term1 = qq * qq / (aa - (order + 2 * (nterms - ii)) as f64 * 
            (order + 2 * (nterms - ii)) as f64 - term1);
    }
    
    term + term1 - aa
}

pub fn mathieu_a_e(order: i32, qq: f64) -> Result<SfResult, MathieuError> {
    let even_odd = order % 2 != 0;
    let nterms = 50;
    let maxcount = 1000;
    
    if qq == 0.0 {
        return Ok(SfResult {
            val: (order * order) as f64,
            err: 0.0,
        });
    }
    
    let order = if order < 0 { -order } else { order };
    let qq = if qq < 0.0 {
        if even_odd {
            return mathieu_b_e(order, -qq);
        } else {
            return mathieu_a_e(order, -qq);
        }
    } else {
        qq
    };
    
    let aa_approx = approx_c(order, qq)?;
    let mut aa_orig = aa_approx;
    let mut aa = aa_approx;
    let mut da = 0.025;
    let mut dir = 0;
    let mut counter = 0;
    
    let mut result = SfResult { val: 0.0, err: 0.0 };
    
    while counter < maxcount {
        let a1 = aa + 0.001;
        let mut ii = 0;
        let mut fa1 = if even_odd {
            ceor(order, qq, a1, nterms)
        } else {
            ceer(order, qq, a1, nterms)
        };
        
        loop {
            let fa = if even_odd {
                ceor(order, qq, aa, nterms)
            } else {
                ceer(order, qq, aa, nterms)
            };
            
            let a2 = a1;
            let a1 = aa;
            
            if fa == fa1 {
                result.err = f64::EPSILON;
                break;
            }
            
            aa -= (aa - a2) / (fa - fa1) * fa;
            let dela = (aa - a2).abs();
            
            if dela < f64::EPSILON {
                result.err = f64::EPSILON;
                break;
            }
            
            if ii > 40 {
                result.err = dela;
                break;
            }
            
            fa1 = fa;
            ii += 1;
        }
        
        if (aa - aa_orig).abs() > (3.0 + 0.01 * order as f64 * aa_orig.abs()) ||
            (order > 10