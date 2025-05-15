use std::mem;
use std::ptr;
use std::cmp::Ordering;
use std::f64;

#[derive(Debug, Clone)]
pub struct GlpGraph {
    pub pool: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub nv_max: i32,
    pub nv: i32,
    pub na: i32,
    pub v: *mut *mut GlpVertex,
    pub index: *mut libc::c_void,
    pub v_size: i32,
    pub a_size: i32,
}

#[derive(Debug, Clone)]
pub struct GlpVertex {
    pub i: i32,
    pub name: *mut libc::c_char,
    pub entry: *mut libc::c_void,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub in_0: *mut GlpArc,
    pub out: *mut GlpArc,
}

#[derive(Debug, Clone)]
pub struct GlpArc {
    pub tail: *mut GlpVertex,
    pub head: *mut GlpVertex,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub t_prev: *mut GlpArc,
    pub t_next: *mut GlpArc,
    pub h_prev: *mut GlpArc,
    pub h_next: *mut GlpArc,
}

fn set_edge(nv: i32, a: &mut [u8], i: i32, j: i32) {
    assert!(1 <= j && j < i && i <= nv);
    let k = (i - 1) * (i - 2) / 2 + (j - 1);
    let byte = (k / 8) as usize;
    let bit = 7 - (k % 8);
    a[byte] |= 1 << bit;
}

pub fn glp_wclique_exact(
    graph: &mut GlpGraph,
    v_wgt: i32,
    sol: Option<&mut f64>,
    v_set: i32,
) -> i32 {
    if v_wgt >= 0 && v_wgt > graph.v_size - mem::size_of::<f64>() as i32 {
        panic!("glp_wclique_exact: v_wgt = {}; invalid parameter", v_wgt);
    }
    if v_set >= 0 && v_set > graph.v_size - mem::size_of::<i32>() as i32 {
        panic!("glp_wclique_exact: v_set = {}; invalid parameter", v_set);
    }

    if graph.nv == 0 {
        if let Some(s) = sol {
            *s = 0.0;
        }
        return 0;
    }

    let mut w = vec![0; (graph.nv + 1) as usize];
    let mut ind = vec![0; (graph.nv + 1) as usize];
    let mut len = graph.nv * (graph.nv - 1) / 2;
    len = (len + 7) / 8;
    let mut a = vec![0u8; len as usize];

    let mut s = 0.0;
    let mut ret = 0;

    for i in 1..=graph.nv {
        let weight = if v_wgt >= 0 {
            let mut t = 0.0;
            unsafe {
                ptr::copy_nonoverlapping(
                    (*graph.v.add(i as usize)).data.cast::<u8>().add(v_wgt as usize),
                    &mut t as *mut f64 as *mut u8,
                    mem::size_of::<f64>(),
                );
            }
            if !(0.0 <= t && t <= i32::MAX as f64 && t == t.floor()) {
                ret = 0x12;
                break;
            }
            t as i32
        } else {
            1
        };
        w[i as usize] = weight;
        s += weight as f64;
    }

    if ret != 0 {
        return ret;
    }

    if s > i32::MAX as f64 {
        ret = 0x12;
    } else {
        for i in 1..=graph.nv {
            unsafe {
                let mut e = (*graph.v.add(i as usize)).in_0;
                while !e.is_null() {
                    let j = (*(*e).tail).i;
                    if i > j {
                        set_edge(graph.nv, &mut a, i, j);
                    }
                    e = (*e).h_next;
                }

                e = (*graph.v.add(i as usize)).out;
                while !e.is_null() {
                    let j = (*(*e).head).i;
                    if i > j {
                        set_edge(graph.nv, &mut a, i, j);
                    }
                    e = (*e).t_next;
                }
            }
        }

        let len = unsafe {
            _glp_wclique(
                graph.nv,
                w.as_ptr(),
                a.as_ptr(),
                ind.as_mut_ptr(),
            )
        };

        s = 0.0;
        for k in 1..=len {
            let i = ind[k as usize];
            assert!(1 <= i && i <= graph.nv);
            s += w[i as usize] as f64;
        }

        if let Some(sol_ref) = sol {
            *sol_ref = s;
        }

        if v_set >= 0 {
            let mut x = 0;
            for i in 1..=graph.nv {
                unsafe {
                    ptr::copy_nonoverlapping(
                        &x as *const i32 as *const u8,
                        (*graph.v.add(i as usize)).data.cast::<u8>().add(v_set as usize),
                        mem::size_of::<i32>(),
                    );
                }
            }

            x = 1;
            for k in 1..=len {
                let i = ind[k as usize];
                unsafe {
                    ptr::copy_nonoverlapping(
                        &x as *const i32 as *const u8,
                        (*graph.v.add(i as usize)).data.cast::<u8>().add(v_set as usize),
                        mem::size_of::<i32>(),
                    );
                }
            }
        }
    }

    ret
}

extern "C" {
    fn _glp_wclique(
        n: i32,
        w: *const i32,
        a: *const u8,
        ind: *mut i32,
    ) -> i32;
}