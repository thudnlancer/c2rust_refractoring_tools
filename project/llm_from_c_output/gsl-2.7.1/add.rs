// histogram/add.rs
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
            HistogramError::DomainError => write!(f, "input value is outside histogram range"),
            HistogramError::SanityError(msg) => write!(f, "{}", msg),
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
    pub fn increment(&mut self, x: f64) -> Result<(), HistogramError> {
        self.accumulate(x, 1.0)
    }

    pub fn accumulate(&mut self, x: f64, weight: f64) -> Result<(), HistogramError> {
        let n = self.n;
        let index = match find(n, &self.range, x) {
            Ok(i) => i,
            Err(_) => return Err(HistogramError::DomainError),
        };

        if index >= n {
            return Err(HistogramError::SanityError(
                "index lies outside valid range of 0 .. n - 1".to_string(),
            ));
        }

        self.bin[index] += weight;
        Ok(())
    }
}

fn find(n: usize, range: &[f64], x: f64) -> Result<usize, ()> {
    // Implementation of find function would go here
    // For now, we'll return a placeholder
    Err(())
}