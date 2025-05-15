use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::ptr;

type GlpProb = c_void;
type GlpErrFunc = Option<unsafe extern "C" fn(*const c_char, ...)>;

#[repr(C)]
pub struct GlpGraph {
    pub pool: *mut c_void,
    pub name: *mut c_char,
    pub nv_max: c_int,
    pub nv: c_int,
    pub na: c_int,
    pub v: *mut *mut GlpVertex,
    pub index: *mut c_void,
    pub v_size: c_int,
    pub a_size: c_int,
}

#[repr(C)]
pub struct GlpVertex {
    pub i: c_int,
    pub name: *mut c_char,
    pub entry: *mut c_void,
    pub data: *mut c_void,
    pub temp: *mut c_void,
    pub in_: *mut GlpArc,
    pub out: *mut GlpArc,
}

#[repr(C)]
pub struct GlpArc {
    pub tail: *mut GlpVertex,
    pub head: *mut GlpVertex,
    pub data: *mut c_void,
    pub temp: *mut c_void,
    pub t_prev: *mut GlpArc,
    pub t_next: *mut GlpArc,
    pub h_prev: *mut GlpArc,
    pub h_next: *mut GlpArc,
}

#[link(name = "glpk")]
extern "C" {
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
    fn glp_error_(file: *const c_char, line: c_int) -> GlpErrFunc;
    fn glp_set_prob_name(P: *mut GlpProb, name: *const c_char);
    fn glp_add_rows(P: *mut GlpProb, nrs: c_int) -> c_int;
    fn glp_add_cols(P: *mut GlpProb, ncs: c_int) -> c_int;
    fn glp_set_row_name(P: *mut GlpProb, i: c_int, name: *const c_char);
    fn glp_set_col_name(P: *mut GlpProb, j: c_int, name: *const c_char);
    fn glp_set_row_bnds(P: *mut GlpProb, i: c_int, type_: c_int, lb: c_double, ub: c_double);
    fn glp_set_col_bnds(P: *mut GlpProb, j: c_int, type_: c_int, lb: c_double, ub: c_double);
    fn glp_set_obj_coef(P: *mut GlpProb, j: c_int, coef: c_double);
    fn glp_set_mat_col(P: *mut GlpProb, j: c_int, len: c_int, ind: *const c_int, val: *const c_double);
    fn glp_erase_prob(P: *mut GlpProb);
}

pub fn glp_mincost_lp(
    lp: *mut GlpProb,
    G: *mut GlpGraph,
    names: c_int,
    v_rhs: c_int,
    a_low: c_int,
    a_cap: c_int,
    a_cost: c_int,
) -> Result<(), String> {
    unsafe {
        // Validate input parameters
        if !(names == 0 || names == 1) {
            return Err(format!("glp_mincost_lp: names = {}; invalid parameter", names));
        }

        if v_rhs >= 0 && v_rhs > (*G).v_size - std::mem::size_of::<c_double>() as c_int {
            return Err(format!("glp_mincost_lp: v_rhs = {}; invalid offset", v_rhs));
        }

        if a_low >= 0 && a_low > (*G).a_size - std::mem::size_of::<c_double>() as c_int {
            return Err(format!("glp_mincost_lp: a_low = {}; invalid offset", a_low));
        }

        if a_cap >= 0 && a_cap > (*G).a_size - std::mem::size_of::<c_double>() as c_int {
            return Err(format!("glp_mincost_lp: a_cap = {}; invalid offset", a_cap));
        }

        if a_cost >= 0 && a_cost > (*G).a_size - std::mem::size_of::<c_double>() as c_int {
            return Err(format!("glp_mincost_lp: a_cost = {}; invalid offset", a_cost));
        }

        glp_erase_prob(lp);

        // Set problem name if requested
        if names != 0 && !(*G).name.is_null() {
            glp_set_prob_name(lp, (*G).name);
        }

        // Add rows for vertices
        if (*G).nv > 0 {
            glp_add_rows(lp, (*G).nv);
        }

        // Process vertices
        for i in 1..=(*G).nv {
            let v = *(*G).v.offset(i as isize);
            if names != 0 && !(*v).name.is_null() {
                glp_set_row_name(lp, i, (*v).name);
            }

            let rhs = if v_rhs >= 0 {
                ptr::read((*v).data.cast::<c_char>().offset(v_rhs as isize).cast::<c_double>()
            } else {
                0.0
            };

            glp_set_row_bnds(lp, i, 5, rhs, rhs);
        }

        // Add columns for arcs
        if (*G).na > 0 {
            glp_add_cols(lp, (*G).na);
        }

        let mut j = 0;
        for i in 1..=(*G).nv {
            let mut a = (*(*(*G).v.offset(i as isize))).out;
            while !a.is_null() {
                j += 1;

                // Set column name if requested
                if names != 0 {
                    let name = format!("x[{},{}]", (*(*a).tail).i, (*(*a).head).i);
                    let c_name = CString::new(name).unwrap();
                    glp_set_col_name(lp, j, c_name.as_ptr());
                }

                // Set matrix column for non-self loops
                if (*(*a).tail).i != (*(*a).head).i {
                    let ind = [(*(*a).tail).i, (*(*a).head).i];
                    let val = [1.0, -1.0];
                    glp_set_mat_col(lp, j, 2, ind.as_ptr(), val.as_ptr());
                }

                // Get bounds
                let low = if a_low >= 0 {
                    ptr::read((*a).data.cast::<c_char>().offset(a_low as isize).cast::<c_double>())
                } else {
                    0.0
                };

                let cap = if a_cap >= 0 {
                    ptr::read((*a).data.cast::<c_char>().offset(a_cap as isize).cast::<c_double>())
                } else {
                    1.0
                };

                // Determine bound type
                let type_ = if cap == f64::INFINITY {
                    2
                } else if low != cap {
                    4
                } else {
                    5
                };

                glp_set_col_bnds(lp, j, type_, low, cap);

                // Set objective coefficient
                let cost = if a_cost >= 0 {
                    ptr::read((*a).data.cast::<c_char>().offset(a_cost as isize).cast::<c_double>())
                } else {
                    0.0
                };

                glp_set_obj_coef(lp, j, cost);

                a = (*a).t_next;
            }
        }

        if j != (*G).na {
            return Err("Number of processed arcs doesn't match G->na".to_string());
        }

        Ok(())
    }
}