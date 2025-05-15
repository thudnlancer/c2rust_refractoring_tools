use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Write, Error};
use std::mem;
use std::ptr;

struct GlpGraph {
    pool: *mut libc::c_void,
    name: Option<CString>,
    nv_max: i32,
    nv: i32,
    na: i32,
    v: Vec<*mut GlpVertex>,
    index: *mut libc::c_void,
    v_size: i32,
    a_size: i32,
}

struct GlpVertex {
    i: i32,
    name: Option<CString>,
    entry: *mut libc::c_void,
    data: *mut libc::c_void,
    temp: *mut libc::c_void,
    in_0: *mut GlpArc,
    out: *mut GlpArc,
}

struct GlpArc {
    tail: *mut GlpVertex,
    head: *mut GlpVertex,
    data: *mut libc::c_void,
    temp: *mut libc::c_void,
    t_prev: *mut GlpArc,
    t_next: *mut GlpArc,
    h_prev: *mut GlpArc,
    h_next: *mut GlpArc,
}

fn glp_write_mincost(
    graph: &GlpGraph,
    v_rhs: i32,
    a_low: i32,
    a_cap: i32,
    a_cost: i32,
    fname: &str,
) -> Result<(), Error> {
    let mut count = 0;
    let mut file = File::create(fname)?;

    writeln!(file, "Writing min-cost flow problem data to '{}'...", fname)?;

    let name = graph.name.as_ref().map(|n| n.to_str().unwrap()).unwrap_or("unknown");
    writeln!(file, "c {}", name)?;
    count += 1;

    writeln!(file, "p min {} {}", graph.nv, graph.na)?;
    count += 1;

    if v_rhs >= 0 {
        for i in 1..=graph.nv {
            let v = unsafe { *graph.v.get_unchecked(i as usize) };
            let rhs = unsafe { ptr::read((*v).data.add(v_rhs as usize) as f64 };
            if rhs != 0.0 {
                writeln!(file, "n {} {:.15}", i, rhs)?;
                count += 1;
            }
        }
    }

    for i in 1..=graph.nv {
        let v = unsafe { *graph.v.get_unchecked(i as usize) };
        let mut a = (*v).out;
        while !a.is_null() {
            let low = if a_low >= 0 {
                unsafe { ptr::read((*a).data.add(a_low as usize) as *const f64) }
            } else {
                0.0
            };

            let cap = if a_cap >= 0 {
                unsafe { ptr::read((*a).data.add(a_cap as usize) as f64 }
            } else {
                1.0
            };

            let cost = if a_cost >= 0 {
                unsafe { ptr::read((*a).data.add(a_cost as usize) as f64) }
            } else {
                0.0
            };

            let tail = unsafe { (*(*a).tail).i };
            let head = unsafe { (*(*a).head).i };
            writeln!(file, "a {} {} {:.15} {:.15} {:.15}", tail, head, low, cap, cost)?;
            count += 1;

            a = unsafe { (*a).t_next };
        }
    }

    writeln!(file, "c eof")?;
    count += 1;

    writeln!(file, "{} lines were written", count)?;
    Ok(())
}