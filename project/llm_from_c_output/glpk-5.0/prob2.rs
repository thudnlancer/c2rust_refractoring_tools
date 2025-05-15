/* prob2.rs (problem retrieving routines) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2000-2013 Free Software Foundation, Inc.
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

use std::f64;

#[derive(Debug)]
pub enum GlpError {
    OutOfRange(String),
    InternalError(String),
}

pub struct GlpProb {
    pub name: Option<String>,
    pub obj: Option<String>,
    pub dir: i32,
    pub m: i32,
    pub n: i32,
    pub c0: f64,
    pub nnz: i32,
    pub row: Vec<Row>,
    pub col: Vec<Col>,
}

pub struct Row {
    pub name: Option<String>,
    pub type_: i32,
    pub lb: f64,
    pub ub: f64,
    pub ptr: Option<Box<GlpAij>>,
}

pub struct Col {
    pub name: Option<String>,
    pub type_: i32,
    pub lb: f64,
    pub ub: f64,
    pub coef: f64,
    pub ptr: Option<Box<GlpAij>>,
}

pub struct GlpAij {
    pub row: RowRef,
    pub col: ColRef,
    pub val: f64,
    pub r_next: Option<Box<GlpAij>>,
    pub c_next: Option<Box<GlpAij>>,
}

pub struct RowRef {
    pub i: i32,
}

pub struct ColRef {
    pub j: i32,
}

pub const GLP_MIN: i32 = 1;
pub const GLP_MAX: i32 = 2;
pub const GLP_FR: i32 = 1;
pub const GLP_LO: i32 = 2;
pub const GLP_UP: i32 = 3;
pub const GLP_DB: i32 = 4;
pub const GLP_FX: i32 = 5;

impl GlpProb {
    pub fn get_prob_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn get_obj_name(&self) -> Option<&str> {
        self.obj.as_deref()
    }

    pub fn get_obj_dir(&self) -> i32 {
        self.dir
    }

    pub fn get_num_rows(&self) -> i32 {
        self.m
    }

    pub fn get_num_cols(&self) -> i32 {
        self.n
    }

    pub fn get_row_name(&self, i: i32) -> Result<Option<&str>, GlpError> {
        if !(1 <= i && i <= self.m) {
            return Err(GlpError::OutOfRange(format!(
                "glp_get_row_name: i = {}; row number out of range",
                i
            )));
        }
        Ok(self.row[i as usize - 1].name.as_deref())
    }

    pub fn get_col_name(&self, j: i32) -> Result<Option<&str>, GlpError> {
        if !(1 <= j && j <= self.n) {
            return Err(GlpError::OutOfRange(format!(
                "glp_get_col_name: j = {}; column number out of range",
                j
            )));
        }
        Ok(self.col[j as usize - 1].name.as_deref())
    }

    pub fn get_row_type(&self, i: i32) -> Result<i32, GlpError> {
        if !(1 <= i && i <= self.m) {
            return Err(GlpError::OutOfRange(format!(
                "glp_get_row_type: i = {}; row number out of range",
                i
            )));
        }
        Ok(self.row[i as usize - 1].type_)
    }

    pub fn get_row_lb(&self, i: i32) -> Result<f64, GlpError> {
        if !(1 <= i && i <= self.m) {
            return Err(GlpError::OutOfRange(format!(
                "glp_get_row_lb: i = {}; row number out of range",
                i
            )));
        }
        let row = &self.row[i as usize - 1];
        match row.type_ {
            GLP_FR | GLP_UP => Ok(-f64::MAX),
            GLP_LO | GLP_DB | GLP_FX => Ok(row.lb),
            _ => Err(GlpError::InternalError("invalid row type".to_string())),
        }
    }

    pub fn get_row_ub(&self, i: i32) -> Result<f64, GlpError> {
        if !(1 <= i && i <= self.m) {
            return Err(GlpError::OutOfRange(format!(
                "glp_get_row_ub: i = {}; row number out of range",
                i
            )));
        }
        let row = &self.row[i as usize - 1];
        match row.type_ {
            GLP_FR | GLP_LO => Ok(f64::MAX),
            GLP_UP | GLP_DB | GLP_FX => Ok(row.ub),
            _ => Err(GlpError::InternalError("invalid row type".to_string())),
        }
    }

    pub fn get_col_type(&self, j: i32) -> Result<i32, GlpError> {
        if !(1 <= j && j <= self.n) {
            return Err(GlpError::OutOfRange(format!(
                "glp_get_col_type: j = {}; column number out of range",
                j
            )));
        }
        Ok(self.col[j as usize - 1].type_)
    }

    pub fn get_col_lb(&self, j: i32) -> Result<f64, GlpError> {
        if !(1 <= j && j <= self.n) {
            return Err(GlpError::OutOfRange(format!(
                "glp_get_col_lb: j = {}; column number out of range",
                j
            )));
        }
        let col = &self.col[j as usize - 1];
        match col.type_ {
            GLP_FR | GLP_UP => Ok(-f64::MAX),
            GLP_LO | GLP_DB | GLP_FX => Ok(col.lb),
            _ => Err(GlpError::InternalError("invalid column type".to_string())),
        }
    }

    pub fn get_col_ub(&self, j: i32) -> Result<f64, GlpError> {
        if !(1 <= j && j <= self.n) {
            return Err(GlpError::OutOfRange(format!(
                "glp_get_col_ub: j = {}; column number out of range",
                j
            )));
        }
        let col = &self.col[j as usize - 1];
        match col.type_ {
            GLP_FR | GLP_LO => Ok(f64::MAX),
            GLP_UP | GLP_DB | GLP_FX => Ok(col.ub),
            _ => Err(GlpError::InternalError("invalid column type".to_string())),
        }
    }

    pub fn get_obj_coef(&self, j: i32) -> Result<f64, GlpError> {
        if !(0 <= j && j <= self.n) {
            return Err(GlpError::OutOfRange(format!(
                "glp_get_obj_coef: j = {}; column number out of range",
                j
            )));
        }
        Ok(if j == 0 { self.c0 } else { self.col[j as usize - 1].coef })
    }

    pub fn get_num_nz(&self) -> i32 {
        self.nnz
    }

    pub fn get_mat_row(
        &self,
        i: i32,
        ind: &mut [i32],
        val: &mut [f64],
    ) -> Result<i32, GlpError> {
        if !(1 <= i && i <= self.m) {
            return Err(GlpError::OutOfRange(format!(
                "glp_get_mat_row: i = {}; row number out of range",
                i
            )));
        }

        let mut len = 0;
        let mut aij = &self.row[i as usize - 1].ptr;
        while let Some(a) = aij {
            len += 1;
            if !ind.is_empty() {
                ind[len as usize - 1] = a.col.j;
            }
            if !val.is_empty() {
                val[len as usize - 1] = a.val;
            }
            aij = &a.r_next;
        }
        assert!(len <= self.n);
        Ok(len)
    }

    pub fn get_mat_col(
        &self,
        j: i32,
        ind: &mut [i32],
        val: &mut [f64],
    ) -> Result<i32, GlpError> {
        if !(1 <= j && j <= self.n) {
            return Err(GlpError::OutOfRange(format!(
                "glp_get_mat_col: j = {}; column number out of range",
                j
            )));
        }

        let mut len = 0;
        let mut aij = &self.col[j as usize - 1].ptr;
        while let Some(a) = aij {
            len += 1;
            if !ind.is_empty() {
                ind[len as usize - 1] = a.row.i;
            }
            if !val.is_empty() {
                val[len as usize - 1] = a.val;
            }
            aij = &a.c_next;
        }
        assert!(len <= self.m);
        Ok(len)
    }
}