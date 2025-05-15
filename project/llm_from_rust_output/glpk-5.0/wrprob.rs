use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::path::Path;

#[derive(Debug)]
pub enum GlpProblemType {
    LP,
    MIP,
}

#[derive(Debug)]
pub enum GlpDirection {
    Min,
    Max,
    Unknown,
}

#[derive(Debug)]
pub struct GlpProblem {
    pub name: Option<String>,
    pub obj_name: Option<String>,
    pub direction: GlpDirection,
    pub c0: f64,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub rows: Vec<GlpRow>,
    pub cols: Vec<GlpCol>,
    pub mip: bool,
}

#[derive(Debug)]
pub struct GlpRow {
    pub i: i32,
    pub name: Option<String>,
    pub row_type: GlpRowType,
    pub lb: f64,
    pub ub: f64,
}

#[derive(Debug)]
pub enum GlpRowType {
    Free,
    LowerBound,
    UpperBound,
    DoubleBound,
    Fixed,
}

#[derive(Debug)]
pub struct GlpCol {
    pub j: i32,
    pub name: Option<String>,
    pub kind: GlpColKind,
    pub col_type: GlpColType,
    pub lb: f64,
    pub ub: f64,
    pub coef: f64,
    pub aij: Vec<GlpAij>,
}

#[derive(Debug)]
pub enum GlpColKind {
    Continuous,
    Integer,
}

#[derive(Debug)]
pub enum GlpColType {
    Free,
    LowerBound,
    UpperBound,
    DoubleBound,
    Fixed,
}

#[derive(Debug)]
pub struct GlpAij {
    pub row: i32,
    pub col: i32,
    pub val: f64,
}

impl GlpProblem {
    pub fn write_prob(&self, fname: &str) -> Result<(), Error> {
        let path = Path::new(fname);
        let mut file = File::create(path)?;
        
        println!("Writing problem data to '{}'...", fname);
        
        // Write problem header
        let problem_type = if self.mip { "mip" } else { "lp" };
        let direction = match self.direction {
            GlpDirection::Min => "min",
            GlpDirection::Max => "max",
            GlpDirection::Unknown => "???",
        };
        
        writeln!(
            file,
            "p {} {} {} {} {}",
            problem_type,
            direction,
            self.m,
            self.n,
            self.nnz
        )?;
        
        let mut count = 1;
        
        // Write problem name if exists
        if let Some(name) = &self.name {
            writeln!(file, "n p {}", name)?;
            count += 1;
        }
        
        // Write objective name if exists
        if let Some(obj_name) = &self.obj_name {
            writeln!(file, "n z {}", obj_name)?;
            count += 1;
        }
        
        // Write rows
        for row in &self.rows {
            if !(matches!(row.row_type, GlpRowType::Fixed) || row.lb != 0.0 {
                write!(file, "i {} ", row.i)?;
                count += 1;
                
                match row.row_type {
                    GlpRowType::Free => writeln!(file, "f")?,
                    GlpRowType::LowerBound => writeln!(file, "l {:.15}", row.lb)?,
                    GlpRowType::UpperBound => writeln!(file, "u {:.15}", row.ub)?,
                    GlpRowType::DoubleBound => writeln!(file, "d {:.15} {:.15}", row.lb, row.ub)?,
                    GlpRowType::Fixed => writeln!(file, "s {:.15}", row.lb)?,
                }
            }
            
            if let Some(name) = &row.name {
                writeln!(file, "n i {} {}", row.i, name)?;
                count += 1;
            }
        }
        
        // Write columns
        for col in &self.cols {
            if !(!self.mip && matches!(col.col_type, GlpColType::LowerBound) && col.lb == 0.0) &&
               !(self.mip && matches!(col.kind, GlpColKind::Integer) && 
                matches!(col.col_type, GlpColType::DoubleBound) && 
                col.lb == 0.0 && col.ub == 1.0
            {
                write!(file, "j {} ", col.j)?;
                count += 1;
                
                if self.mip {
                    match col.kind {
                        GlpColKind::Continuous => write!(file, "c ")?,
                        GlpColKind::Integer => write!(file, "i ")?,
                    }
                }
                
                match col.col_type {
                    GlpColType::Free => writeln!(file, "f")?,
                    GlpColType::LowerBound => writeln!(file, "l {:.15}", col.lb)?,
                    GlpColType::UpperBound => writeln!(file, "u {:.15}", col.ub)?,
                    GlpColType::DoubleBound => writeln!(file, "d {:.15} {:.15}", col.lb, col.ub)?,
                    GlpColType::Fixed => writeln!(file, "s {:.15}", col.lb)?,
                }
            }
            
            if let Some(name) = &col.name {
                writeln!(file, "n j {} {}", col.j, name)?;
                count += 1;
            }
        }
        
        // Write constant term if non-zero
        if self.c0 != 0.0 {
            writeln!(file, "a 0 0 {:.15}", self.c0)?;
            count += 1;
        }
        
        // Write objective coefficients
        for col in &self.cols {
            if col.coef != 0.0 {
                writeln!(file, "a 0 {} {:.15}", col.j, col.coef)?;
                count += 1;
            }
        }
        
        // Write constraint matrix
        for row in &self.rows {
            for aij in &col.aij {
                writeln!(file, "a {} {} {:.15}", row.i, aij.col, aij.val)?;
                count += 1;
            }
        }
        
        // Write end marker
        writeln!(file, "e o f")?;
        count += 1;
        
        println!("{} lines were written", count);
        Ok(())
    }
}