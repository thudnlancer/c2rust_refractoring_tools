/* multifit_nlinear/scaling.rs
 * 
 * Copyright (C) 2015, 2016 Patrick Alken
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
 * This module handles the updating of the scaling matrix D_k in the
 * trust region subproblem:
 *
 * min m_k (dx), || D_k dx || <= Delta_k
 *
 * where m_k(dx) is a model which approximates the cost function
 * F(x_k + dx) near the current iteration point x_k
 *
 * D_k can be updated according to several different strategies.
 */

use ndarray::{Array1, Array2, ArrayView1};
use std::f64;

pub struct NLinearScale {
    pub name: &'static str,
    pub init_diag: fn(&Array2<f64>, &mut Array1<f64>) -> Result<(), &'static str>,
    pub update_diag: fn(&Array2<f64>, &mut Array1<f64>) -> Result<(), &'static str>,
}

/* Levenberg scaling, D = I */
fn init_diag_levenberg(_j: &Array2<f64>, diag: &mut Array1<f64>) -> Result<(), &'static str> {
    diag.fill(1.0);
    Ok(())
}

fn update_diag_levenberg(_j: &Array2<f64>, _diag: &mut Array1<f64>) -> Result<(), &'static str> {
    /* nothing to do */
    Ok(())
}

/* initialize diagonal scaling matrix D according to Marquardt method */
fn init_diag_marquardt(j: &Array2<f64>, diag: &mut Array1<f64>) -> Result<(), &'static str> {
    update_diag_marquardt(j, diag)
}

/* update diagonal scaling matrix D according to Marquardt method */
fn update_diag_marquardt(j: &Array2<f64>, diag: &mut Array1<f64>) -> Result<(), &'static str> {
    let p = j.ncols();
    
    for j_col in 0..p {
        let column = j.column(j_col);
        let norm = column.dot(&column).sqrt();
        
        let norm = if norm == 0.0 { 1.0 } else { norm };
        diag[j_col] = norm;
    }

    Ok(())
}

/* initialize diagonal scaling matrix D according to Eq 6.3 of More, 1978 */
fn init_diag_more(j: &Array2<f64>, diag: &mut Array1<f64>) -> Result<(), &'static str> {
    diag.fill(0.0);
    update_diag_more(j, diag)
}

/* update diagonal scaling matrix D according to Eq. 6.3 of More, 1978 */
fn update_diag_more(j: &Array2<f64>, diag: &mut Array1<f64>) -> Result<(), &'static str> {
    let p = j.ncols();
    
    for j_col in 0..p {
        let column = j.column(j_col);
        let norm = column.dot(&column).sqrt();
        
        let norm = if norm == 0.0 { 1.0 } else { norm };
        diag[j_col] = f64::max(diag[j_col], norm);
    }

    Ok(())
}

pub const LEVENBERG_SCALE: NLinearScale = NLinearScale {
    name: "levenberg",
    init_diag: init_diag_levenberg,
    update_diag: update_diag_levenberg,
};

pub const MARQUARDT_SCALE: NLinearScale = NLinearScale {
    name: "marquardt",
    init_diag: init_diag_marquardt,
    update_diag: update_diag_marquardt,
};

pub const MORE_SCALE: NLinearScale = NLinearScale {
    name: "more",
    init_diag: init_diag_more,
    update_diag: update_diag_more,
};