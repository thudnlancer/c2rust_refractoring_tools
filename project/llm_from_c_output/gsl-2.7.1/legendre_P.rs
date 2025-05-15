/* specfunc/legendre_P.rs
 * 
 * Copyright (C) 2009-2013 Patrick Alken
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

/*
 * The routines in this module compute associated Legendre functions
 * (ALFs) up to order and degree 2700, using the method described
 * in
 *
 * [1] S. A. Holmes and W. E. Featherstone, A unified approach
 *     to the Clenshaw summation and the recursive computation of very
 *     high degree and order normalised associated Legendre functions,
 *     Journal of Geodesy, 76, pg. 279-299, 2002.
 *
 * Further information on ALFs can be found in
 *
 * [2] Abramowitz and Stegun, Handbook of Mathematical Functions,
 *     Chapter 8, 1972.
 */

/// Number of P_{lm} functions for a given lmax
pub fn gsl_sf_legendre_nlm(lmax: usize) -> usize {
    ((lmax + 1) * (lmax + 2)) / 2
}

/// Returns the minimum result_array size needed for a given lmax
pub fn gsl_sf_legendre_array_n(lmax: usize) -> usize {
    let nlm = gsl_sf_legendre_nlm(lmax);
    let nsqrt = 2 * lmax + 2; // extra room to precompute sqrt factors
    
    nlm + nsqrt
}

/*********************************************************
 *                 INTERNAL ROUTINES                     *
 *********************************************************/

/// Precompute square root factors needed for Legendre recurrence.
/// On output, array[i] = sqrt(i)
fn legendre_sqrts(lmax: usize, array: &mut [f64]) {
    for l in 0..=(2 * lmax + 1) {
        array[l] = (l as f64).sqrt();
    }
}

// Note: The original C code includes several legendre_source.c files with different
// macro definitions. In Rust, we would typically implement these as separate modules
// or functions with different trait implementations. Since the content of these
// files isn't provided in the original code, they're not translated here.