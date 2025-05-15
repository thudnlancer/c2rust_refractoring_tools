/*!
 * Rust translation of GSL's poly/zsolve.c
 * 
 * Original C code:
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

use num_complex::Complex64;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PolyError {
    #[error("number of terms must be a positive integer")]
    ZeroTerms,
    #[error("cannot solve for only one term")]
    SingleTerm,
    #[error("leading term of polynomial must be non-zero")]
    ZeroLeadingTerm,
    #[error("size of workspace does not match polynomial")]
    WorkspaceSizeMismatch,
    #[error("root solving qr method failed to converge")]
    QRMethodFailed,
}

pub struct PolyComplexWorkspace {
    nc: usize,
    matrix: Vec<f64>,
}

impl PolyComplexWorkspace {
    pub fn new(n: usize) -> Self {
        let size = n * n;
        Self {
            nc: n,
            matrix: vec![0.0; size],
        }
    }
}

pub fn poly_complex_solve(
    a: &[f64],
    workspace: &mut PolyComplexWorkspace,
    z: &mut [Complex64],
) -> Result<(), PolyError> {
    let n = a.len();

    if n == 0 {
        return Err(PolyError::ZeroTerms);
    }

    if n == 1 {
        return Err(PolyError::SingleTerm);
    }

    if a[n - 1] == 0.0 {
        return Err(PolyError::ZeroLeadingTerm);
    }

    if workspace.nc != n - 1 {
        return Err(PolyError::WorkspaceSizeMismatch);
    }

    set_companion_matrix(a, n - 1, &mut workspace.matrix);
    balance_companion_matrix(&mut workspace.matrix, n - 1);

    if !qr_companion(&mut workspace.matrix, n - 1, z) {
        return Err(PolyError::QRMethodFailed);
    }

    Ok(())
}

// Note: The following functions would need to be implemented separately
// with Rust equivalents of the original C implementations:
// - set_companion_matrix
// - balance_companion_matrix
// - qr_companion
// These implementations would need to be adapted to Rust's safety guarantees
// and would typically use slices or vectors instead of raw pointers.