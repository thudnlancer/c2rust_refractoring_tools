/* ode-initval/control.rs
 * 
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

/* Author:  G. Jungman */

use std::error::Error;
use std::fmt;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Odeiv2Error {
    message: String,
    code: i32,
}

impl fmt::Display for Odeiv2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (code: {})", self.message, self.code)
    }
}

impl Error for Odeiv2Error {}

pub const GSL_SUCCESS: i32 = 0;
pub const GSL_ENOMEM: i32 = -1;
pub const GSL_EFAULT: i32 = -2;

pub trait ControlType {
    fn alloc(&self) -> Option<Box<dyn std::any::Any>>;
    fn init(&self, state: &mut dyn std::any::Any, eps_abs: f64, eps_rel: f64, a_y: f64, a_dydt: f64) -> i32;
    fn free(&self, state: Box<dyn std::any::Any>);
    fn name(&self) -> &'static str;
    fn hadjust(
        &self,
        state: &mut dyn std::any::Any,
        dimension: usize,
        order: usize,
        y: &[f64],
        yerr: &[f64],
        dydt: &[f64],
        h: &mut f64,
    ) -> i32;
    fn errlevel(
        &self,
        state: &mut dyn std::any::Any,
        y: f64,
        dydt: f64,
        h: f64,
        ind: usize,
        errlev: &mut f64,
    ) -> i32;
    fn set_driver(&self, state: &mut dyn std::any::Any, driver: &dyn std::any::Any);
}

pub struct Odeiv2Control {
    pub type_: &'static dyn ControlType,
    pub state: Box<dyn std::any::Any>,
}

impl Odeiv2Control {
    pub fn new(type_: &'static dyn ControlType) -> Result<Self, Odeiv2Error> {
        let state = type_.alloc().ok_or_else(|| Odeiv2Error {
            message: "failed to allocate space for control state".to_string(),
            code: GSL_ENOMEM,
        })?;

        Ok(Self { type_, state })
    }

    pub fn init(&mut self, eps_abs: f64, eps_rel: f64, a_y: f64, a_dydt: f64) -> i32 {
        self.type_.init(&mut *self.state, eps_abs, eps_rel, a_y, a_dydt)
    }

    pub fn name(&self) -> &'static str {
        self.type_.name()
    }

    pub fn hadjust(
        &mut self,
        step: &Odeiv2Step,
        y: &[f64],
        yerr: &[f64],
        dydt: &[f64],
        h: &mut f64,
    ) -> i32 {
        self.type_.hadjust(
            &mut *self.state,
            step.dimension,
            step.type_.order(&step.state),
            y,
            yerr,
            dydt,
            h,
        )
    }

    pub fn errlevel(
        &mut self,
        y: f64,
        dydt: f64,
        h: f64,
        ind: usize,
        errlev: &mut f64,
    ) -> i32 {
        self.type_.errlevel(&mut *self.state, y, dydt, h, ind, errlev)
    }

    pub fn set_driver(&mut self, driver: &dyn std::any::Any) -> Result<(), Odeiv2Error> {
        if let Some(drv) = NonNull::new(driver as *const _ as *mut _) {
            self.type_.set_driver(&mut *self.state, unsafe { drv.as_ref() });
            Ok(())
        } else {
            Err(Odeiv2Error {
                message: "driver pointer is null".to_string(),
                code: GSL_EFAULT,
            })
        }
    }
}

impl Drop for Odeiv2Control {
    fn drop(&mut self) {
        self.type_.free(std::mem::replace(&mut self.state, Box::new(())));
    }
}

pub struct Odeiv2Step {
    pub type_: &'static dyn StepType,
    pub state: Box<dyn std::any::Any>,
    pub dimension: usize,
}

pub trait StepType {
    fn order(&self, state: &dyn std::any::Any) -> usize;
    // Other required methods...
}