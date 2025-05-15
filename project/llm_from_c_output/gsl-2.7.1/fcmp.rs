// sys/gsl_compare.rs
// 
// Copyright (C) 2002 Gert Van den Eynde
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
// 
// Based on fcmp 1.2.2 Copyright (c) 1998-2000 Theodore C. Belding
// University of Michigan Center for the Study of Complex Systems
// Ted.Belding@umich.edu

pub fn gsl_fcmp(x1: f64, x2: f64, epsilon: f64) -> i32 {
    let exponent;
    let delta;
    let difference;

    // Find exponent of largest absolute value
    {
        let max = if x1.abs() > x2.abs() { x1 } else { x2 };
        exponent = max.abs().log2().floor() as i32 + 1;
    }

    // Form a neighborhood of size 2 * delta
    delta = epsilon * 2f64.powi(exponent);
    difference = x1 - x2;

    if difference > delta {       // x1 > x2
        1
    } else if difference < -delta { // x1 < x2
        -1
    } else {                      // -delta <= difference <= delta
        0                          // x1 ~=~ x2
    }
}