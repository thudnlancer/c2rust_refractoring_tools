// rng1.rs (pseudo-random number generator)

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2003 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct RngError {
    details: String,
}

impl RngError {
    fn new(msg: &str) -> RngError {
        RngError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for RngError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for RngError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub struct RNG {
    // Implementation details of the RNG would go here
}

impl RNG {
    pub fn next_rand(&mut self) -> i32 {
        // Implementation of next_rand would go here
        unimplemented!()
    }
}

/***********************************************************************
*  NAME
*
*  rng_unif_01 - obtain pseudo-random number in the range [0, 1]
*
*  SYNOPSIS
*
*  fn rng_unif_01(rand: &mut RNG) -> Result<f64, RngError>
*
*  RETURNS
*
*  The routine rng_unif_01 returns a next pseudo-random number which is
*  uniformly distributed in the range [0, 1]. */

pub fn rng_unif_01(rand: &mut RNG) -> Result<f64, RngError> {
    let x = rand.next_rand() as f64 / 2147483647.0;
    if !(0.0 <= x && x <= 1.0) {
        return Err(RngError::new("Generated number out of [0, 1] range"));
    }
    Ok(x)
}

/***********************************************************************
*  NAME
*
*  rng_uniform - obtain pseudo-random number in the range [a, b]
*
*  SYNOPSIS
*
*  fn rng_uniform(rand: &mut RNG, a: f64, b: f64) -> Result<f64, RngError>
*
*  RETURNS
*
*  The routine rng_uniform returns a next pseudo-random number which is
*  uniformly distributed in the range [a, b]. */

pub fn rng_uniform(rand: &mut RNG, a: f64, b: f64) -> Result<f64, RngError> {
    if a >= b {
        return Err(RngError::new("Invalid range: a must be less than b"));
    }
    let x = rng_unif_01(rand)?;
    let result = a * (1.0 - x) + b * x;
    if !(a <= result && result <= b) {
        return Err(RngError::new("Generated number out of [a, b] range"));
    }
    Ok(result)
}