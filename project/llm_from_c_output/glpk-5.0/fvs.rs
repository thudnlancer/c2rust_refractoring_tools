/* fvs.rs (sparse vector in FVS format) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2016 Free Software Foundation, Inc.
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
pub struct FVS {
    n: usize,
    nnz: usize,
    ind: Vec<usize>,
    vec: Vec<f64>,
}

#[derive(Debug)]
pub struct FVSError {
    details: String,
}

impl FVSError {
    fn new(msg: &str) -> FVSError {
        FVSError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for FVSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for FVSError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl FVS {
    pub fn new(n: usize) -> Result<FVS, FVSError> {
        if n == 0 {
            return Err(FVSError::new("Vector dimension must be positive"));
        }
        let mut vec = vec![0.0; n + 1];
        for j in 1..=n {
            vec[j] = 0.0;
        }
        Ok(FVS {
            n,
            nnz: 0,
            ind: vec![0; n + 1],
            vec,
        })
    }

    pub fn check_vec(&self) -> Result<(), FVSError> {
        if self.n == 0 {
            return Err(FVSError::new("Vector dimension must be positive"));
        }
        if self.nnz > self.n {
            return Err(FVSError::new("Number of non-zero elements exceeds dimension"));
        }

        let mut map = vec![false; self.n + 1];
        for j in 1..=self.n {
            map[j] = self.vec[j] != 0.0;
        }

        for k in 1..=self.nnz {
            let j = self.ind[k];
            if j < 1 || j > self.n {
                return Err(FVSError::new("Invalid index in sparse vector"));
            }
            if !map[j] {
                return Err(FVSError::new("Inconsistent sparse vector"));
            }
            map[j] = false;
        }

        for j in 1..=self.n {
            if map[j] {
                return Err(FVSError::new("Inconsistent sparse vector"));
            }
        }

        Ok(())
    }

    pub fn gather_vec(&mut self, eps: f64) {
        let mut nnz = 0;
        for j in (1..=self.n).rev() {
            if -eps < self.vec[j] && self.vec[j] < eps {
                self.vec[j] = 0.0;
            } else {
                nnz += 1;
                self.ind[nnz] = j;
            }
        }
        self.nnz = nnz;
    }

    pub fn clear_vec(&mut self) {
        for k in 1..=self.nnz {
            self.vec[self.ind[k]] = 0.0;
        }
        self.nnz = 0;
    }

    pub fn copy_vec(&mut self, other: &FVS) -> Result<(), FVSError> {
        if self.n != other.n {
            return Err(FVSError::new("Vector dimensions don't match"));
        }
        if std::ptr::eq(self, other) {
            return Err(FVSError::new("Cannot copy from self"));
        }

        self.clear_vec();
        self.nnz = other.nnz;
        for k in 1..=other.nnz {
            let j = other.ind[k];
            self.ind[k] = j;
            self.vec[j] = other.vec[j];
        }

        Ok(())
    }

    pub fn adjust_vec(&mut self, eps: f64) {
        let mut cnt = 0;
        for k in 1..=self.nnz {
            let j = self.ind[k];
            if -eps < self.vec[j] && self.vec[j] < eps {
                self.vec[j] = 0.0;
            } else {
                cnt += 1;
                self.ind[cnt] = j;
            }
        }
        self.nnz = cnt;
    }
}

impl Drop for FVS {
    fn drop(&mut self) {
        self.n = 0;
        self.nnz = 0;
    }
}