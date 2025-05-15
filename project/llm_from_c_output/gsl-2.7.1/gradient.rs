// multifit/gradient.rs
// 
// Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Brian Gough
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

use ndarray::{Array2, Array1};
use ndarray_linalg::Scalar;

pub fn multifit_gradient(j: &Array2<f64>, f: &Array1<f64>, g: &mut Array1<f64>) -> Result<(), String> {
    // Perform matrix-vector multiplication: g = J^T * f
    match g.dot_mm(j.t(), f) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Matrix-vector multiplication failed: {}", e)),
    }
}