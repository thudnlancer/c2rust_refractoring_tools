/* specfunc/hyperg_2F0.rs
 * 
 * Copyright (C) 1996, 1997, 1998, 1999, 2000 Gerard Jungman
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

/* Author:  G. Jungman */

use std::f64::consts;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

pub const GSL_SUCCESS: i32 = 0;
pub const GSL_DBL_EPSILON: f64 = f64::EPSILON;

pub fn hyperg_u_e(a: f64, b: f64, x: f64) -> Result<SfResult, i32> {
    // Placeholder for actual hyperg_U implementation
    Err(GSL_SUCCESS)
}

pub fn gsl_sf_hyperg_2f0_e(a: f64, b: f64, x: f64) -> Result<SfResult, i32> {
    if x < 0.0 {
        let pre = (-1.0 / x).powf(a);
        match hyperg_u_e(a, 1.0 + a - b, -1.0 / x) {
            Ok(U) => {
                let val = pre * U.val;
                let err = GSL_DBL_EPSILON * val.abs() + pre * U.err;
                Ok(SfResult { val, err })
            }
            Err(e) => Err(e),
        }
    } else if x == 0.0 {
        Ok(SfResult {
            val: 1.0,
            err: 0.0,
        })
    } else {
        // Use asymptotic series
        Err(GSL_SUCCESS)
    }
}

pub fn gsl_sf_hyperg_2f0(a: f64, b: f64, x: f64) -> f64 {
    match gsl_sf_hyperg_2f0_e(a, b, x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}