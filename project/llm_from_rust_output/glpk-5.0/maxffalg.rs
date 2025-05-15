use std::ffi::{CString, CStr};
use std::os::raw::{c_int, c_double, c_char, c_void};
use std::ptr;
use std::mem;

#[repr(C)]
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

#[repr(C)]
pub struct GlpVertex {
    i: c_int,
    name: *mut c_char,
    entry: *mut c_void,
    data: *mut c_void,
    temp: *mut c_void,
    in_: *mut GlpArc,
    out: *mut GlpArc,
}

#[repr(C)]
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

type GlpErrFunc = Option<unsafe extern "C" fn(*const c_char, ...)>;

extern "C" {
    fn floor(_: c_double) -> c_double;
    fn glp_free(ptr: *mut c_void);
    fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
    fn glp_error_(file: *const c_char, line: c_int) -> GlpErrFunc;
    fn _glp_ffalg(
        nv: c_int,
        na: c_int,
        tail: *const c_int,
        head: *const c_int,
        s: c_int,
        t: c_int,
        cap: *const c_int,
        x: *mut c_int,
        cut: *mut c_char,
    );
}

#[no_mangle]
pub extern "C" fn glp_maxflow_ffalg(
    G: *mut GlpGraph,
    s: c_int,
    t: c_int,
    a_cap: c_int,
    sol: *mut c_double,
    a_x: c_int,
    v_cut: c_int,
) -> c_int {
    unsafe {
        // Validate input parameters
        if !(1 <= s && s <= (*G).nv) {
            let msg = CString::new("glp_maxflow_ffalg: s = %d; source node number out of range\n")
                .unwrap();
            (glp_error_(
                CString::new("api/maxffalg.c").unwrap().as_ptr(),
                35,
            ))
            .unwrap()(msg.as_ptr(), s);
        }

        if !(1 <= t && t <= (*G).nv) {
            let msg = CString::new("glp_maxflow_ffalg: t = %d: sink node number out of range\n")
                .unwrap();
            (glp_error_(
                CString::new("api/maxffalg.c").unwrap().as_ptr(),
                38,
            ))
            .unwrap()(msg.as_ptr(), t);
        }

        if s == t {
            let msg = CString::new(
                "glp_maxflow_ffalg: s = t = %d; source and sink nodes must be distinct\n",
            )
            .unwrap();
            (glp_error_(
                CString::new("api/maxffalg.c").unwrap().as_ptr(),
                41,
            ))
            .unwrap()(msg.as_ptr(), s);
        }

        if a_cap >= 0 && a_cap > (*G).a_size - mem::size_of::<c_double>() as c_int {
            let msg = CString::new("glp_maxflow_ffalg: a_cap = %d; invalid offset\n").unwrap();
            (glp_error_(
                CString::new("api/maxffalg.c").unwrap().as_ptr(),
                44,
            ))
            .unwrap()(msg.as_ptr(), a_cap);
        }

        if v_cut >= 0 && v_cut > (*G).v_size - mem::size_of::<c_int>() as c_int {
            let msg = CString::new("glp_maxflow_ffalg: v_cut = %d; invalid offset\n").unwrap();
            (glp_error_(
                CString::new("api/maxffalg.c").unwrap().as_ptr(),
                47,
            ))
            .unwrap()(msg.as_ptr(), v_cut);
        }

        let nv = (*G).nv;
        let na = (*G).na;

        // Allocate arrays
        let tail = glp_alloc(1 + na, mem::size_of::<c_int>() as c_int) as *mut c_int;
        let head = glp_alloc(1 + na, mem::size_of::<c_int>() as c_int) as *mut c_int;
        let cap = glp_alloc(1 + na, mem::size_of::<c_int>() as c_int) as *mut c_int;
        let x = glp_alloc(1 + na, mem::size_of::<c_int>() as c_int) as *mut c_int;
        let cut = if v_cut < 0 {
            ptr::null_mut()
        } else {
            glp_alloc(1 + nv, mem::size_of::<c_char>() as c_int) as *mut c_char
        };

        // Process arcs
        let mut k = 0;
        let mut ret = 0;
        for i in 1..=(*G).nv {
            let v = *(*G).v.offset(i as isize);
            let mut a = (*v).out;
            while !a.is_null() {
                k += 1;
                *tail.offset(k as isize) = (*(*a).tail).i;
                *head.offset(k as isize) = (*(*a).head).i;

                if *tail.offset(k as isize) == *head.offset(k as isize) {
                    ret = 0x12;
                    break;
                }

                let mut temp = if a_cap >= 0 {
                    let mut val = 0.0;
                    ptr::copy_nonoverlapping(
                        ((*a).data as *mut c_char).offset(a_cap as isize) as *const c_double,
                        &mut val as *mut c_double,
                        1,
                    );
                    val
                } else {
                    1.0
                };

                if !(0.0 <= temp && temp <= i32::MAX as f64 && temp == floor(temp)) {
                    ret = 0x12;
                    break;
                }

                *cap.offset(k as isize) = temp as c_int;
                a = (*a).t_next;
            }
            if ret != 0 {
                break;
            }
        }

        if ret == 0 {
            assert!(k == na, "k == na");
            _glp_ffalg(nv, na, tail, head, s, t, cap, x, cut);

            // Calculate solution
            if !sol.is_null() {
                let mut temp = 0.0;
                for k in 1..=na {
                    if *tail.offset(k as isize) == s {
                        temp += *x.offset(k as isize) as f64;
                    } else if *head.offset(k as isize) == s {
                        temp -= *x.offset(k as isize) as f64;
                    }
                }
                *sol = temp;
            }

            // Store flow values
            if a_x >= 0 {
                let mut k = 0;
                for i in 1..=(*G).nv {
                    let v = *(*G).v.offset(i as isize);
                    let mut a = (*v).out;
                    while !a.is_null() {
                        k += 1;
                        let temp = *x.offset(k as isize) as f64;
                        ptr::copy_nonoverlapping(
                            &temp as *const c_double,
                            ((*a).data as *mut c_char).offset(a_x as isize)) as *mut c_double,
                            1,
                        );
                        a = (*a).t_next;
                    }
                }
            }

            // Store cut flags
            if v_cut >= 0 {
                for i in 1..=(*G).nv {
                    let v = *(*G).v.offset(i as isize);
                    let flag = *cut.offset(i as isize) as c_int;
                    ptr::copy_nonoverlapping(
                        &flag as *const c_int,
                        ((*v).data as *mut c_char).offset(v_cut as isize)) as *mut c_int,
                        1,
                    );
                }
            }
        }

        // Free allocated memory
        glp_free(tail as *mut c_void);
        glp_free(head as *mut c_void);
        glp_free(cap as *mut c_void);
        glp_free(x as *mut c_void);
        if !cut.is_null() {
            glp_free(cut as *mut c_void);
        }

        ret
    }
}