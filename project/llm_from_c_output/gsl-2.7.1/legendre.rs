/* 
 * integration/legendre.rs
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

use std::f64::EPSILON;

#[derive(Debug, Clone)]
pub struct IntegrationFixedParams {
    pub a: f64,
    pub b: f64,
    pub zemu: f64,
    pub shft: f64,
    pub slp: f64,
    pub al: f64,
    pub be: f64,
}

pub struct IntegrationFixedType {
    pub check: fn(usize, &IntegrationFixedParams) -> Result<(), &'static str>,
    pub init: fn(usize, &mut [f64], &mut [f64], &mut IntegrationFixedParams) -> Result<(), &'static str>,
}

fn legendre_check(n: usize, params: &IntegrationFixedParams) -> Result<(), &'static str> {
    let _ = n; // unused parameter
    
    if (params.b - params.a).abs() <= EPSILON {
        Err("|b - a| too small")
    } else {
        Ok(())
    }
}

fn legendre_init(
    n: usize,
    diag: &mut [f64],
    subdiag: &mut [f64],
    params: &mut IntegrationFixedParams,
) -> Result<(), &'static str> {
    // construct the diagonal and subdiagonal elements of Jacobi matrix
    for i in 1..=n {
        diag[i - 1] = 0.0;
        subdiag[i - 1] = (i as f64) / (4.0 * (i as f64).powi(2) - 1.0).sqrt();
    }

    params.zemu = 2.0;
    params.shft = 0.5 * (params.b + params.a);
    params.slp = 0.5 * (params.b - params.a);
    params.al = 0.0;
    params.be = 0.0;

    Ok(())
}

pub static LEGENDRE_TYPE: IntegrationFixedType = IntegrationFixedType {
    check: legendre_check,
    init: legendre_init,
};