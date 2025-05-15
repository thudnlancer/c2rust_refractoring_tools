use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Write, Error, ErrorKind};
use std::ptr;
use std::mem;

struct GlpGraph {
    name: Option<CString>,
    nv: i32,
    na: i32,
    vertices: Vec<GlpVertex>,
}

struct GlpVertex {
    i: i32,
    name: Option<CString>,
    out_arcs: Vec<GlpArc>,
}

struct GlpArc {
    tail: usize,
    head: usize,
    capacity: f64,
}

fn glp_write_maxflow(
    graph: &GlpGraph,
    source: i32,
    sink: i32,
    filename: &str,
) -> Result<(), Error> {
    if source < 1 || source > graph.nv {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("glp_write_maxflow: s = {}; source node number out of range", source),
        );
    }

    if sink < 1 || sink > graph.nv {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("glp_write_maxflow: t = {}: sink node number out of range", sink),
        ));
    }

    println!("Writing maximum flow problem data to '{}'...", filename);

    let mut file = File::create(filename)?;
    let mut count = 0;

    writeln!(file, "c {}", graph.name.as_ref().map_or("unknown", |n| n.to_str().unwrap_or("unknown")))?;
    count += 1;

    writeln!(file, "p max {} {}", graph.nv, graph.na)?;
    count += 1;

    writeln!(file, "n {} s", source)?;
    count += 1;

    writeln!(file, "n {} t", sink)?;
    count += 1;

    for vertex in &graph.vertices {
        for arc in &vertex.out_arcs {
            writeln!(file, "a {} {} {:.15}", vertex.i, graph.vertices[arc.head].i, arc.capacity)?;
            count += 1;
        }
    }

    writeln!(file, "c eof")?;
    count += 1;

    println!("{} lines were written", count);
    Ok(())
}