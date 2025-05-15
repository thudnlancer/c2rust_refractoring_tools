/* combination/init.rs
 * based on permutation/init.c by Brian Gough
 * 
 * Copyright (C) 2001 Szymon Jaroszewicz
 * Copyright (C) 2009 Brian Gough
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
pub struct Combination {
    data: Vec<usize>,
    n: usize,
    k: usize,
}

#[derive(Debug)]
pub enum CombinationError {
    DomainError(String),
    MemoryError(String),
}

impl fmt::Display for CombinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombinationError::DomainError(msg) => write!(f, "Domain error: {}", msg),
            CombinationError::MemoryError(msg) => write!(f, "Memory error: {}", msg),
        }
    }
}

impl Error for CombinationError {}

impl Combination {
    pub fn new(n: usize, k: usize) -> Result<Self, CombinationError> {
        if n == 0 {
            return Err(CombinationError::DomainError(
                "combination parameter n must be positive integer".to_string(),
            ));
        }
        if k > n {
            return Err(CombinationError::DomainError(
                "combination length k must be an integer less than or equal to n".to_string(),
            ));
        }

        let data = if k > 0 {
            vec![0; k]
        } else {
            Vec::new()
        };

        Ok(Self { data, n, k })
    }

    pub fn with_identity(n: usize, k: usize) -> Result<Self, CombinationError> {
        let mut c = Self::new(n, k)?;
        
        // initialize combination to identity
        for i in 0..k {
            c.data[i] = i;
        }

        Ok(c)
    }

    pub fn init_first(&mut self) {
        let k = self.k;
        
        // initialize combination to identity
        for i in 0..k {
            self.data[i] = i;
        }
    }

    pub fn init_last(&mut self) {
        let k = self.k;
        let n = self.n;
        
        // initialize combination to identity
        for i in 0..k {
            self.data[i] = n - k + i;
        }
    }
}

impl Drop for Combination {
    fn drop(&mut self) {
        // Rust's Vec will automatically deallocate when dropped
    }
}