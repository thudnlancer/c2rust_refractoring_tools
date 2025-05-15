/* specfunc/expint3.rs

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

/* Author: G. Jungman */

use std::f64::consts::EPSILON;
use std::f64::consts::LN_2;

const VAL_INFINITY: f64 = 0.892979511569249211;
const GSL_ROOT3_DBL_EPSILON: f64 = EPSILON.powf(1.0/3.0);
const GSL_LOG_DBL_EPSILON: f64 = LN_2 * (EPSILON.log2());

struct ChebSeries {
    data: &'static [f64],
    order: usize,
    a: f64,
    b: f64,
    order_sp: usize,
}

static EXPINT3_DATA: [f64; 24] = [
    1.269198414221126014,
    -0.248846446384140982,
    0.80526220717231041e-01,
    -0.25772733251968330e-01,
    0.7599878873073774e-02,
    -0.2030695581940405e-02,
    0.490834586699330e-03,
    -0.107682239142021e-03,
    0.21551726264290e-04,
    -0.3956705137384e-05,
    0.6699240933896e-06,
    -0.105132180807e-06,
    0.15362580199e-07,
    -0.20990960364e-08,
    0.2692109538e-09,
    -0.325195242e-10,
    0.37114816e-11,
    -0.4013652e-12,
    0.412334e-13,
    -0.40338e-14,
    0.3766e-15,
    -0.336e-16,
    0.29e-17,
    -0.2e-18
];

static EXPINT3_CS: ChebSeries = ChebSeries {
    data: &EXPINT3_DATA,
    order: 23,
    a: -1.0,
    b: 1.0,
    order_sp: 15,
};

static EXPINT3A_DATA: [f64; 23] = [
    1.9270464955068273729,
    -0.349293565204813805e-01,
    0.14503383718983009e-02,
    -0.8925336718327903e-04,
    0.70542392191184e-05,
    -0.6671727454761e-06,
    0.724267589982e-07,
    -0.87825825606e-08,
    0.11672234428e-08,
    -0.1676631281e-09,
    0.257550158e-10,
    -0.41957888e-11,
    0.7201041e-12,
    -0.1294906e-12,
    0.24287e-13,
    -0.47331e-14,
    0.95531e-15,
    -0.1991e-15,
    0.428e-16,
    -0.94e-17,
    0.21e-17,
    -0.5e-18,
    0.1e-18
];

static EXPINT3A_CS: ChebSeries = ChebSeries {
    data: &EXPINT3A_DATA,
    order: 22,
    a: -1.0,
    b: 1.0,
    order_sp: 10,
};

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

pub enum SfError {
    DomainError,
}

fn cheb_eval_e(cs: &ChebSeries, x: f64) -> SfResult {
    let d = 0.0;
    let dd = 0.0;
    
    let y = (2.0 * x - cs.a - cs.b) / (cs.b - cs.a);
    let y2 = 2.0 * y;
    
    let mut result = SfResult { val: 0.0, err: 0.0 };
    
    for j in (1..=cs.order).rev() {
        let temp = d;
        d = y2 * d - dd + cs.data[j];
        dd = temp;
    }
    
    result.val = y * d - dd + 0.5 * cs.data[0];
    result.err = result.val.abs() * EPSILON;
    result
}

pub fn gsl_sf_expint_3_e(x: f64) -> Result<SfResult, SfError> {
    if x < 0.0 {
        Err(SfError::DomainError)
    } else if x < 1.6 * GSL_ROOT3_DBL_EPSILON {
        Ok(SfResult {
            val: x,
            err: 0.0,
        })
    } else if x <= 2.0 {
        let t = x * x * x / 4.0 - 1.0;
        let result_c = cheb_eval_e(&EXPINT3_CS, t);
        Ok(SfResult {
            val: x * result_c.val,
            err: x * result_c.err,
        })
    } else if x < (-GSL_LOG_DBL_EPSILON).powf(1.0 / 3.0) {
        let t = 16.0 / (x * x * x) - 1.0;
        let s = (-x * x * x).exp() / (3.0 * x * x);
        let result_c = cheb_eval_e(&EXPINT3A_CS, t);
        Ok(SfResult {
            val: VAL_INFINITY - result_c.val * s,
            err: VAL_INFINITY * EPSILON + s * result_c.err,
        })
    } else {
        Ok(SfResult {
            val: VAL_INFINITY,
            err: VAL_INFINITY * EPSILON,
        })
    }
}

pub fn gsl_sf_expint_3(x: f64) -> f64 {
    match gsl_sf_expint_3_e(x) {
        Ok(result) => result.val,
        Err(_) => f64::NAN,
    }
}