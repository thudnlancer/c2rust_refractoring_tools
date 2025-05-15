use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Write, Error};
use std::ptr;
use std::mem;

#[repr(C)]
pub struct GlpGraph {
    pool: *mut libc::c_void,
    name: *mut libc::c_char,
    nv_max: libc::c_int,
    nv: libc::c_int,
    na: libc::c_int,
    v: *mut *mut GlpVertex,
    index: *mut libc::c_void,
    v_size: libc::c_int,
    a_size: libc::c_int,
}

#[repr(C)]
pub struct GlpVertex {
    i: libc::c_int,
    name: *mut libc::c_char,
    entry: *mut libc::c_void,
    data: *mut libc::c_void,
    temp: *mut libc::c_void,
    in_: *mut GlpArc,
    out: *mut GlpArc,
}

#[repr(C)]
pub struct GlpArc {
    tail: *mut GlpVertex,
    head: *mut GlpVertex,
    data: *mut libc::c_void,
    temp: *mut libc::c_void,
    t_prev: *mut GlpArc,
    t_next: *mut GlpArc,
    h_prev: *mut GlpArc,
    h_next: *mut GlpArc,
}

pub fn glp_write_ccdata(
    graph: &GlpGraph,
    v_wgt: libc::c_int,
    fname: &str,
) -> Result<(), Error> {
    if v_wgt >= 0 && v_wgt > graph.v_size - mem::size_of::<libc::c_double>() as libc::c_int {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("glp_write_ccdata: v_wgt = {}; invalid offset", v_wgt),
        ));
    }

    println!("Writing graph to '{}'", fname);
    let mut file = File::create(fname)?;
    let mut count = 0;

    let graph_name = if graph.name.is_null() {
        "unknown"
    } else {
        unsafe { CStr::from_ptr(graph.name).to_str().unwrap_or("unknown") }
    };
    writeln!(file, "c {}", graph_name)?;
    count += 1;

    writeln!(file, "p edge {} {}", graph.nv, graph.na)?;
    count += 1;

    if v_wgt >= 0 {
        for i in 1..=graph.nv {
            let v = unsafe { *graph.v.offset(i as isize) };
            let w = unsafe {
                let mut w = 0.0;
                ptr::copy_nonoverlapping(
                    (v.data as *mut libc::c_char).offset(v_wgt as isize) as *const libc::c_void,
                    &mut w as *mut libc::c_double as *mut libc::c_void,
                    mem::size_of::<libc::c_double>(),
                );
                w
            };
            if w != 1.0 {
                writeln!(file, "n {} {:.15}", i, w)?;
                count += 1;
            }
        }
    }

    for i in 1..=graph.nv {
        let v = unsafe { *graph.v.offset(i as isize) };
        let mut e = v.out;
        while !e.is_null() {
            let arc = unsafe { &*e };
            writeln!(file, "e {} {}", unsafe { (*arc.tail).i }, unsafe { (*arc.head).i })?;
            count += 1;
            e = arc.t_next;
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