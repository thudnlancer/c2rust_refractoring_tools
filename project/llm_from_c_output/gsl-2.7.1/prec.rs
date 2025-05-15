//! sys/prec.rs
//!
//! Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Gerard Jungman, Brian Gough
//! 
//! This program is free software; you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation; either version 3 of the License, or (at
//! your option) any later version.
//! 
//! This program is distributed in the hope that it will be useful, but
//! WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
//! General Public License for more details.
//! 
//! You should have received a copy of the GNU General Public License
//! along with this program; if not, write to the Free Software
//! Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

//! Author:  G. Jungman

use std::f64;

pub const _GSL_PREC_T_NUM: usize = 3;

pub const GSL_DBL_EPSILON: f64 = f64::EPSILON;
pub const GSL_FLT_EPSILON: f64 = f32::EPSILON as f64;
pub const GSL_SFLT_EPSILON: f64 = GSL_FLT_EPSILON * 0.5;

pub const GSL_SQRT_DBL_EPSILON: f64 = GSL_DBL_EPSILON.sqrt();
pub const GSL_SQRT_FLT_EPSILON: f64 = GSL_FLT_EPSILON.sqrt();
pub const GSL_SQRT_SFLT_EPSILON: f64 = GSL_SFLT_EPSILON.sqrt();

pub const GSL_ROOT3_DBL_EPSILON: f64 = GSL_DBL_EPSILON.cbrt();
pub const GSL_ROOT3_FLT_EPSILON: f64 = GSL_FLT_EPSILON.cbrt();
pub const GSL_ROOT3_SFLT_EPSILON: f64 = GSL_SFLT_EPSILON.cbrt();

pub const GSL_ROOT4_DBL_EPSILON: f64 = GSL_DBL_EPSILON.powf(0.25);
pub const GSL_ROOT4_FLT_EPSILON: f64 = GSL_FLT_EPSILON.powf(0.25);
pub const GSL_ROOT4_SFLT_EPSILON: f64 = GSL_SFLT_EPSILON.powf(0.25);

pub const GSL_ROOT5_DBL_EPSILON: f64 = GSL_DBL_EPSILON.powf(0.2);
pub const GSL_ROOT5_FLT_EPSILON: f64 = GSL_FLT_EPSILON.powf(0.2);
pub const GSL_ROOT5_SFLT_EPSILON: f64 = GSL_SFLT_EPSILON.powf(0.2);

pub const GSL_ROOT6_DBL_EPSILON: f64 = GSL_DBL_EPSILON.powf(1.0/6.0);
pub const GSL_ROOT6_FLT_EPSILON: f64 = GSL_FLT_EPSILON.powf(1.0/6.0);
pub const GSL_ROOT6_SFLT_EPSILON: f64 = GSL_SFLT_EPSILON.powf(1.0/6.0);

pub const gsl_prec_eps: [f64; _GSL_PREC_T_NUM] = [
    GSL_DBL_EPSILON,
    GSL_FLT_EPSILON,
    GSL_SFLT_EPSILON
];

pub const gsl_prec_sqrt_eps: [f64; _GSL_PREC_T_NUM] = [
    GSL_SQRT_DBL_EPSILON,
    GSL_SQRT_FLT_EPSILON,
    GSL_SQRT_SFLT_EPSILON
];

pub const gsl_prec_root3_eps: [f64; _GSL_PREC_T_NUM] = [
    GSL_ROOT3_DBL_EPSILON,
    GSL_ROOT3_FLT_EPSILON,
    GSL_ROOT3_SFLT_EPSILON
];

pub const gsl_prec_root4_eps: [f64; _GSL_PREC_T_NUM] = [
    GSL_ROOT4_DBL_EPSILON,
    GSL_ROOT4_FLT_EPSILON,
    GSL_ROOT4_SFLT_EPSILON
];

pub const gsl_prec_root5_eps: [f64; _GSL_PREC_T_NUM] = [
    GSL_ROOT5_DBL_EPSILON,
    GSL_ROOT5_FLT_EPSILON,
    GSL_ROOT5_SFLT_EPSILON
];

pub const gsl_prec_root6_eps: [f64; _GSL_PREC_T_NUM] = [
    GSL_ROOT6_DBL_EPSILON,
    GSL_ROOT6_FLT_EPSILON,
    GSL_ROOT6_SFLT_EPSILON
];