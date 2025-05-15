/* movstat/movvar.rs
 *
 * Routines related to a moving window variance
 * 
 * Copyright (C) 2018 Patrick Alken
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

use std::error::Error;

/// Apply moving variance to input vector
///
/// # Arguments
/// * `endtype` - end point handling criteria
/// * `x` - input vector, size n
/// * `y` - output vector, size n (will be modified)
/// * `w` - workspace
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Ok on success, error otherwise
pub fn gsl_movstat_variance(
    endtype: gsl_movstat_end_t,
    x: &gsl_vector,
    y: &mut gsl_vector,
    w: &mut gsl_movstat_workspace,
) -> Result<(), Box<dyn Error>> {
    gsl_movstat_apply_accum(
        endtype,
        x,
        gsl_movstat_accum_variance,
        None,
        y,
        None,
        w,
    )
}

/// Apply moving standard deviation to input vector
///
/// # Arguments
/// * `endtype` - end point handling criteria
/// * `x` - input vector, size n
/// * `y` - output vector, size n (will be modified)
/// * `w` - workspace
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Ok on success, error otherwise
pub fn gsl_movstat_sd(
    endtype: gsl_movstat_end_t,
    x: &gsl_vector,
    y: &mut gsl_vector,
    w: &mut gsl_movstat_workspace,
) -> Result<(), Box<dyn Error>> {
    gsl_movstat_apply_accum(endtype, x, gsl_movstat_accum_sd, None, y, None, w)
}

// Note: The following types and functions would need to be defined elsewhere in the Rust codebase:
// - gsl_movstat_end_t
// - gsl_vector
// - gsl_movstat_workspace
// - gsl_movstat_apply_accum
// - gsl_movstat_accum_variance
// - gsl_movstat_accum_sd