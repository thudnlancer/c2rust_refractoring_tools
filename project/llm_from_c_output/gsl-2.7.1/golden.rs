/* min/golden.rs
 * 
 * Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Brian Gough
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

/* goldensection.rs -- goldensection minimum finding algorithm */

use std::f64;

pub struct GoldensectionState {
    dummy: f64,
}

impl GoldensectionState {
    pub fn new() -> Self {
        GoldensectionState { dummy: 0.0 }
    }
}

pub struct GslFunction {
    pub function: Box<dyn Fn(f64) -> f64>,
}

pub enum GslError {
    Success,
    Failure,
}

pub struct GslMinFminimizerType {
    pub name: &'static str,
    pub size: usize,
    pub init: fn(&mut GoldensectionState, &GslFunction, f64, f64, f64, f64, f64, f64) -> GslError,
    pub iterate: fn(&mut GoldensectionState, &GslFunction, &mut f64, &mut f64, &mut f64, &mut f64, &mut f64, &mut f64) -> GslError,
}

fn goldensection_init(
    state: &mut GoldensectionState,
    _f: &GslFunction,
    _x_minimum: f64,
    _f_minimum: f64,
    _x_lower: f64,
    _f_lower: f64,
    _x_upper: f64,
    _f_upper: f64,
) -> GslError {
    // no initialization required, prevent warnings about unused variables
    state.dummy = 0.0;
    GslError::Success
}

fn goldensection_iterate(
    _state: &mut GoldensectionState,
    f: &GslFunction,
    x_minimum: &mut f64,
    f_minimum: &mut f64,
    x_lower: &mut f64,
    f_lower: &mut f64,
    x_upper: &mut f64,
    f_upper: &mut f64,
) -> GslError {
    let x_center = *x_minimum;
    let x_left = *x_lower;
    let x_right = *x_upper;

    let f_min = *f_minimum;

    const GOLDEN: f64 = 0.3819660; // golden = (3 - sqrt(5))/2
    
    let w_lower = x_center - x_left;
    let w_upper = x_right - x_center;

    let x_new = x_center + GOLDEN * if w_upper > w_lower { w_upper } else { -w_lower };
    let f_new = (f.function)(x_new);

    if f_new < f_min {
        *x_minimum = x_new;
        *f_minimum = f_new;
        GslError::Success
    } else if x_new < x_center && f_new > f_min {
        *x_lower = x_new;
        *f_lower = f_new;
        GslError::Success
    } else if x_new > x_center && f_new > f_min {
        *x_upper = x_new;
        *f_upper = f_new;
        GslError::Success
    } else {
        GslError::Failure
    }
}

pub static GOLDENSECTION_TYPE: GslMinFminimizerType = GslMinFminimizerType {
    name: "goldensection",
    size: std::mem::size_of::<GoldensectionState>(),
    init: goldensection_init,
    iterate: goldensection_iterate,
};

pub const GSL_MIN_FMINIMIZER_GOLDENSECTION: &GslMinFminimizerType = &GOLDENSECTION_TYPE;