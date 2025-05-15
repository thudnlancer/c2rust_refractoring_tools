// sys/expm1.rs
//
// Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Gerard Jungman, Brian Gough
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

pub const GSL_DBL_EPSILON: f64 = std::f64::EPSILON;
pub const M_LN2: f64 = std::f64::consts::LN_2;

pub fn gsl_expm1(x: f64) -> f64 {
    // FIXME: this should be improved

    if x.abs() < M_LN2 {
        // Compute the taylor series S = x + (1/2!) x^2 + (1/3!) x^3 + ...

        let mut i = 1.0;
        let mut sum = x;
        let mut term = x / 1.0;

        loop {
            i += 1.0;
            term *= x / i;
            sum += term;
            if term.abs() <= sum.abs() * GSL_DBL_EPSILON {
                break;
            }
        }

        sum
    } else {
        x.exp() - 1.0
    }
}