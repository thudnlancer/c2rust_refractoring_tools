/* roots/fdfsolver.rs
 * 
 * Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Reid Priedhorsky, Brian Gough
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
pub enum RootError {
    NoMemory,
}

impl fmt::Display for RootError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RootError::NoMemory => write!(f, "failed to allocate space"),
        }
    }
}

impl Error for RootError {}

pub struct GslFunctionFdf {
    // Placeholder for GSL function with derivatives
}

pub struct GslRootFdfsolverType {
    pub size: usize,
    pub name: &'static str,
    pub set: fn(&mut [u8], &GslFunctionFdf, &mut f64) -> Result<(), RootError>,
    pub iterate: fn(&mut [u8], &GslFunctionFdf, &mut f64) -> Result<(), RootError>,
}

pub struct GslRootFdfsolver {
    pub type_: &'static GslRootFdfsolverType,
    pub state: Vec<u8>,
    pub fdf: Option<GslFunctionFdf>,
    pub root: f64,
}

impl GslRootFdfsolver {
    pub fn new(solver_type: &'static GslRootFdfsolverType) -> Result<Self, RootError> {
        let state = vec![0; solver_type.size];
        
        Ok(Self {
            type_: solver_type,
            state,
            fdf: None,
            root: 0.0,
        })
    }

    pub fn set(&mut self, f: GslFunctionFdf, root: f64) -> Result<(), RootError> {
        self.fdf = Some(f);
        self.root = root;
        (self.type_.set)(&mut self.state, self.fdf.as_ref().unwrap(), &mut self.root)
    }

    pub fn iterate(&mut self) -> Result<(), RootError> {
        (self.type_.iterate)(&mut self.state, self.fdf.as_ref().unwrap(), &mut self.root)
    }

    pub fn name(&self) -> &'static str {
        self.type_.name
    }

    pub fn root(&self) -> f64 {
        self.root
    }
}