/* integration/chebyshev.rs

 * This code is based on IQPACK, specifically the LGPL
 * implementation found in HERMITE_RULE:
 * https://people.sc.fsu.edu/~jburkardt/c_src/hermite_rule/hermite_rule.html
 */

use std::f64::consts::{PI, FRAC_1_SQRT_2};
use std::f64::EPSILON;

#[derive(Debug)]
pub struct FixedParams {
    pub a: f64,
    pub b: f64,
    pub zemu: f64,
    pub shft: f64,
    pub slp: f64,
    pub al: f64,
    pub be: f64,
}

#[derive(Debug)]
pub struct ChebyshevType;

impl ChebyshevType {
    pub fn check(n: usize, params: &FixedParams) -> Result<(), &'static str> {
        let _ = n; // unused parameter

        if (params.b - params.a).abs() <= EPSILON {
            Err("|b - a| too small")
        } else if params.a >= params.b {
            Err("lower integration limit must be smaller than upper limit")
        } else {
            Ok(())
        }
    }

    pub fn init(n: usize, diag: &mut [f64], subdiag: &mut [f64], params: &mut FixedParams) -> Result<(), &'static str> {
        // construct the diagonal and subdiagonal elements of Jacobi matrix
        diag[0] = 0.0;
        subdiag[0] = FRAC_1_SQRT_2;
        
        for i in 1..n {
            diag[i] = 0.0;
            subdiag[i] = 0.5;
        }

        params.zemu = PI;
        params.shft = 0.5 * (params.b + params.a);
        params.slp = 0.5 * (params.b - params.a);
        params.al = -0.5;
        params.be = -0.5;

        Ok(())
    }
}

pub static INTEGRATION_FIXED_CHEBYSHEV: ChebyshevType = ChebyshevType;