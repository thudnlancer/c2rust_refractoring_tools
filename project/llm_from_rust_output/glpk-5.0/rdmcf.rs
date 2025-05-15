use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::ptr;

struct DMX {
    fname: String,
    fp: Option<File>,
    count: c_int,
    c: c_int,
    field: [c_char; 256],
    empty: c_int,
    nonint: c_int,
}

struct GlpGraph {
    name: Option<CString>,
    nv_max: c_int,
    nv: c_int,
    na: c_int,
    v: Vec<*mut GlpVertex>,
    v_size: c_int,
    a_size: c_int,
}

struct GlpVertex {
    i: c_int,
    name: Option<CString>,
    data: *mut c_void,
    in_arcs: Vec<*mut GlpArc>,
    out_arcs: Vec<*mut GlpArc>,
}

struct GlpArc {
    tail: *mut GlpVertex,
    head: *mut GlpVertex,
    data: *mut c_void,
}

fn glp_read_mincost(
    graph: &mut GlpGraph,
    v_rhs: c_int,
    a_low: c_int,
    a_cap: c_int,
    a_cost: c_int,
    fname: &str,
) -> c_int {
    let mut csa = DMX {
        fname: fname.to_string(),
        fp: None,
        count: 0,
        c: b'\n' as c_int,
        field: [0; 256],
        empty: 0,
        nonint: 0,
    };

    let mut ret = 0;
    let mut nv = 0;
    let mut na = 0;

    // Validate offsets
    if v_rhs >= 0 && v_rhs > graph.v_size - mem::size_of::<c_double>() as c_int {
        eprintln!("glp_read_mincost: v_rhs = {}; invalid offset", v_rhs);
        return 1;
    }

    if a_low >= 0 && a_low > graph.a_size - mem::size_of::<c_double>() as c_int {
        eprintln!("glp_read_mincost: a_low = {}; invalid offset", a_low);
        return 1;
    }

    if a_cap >= 0 && a_cap > graph.a_size - mem::size_of::<c_double>() as c_int {
        eprintln!("glp_read_mincost: a_cap = {}; invalid offset", a_cap);
        return 1;
    }

    if a_cost >= 0 && a_cost > graph.a_size - mem::size_of::<c_double>() as c_int {
        eprintln!("glp_read_mincost: a_cost = {}; invalid offset", a_cost);
        return 1;
    }

    // Clear existing graph
    graph.v.clear();
    graph.nv = 0;
    graph.na = 0;

    // Open file
    let file = match File::open(&fname) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Unable to open '{}' - {}", fname, e);
            return 1;
        }
    };
    csa.fp = Some(file);
    let reader = BufReader::new(csa.fp.as_ref().unwrap());

    println!("Reading min-cost flow problem data from '{}'...", fname);

    // Read problem line
    let mut lines = reader.lines();
    let first_line = match lines.next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => {
            eprintln!("Error reading file: {}", e);
            return 1;
        }
        None => {
            eprintln!("Empty file");
            return 1;
        }
    };
    csa.count += 1;

    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 4 || parts[0] != "p" {
        eprintln!("Problem line missing or invalid");
        return 1;
    }

    if parts[1] != "min" {
        eprintln!("Wrong problem designator; 'min' expected");
        return 1;
    }

    nv = parts[2].parse().unwrap_or(0);
    na = parts[3].parse().unwrap_or(0);

    println!(
        "Flow network has {} node{} and {} arc{}",
        nv,
        if nv == 1 { "" } else { "s" },
        na,
        if na == 1 { "" } else { "s" }
    );

    // Add vertices
    if nv > 0 {
        graph.v = vec![ptr::null_mut(); nv as usize + 1];
        graph.nv = nv;
    }

    // Read node descriptors
    let mut flag = vec![0; nv as usize + 1];
    if v_rhs >= 0 {
        for i in 1..=nv {
            let v = &mut graph.v[i as usize];
            unsafe {
                if !v.is_null() {
                    ptr::write((*v).data.offset(v_rhs as isize) as *mut c_double, 0.0);
                }
            }
        }
    }

    // Process remaining lines
    for line in lines {
        let line = line.unwrap();
        csa.count += 1;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "n" => {
                // Process node descriptor
                if parts.len() < 3 {
                    eprintln!("Invalid node descriptor");
                    return 1;
                }
                let i: c_int = parts[1].parse().unwrap_or(0);
                if i < 1 || i > nv {
                    eprintln!("Node number {} out of range", i);
                    return 1;
                }
                if flag[i as usize] != 0 {
                    eprintln!("Duplicate descriptor of node {}", i);
                    return 1;
                }
                let rhs: c_double = parts[2].parse().unwrap_or(0.0);
                if v_rhs >= 0 {
                    unsafe {
                        ptr::write(
                            (*graph.v[i as usize]).data.offset(v_rhs as isize) as *mut c_double,
                            rhs,
                        );
                    }
                }
                flag[i as usize] = 1;
            }
            "a" => {
                // Process arc descriptor
                if parts.len() < 6 {
                    eprintln!("Invalid arc descriptor");
                    return 1;
                }
                let i: c_int = parts[1].parse().unwrap_or(0);
                let j: c_int = parts[2].parse().unwrap_or(0);
                if i < 1 || i > nv || j < 1 || j > nv {
                    eprintln!("Node number out of range");
                    return 1;
                }
                let low: c_double = parts[3].parse().unwrap_or(0.0);
                let cap: c_double = parts[4].parse().unwrap_or(0.0);
                let cost: c_double = parts[5].parse().unwrap_or(0.0);

                // Add arc
                let arc = Box::new(GlpArc {
                    tail: graph.v[i as usize],
                    head: graph.v[j as usize],
                    data: ptr::null_mut(),
                });
                let arc_ptr = Box::into_raw(arc);

                // Store arc data
                if a_low >= 0 {
                    unsafe {
                        ptr::write(
                            (*arc_ptr).data.offset(a_low as isize) as *mut c_double,
                            low,
                        );
                    }
                }
                if a_cap >= 0 {
                    unsafe {
                        ptr::write(
                            (*arc_ptr).data.offset(a_cap as isize) as *mut c_double,
                            cap,
                        );
                    }
                }
                if a_cost >= 0 {
                    unsafe {
                        ptr::write(
                            (*arc_ptr).data.offset(a_cost as isize) as *mut c_double,
                            cost,
                        );
                    }
                }

                // Add to graph
                graph.na += 1;
            }
            _ => {
                eprintln!("Unknown line type: {}", parts[0]);
                return 1;
            }
        }
    }

    println!("{} lines were read", csa.count);

    if ret != 0 {
        // Clear graph on error
        graph.v.clear();
        graph.nv = 0;
        graph.na = 0;
    }

    ret
}