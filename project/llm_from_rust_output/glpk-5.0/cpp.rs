use std::ptr;
use std::mem;

struct GlpGraph {
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

struct GlpVertex {
    i: libc::c_int,
    name: *mut libc::c_char,
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

fn glp_cpp(
    G: &mut GlpGraph,
    v_t: libc::c_int,
    v_es: libc::c_int,
    v_ls: libc::c_int,
) -> libc::c_double {
    let nv = G.nv;
    if nv == 0 {
        return 0.0;
    }

    // Validate offsets
    if v_t >= 0 && v_t > G.v_size - mem::size_of::<libc::c_double>() as libc::c_int {
        panic!("glp_cpp: v_t = {}; invalid offset", v_t);
    }
    if v_es >= 0 && v_es > G.v_size - mem::size_of::<libc::c_double>() as libc::c_int {
        panic!("glp_cpp: v_es = {}; invalid offset", v_es);
    }
    if v_ls >= 0 && v_ls > G.v_size - mem::size_of::<libc::c_double>() as libc::c_int {
        panic!("glp_cpp: v_ls = {}; invalid offset", v_ls);
    }

    // Allocate arrays
    let mut t = vec![0.0; (nv + 1) as usize];
    let mut es = vec![0.0; (nv + 1) as usize];
    let mut ls = vec![0.0; (nv + 1) as usize];
    let mut list = vec![0; (nv + 1) as usize];

    // Initialize t values
    for i in 1..=nv {
        unsafe {
            let v = *G.v.offset(i as isize);
            if v_t >= 0 {
                t[i as usize] = ptr::read((*v).data.cast::<libc::c_char>().offset(v_t as isize).cast::<libc::c_double>();
                if t[i as usize] < 0.0 {
                    panic!("glp_cpp: t[{}] = {}; invalid time", i, t[i as usize]);
                }
            } else {
                t[i as usize] = 1.0;
            }
        }
    }

    // Perform topological sort
    sorting(G, &mut list);

    // Calculate earliest start times
    for k in 1..=nv {
        let j = list[k as usize];
        es[j as usize] = 0.0;
        unsafe {
            let mut a = (*(*G.v.offset(j as isize)).in_0;
            while !a.is_null() {
                let i = (*(*a).tail).i;
                let temp = es[i as usize] + t[i as usize];
                if es[j as usize] < temp {
                    es[j as usize] = temp;
                }
                a = (*a).h_next;
            }
        }
    }

    // Calculate total time
    let mut total = 0.0;
    for i in 1..=nv {
        let temp = es[i as usize] + t[i as usize];
        if total < temp {
            total = temp;
        }
    }

    // Calculate latest start times
    for k in (1..=nv).rev() {
        let i = list[k as usize];
        ls[i as usize] = total - t[i as usize];
        unsafe {
            let mut a = (*(*G.v.offset(i as isize)).out);
            while !a.is_null() {
                let j = (*(*a).head).i;
                let temp = ls[j as usize] - t[i as usize];
                if ls[i as usize] > temp {
                    ls[i as usize] = temp;
                }
                a = (*a).t_next;
            }
        }
        if ls[i as usize] < es[i as usize] {
            ls[i as usize] = es[i as usize];
        }
    }

    // Store results if requested
    if v_es >= 0 {
        for i in 1..=nv {
            unsafe {
                let v = *G.v.offset(i as isize);
                ptr::write(
                    (*v).data.cast::<libc::c_char>().offset(v_es as isize).cast(),
                    es[i as usize],
                );
            }
        }
    }
    if v_ls >= 0 {
        for i in 1..=nv {
            unsafe {
                let v = *G.v.offset(i as isize);
                ptr::write(
                    (*v).data.cast::<libc::c_char>().offset(v_ls as isize).cast(),
                    ls[i as usize],
                );
            }
        }
    }

    total
}

fn sorting(G: &mut GlpGraph, list: &mut [libc::c_int]) {
    let nv = G.nv;
    let v_size = G.v_size;
    let mut save = vec![ptr::null_mut(); (nv + 1) as usize];
    let mut num = vec![0; (nv + 1) as usize];

    // Temporarily modify vertex data pointers
    for i in 1..=nv {
        unsafe {
            let v = *G.v.offset(i as isize);
            save[i as usize] = (*v).data;
            (*v).data = num.as_mut_ptr().offset(i as isize).cast();
            list[i as usize] = 0;
        }
    }

    // Perform topological sort
    G.v_size = mem::size_of::<libc::c_int>() as libc::c_int;
    if unsafe { glp_top_sort(G, 0) } != 0 {
        panic!("glp_cpp: project network is not acyclic");
    }
    G.v_size = v_size;

    // Restore vertex data and build list
    for i in 1..=nv {
        unsafe {
            let v = *G.v.offset(i as isize);
            (*v).data = save[i as usize];
            let k = num[i as usize];
            assert!(1 <= k && k <= nv, "Invalid vertex number");
            assert_eq!(list[k as usize], 0, "Vertex already in list");
            list[k as usize] = i;
        }
    }
}

// External C functions
extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_top_sort(G: *mut GlpGraph, v_num: libc::c_int) -> libc::c_int;
}