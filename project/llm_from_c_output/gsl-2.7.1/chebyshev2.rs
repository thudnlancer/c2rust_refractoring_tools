/* integration/chebyshev2.rs
 *
 * Copyright (C) 2017 Konrad Griessinger, Patrick Alken
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or (at
 * your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
 */

/*
 * The code in this module is based on IQPACK, specifically the LGPL
 * implementation found in HERMITE_RULE:
 * https://people.sc.fsu.edu/~jburkardt/c_src/hermite_rule/hermite_rule.html
 */

use std::f64::consts::PI;
use std::f64::EPSILON as DBL_EPSILON;

#[derive(Debug, Clone)]
pub struct FixedParams {
    pub a: f64,
    pub b: f64,
    pub zemu: f64,
    pub shft: f64,
    pub slp: f64,
    pub al: f64,
    pub be: f64,
}

pub trait FixedType {
    fn check(&self, n: usize, params: &FixedParams) -> Result<(), &'static str>;
    fn init(&self, n: usize, diag: &mut [f64], subdiag: &mut [f64], params: &mut FixedParams) -> Result<(), &'static str>;
}

pub struct Chebyshev2Type;

impl FixedType for Chebyshev2Type {
    fn check(&self, _n: usize, params: &FixedParams) -> Result<(), &'static str> {
        if (params.b - params.a).abs() <= DBL_EPSILON {
            Err("|b - a| too small")
        } else if params.a >= params.b {
            Err("lower integration limit must be smaller than upper limit")
        } else {
            Ok(())
        }
    }

    fn init(&self, n: usize, diag: &mut [f64], subdiag: &mut [f64], params: &mut FixedParams) -> Result<(), &'static str> {
        // construct the diagonal and subdiagonal elements of Jacobi matrix
        for i in 0..n {
            diag[i] = 0.0;
            subdiag[i] = 0.5;
        }

        params.zemu = PI / 2.0;
        params.shft = 0.5 * (params.b + params.a);
        params.slp = 0.5 * (params.b - params.a);
        params.al = 0.5;
        params.be = 0.5;

        Ok(())
    }
}

pub static GSL_INTEGRATION_FIXED_CHEBYSHEV2: Chebyshev2Type = Chebyshev2Type;