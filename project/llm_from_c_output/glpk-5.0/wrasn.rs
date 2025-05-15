use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::path::Path;
use std::fmt;

struct GlpGraph {
    name: Option<String>,
    nv: i32,
    na: i32,
    v: Vec<GlpVertex>,
    v_size: usize,
    a_size: usize,
}

struct GlpVertex {
    i: i32,
    out: Option<Box<GlpArc>>,
    data: Vec<u8>,
}

struct GlpArc {
    tail: *const GlpVertex,
    head: *const GlpVertex,
    t_next: Option<Box<GlpArc>>,
    data: Vec<u8>,
}

fn glp_write_asnprob(
    g: &GlpGraph,
    v_set: i32,
    a_cost: i32,
    fname: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if v_set >= 0 && v_set > g.v_size as i32 - std::mem::size_of::<i32>() as i32 {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            format!("glp_write_asnprob: v_set = {}; invalid offset", v_set),
        )));
    }
    if a_cost >= 0 && a_cost > g.a_size as i32 - std::mem::size_of::<f64>() as i32 {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            format!("glp_write_asnprob: a_cost = {}; invalid offset", a_cost),
        )));
    }

    println!("Writing assignment problem data to '{}'...", fname);
    let mut file = File::create(fname)?;
    let mut count = 0;

    writeln!(
        file,
        "c {}",
        g.name.as_ref().unwrap_or(&"unknown".to_string())
    )?;
    count += 1;

    writeln!(file, "p asn {} {}", g.nv, g.na)?;
    count += 1;

    for i in 1..=g.nv {
        let v = &g.v[i as usize - 1];
        let k = if v_set >= 0 {
            let offset = v_set as usize;
            let bytes = &v.data[offset..offset + std::mem::size_of::<i32>()];
            i32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        } else {
            if v.out.is_some() { 0 } else { 1 }
        };

        if k == 0 {
            writeln!(file, "n {}", i)?;
            count += 1;
        }
    }

    for i in 1..=g.nv {
        let v = &g.v[i as usize - 1];
        let mut a = &v.out;
        while let Some(arc) = a {
            let cost = if a_cost >= 0 {
                let offset = a_cost as usize;
                let bytes = &arc.data[offset..offset + std::mem::size_of::<f64>()];
                f64::from_ne_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3],
                    bytes[4], bytes[5], bytes[6], bytes[7],
                ])
            } else {
                1.0
            };

            writeln!(
                file,
                "a {} {} {:.15}",
                unsafe { (*arc.tail).i },
                unsafe { (*arc.head).i },
                cost
            )?;
            count += 1;
            a = &arc.t_next;
        }
    }

    writeln!(file, "c eof")?;
    count += 1;

    println!("{} lines were written", count);
    Ok(())
}