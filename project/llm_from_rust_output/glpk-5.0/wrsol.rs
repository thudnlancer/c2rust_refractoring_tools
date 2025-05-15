use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Write, Error};
use std::ptr;

#[derive(Debug)]
pub enum GlpStatus {
    Optimal,
    Feasible,
    InfeasibleIntermediate,
    InfeasibleFinal,
    Unbounded,
    Undefined,
    Unknown,
}

#[derive(Debug)]
pub enum GlpDir {
    Min,
    Max,
    Unknown,
}

#[derive(Debug)]
pub enum RowStat {
    Basic,
    Lower,
    Upper,
    Free,
    Fixed,
    Unknown,
}

#[derive(Debug)]
pub enum ColStat {
    Basic,
    Lower,
    Upper,
    Free,
    Fixed,
    Unknown,
}

#[derive(Debug)]
pub struct GlpProb {
    pub name: Option<String>,
    pub obj: Option<String>,
    pub dir: GlpDir,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub obj_val: f64,
    pub pbs_stat: i32,
    pub dbs_stat: i32,
    pub rows: Vec<GLPROW>,
    pub cols: Vec<GLPCOL>,
}

#[derive(Debug)]
pub struct GLPCOL {
    pub j: i32,
    pub name: Option<String>,
    pub kind: i32,
    pub type_: i32,
    pub lb: f64,
    pub ub: f64,
    pub coef: f64,
    pub stat: ColStat,
    pub prim: f64,
    pub dual: f64,
}

#[derive(Debug)]
pub struct GLPROW {
    pub i: i32,
    pub name: Option<String>,
    pub type_: i32,
    pub lb: f64,
    pub ub: f64,
    pub stat: RowStat,
    pub prim: f64,
    pub dual: f64,
}

pub fn glp_write_sol(p: &GlpProb, fname: &str) -> Result<(), Error> {
    let mut file = File::create(fname)?;
    let mut count = 0;

    writeln!(file, "Writing basic solution to '{}'...", fname)?;

    // Write problem information
    writeln!(file, "c {:12}{}", "Problem:", p.name.as_deref().unwrap_or(""))?;
    count += 1;
    writeln!(file, "c {:12}{}", "Rows:", p.m)?;
    count += 1;
    writeln!(file, "c {:12}{}", "Columns:", p.n)?;
    count += 1;
    writeln!(file, "c {:12}{}", "Non-zeros:", p.nnz)?;
    count += 1;

    let status_str = match p.status() {
        GlpStatus::Optimal => "OPTIMAL",
        GlpStatus::Feasible => "FEASIBLE",
        GlpStatus::InfeasibleIntermediate => "INFEASIBLE (INTERMEDIATE)",
        GlpStatus::InfeasibleFinal => "INFEASIBLE (FINAL)",
        GlpStatus::Unbounded => "UNBOUNDED",
        GlpStatus::Undefined => "UNDEFINED",
        GlpStatus::Unknown => "???",
    };
    writeln!(file, "c {:12}{}", "Status:", status_str)?;
    count += 1;

    let dir_str = match p.dir {
        GlpDir::Min => "MINimum",
        GlpDir::Max => "MAXimum",
        GlpDir::Unknown => "???",
    };
    writeln!(
        file,
        "c {:12}{}{}{:.10} ({})",
        "Objective:",
        p.obj.as_deref().unwrap_or(""),
        if p.obj.is_some() { " = " } else { "" },
        p.obj_val,
        dir_str
    )?;
    count += 1;

    writeln!(file, "c")?;
    count += 1;

    // Write solution header
    write!(file, "s bas {} {} ", p.m, p.n)?;
    match p.pbs_stat {
        1 => write!(file, "u")?,
        2 => write!(file, "f")?,
        3 => write!(file, "i")?,
        4 => write!(file, "n")?,
        _ => write!(file, "?")?,
    }
    write!(file, " ")?;
    match p.dbs_stat {
        1 => write!(file, "u")?,
        2 => write!(file, "f")?,
        3 => write!(file, "i")?,
        4 => write!(file, "n")?,
        _ => write!(file, "?")?,
    }
    writeln!(file, " {:.15}", p.obj_val)?;
    count += 1;

    // Write rows
    for row in &p.rows {
        write!(file, "i {} ", row.i)?;
        match row.stat {
            RowStat::Basic => write!(file, "b")?,
            RowStat::Lower => write!(file, "l")?,
            RowStat::Upper => write!(file, "u")?,
            RowStat::Free => write!(file, "f")?,
            RowStat::Fixed => write!(file, "s")?,
            RowStat::Unknown => panic!("Invalid row status"),
        }
        writeln!(file, " {:.15} {:.15}", row.prim, row.dual)?;
        count += 1;
    }

    // Write columns
    for col in &p.cols {
        write!(file, "j {} ", col.j)?;
        match col.stat {
            ColStat::Basic => write!(file, "b")?,
            ColStat::Lower => write!(file, "l")?,
            ColStat::Upper => write!(file, "u")?,
            ColStat::Free => write!(file, "f")?,
            ColStat::Fixed => write!(file, "s")?,
            ColStat::Unknown => panic!("Invalid column status"),
        }
        writeln!(file, " {:.15} {:.15}", col.prim, col.dual)?;
        count += 1;
    }

    writeln!(file, "e o f")?;
    count += 1;

    println!("{} lines were written", count);
    Ok(())
}

impl GlpProb {
    fn status(&self) -> GlpStatus {
        match self.pbs_stat {
            5 => GlpStatus::Optimal,
            2 => GlpStatus::Feasible,
            3 => GlpStatus::InfeasibleIntermediate,
            4 => GlpStatus::InfeasibleFinal,
            6 => GlpStatus::Unbounded,
            1 => GlpStatus::Undefined,
            _ => GlpStatus::Unknown,
        }
    }
}