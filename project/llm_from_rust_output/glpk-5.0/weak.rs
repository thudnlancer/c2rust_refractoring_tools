use std::ptr;
use std::mem;

type GlpErrFunc = Option<unsafe extern "C" fn(*const libc::c_char, ...)>;

struct GlpGraph {
    pool: *mut libc::c_void,
    name: *mut libc::c_char,
    nv_max: i32,
    nv: i32,
    na: i32,
    v: *mut *mut GlpVertex,
    index: *mut libc::c_void,
    v_size: i32,
    a_size: i32,
}

struct GlpVertex {
    i: i32,
    name: *mut libc::c_char,
    entry: *mut libc::c_void,
    data: *mut libc::c_void,
    temp: *mut libc::c_void,
    in_: *mut GlpArc,
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

fn glp_weak_comp(G: &mut GlpGraph, v_num: i32) -> i32 {
    if v_num >= 0 && v_num > G.v_size - mem::size_of::<i32>() as i32 {
        unsafe {
            if let Some(err_func) = glp_error_(
                b"api/weak.c\0".as_ptr() as *const libc::c_char,
                56,
            ) {
                err_func(
                    b"glp_weak_comp: v_num = %d; invalid offset\n\0".as_ptr() as *const libc::c_char,
                    v_num,
                );
            }
        }
    }

    let nv = G.nv;
    if nv == 0 {
        return 0;
    }

    let mut prev = vec![0; (nv + 1) as usize];
    let mut next = vec![0; (nv + 1) as usize];
    let mut list = vec![0; (nv + 1) as usize];

    let mut f = 1;
    for i in 1..=nv {
        prev[i as usize] = i - 1;
        next[i as usize] = i + 1;
    }
    next[nv as usize] = 0;

    let mut nc = 0;
    while f != 0 {
        let i = f;
        f = next[i as usize];
        if f != 0 {
            prev[f as usize] = 0;
        }
        prev[i as usize] = -1;
        nc += 1;
        next[i as usize] = nc;
        list[1] = i;

        let mut pos1 = 1;
        let mut pos2 = 1;
        while pos1 <= pos2 {
            let i = list[pos1 as usize];
            pos1 += 1;

            unsafe {
                let mut a = (*(*G.v.add(i as usize)).in_;
                while !a.is_null() {
                    let j = (*(*a).tail).i;
                    if prev[j as usize] >= 0 {
                        if prev[j as usize] == 0 {
                            f = next[j as usize];
                        } else {
                            next[prev[j as usize] as usize] = next[j as usize];
                        }
                        if next[j as usize] != 0 {
                            prev[next[j as usize] as usize] = prev[j as usize];
                        }
                        prev[j as usize] = -1;
                        next[j as usize] = nc;
                        pos2 += 1;
                        list[pos2 as usize] = j;
                    }
                    a = (*a).h_next;
                }

                let mut a = (*(*G.v.add(i as usize)).out;
                while !a.is_null() {
                    let j = (*(*a).head).i;
                    if prev[j as usize] >= 0 {
                        if prev[j as usize] == 0 {
                            f = next[j as usize];
                        } else {
                            next[prev[j as usize] as usize] = next[j as usize];
                        }
                        if next[j as usize] != 0 {
                            prev[next[j as usize] as usize] = prev[j as usize];
                        }
                        prev[j as usize] = -1;
                        next[j as usize] = nc;
                        pos2 += 1;
                        list[pos2 as usize] = j;
                    }
                    a = (*a).t_next;
                }
            }
        }
    }

    if v_num >= 0 {
        for i in 1..=nv {
            unsafe {
                let v = *G.v.add(i as usize);
                ptr::copy_nonoverlapping(
                    &next[i as usize] as *const i32,
                    ((*v).data as *mut i32).add(v_num as usize),
                    1,
                );
            }
        }
    }

    nc
}

extern "C" {
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_error_(file: *const libc::c_char, line: i32) -> GlpErrFunc;
}