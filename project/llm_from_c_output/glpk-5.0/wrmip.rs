use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::path::Path;

#[derive(Debug)]
pub enum MipStatus {
    Opt,
    Feas,
    NoFeas,
    Undef,
}

#[derive(Debug)]
pub enum Direction {
    Min,
    Max,
}

pub struct GlpProb {
    pub name: Option<String>,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub mip_stat: MipStatus,
    pub dir: Direction,
    pub obj: Option<String>,
    pub mip_obj: f64,
    pub rows: Vec<GlpRow>,
    pub cols: Vec<GlpCol>,
}

pub struct GlpRow {
    pub mipx: f64,
}

pub struct GlpCol {
    pub mipx: f64,
}

pub fn glp_write_mip(p: &GlpProb, fname: &str) -> Result<(), Error> {
    if fname.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid filename"));
    }

    println!("Writing MIP solution to '{}'...", fname);
    let path = Path::new(fname);
    let mut file = File::create(path)?;
    let mut count = 0;

    // Write comment lines
    writeln!(file, "c {:<12}{}", "Problem:", p.name.as_deref().unwrap_or(""))?;
    count += 1;
    writeln!(file, "c {:<12}{}", "Rows:", p.m)?;
    count += 1;
    writeln!(file, "c {:<12}{}", "Columns:", p.n)?;
    count += 1;
    writeln!(file, "c {:<12}{}", "Non-zeros:", p.nnz)?;
    count += 1;

    let status_str = match p.mip_stat {
        MipStatus::Opt => "INTEGER OPTIMAL",
        MipStatus::Feas => "INTEGER NON-OPTIMAL",
        MipStatus::NoFeas => "INTEGER EMPTY",
        MipStatus::Undef => "INTEGER UNDEFINED",
    };
    writeln!(file, "c {:<12}{}", "Status:", status_str)?;
    count += 1;

    let dir_str = match p.dir {
        Direction::Min => "MINimum",
        Direction::Max => "MAXimum",
    };
    writeln!(
        file,
        "c {:<12}{}{}{:.10} ({})",
        "Objective:",
        p.obj.as_deref().unwrap_or(""),
        if p.obj.is_some() { " = " } else { "" },
        p.mip_obj,
        dir_str
    )?;
    count += 1;
    writeln!(file, "c")?;
    count += 1;

    // Write solution line
    write!(file, "s mip {} {} ", p.m, p.n)?;
    count += 1;
    match p.mip_stat {
        MipStatus::Opt => write!(file, "o")?,
        MipStatus::Feas => write!(file, "f")?,
        MipStatus::NoFeas => write!(file, "n")?,
        MipStatus::Undef => write!(file, "u")?,
    };
    writeln!(file, " {:.15}", p.mip_obj)?;
    count += 1;

    // Write row solution descriptor lines
    for (i, row) in p.rows.iter().enumerate() {
        writeln!(file, "i {} {:.15}", i + 1, row.mipx)?;
        count += 1;
    }

    // Write column solution descriptor lines
    for (j, col) in p.cols.iter().enumerate() {
        writeln!(file, "j {} {:.15}", j + 1, col.mipx)?;
        count += 1;
    }

    // Write end line
    writeln!(file, "e o f")?;
    count += 1;

    println!("{} lines were written", count);
    Ok(())
}