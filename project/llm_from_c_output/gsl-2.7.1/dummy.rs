/* multilarge_nlinear/dummy.rs

 * Copyright (C) 2016 Patrick Alken
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

/* dummy linear solver */

use ndarray::{Array1, Array2};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolverError {
    #[error("operation succeeded")]
    Success,
}

pub type SolverResult<T> = Result<T, SolverError>;

pub struct DummySolver;

impl DummySolver {
    pub fn new() -> Self {
        DummySolver
    }

    pub fn alloc(_n: usize, _p: usize) -> Option<()> {
        None
    }

    pub fn free(&mut self) {}

    pub fn init(&mut self) -> SolverResult<()> {
        Ok(())
    }

    pub fn presolve(&mut self, _mu: f64) -> SolverResult<()> {
        Ok(())
    }

    pub fn solve(&mut self, _g: &Array1<f64>, _x: &mut Array1<f64>) -> SolverResult<()> {
        Ok(())
    }

    pub fn rcond(&mut self, _jtj: &Array2<f64>) -> SolverResult<f64> {
        Ok(0.0)
    }

    pub fn covar(&mut self, _jtj: &Array2<f64>, covar: &mut Array2<f64>) -> SolverResult<()> {
        covar.fill(0.0);
        Ok(())
    }
}

pub struct MultiLargeNLinearSolver {
    name: &'static str,
    alloc: fn(usize, usize) -> Option<()>,
    init: fn() -> SolverResult<()>,
    presolve: fn(f64) -> SolverResult<()>,
    solve: fn(&Array1<f64>, &mut Array1<f64>) -> SolverResult<()>,
    rcond: fn(&Array2<f64>) -> SolverResult<f64>,
    covar: fn(&Array2<f64>, &mut Array2<f64>) -> SolverResult<()>,
    free: fn(),
}

pub static MULTILARGE_NLINEAR_SOLVER_NONE: MultiLargeNLinearSolver = MultiLargeNLinearSolver {
    name: "dummy",
    alloc: DummySolver::alloc,
    init: || DummySolver.init(),
    presolve: |mu| DummySolver.presolve(mu),
    solve: |g, x| DummySolver.solve(g, x),
    rcond: |jtj| DummySolver.rcond(jtj),
    covar: |jtj, covar| DummySolver.covar(jtj, covar),
    free: || {},
};