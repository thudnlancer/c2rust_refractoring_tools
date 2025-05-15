// sys/ldfrexp.rs
// 
// Copyright (C) 2002, Gert Van den Eynde
// Copyright (C) 2007, Brian Gough
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

use std::f64::{self, MAX_EXP, MIN_EXP, INFINITY, NEG_INFINITY};

pub fn gsl_ldexp(x: f64, e: i32) -> f64 {
    if x == 0.0 {
        return x;
    }

    let (y, mut ex) = gsl_frexp(x);
    let mut e2 = e + ex;
    
    if e2 >= MAX_EXP as i32 {
        let y = y * 2f64.powi(e2 - MAX_EXP as i32 + 1);
        e2 = MAX_EXP as i32 - 1;
        y * 2f64.powi(e2)
    } else if e2 <= MIN_EXP as i32 {
        let y = y * 2f64.powi(e2 - MIN_EXP as i32 - 1);
        e2 = MIN_EXP as i32 + 1;
        y * 2f64.powi(e2)
    } else {
        y * 2f64.powi(e2)
    }
}

pub fn gsl_frexp(x: f64) -> (f64, i32) {
    if x == 0.0 {
        (0.0, 0)
    } else if !x.is_finite() {
        (x, 0)
    } else if x.abs() >= 0.5 && x.abs() < 1.0 {
        (x, 0)
    } else {
        let ex = (x.abs().ln() / std::f64::consts::LN_2).ceil();
        let mut ei = ex as i32;

        // Prevent underflow and overflow of 2**(-ei)
        if ei < MIN_EXP as i32 {
            ei = MIN_EXP as i32;
        }

        if ei > -(MIN_EXP as i32) {
            ei = -(MIN_EXP as i32);
        }

        let mut f = x * 2f64.powi(-ei);

        if !f.is_finite() {
            return (f, 0);
        }

        while f.abs() >= 1.0 {
            ei += 1;
            f /= 2.0;
        }

        while f.abs() > 0.0 && f.abs() < 0.5 {
            ei -= 1;
            f *= 2.0;
        }

        (f, ei)
    }
}