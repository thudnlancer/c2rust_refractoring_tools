/* histogram/get.rs
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
    OutOfRange,
    NotFound,
}

impl fmt::Display for HistogramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HistogramError::OutOfRange => write!(f, "index lies outside valid range of 0 .. n - 1"),
            HistogramError::NotFound => write!(f, "x not found in range of h"),
        }
    }
}

impl Error for HistogramError {}

pub struct Histogram {
    pub n: usize,
    pub bin: Vec<f64>,
    pub range: Vec<f64>,
}

impl Histogram {
    pub fn get(&self, i: usize) -> Result<f64, HistogramError> {
        if i >= self.n {
            return Err(HistogramError::OutOfRange);
        }
        Ok(self.bin[i])
    }

    pub fn get_range(&self, i: usize) -> Result<(f64, f64), HistogramError> {
        if i >= self.n {
            return Err(HistogramError::OutOfRange);
        }
        Ok((self.range[i], self.range[i + 1]))
    }

    pub fn find(&self, x: f64) -> Result<usize, HistogramError> {
        match find(self.n, &self.range, x) {
            Ok(i) => Ok(i),
            Err(_) => Err(HistogramError::NotFound),
        }
    }
}

fn find(n: usize, range: &[f64], x: f64) -> Result<usize, ()> {
    if x < range[0] || x >= range[n] {
        return Err(());
    }

    let mut i = 0;
    let mut j = n;
    
    while j - i > 1 {
        let mid = (i + j) / 2;
        if x >= range[mid] {
            i = mid;
        } else {
            j = mid;
        }
    }
    
    Ok(i)
}