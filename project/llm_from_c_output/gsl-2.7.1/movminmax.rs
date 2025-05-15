/* movstat/movminmax.rs
 *
 * Routines related to a moving window min/max
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

/// Enum representing end point handling criteria
#[derive(Debug, Clone, Copy)]
pub enum MovStatEndType {
    // Define appropriate variants based on gsl_movstat_end_t
    // Example:
    Pad,
    Truncate,
    // etc.
}

/// Workspace for moving statistics calculations
pub struct MovStatWorkspace {
    // Define workspace fields as needed
}

/// Apply minmax filter to input vector
///
/// # Arguments
/// * `endtype` - end point handling criteria
/// * `x` - input vector
/// * `y_min` - output vector of minimum values
/// * `y_max` - output vector of maximum values
/// * `w` - workspace
///
/// # Returns
/// Result indicating success or failure
pub fn movstat_minmax(
    endtype: MovStatEndType,
    x: &[f64],
    y_min: &mut [f64],
    y_max: &mut [f64],
    w: &mut MovStatWorkspace,
) -> Result<(), Box<dyn Error>> {
    movstat_apply_accum(endtype, x, movstat_accum_minmax, None, Some(y_min), Some(y_max), w)
}

/// Apply minimum filter to input vector
///
/// # Arguments
/// * `endtype` - end point handling criteria
/// * `x` - input vector
/// * `y` - output vector of minimum values
/// * `w` - workspace
///
/// # Returns
/// Result indicating success or failure
pub fn movstat_min(
    endtype: MovStatEndType,
    x: &[f64],
    y: &mut [f64],
    w: &mut MovStatWorkspace,
) -> Result<(), Box<dyn Error>> {
    movstat_apply_accum(endtype, x, movstat_accum_min, None, Some(y), None, w)
}

/// Apply maximum filter to input vector
///
/// # Arguments
/// * `endtype` - end point handling criteria
/// * `x` - input vector
/// * `y` - output vector of maximum values
/// * `w` - workspace
///
/// # Returns
/// Result indicating success or failure
pub fn movstat_max(
    endtype: MovStatEndType,
    x: &[f64],
    y: &mut [f64],
    w: &mut MovStatWorkspace,
) -> Result<(), Box<dyn Error>> {
    movstat_apply_accum(endtype, x, movstat_accum_max, None, Some(y), None, w)
}

// Helper functions that would be defined elsewhere in the module
fn movstat_apply_accum(
    endtype: MovStatEndType,
    x: &[f64],
    accum_func: fn(&[f64]) -> (f64, f64),
    _: Option<()>,
    y_min: Option<&mut [f64]>,
    y_max: Option<&mut [f64]>,
    w: &mut MovStatWorkspace,
) -> Result<(), Box<dyn Error>> {
    // Implementation would go here
    Ok(())
}

fn movstat_accum_minmax(window: &[f64]) -> (f64, f64) {
    let min = window.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = window.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    (min, max)
}

fn movstat_accum_min(window: &[f64]) -> f64 {
    window.iter().fold(f64::INFINITY, |a, &b| a.min(b))
}

fn movstat_accum_max(window: &[f64]) -> f64 {
    window.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
}