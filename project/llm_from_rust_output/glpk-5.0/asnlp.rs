use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_double, c_void};
use std::ptr;
use std::mem;

#[repr(C)]
pub struct glp_prob {
    // Opaque type
    _unused: [u8; 0],
}

#[repr(C)]
pub struct glp_graph {
    pub pool: *mut c_void,
    pub name: *mut c_char,
    pub nv_max: c_int,
    pub nv: c_int,
    pub na: c_int,
    pub v: *mut *mut glp_vertex,
    pub index: *mut c_void,
    pub v_size: c_int,
    pub a_size: c_int,
}

#[repr(C)]
pub struct glp_vertex {
    pub i: c_int,
    pub name: *mut c_char,
    pub entry: *mut c_void,
    pub data: *mut c_void,
    pub temp: *mut c_void,
    pub in_: *mut glp_arc,
    pub out: *mut glp_arc,
}

#[repr(C)]
pub struct glp_arc {
    pub tail: *mut glp_vertex,
    pub head: *mut glp_vertex,
    pub data: *mut c_void,
    pub temp: *mut c_void,
    pub t_prev: *mut glp_arc,
    pub t_next: *mut glp_arc,
    pub h_prev: *mut glp_arc,
    pub h_next: *mut glp_arc,
}

type GlpErrFunc = Option<unsafe extern "C" fn(*const c_char, ...)>;

extern "C" {
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
    fn glp_error_(file: *const c_char, line: c_int) -> GlpErrFunc;
    fn glp_set_prob_name(P: *mut glp_prob, name: *const c_char);
    fn glp_set_obj_dir(P: *mut glp_prob, dir: c_int);
    fn glp_add_rows(P: *mut glp_prob, nrs: c_int) -> c_int;
    fn glp_add_cols(P: *mut glp_prob, ncs: c_int) -> c_int;
    fn glp_set_row_name(P: *mut glp_prob, i: c_int, name: *const c_char);
    fn glp_set_col_name(P: *mut glp_prob, j: c_int, name: *const c_char);
    fn glp_set_row_bnds(P: *mut glp_prob, i: c_int, type_: c_int, lb: c_double, ub: c_double);
    fn glp_set_col_bnds(P: *mut glp_prob, j: c_int, type_: c_int, lb: c_double, ub: c_double);
    fn glp_set_obj_coef(P: *mut glp_prob, j: c_int, coef: c_double);
    fn glp_set_mat_col(P: *mut glp_prob, j: c_int, len: c_int, ind: *const c_int, val: *const c_double);
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_check_asnprob(G: *mut glp_graph, v_set: c_int) -> c_int;
}

pub fn glp_asnprob_lp(
    P: *mut glp_prob,
    form: c_int,
    G: *mut glp_graph,
    names: c_int,
    v_set: c_int,
    a_cost: c_int,
) -> c_int {
    unsafe {
        if !(form == 1 || form == 2 || form == 3) {
            let msg = CString::new("glp_asnprob_lp: form = %d; invalid parameter\n").unwrap();
            let file = CString::new("api/asnlp.c").unwrap();
            if let Some(err_func) = glp_error_(file.as_ptr(), 53) {
                err_func(msg.as_ptr(), form);
            }
            return -1;
        }

        if !(names == 0 || names == 1) {
            let msg = CString::new("glp_asnprob_lp: names = %d; invalid parameter\n").unwrap();
            let file = CString::new("api/asnlp.c").unwrap();
            if let Some(err_func) = glp_error_(file.as_ptr(), 56) {
                err_func(msg.as_ptr(), names);
            }
            return -1;
        }

        if v_set >= 0 && v_set > (*G).v_size - mem::size_of::<c_int>() as c_int {
            let msg = CString::new("glp_asnprob_lp: v_set = %d; invalid offset\n").unwrap();
            let file = CString::new("api/asnlp.c").unwrap();
            if let Some(err_func) = glp_error_(file.as_ptr(), 59) {
                err_func(msg.as_ptr(), v_set);
            }
            return -1;
        }

        if a_cost >= 0 && a_cost > (*G).a_size - mem::size_of::<c_double>() as c_int {
            let msg = CString::new("glp_asnprob_lp: a_cost = %d; invalid offset\n").unwrap();
            let file = CString::new("api/asnlp.c").unwrap();
            if let Some(err_func) = glp_error_(file.as_ptr(), 62) {
                err_func(msg.as_ptr(), a_cost);
            }
            return -1;
        }

        let ret = glp_check_asnprob(G, v_set);
        if ret != 0 {
            return ret;
        }

        glp_erase_prob(P);

        if names != 0 && !(*G).name.is_null() {
            glp_set_prob_name(P, (*G).name);
        }

        glp_set_obj_dir(P, if form == 1 { 1 } else { 2 });

        if (*G).nv > 0 {
            glp_add_rows(P, (*G).nv);
        }

        for i in 1..=(*G).nv {
            let v = *(*G).v.offset(i as isize);
            if names != 0 && !(*v).name.is_null() {
                glp_set_row_name(P, i, (*v).name);
            }
            glp_set_row_bnds(
                P,
                i,
                if form == 3 { 3 } else { 5 },
                1.0,
                1.0,
            );
        }

        if (*G).na > 0 {
            glp_add_cols(P, (*G).na);
        }

        let mut j = 0;
        for i in 1..=(*G).nv {
            let mut v = *(*G).v.offset(i as isize);
            let mut a = (*v).out;
            while !a.is_null() {
                j += 1;
                if names != 0 {
                    let name = format!("x[{},{}]", (*(*a).tail).i, (*(*a).head).i);
                    let c_name = CString::new(name).unwrap();
                    glp_set_col_name(P, j, c_name.as_ptr());
                }

                let mut ind = [0; 3];
                let mut val = [0.0; 3];
                ind[1] = (*(*a).tail).i;
                val[1] = 1.0;
                ind[2] = (*(*a).head).i;
                val[2] = 1.0;

                glp_set_mat_col(P, j, 2, ind.as_ptr(), val.as_ptr());
                glp_set_col_bnds(P, j, 4, 0.0, 1.0);

                let cost = if a_cost >= 0 {
                    ptr::read_unaligned(((*a).data as *mut c_char).offset(a_cost as isize) as *const c_double)
                } else {
                    1.0
                };
                glp_set_obj_coef(P, j, cost);

                a = (*a).t_next;
            }
        }

        assert!(j == (*G).na, "j == G->na");

        ret
    }
}