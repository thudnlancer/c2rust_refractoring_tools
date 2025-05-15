/* itersolve.rs
 * 
 * Copyright (C) 2014 Patrick Alken
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
use std::ptr::NonNull;

#[derive(Debug)]
pub enum SplinalgError {
    MemoryAllocationFailed,
    IterationFailed,
}

impl fmt::Display for SplinalgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SplinalgError::MemoryAllocationFailed => 
                write!(f, "failed to allocate space for itersolve struct"),
            SplinalgError::IterationFailed => 
                write!(f, "iteration failed"),
        }
    }
}

impl Error for SplinalgError {}

pub trait IterSolveType {
    fn alloc(&self, n: usize, m: usize) -> Option<Box<dyn State>>;
    fn name(&self) -> &'static str;
}

pub trait State {
    fn iterate(&mut self, a: &SparseMatrix, b: &Vector, tol: f64, x: &mut Vector) -> Result<(), SplinalgError>;
    fn normr(&self) -> f64;
    fn free(self: Box<Self>);
}

pub struct IterSolve {
    type_: Box<dyn IterSolveType>,
    normr: f64,
    state: Option<Box<dyn State>>,
}

impl IterSolve {
    pub fn new(type_: Box<dyn IterSolveType>, n: usize, m: usize) -> Result<Self, SplinalgError> {
        let state = type_.alloc(n, m).ok_or(SplinalgError::MemoryAllocationFailed)?;
        
        Ok(Self {
            type_,
            normr: 0.0,
            state: Some(state),
        })
    }

    pub fn name(&self) -> &'static str {
        self.type_.name()
    }

    pub fn iterate(&mut self, a: &SparseMatrix, b: &Vector, tol: f64, x: &mut Vector) -> Result<(), SplinalgError> {
        if let Some(state) = &mut self.state {
            state.iterate(a, b, tol, x)?;
            self.normr = state.normr();
            Ok(())
        } else {
            Err(SplinalgError::IterationFailed)
        }
    }

    pub fn normr(&self) -> f64 {
        self.normr
    }
}

impl Drop for IterSolve {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            state.free();
        }
    }
}

// Placeholder types - these would need to be properly defined
pub struct SparseMatrix;
pub struct Vector;