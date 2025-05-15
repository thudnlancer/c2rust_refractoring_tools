use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::path::Path;
use std::ffi::CStr;
use std::os::raw::c_char;

struct GlpGraph {
    name: Option<String>,
    nv: i32,
    na: i32,
    v: Vec<GlpVertex>,
    v_size: usize,
}

struct GlpVertex {
    i: i32,
    data: Vec<u8>,
    out: Vec<GlpArc>,
}

struct GlpArc {
    tail: *const GlpVertex,
    head: *const GlpVertex,
}

pub fn glp_write_ccdata(graph: &GlpGraph, v_wgt: i32, fname: &str) -> Result<(), Error> {
    if v_wgt >= 0 && v_wgt as usize > graph.v_size - std::mem::size_of::<f64>() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("glp_write_ccdata: v_wgt = {}; invalid offset", v_wgt),
        ));
    }

    println!("Writing graph to '{}'", fname);
    let mut file = File::create(fname)?;
    let mut count = 0;

    writeln!(
        file,
        "c {}",
        graph.name.as_ref().unwrap_or(&"unknown".to_string())
    )?;
    count += 1;

    writeln!(file, "p edge {} {}", graph.nv, graph.na)?;
    count += 1;

    if v_wgt >= 0 {
        for i in 1..=graph.nv {
            let v = &graph.v[i as usize - 1];
            let w = unsafe {
                let ptr = v.data.as_ptr().offset(v_wgt as isize) as *const f64;
                *ptr
            };
            if w != 1.0 {
                writeln!(file, "n {} {:.15}", i, w)?;
                count += 1;
            }
        }
    }

    for i in 1..=graph.nv {
        let v = &graph.v[i as usize - 1];
        for e in &v.out {
            let tail = unsafe { &*e.tail };
            let head = unsafe { &*e.head };
            writeln!(file, "e {} {}", tail.i, head.i)?;
            count += 1;
        }
    }

    writeln!(file, "c eof")?;
    count += 1;

    println!("{} lines were written", count);
    Ok(())
}

pub fn glp_write_graph(graph: &GlpGraph, fname: &str) -> Result<(), Error> {
    glp_write_ccdata(graph, -1, fname)
}