/* gsl_histogram_copy.rs
 * Copyright (C) 2000  Simone Piccardi
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation; either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this library; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
 */
/***************************************************************
 *
 * File gsl_histogram_copy.rs: 
 * Routine to copy an histogram. 
 * Need GSL library and headers.
 *
 * Author: S. Piccardi
 * Jan. 2000
 *
 ***************************************************************/

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HistogramError {
    InvalidValue(String),
    NoMemory(String),
}

impl fmt::Display for HistogramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HistogramError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            HistogramError::NoMemory(msg) => write!(f, "Memory allocation failed: {}", msg),
        }
    }
}

impl Error for HistogramError {}

pub struct Histogram {
    n: usize,
    range: Vec<f64>,
    bin: Vec<f64>,
}

impl Histogram {
    /// Copy the contents of a histogram into another
    pub fn memcpy(&mut self, src: &Histogram) -> Result<(), HistogramError> {
        if self.n != src.n {
            return Err(HistogramError::InvalidValue(
                "histograms have different sizes, cannot copy".to_string(),
            ));
        }

        self.range.copy_from_slice(&src.range);
        self.bin.copy_from_slice(&src.bin);

        Ok(())
    }

    /// Duplicate a histogram creating an identical new one
    pub fn clone(src: &Histogram) -> Result<Histogram, HistogramError> {
        let n = src.n;
        let mut h = Histogram {
            n,
            range: src.range.clone(),
            bin: src.bin.clone(),
        };

        if h.range.is_empty() || h.bin.is_empty() {
            return Err(HistogramError::NoMemory(
                "failed to allocate space for histogram struct".to_string(),
            ));
        }

        Ok(h)
    }

    /// Create a new histogram with the given range
    pub fn calloc_range(n: usize, range: &[f64]) -> Result<Histogram, HistogramError> {
        if range.len() != n + 1 {
            return Err(HistogramError::InvalidValue(
                "range length must be n + 1".to_string(),
            ));
        }

        Ok(Histogram {
            n,
            range: range.to_vec(),
            bin: vec![0.0; n],
        })
    }
}