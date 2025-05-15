/* ode-initval/odeiv.rs

 * Copyright (C) 1996, 1997, 1998, 1999, 2000 Gerard Jungman
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

/* Author:  G. Jungman
 */

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum OdeError {
    NoMemory,
    NullPointer,
    Other(String),
}

impl fmt::Display for OdeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OdeError::NoMemory => write!(f, "failed to allocate space"),
            OdeError::NullPointer => write!(f, "null pointer encountered"),
            OdeError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for OdeError {}

pub type OdeResult<T> = Result<T, OdeError>;

pub struct OdeStep<'a> {
    step_type: &'a OdeStepType,
    dimension: usize,
    state: Box<dyn OdeState>,
}

pub trait OdeState: std::fmt::Debug {}
pub trait OdeSystem {}

pub struct OdeStepType {
    pub name: &'static str,
    pub alloc: fn(usize) -> OdeResult<Box<dyn OdeState>>,
    pub order: fn(&dyn OdeState) -> u32,
    pub apply: fn(&mut dyn OdeState, usize, f64, f64, &mut [f64], &mut [f64], &[f64], &mut [f64], &dyn OdeSystem) -> OdeResult<()>,
    pub reset: fn(&mut dyn OdeState, usize) -> OdeResult<()>,
    pub free: fn(Box<dyn OdeState>),
    pub set_driver: fn(&mut dyn OdeState, &dyn OdeDriver),
}

pub trait OdeDriver {}

impl<'a> OdeStep<'a> {
    pub fn new(step_type: &'a OdeStepType, dim: usize) -> OdeResult<Self> {
        let state = (step_type.alloc)(dim)?;
        
        Ok(Self {
            step_type,
            dimension: dim,
            state,
        })
    }

    pub fn name(&self) -> &str {
        self.step_type.name
    }

    pub fn order(&self) -> u32 {
        (self.step_type.order)(&*self.state)
    }

    pub fn apply(
        &mut self,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: &[f64],
        dydt_out: &mut [f64],
        dydt: &dyn OdeSystem,
    ) -> OdeResult<()> {
        (self.step_type.apply)(&mut *self.state, self.dimension, t, h, y, yerr, dydt_in, dydt_out, dydt)
    }

    pub fn reset(&mut self) -> OdeResult<()> {
        (self.step_type.reset)(&mut *self.state, self.dimension)
    }

    pub fn set_driver(&mut self, driver: &dyn OdeDriver) -> OdeResult<()> {
        (self.step_type.set_driver)(&mut *self.state, driver);
        Ok(())
    }
}

impl<'a> Drop for OdeStep<'a> {
    fn drop(&mut self) {
        (self.step_type.free)(std::mem::take(&mut self.state));
    }
}