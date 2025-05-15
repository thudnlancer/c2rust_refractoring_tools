use std::ptr;
use std::slice;
use std::ffi::{CStr, CString};
use std::mem;

#[derive(Debug)]
pub struct GlpGraph {
    pool: *mut libc::c_void,
    name: Option<CString>,
    nv_max: i32,
    nv: i32,
    na: i32,
    vertices: Vec<Option<GlpVertex>>,
    index: *mut libc::c_void,
    v_size: i32,
    a_size: i32,
}

#[derive(Debug)]
pub struct GlpVertex {
    i: i32,
    name: Option<CString>,
    entry: *mut libc::c_void,
    data: *mut libc::c_void,
    temp: *mut libc::c_void,
    in_arcs: Vec<GlpArc>,
    out_arcs: Vec<GlpArc>,
}

#[derive(Debug)]
pub struct GlpArc {
    tail: usize,
    head: usize,
    data: *mut libc::c_void,
    temp: *mut libc::c_void,
}

pub fn glp_top_sort(graph: &mut GlpGraph, v_num: i32) -> i32 {
    if v_num >= 0 && v_num > graph.v_size - mem::size_of::<i32>() as i32 {
        panic!("glp_top_sort: v_num = {}; invalid offset", v_num);
    }

    if graph.nv == 0 {
        return 0;
    }

    let mut num = vec![0; (graph.nv + 1) as usize];
    let cnt = top_sort(graph, &mut num);

    if v_num >= 0 {
        for i in 1..=graph.nv {
            if let Some(vertex) = &mut graph.vertices[i as usize] {
                unsafe {
                    ptr::copy_nonoverlapping(
                        &num[i as usize] as *const i32,
                        (vertex.data as *mut i8).offset(v_num as isize) as *mut i32,
                        1,
                    );
                }
            }
        }
    }

    graph.nv - cnt
}

fn top_sort(graph: &GlpGraph, num: &mut [i32]) -> i32 {
    let mut indeg = vec![0; (graph.nv + 1) as usize];
    let mut stack = Vec::with_capacity(graph.nv as usize);
    let mut top = 0;

    for i in 1..=graph.nv {
        num[i as usize] = 0;
        if let Some(vertex) = &graph.vertices[i as usize] {
            indeg[i as usize] = vertex.in_arcs.len() as i32;
            if indeg[i as usize] == 0 {
                stack.push(i);
                top += 1;
            }
        }
    }

    let mut cnt = 0;
    while top > 0 {
        top -= 1;
        let i = stack[top];
        assert!(indeg[i as usize] == 0, "indeg[i] == 0");
        assert!(num[i as usize] == 0, "num[i] == 0");

        cnt += 1;
        num[i as usize] = cnt;

        if let Some(vertex) = &graph.vertices[i as usize] {
            for arc in &vertex.out_arcs {
                let j = arc.head as i32;
                assert!(indeg[j as usize] > 0, "indeg[j] > 0");
                indeg[j as usize] -= 1;
                if indeg[j as usize] == 0 {
                    stack.push(j);
                    top += 1;
                }
            }
        }
    }

    cnt
}