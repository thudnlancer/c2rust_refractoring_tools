use std::ffi::CString;
use std::fs::File;
use std::io::{Write, Result as IoResult};
use std::ptr;

#[derive(Debug)]
pub enum GlpMipStatus {
    Undefined = 1,
    NonOptimal = 2,
    Empty = 4,
    Optimal = 5,
}

#[derive(Debug)]
pub enum GlpDir {
    Min = 1,
    Max = 2,
}

pub struct GlpProb {
    pub name: Option<CString>,
    pub obj: Option<CString>,
    pub dir: GlpDir,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub mip_stat: GlpMipStatus,
    pub mip_obj: f64,
    pub rows: Vec<GlpRow>,
    pub cols: Vec<GlpCol>,
}

pub struct GlpRow {
    pub i: i32,
    pub name: Option<CString>,
    pub mipx: f64,
}

pub struct GlpCol {
    pub j: i32,
    pub name: Option<CString>,
    pub mipx: f64,
}

pub fn glp_write_mip(p: &GlpProb, fname: &str) -> IoResult<()> {
    let mut file = File::create(fname)?;
    let mut count = 0;

    writeln!(file, "c {:<12}{}", "Problem:", 
        p.name.as_ref().map(|s| s.to_str().unwrap_or("")).unwrap_or(""))?;
    count += 1;

    writeln!(file, "c {:<12}{}", "Rows:", p.m)?;
    count += 1;

    writeln!(file, "c {:<12}{}", "Columns:", p.n)?;
    count += 1;

    writeln!(file, "c {:<12}{}", "Non-zeros:", p.nnz)?;
    count += 1;

    let status_str = match p.mip_stat {
        GlpMipStatus::Optimal => "INTEGER OPTIMAL",
        GlpMipStatus::NonOptimal => "INTEGER NON-OPTIMAL",
        GlpMipStatus::Empty => "INTEGER EMPTY",
        GlpMipStatus::Undefined => "INTEGER UNDEFINED",
    };
    writeln!(file, "c {:<12}{}", "Status:", status_str)?;
    count += 1;

    let dir_str = match p.dir {
        GlpDir::Min => "MINimum",
        GlpDir::Max => "MAXimum",
    };
    let obj_name = p.obj.as_ref().map(|s| s.to_str().unwrap_or("")).unwrap_or("");
    let obj_eq = if obj_name.is_empty() { "" } else { " = " };
    writeln!(file, "c {:<12}{}{}{:.10} ({})", 
        "Objective:", obj_name, obj_eq, p.mip_obj, dir_str)?;
    count += 1;

    writeln!(file, "c")?;
    count += 1;

    write!(file, "s mip {} {} ", p.m, p.n)?;
    let stat_char = match p.mip_stat {
        GlpMipStatus::Optimal => 'o',
        GlpMipStatus::NonOptimal => 'f',
        GlpMipStatus::Empty => 'n',
        GlpMipStatus::Undefined => 'u',
    };
    writeln!(file, "{} {:.15}", stat_char, p.mip_obj)?;
    count += 1;

    for row in &p.rows {
        writeln!(file, "i {} {:.15}", row.i, row.mipx)?;
        count += 1;
    }

    for col in &p.cols {
        writeln!(file, "j {} {:.15}", col.j, col.mipx)?;
        count += 1;
    }

    writeln!(file, "e o f")?;
    count += 1;

    println!("{} lines were written", count);
    Ok(())
}