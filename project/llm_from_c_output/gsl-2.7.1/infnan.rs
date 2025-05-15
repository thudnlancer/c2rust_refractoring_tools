// sys/infnan.rs
//
// Copyright (C) 2001, 2004, 2007, 2010 Brian Gough
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

use std::f64;

pub fn gsl_nan() -> f64 {
    f64::NAN
}

pub fn gsl_posinf() -> f64 {
    f64::INFINITY
}

pub fn gsl_neginf() -> f64 {
    f64::NEG_INFINITY
}

pub fn gsl_isnan(x: f64) -> bool {
    x.is_nan()
}

pub fn gsl_isinf(x: f64) -> i32 {
    if x.is_infinite() {
        if x > 0.0 {
            1
        } else {
            -1
        }
    } else {
        0
    }
}

pub fn gsl_finite(x: f64) -> bool {
    x.is_finite()
}