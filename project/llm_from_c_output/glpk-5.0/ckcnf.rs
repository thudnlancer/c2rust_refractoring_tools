/* ckcnf.rs (check for CNF-SAT problem instance) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2010-2016 Free Software Foundation, Inc.
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
pub enum CnfSatError {
    InvalidVariable,
    NonZeroObjective,
    NonZeroCoefficient,
    InvalidRowType,
    InvalidCoefficient,
    InvalidRhs,
}

impl fmt::Display for CnfSatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CnfSatError::InvalidVariable => write!(f, "Variable is not binary"),
            CnfSatError::NonZeroObjective => write!(f, "Objective function is not zero"),
            CnfSatError::NonZeroCoefficient => write!(f, "Non-zero coefficient in objective"),
            CnfSatError::InvalidRowType => write!(f, "Row is not of '>=' type"),
            CnfSatError::InvalidCoefficient => write!(f, "Invalid constraint coefficient"),
            CnfSatError::InvalidRhs => write!(f, "Invalid right-hand side"),
        }
    }
}

impl Error for CnfSatError {}

pub trait Problem {
    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;
    fn is_binary_var(&self, j: usize) -> bool;
    fn objective_constant(&self) -> f64;
    fn objective_coefficient(&self, j: usize) -> f64;
    fn is_geq_row(&self, i: usize) -> bool;
    fn row_coefficients(&self, i: usize) -> Vec<(usize, f64)>;
    fn row_rhs(&self, i: usize) -> f64;
}

pub fn check_cnfsat<P: Problem>(p: &P) -> Result<(), CnfSatError> {
    let n = p.num_cols();
    let m = p.num_rows();

    // Check columns (variables)
    for j in 1..=n {
        if !p.is_binary_var(j) {
            return Err(CnfSatError::InvalidVariable);
        }
    }

    // Check objective function
    if p.objective_constant() != 0.0 {
        return Err(CnfSatError::NonZeroObjective);
    }

    for j in 1..=n {
        if p.objective_coefficient(j) != 0.0 {
            return Err(CnfSatError::NonZeroCoefficient);
        }
    }

    // Check rows (constraints)
    for i in 1..=m {
        if !p.is_geq_row(i) {
            return Err(CnfSatError::InvalidRowType);
        }

        let mut neg = 0;
        for (_, val) in p.row_coefficients(i) {
            if val == 1.0 {
                continue;
            } else if val == -1.0 {
                neg += 1;
            } else {
                return Err(CnfSatError::InvalidCoefficient);
            }
        }

        let expected_rhs = 1.0 - neg as f64;
        if (p.row_rhs(i) - expected_rhs).abs() > f64::EPSILON {
            return Err(CnfSatError::InvalidRhs);
        }
    }

    Ok(())
}