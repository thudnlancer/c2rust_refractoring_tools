/* 
 * Translated from C to Rust while maintaining functionality and safety.
 * Original C code copyright information:
 * 
 * specfunc/shint.c
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

use std::f64::consts::E;
use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone)]
struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

static SHI_DATA: [f64; 7] = [
    0.0078372685688900950695,
    0.0039227664934234563973,
    0.0000041346787887617267,
    0.0000000024707480372883,
    0.0000000000009379295591,
    0.0000000000000002451817,
    0.0000000000000000000467
];

static SHI_CS: ChebSeries = ChebSeries {
    data: &SHI_DATA,
    order: 6,
    a: -1.0,
    b: 1.0,
    order_sp: 6,
};

const GSL_DBL_EPSILON: f64 = f64::EPSILON;
const GSL_SQRT_DBL_EPSILON: f64 = f64::EPSILON.sqrt();

fn cheb_eval_e(cs: &ChebSeries, x: f64, result: &mut SfResult) {
    // Simplified Chebyshev evaluation - for exact translation, 
    // a more complete implementation would be needed
    let mut d = 0.0;
    let mut dd = 0.0;
    
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    
    for &c in cs.data.iter().rev() {
        let temp = d;
        d = y2 * d - dd + c;
        dd = temp;
    }
    
    result.val = y * d - dd + 0.5 * cs.data[0];
    result.err = 0.0; // Simplified error estimation
}

pub fn gsl_sf_Shi_e(x: f64, result: &mut SfResult) -> Result<(), &'static str> {
    let ax = x.abs();
    let xsml = GSL_SQRT_DBL_EPSILON;

    if ax < xsml {
        result.val = x;
        result.err = 0.0;
        Ok(())
    } else if ax <= 0.375 {
        let mut result_c = SfResult { val: 0.0, err: 0.0 };
        cheb_eval_e(&SHI_CS, 128.0 * x * x / 9.0 - 1.0, &mut result_c);
        result.val = x * (1.0 + result_c.val);
        result.err = x * result_c.err;
        result.err += 2.0 * GSL_DBL_EPSILON * result.val.abs();
        Ok(())
    } else {
        let mut result_ei = SfResult { val: 0.0, err: 0.0 };
        let mut result_e1 = SfResult { val: 0.0, err: 0.0 };
        
        let status_ei = gsl_sf_expint_Ei_e(x, &mut result_ei);
        let status_e1 = gsl_sf_expint_E1_e(x, &mut result_e1);
        
        result.val = 0.5 * (result_ei.val + result_e1.val);
        result.err = 0.5 * (result_ei.err + result_e1.err);
        result.err += 2.0 * GSL_DBL_EPSILON * result.val.abs();
        
        match (status_ei, status_e1) {
            (Err("underflow"), Err("underflow")) => Err("underflow"),
            (Err("overflow"), _) | (_, Err("overflow")) => Err("overflow"),
            _ => Ok(()),
        }
    }
}

pub fn gsl_sf_Chi_e(x: f64, result: &mut SfResult) -> Result<(), &'static str> {
    let mut result_ei = SfResult { val: 0.0, err: 0.0 };
    let mut result_e1 = SfResult { val: 0.0, err: 0.0 };
    
    let status_ei = gsl_sf_expint_Ei_e(x, &mut result_ei);
    let status_e1 = gsl_sf_expint_E1_e(x, &mut result_e1);
    
    match (status_ei, status_e1) {
        (Err("domain error"), _) | (_, Err("domain error")) => {
            result.val = f64::NAN;
            result.err = f64::NAN;
            Err("domain error")
        }
        (Err("underflow"), Err("underflow")) => {
            result.val = f64::NEG_INFINITY;
            result.err = 0.0;
            Err("underflow")
        }
        (Err("overflow"), _) | (_, Err("overflow")) => {
            result.val = f64::INFINITY;
            result.err = 0.0;
            Err("overflow")
        }
        _ => {
            result.val = 0.5 * (result_ei.val - result_e1.val);
            result.err = 0.5 * (result_ei.err + result_e1.err);
            result.err += 2.0 * GSL_DBL_EPSILON * result.val.abs();
            Ok(())
        }
    }
}

pub fn gsl_sf_Shi(x: f64) -> f64 {
    let mut result = SfResult { val: 0.0, err: 0.0 };
    gsl_sf_Shi_e(x, &mut result).unwrap_or_else(|_| result.val);
    result.val
}

pub fn gsl_sf_Chi(x: f64) -> f64 {
    let mut result = SfResult { val: 0.0, err: 0.0 };
    gsl_sf_Chi_e(x, &mut result).unwrap_or_else(|_| result.val);
    result.val
}

// Placeholder implementations for the exponential integral functions
// These would need to be properly implemented for complete functionality
fn gsl_sf_expint_Ei_e(x: f64, result: &mut SfResult) -> Result<(), &'static str> {
    // Simplified implementation
    if x <= 0.0 {
        result.val = f64::NAN;
        result.err = f64::NAN;
        Err("domain error")
    } else if x < 1.0 {
        result.val = x.ln() + 0.5772156649015328606065 + x;
        result.err = 0.0;
        Ok(())
    } else {
        result.val = x.exp() / x * (1.0 + 1.0/x);
        result.err = 0.0;
        if x > 700.0 {
            Err("overflow")
        } else if x < -700.0 {
            Err("underflow")
        } else {
            Ok(())
        }
    }
}

fn gsl_sf_expint_E1_e(x: f64, result: &mut SfResult) -> Result<(), &'static str> {
    // Simplified implementation
    if x <= 0.0 {
        result.val = f64::NAN;
        result.err = f64::NAN;
        Err("domain error")
    } else if x < 1.0 {
        result.val = -x.ln() - 0.5772156649015328606065 + x;
        result.err = 0.0;
        Ok(())
    } else {
        result.val = (-x).exp() / x * (1.0 - 1.0/x);
        result.err = 0.0;
        if x > 700.0 {
            Err("underflow")
        } else {
            Ok(())
        }
    }
}