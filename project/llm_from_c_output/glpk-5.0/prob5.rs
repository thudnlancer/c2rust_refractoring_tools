/* prob5.rs (LP problem basis constructing routines) */

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
pub enum GlpStatus {
    BS,
    NL,
    NU,
    NF,
    NS,
}

#[derive(Debug)]
pub enum GlpVarType {
    FR,
    LO,
    UP,
    DB,
    FX,
}

pub struct GlpRow {
    pub type_: GlpVarType,
    pub stat: GlpStatus,
}

pub struct GlpCol {
    pub type_: GlpVarType,
    pub stat: GlpStatus,
    pub lb: f64,
    pub ub: f64,
}

pub struct GlpProb {
    pub m: usize,
    pub n: usize,
    pub valid: bool,
    pub row: Vec<GlpRow>,
    pub col: Vec<GlpCol>,
}

impl GlpProb {
    pub fn set_row_stat(&mut self, i: usize, stat: GlpStatus) -> Result<(), String> {
        if !(1 <= i && i <= self.m) {
            return Err(format!("glp_set_row_stat: i = {}; row number out of range", i));
        }

        match stat {
            GlpStatus::BS | GlpStatus::NL | GlpStatus::NU | GlpStatus::NF | GlpStatus::NS => (),
            _ => return Err(format!("glp_set_row_stat: i = {}; invalid status", i)),
        }

        let row = &mut self.row[i - 1];
        let new_stat = if matches!(stat, GlpStatus::BS) {
            stat
        } else {
            match row.type_ {
                GlpVarType::FR => GlpStatus::NF,
                GlpVarType::LO => GlpStatus::NL,
                GlpVarType::UP => GlpStatus::NU,
                GlpVarType::DB => {
                    if matches!(stat, GlpStatus::NU) {
                        GlpStatus::NU
                    } else {
                        GlpStatus::NL
                    }
                }
                GlpVarType::FX => GlpStatus::NS,
            }
        };

        if (matches!(row.stat, GlpStatus::BS) && !matches!(new_stat, GlpStatus::BS)) ||
           (!matches!(row.stat, GlpStatus::BS) && matches!(new_stat, GlpStatus::BS)) {
            self.valid = false;
        }

        row.stat = new_stat;
        Ok(())
    }

    pub fn set_col_stat(&mut self, j: usize, stat: GlpStatus) -> Result<(), String> {
        if !(1 <= j && j <= self.n) {
            return Err(format!("glp_set_col_stat: j = {}; column number out of range", j));
        }

        match stat {
            GlpStatus::BS | GlpStatus::NL | GlpStatus::NU | GlpStatus::NF | GlpStatus::NS => (),
            _ => return Err(format!("glp_set_col_stat: j = {}; invalid status", j)),
        }

        let col = &mut self.col[j - 1];
        let new_stat = if matches!(stat, GlpStatus::BS) {
            stat
        } else {
            match col.type_ {
                GlpVarType::FR => GlpStatus::NF,
                GlpVarType::LO => GlpStatus::NL,
                GlpVarType::UP => GlpStatus::NU,
                GlpVarType::DB => {
                    if matches!(stat, GlpStatus::NU) {
                        GlpStatus::NU
                    } else {
                        GlpStatus::NL
                    }
                }
                GlpVarType::FX => GlpStatus::NS,
            }
        };

        if (matches!(col.stat, GlpStatus::BS) && !matches!(new_stat, GlpStatus::BS)) ||
           (!matches!(col.stat, GlpStatus::BS) && matches!(new_stat, GlpStatus::BS)) {
            self.valid = false;
        }

        col.stat = new_stat;
        Ok(())
    }

    pub fn std_basis(&mut self) -> Result<(), String> {
        // Make all auxiliary variables basic
        for i in 1..=self.m {
            self.set_row_stat(i, GlpStatus::BS)?;
        }

        // Make all structural variables non-basic
        for j in 1..=self.n {
            let col = &self.col[j - 1];
            if matches!(col.type_, GlpVarType::DB) && col.lb.abs() > col.ub.abs() {
                self.set_col_stat(j, GlpStatus::NU)?;
            } else {
                self.set_col_stat(j, GlpStatus::NL)?;
            }
        }
        Ok(())
    }
}