use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::path::Path;
use std::ffi::CStr;

pub struct GlpGraph {
    nv: i32,
    na: i32,
    a_size: i32,
    v: Vec<GlpVertex>,
    name: Option<String>,
}

pub struct GlpVertex {
    i: i32,
    out: Option<Box<GlpArc>>,
}

pub struct GlpArc {
    tail: *const GlpVertex,
    head: *const GlpVertex,
    t_next: Option<Box<GlpArc>>,
    data: Vec<u8>,
}

pub fn glp_write_maxflow(
    G: &GlpGraph,
    s: i32,
    t: i32,
    a_cap: i32,
    fname: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if !(1 <= s && s <= G.nv) {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            format!("glp_write_maxflow: s = {}; source node number out of range", s),
        )));
    }
    if !(1 <= t && t <= G.nv) {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            format!("glp_write_maxflow: t = {}: sink node number out of range", t),
        )));
    }
    if a_cap >= 0 && a_cap > G.a_size - std::mem::size_of::<f64>() as i32 {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            format!("glp_write_mincost: a_cap = {}; invalid offset", a_cap),
        )));
    }

    println!("Writing maximum flow problem data to '{}'...", fname);
    let mut file = File::create(fname)?;
    let mut count = 0;

    writeln!(
        file,
        "c {}",
        G.name.as_ref().map_or("unknown", |n| n.as_str())
    )?;
    count += 1;

    writeln!(file, "p max {} {}", G.nv, G.na)?;
    count += 1;

    writeln!(file, "n {} s", s)?;
    count += 1;

    writeln!(file, "n {} t", t)?;
    count += 1;

    for i in 1..=G.nv {
        let v = &G.v[i as usize - 1];
        let mut a = &v.out;
        while let Some(arc) = a {
            let cap = if a_cap >= 0 {
                let offset = a_cap as usize;
                let bytes = &arc.data[offset..offset + std::mem::size_of::<f64>()];
                f64::from_le_bytes(bytes.try_into().unwrap())
            } else {
                1.0
            };
            writeln!(
                file,
                "a {} {} {:.15}",
                unsafe { (*arc.tail).i },
                unsafe { (*arc.head).i },
                cap
            )?;
            count += 1;
            a = &arc.t_next;
        }
    }

    writeln!(file, "c eof")?;
    count += 1;

    file.sync_all()?;
    println!("{} lines were written", count);

    Ok(())
}