use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_double, c_void, c_char};
use std::ptr;
use std::mem;
use std::cmp::Ordering;
use std::f64;

#[derive(Debug, Clone)]
pub struct GlpGraph {
    pool: *mut c_void,
    name: *mut c_char,
    nv_max: c_int,
    nv: c_int,
    na: c_int,
    v: *mut *mut GlpVertex,
    index: *mut c_void,
    v_size: c_int,
    a_size: c_int,
}

#[derive(Debug, Clone)]
pub struct GlpVertex {
    i: c_int,
    name: *mut c_char,
    entry: *mut c_void,
    data: *mut c_void,
    temp: *mut c_void,
    in_0: *mut GlpArc,
    out: *mut GlpArc,
}

#[derive(Debug, Clone)]
pub struct GlpArc {
    tail: *mut GlpVertex,
    head: *mut GlpVertex,
    data: *mut c_void,
    temp: *mut c_void,
    t_prev: *mut GlpArc,
    t_next: *mut GlpArc,
    h_prev: *mut GlpArc,
    h_next: *mut GlpArc,
}

type GlpErrFunc = Option<unsafe extern "C" fn(*const c_char, ...) -> ()>;

pub fn glp_asnprob_okalg(
    form: c_int,
    g: &mut GlpGraph,
    v_set: c_int,
    a_cost: c_int,
    sol: Option<&mut c_double>,
    a_x: c_int,
) -> c_int {
    // Validate input parameters
    if ![1, 2, 3].contains(&form) {
        unsafe {
            let msg = CString::new("glp_asnprob_okalg: form = %d; invalid parameter\n").unwrap();
            let file = CString::new("api/asnokalg.c").unwrap();
            let err_func = glp_error_(file.as_ptr(), 35);
            err_func.unwrap()(msg.as_ptr(), form);
        }
        return 0;
    }

    if v_set >= 0 && v_set > g.v_size - mem::size_of::<c_int>() as c_int {
        unsafe {
            let msg = CString::new("glp_asnprob_okalg: v_set = %d; invalid offset\n").unwrap();
            let file = CString::new("api/asnokalg.c").unwrap();
            let err_func = glp_error_(file.as_ptr(), 38);
            err_func.unwrap()(msg.as_ptr(), v_set);
        }
        return 0;
    }

    if a_cost >= 0 && a_cost > g.a_size - mem::size_of::<c_double>() as c_int {
        unsafe {
            let msg = CString::new("glp_asnprob_okalg: a_cost = %d; invalid offset\n").unwrap();
            let file = CString::new("api/asnokalg.c").unwrap();
            let err_func = glp_error_(file.as_ptr(), 41);
            err_func.unwrap()(msg.as_ptr(), a_cost);
        }
        return 0;
    }

    if a_x >= 0 && a_x > g.a_size - mem::size_of::<c_int>() as c_int {
        unsafe {
            let msg = CString::new("glp_asnprob_okalg: a_x = %d; invalid offset\n").unwrap();
            let file = CString::new("api/asnokalg.c").unwrap();
            let err_func = glp_error_(file.as_ptr(), 44);
            err_func.unwrap()(msg.as_ptr(), a_x);
        }
        return 0;
    }

    if unsafe { glp_check_asnprob(g as *mut GlpGraph, v_set) } != 0 {
        return 0x12;
    }

    let nv = g.nv + 1;
    let na = g.na + g.nv;

    // Allocate arrays
    let mut tail = vec![0; (na + 1) as usize];
    let mut head = vec![0; (na + 1) as usize];
    let mut low = vec![0; (na + 1) as usize];
    let mut cap = vec![0; (na + 1) as usize];
    let mut cost = vec![0; (na + 1) as usize];
    let mut x = vec![0; (na + 1) as usize];
    let mut pi = vec![0; (nv + 1) as usize];

    let mut ret = 0;
    let mut k = 0;

    // Process vertices and arcs
    for i in 1..=g.nv {
        unsafe {
            let v = *g.v.offset(i as isize);
            let mut a = (*v).out;

            while !a.is_null() {
                k += 1;
                tail[k as usize] = (*(*a).tail).i;
                head[k as usize] = (*(*a).head).i;
                low[k as usize] = 0;
                cap[k as usize] = 1;

                let mut temp = if a_cost >= 0 {
                    let cost_ptr = ((*a).data as *mut c_char).offset(a_cost as isize);
                    ptr::read(cost_ptr as *const c_double)
                } else {
                    1.0
                };

                if temp.abs() > i32::MAX as f64 || temp != temp.floor() {
                    ret = 0x12;
                    break;
                }

                cost[k as usize] = temp as c_int;
                if form != 1 {
                    cost[k as usize] = -cost[k as usize];
                }

                a = (*a).t_next;
            }
        }
    }

    if ret != 0 {
        return ret;
    }

    // Process additional edges
    for i in 1..=g.nv {
        unsafe {
            let v = *g.v.offset(i as isize);
            k += 1;

            if (*v).out.is_null() {
                tail[k as usize] = i;
                head[k as usize] = nv;
            } else if (*v).in_0.is_null() {
                tail[k as usize] = nv;
                head[k as usize] = i;
            } else {
                assert!(v != v, "v != v");
            }

            low[k as usize] = if form == 3 { 0 } else { 1 };
            cap[k as usize] = 1;
            cost[k as usize] = 0;
        }
    }

    assert_eq!(k, na, "k == na");

    // Call the algorithm
    ret = unsafe {
        _glp_okalg(
            nv,
            na,
            tail.as_ptr(),
            head.as_ptr(),
            low.as_ptr(),
            cap.as_ptr(),
            cost.as_ptr(),
            x.as_mut_ptr(),
            pi.as_mut_ptr(),
        )
    };

    match ret {
        0 => ret = 0,
        1 => ret = 0xA,
        2 => ret = 0x13,
        3 => ret = 0x5,
        _ => assert!(false, "invalid return value"),
    }

    if ret == 0 || ret == 0xA {
        if let Some(sol_ref) = sol {
            let mut temp = 0.0;
            for k in 1..=na {
                temp += cost[k as usize] as f64 * x[k as usize] as f64;
            }
            *sol_ref = if form != 1 { -temp } else { temp };
        }

        if a_x >= 0 {
            let mut k = 0;
            for i in 1..=g.nv {
                unsafe {
                    let v = *g.v.offset(i as isize);
                    let mut a = (*v).out;

                    while !a.is_null() {
                        k += 1;
                        if ret == 0 {
                            assert!(
                                x[k as usize] == 0 || x[k as usize] == 1,
                                "x[k] should be 0 or 1"
                            );
                        }
                        ptr::write(
                            ((*a).data as *mut c_char).offset(a_x as isize) as *mut c_int,
                            x[k as usize],
                        );
                        a = (*a).t_next;
                    }
                }
            }
        }
    }

    ret
}

// External C functions (would be linked from the GLPK library)
extern "C" {
    fn glp_check_asnprob(G: *mut GlpGraph, v_set: c_int) -> c_int;
    fn _glp_okalg(
        nv: c_int,
        na: c_int,
        tail: *const c_int,
        head: *const c_int,
        low: *const c_int,
        cap: *const c_int,
        cost: *const c_int,
        x: *mut c_int,
        pi: *mut c_int,
    ) -> c_int;
    fn glp_error_(file: *const c_char, line: c_int) -> GlpErrFunc;
}