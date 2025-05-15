use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;
use std::fmt::Write as FmtWrite;

pub struct GlpProb {
    pub name: Option<String>,
    pub n: i32,
    pub m: i32,
    pub rows: Vec<Row>,
}

pub struct Row {
    pub ptr: Option<Box<Aij>>,
}

pub struct Aij {
    pub col: Col,
    pub val: f64,
    pub r_next: Option<Box<Aij>>,
}

pub struct Col {
    pub j: i32,
}

pub fn glp_write_cnfsat(p: &GlpProb, fname: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut count = 0;
    
    if glp_check_cnfsat(p) != 0 {
        println!("glp_write_cnfsat: problem object does not encode CNF-SAT instance");
        return Err("Invalid CNF-SAT instance".into());
    }

    println!("Writing CNF-SAT problem data to '{}'...", fname);
    let file = File::create(fname)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "c {}", p.name.as_deref().unwrap_or("unknown"))?;
    count += 1;
    
    writeln!(writer, "p cnf {} {}", p.n, p.m)?;
    count += 1;

    for i in 0..p.m as usize {
        let mut len = 0;
        let mut line = String::new();
        let mut aij = &p.rows[i].ptr;

        while let Some(ref current) = aij {
            let mut j = current.col.j;
            if current.val < 0.0 {
                j = -j;
            }
            let s = j.to_string();

            if len > 0 && len + 1 + s.len() > 72 {
                writeln!(writer)?;
                count += 1;
                len = 0;
                line.clear();
            }

            if !line.is_empty() {
                line.push(' ');
                len += 1;
            }
            line.push_str(&s);
            len += s.len();

            aij = &current.r_next;
        }

        if len > 0 && len + 2 > 72 {
            writeln!(writer)?;
            count += 1;
            line.clear();
        }

        if line.is_empty() {
            writeln!(writer, "0")?;
        } else {
            writeln!(writer, "{} 0", line)?;
        }
        count += 1;
    }

    writeln!(writer, "c eof")?;
    count += 1;

    println!("{} lines were written", count);
    Ok(())
}

fn glp_check_cnfsat(_p: &GlpProb) -> i32 {
    // Placeholder for CNF-SAT validation logic
    0
}