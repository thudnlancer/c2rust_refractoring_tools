/* histogram/add2d.rs
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
pub enum HistogramError {
    DomainError,
    SanityError(String),
}

impl fmt::Display for HistogramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HistogramError::DomainError => write!(f, "input value is out of histogram range"),
            HistogramError::SanityError(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for HistogramError {}

pub struct Histogram2D {
    nx: usize,
    ny: usize,
    xrange: Vec<f64>,
    yrange: Vec<f64>,
    bin: Vec<f64>,
}

impl Histogram2D {
    pub fn increment(&mut self, x: f64, y: f64) -> Result<(), HistogramError> {
        self.accumulate(x, y, 1.0)
    }

    pub fn accumulate(&mut self, x: f64, y: f64, weight: f64) -> Result<(), HistogramError> {
        let (i, j) = match find2d(self.nx, &self.xrange, self.ny, &self.yrange, x, y) {
            Ok((i, j)) => (i, j),
            Err(_) => return Err(HistogramError::DomainError),
        };

        if i >= self.nx {
            return Err(HistogramError::SanityError(
                "index lies outside valid range of 0 .. nx - 1".to_string(),
            ));
        }

        if j >= self.ny {
            return Err(HistogramError::SanityError(
                "index lies outside valid range of 0 .. ny - 1".to_string(),
            ));
        }

        self.bin[i * self.ny + j] += weight;
        Ok(())
    }
}

// Note: The find2d function would need to be implemented separately in Rust
// with proper error handling and bounds checking