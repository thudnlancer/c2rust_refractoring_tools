/* poly/zsolve_init.rs
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

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct PolyComplexWorkspace {
    nc: usize,
    matrix: Vec<f64>,
}

#[derive(Debug)]
pub enum PolyError {
    DomainError,
    MemoryAllocationError,
}

impl fmt::Display for PolyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PolyError::DomainError => write!(f, "matrix size n must be positive integer"),
            PolyError::MemoryAllocationError => write!(f, "failed to allocate space"),
        }
    }
}

impl Error for PolyError {}

pub fn poly_complex_workspace_alloc(n: usize) -> Result<PolyComplexWorkspace, PolyError> {
    if n == 0 {
        return Err(PolyError::DomainError);
    }

    let nc = n - 1;
    let matrix_size = nc * nc;
    let matrix = vec![0.0; matrix_size];

    Ok(PolyComplexWorkspace { nc, matrix })
}

pub fn poly_complex_workspace_free(w: PolyComplexWorkspace) {
    // Rust's ownership system automatically frees memory when w goes out of scope
}