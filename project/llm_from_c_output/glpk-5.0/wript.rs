use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::path::Path;

struct GlpProb {
    name: Option<String>,
    m: i32,
    n: i32,
    nnz: i32,
    ipt_stat: GlpStatus,
    dir: GlpDir,
    obj: Option<String>,
    ipt_obj: f64,
    row: Vec<GlpRow>,
    col: Vec<GlpCol>,
}

struct GlpRow {
    pval: f64,
    dval: f64,
}

struct GlpCol {
    pval: f64,
    dval: f64,
}

#[derive(Debug)]
enum GlpStatus {
    Opt,
    Infeas,
    Nofeas,
    Undef,
    Unknown,
}

#[derive(Debug)]
enum GlpDir {
    Min,
    Max,
    Unknown,
}

fn glp_write_ipt(p: &GlpProb, fname: &str) -> Result<(), Error> {
    let path = Path::new(fname);
    let mut file = File::create(path)?;
    let mut count = 0;

    // Write comment lines
    writeln!(file, "c {:12}{}", "Problem:", p.name.as_deref().unwrap_or(""))?;
    count += 1;
    writeln!(file, "c {:12}{}", "Rows:", p.m)?;
    count += 1;
    writeln!(file, "c {:12}{}", "Columns:", p.n)?;
    count += 1;
    writeln!(file, "c {:12}{}", "Non-zeros:", p.nnz)?;
    count += 1;

    let status_str = match p.ipt_stat {
        GlpStatus::Opt => "OPTIMAL",
        GlpStatus::Infeas => "INFEASIBLE (INTERMEDIATE)",
        GlpStatus::Nofeas => "INFEASIBLE (FINAL)",
        GlpStatus::Undef => "UNDEFINED",
        GlpStatus::Unknown => "???",
    };
    writeln!(file, "c {:12}{}", "Status:", status_str)?;
    count += 1;

    let dir_str = match p.dir {
        GlpDir::Min => "MINimum",
        GlpDir::Max => "MAXimum",
        GlpDir::Unknown => "???",
    };
    let obj_str = match &p.obj {
        Some(name) => format!("{} = {:.10} ({})", name, p.ipt_obj, dir_str),
        None => format!("{:.10} ({})", p.ipt_obj, dir_str),
    };
    writeln!(file, "c {:12}{}", "Objective:", obj_str)?;
    count += 1;
    writeln!(file, "c")?;
    count += 1;

    // Write solution line
    let status_char = match p.ipt_stat {
        GlpStatus::Opt => 'o',
        GlpStatus::Infeas => 'i',
        GlpStatus::Nofeas => 'n',
        GlpStatus::Undef => 'u',
        GlpStatus::Unknown => '?',
    };
    write!(file, "s ipt {} {} {} {:.15}\n", p.m, p.n, status_char, p.ipt_obj)?;
    count += 1;

    // Write row solution descriptor lines
    for (i, row) in p.row.iter().enumerate() {
        writeln!(file, "i {} {:.15} {:.15}", i + 1, row.pval, row.dval)?;
        count += 1;
    }

    // Write column solution descriptor lines
    for (j, col) in p.col.iter().enumerate() {
        writeln!(file, "j {} {:.15} {:.15}", j + 1, col.pval, col.dval)?;
        count += 1;
    }

    // Write end line
    writeln!(file, "e o f")?;
    count += 1;

    println!("{} lines were written", count);
    Ok(())
}