/* min/convergence.rs
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

#[derive(Debug, PartialEq)]
pub enum GslError {
    BadTolerance(&'static str),
    InvalidInput(&'static str),
}

pub const GSL_SUCCESS: Result<(), GslError> = Ok(());
pub const GSL_CONTINUE: Result<(), GslError> = Err(GslError::InvalidInput("continue"));

pub fn gsl_min_test_interval(
    x_lower: f64,
    x_upper: f64,
    epsabs: f64,
    epsrel: f64,
) -> Result<(), GslError> {
    let lower = x_lower;
    let upper = x_upper;

    let abs_lower = lower.abs();
    let abs_upper = upper.abs();

    let min_abs;
    let tolerance;

    if epsrel < 0.0 {
        return Err(GslError::BadTolerance("relative tolerance is negative"));
    }

    if epsabs < 0.0 {
        return Err(GslError::BadTolerance("absolute tolerance is negative"));
    }

    if lower > upper {
        return Err(GslError::InvalidInput("lower bound larger than upper_bound"));
    }

    if (lower > 0.0 && upper > 0.0) || (lower < 0.0 && upper < 0.0) {
        min_abs = abs_lower.min(abs_upper);
    } else {
        min_abs = 0.0;
    }

    tolerance = epsabs + epsrel * min_abs;

    if (upper - lower).abs() < tolerance {
        GSL_SUCCESS
    } else {
        GSL_CONTINUE
    }
}