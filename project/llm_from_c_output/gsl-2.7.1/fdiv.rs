// sys/fdiv.rs
// 
// Copyright (C) 2001, 2007 Brian Gough
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

/// Performs floating-point division of x by y
///
/// # Arguments
/// * `x` - Dividend
/// * `y` - Divisor
///
/// # Returns
/// Result containing the quotient if successful, or an error if division by zero occurs
///
/// # Examples
/// ```
/// let result = gsl_fdiv(10.0, 2.0).unwrap();
/// assert_eq!(result, 5.0);
/// ```
pub fn gsl_fdiv(x: f64, y: f64) -> Result<f64, &'static str> {
    if y == 0.0 {
        Err("division by zero")
    } else {
        Ok(x / y)
    }
}