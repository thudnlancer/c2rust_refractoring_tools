use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Write, Result as IoResult};
use std::ptr;
use std::os::raw::{c_int, c_double, c_char, c_uchar};

#[derive(Debug)]
pub struct GlpProb {
    pub name: Option<CString>,
    pub obj: Option<CString>,
    pub dir: i32,
    pub c0: f64,
    pub m_max: i32,
    pub n_max: i32,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub ipt_stat: i32,
    pub ipt_obj: f64,
    pub rows: Vec<GlpRow>,
    pub cols: Vec<GlpCol>,
}

#[derive(Debug)]
pub struct GlpRow {
    pub i: i32,
    pub name: Option<CString>,
    pub type_: i32,
    pub lb: f64,
    pub ub: f64,
    pub pval: f64,
    pub dval: f64,
}

#[derive(Debug)]
pub struct GlpCol {
    pub j: i32,
    pub name: Option<CString>,
    pub type_: i32,
    pub lb: f64,
    pub ub: f64,
    pub pval: f64,
    pub dval: f64,
}

pub fn glp_write_ipt(p: &GlpProb, fname: &str) -> IoResult<()> {
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

    let status = match p.ipt_stat {
        5 => "OPTIMAL",
        3 => "INFEASIBLE (INTERMEDIATE)",
        4 => "INFEASIBLE (FINAL)",
        1 => "UNDEFINED",
        _ => "???",
    };
    writeln!(file, "c {:<12}{}", "Status:", status)?;
    count += 1;

    let direction = match p.dir {
        1 => "MINimum",
        2 => "MAXimum",
        _ => "???",
    };
    writeln!(file, "c {:<12}{}{}{:.10} ({})", 
        "Objective:",
        p.obj.as_ref().map(|s| s.to_str().unwrap_or("")).unwrap_or(""),
        if p.obj.is_some() { " = " } else { "" },
        p.ipt_obj,
        direction)?;
    count += 1;

    writeln!(file, "c")?;
    count += 1;

    let status_char = match p.ipt_stat {
        5 => 'o',
        3 => 'i',
        4 => 'n',
        1 => 'u',
        _ => '?',
    };
    writeln!(file, "s ipt {} {} {} {:.15}", p.m, p.n, status_char, p.ipt_obj)?;
    count += 1;

    for row in &p.rows {
        writeln!(file, "i {} {:.15} {:.15}", row.i, row.pval, row.dval)?;
        count += 1;
    }

    for col in &p.cols {
        writeln!(file, "j {} {:.15} {:.15}", col.j, col.pval, col.dval)?;
        count += 1;
    }

    writeln!(file, "e o f")?;
    count += 1;

    Ok(())
}