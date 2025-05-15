use std::cmp::Ordering;
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int, c_uchar, c_void};
use std::ptr;

type SizeT = usize;
type GlpErrFunc = Option<unsafe extern "C" fn(*const c_char, ...)>;

#[repr(C)]
pub struct GlpProb {
    pool: *mut c_void,
    tree: *mut c_void,
    name: *mut c_char,
    obj: *mut c_char,
    dir: c_int,
    c0: c_double,
    m_max: c_int,
    n_max: c_int,
    m: c_int,
    n: c_int,
    nnz: c_int,
    row: *mut *mut GlpRow,
    col: *mut *mut GlpCol,
    r_tree: *mut c_void,
    c_tree: *mut c_void,
    valid: c_int,
    head: *mut c_int,
    bfd: *mut c_void,
    pbs_stat: c_int,
    dbs_stat: c_int,
    obj_val: c_double,
    it_cnt: c_int,
    some: c_int,
    ipt_stat: c_int,
    ipt_obj: c_double,
    mip_stat: c_int,
    mip_obj: c_double,
}

#[repr(C)]
pub struct GlpCol {
    j: c_int,
    name: *mut c_char,
    node: *mut c_void,
    kind: c_int,
    type_: c_int,
    lb: c_double,
    ub: c_double,
    coef: c_double,
    ptr: *mut GlpAij,
    sjj: c_double,
    stat: c_int,
    bind: c_int,
    prim: c_double,
    dual: c_double,
    pval: c_double,
    dval: c_double,
    mipx: c_double,
}

#[repr(C)]
pub struct GlpAij {
    row: *mut GlpRow,
    col: *mut GlpCol,
    val: c_double,
    r_prev: *mut GlpAij,
    r_next: *mut GlpAij,
    c_prev: *mut GlpAij,
    c_next: *mut GlpAij,
}

#[repr(C)]
pub struct GlpRow {
    i: c_int,
    name: *mut c_char,
    node: *mut c_void,
    level: c_int,
    origin: c_uchar,
    klass: c_uchar,
    type_: c_int,
    lb: c_double,
    ub: c_double,
    ptr: *mut GlpAij,
    rii: c_double,
    stat: c_int,
    bind: c_int,
    prim: c_double,
    dual: c_double,
    pval: c_double,
    dval: c_double,
    mipx: c_double,
}

#[repr(C)]
struct Var {
    j: c_int,
    f: c_double,
}

fn fcmp(p1: &Var, p2: &Var) -> Ordering {
    p1.f.partial_cmp(&p2.f).unwrap_or(Ordering::Equal)
}

pub fn glp_gmi_gen(p: &mut GlpProb, pool: &mut GlpProb, max_cuts: c_int) -> c_int {
    let m = p.m;
    let n = p.n;

    // Validate inputs
    if p.m == 0 || p.valid == 0 {
        unsafe {
            let msg = CString::new("glp_gmi_gen: basis factorization does not exist\n").unwrap();
            let file = CString::new("intopt/gmigen.c").unwrap();
            let err_func = glp_error_(file.as_ptr(), 77);
            err_func.unwrap()(msg.as_ptr());
        }
    }

    if !(p.pbs_stat == 2 && p.dbs_stat == 2) {
        unsafe {
            let msg = CString::new("glp_gmi_gen: optimal basic solution required\n").unwrap();
            let file = CString::new("intopt/gmigen.c").unwrap();
            let err_func = glp_error_(file.as_ptr(), 79);
            err_func.unwrap()(msg.as_ptr());
        }
    }

    if pool.n != n {
        unsafe {
            let msg = CString::new("glp_gmi_gen: cut pool has wrong number of columns\n").unwrap();
            let file = CString::new("intopt/gmigen.c").unwrap();
            let err_func = glp_error_(file.as_ptr(), 81);
            err_func.unwrap()(msg.as_ptr());
        }
    }

    // Allocate memory safely
    let mut vars: Vec<Var> = Vec::with_capacity((n + 1) as usize);
    let mut ind: Vec<c_int> = Vec::with_capacity((n + 1) as usize);
    let mut val: Vec<c_double> = Vec::with_capacity((n + 1) as usize);
    let mut phi: Vec<c_double> = Vec::with_capacity((m + n + 1) as usize);

    // Find fractional variables
    let mut nv = 0;
    for j in 1..=n {
        unsafe {
            let col = *p.col.offset(j as isize);
            if col.kind != 2 && col.type_ != 5 && col.stat == 1 {
                let prim = col.prim;
                let frac = prim - prim.floor();
                if (0.05..=0.95).contains(&frac) {
                    nv += 1;
                    vars.push(Var { j, f: frac });
                }
            }
        }
    }

    // Sort variables by fractional part
    vars.sort_by(fcmp);

    // Generate cuts
    let mut nnn = 0;
    for var in vars.iter().take(nv as usize) {
        unsafe {
            let len = glp_gmi_cut(
                p,
                var.j,
                ind.as_mut_ptr(),
                val.as_mut_ptr(),
                phi.as_mut_ptr(),
            );

            if len >= 1 {
                let mut valid = true;
                for k in 1..=len {
                    let v = *val.get_unchecked(k as usize);
                    if v.abs() < 1e-03 || v.abs() > 1e+03 {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    let i = glp_add_rows(pool, 1);
                    glp_set_row_bnds(
                        pool,
                        i,
                        2,
                        *val.get_unchecked(0),
                        0.0,
                    );
                    glp_set_mat_row(
                        pool,
                        i,
                        len,
                        ind.as_ptr(),
                        val.as_ptr(),
                    );
                    nnn += 1;
                    if nnn == max_cuts {
                        break;
                    }
                }
            }
        }
    }

    nnn
}

// These would need to be properly bound to the actual GLPK functions
extern "C" {
    fn glp_error_(file: *const c_char, line: c_int) -> GlpErrFunc;
    fn glp_gmi_cut(
        P: *mut GlpProb,
        j: c_int,
        ind: *mut c_int,
        val: *mut c_double,
        phi: *mut c_double,
    ) -> c_int;
    fn glp_add_rows(P: *mut GlpProb, nrs: c_int) -> c_int;
    fn glp_set_row_bnds(
        P: *mut GlpProb,
        i: c_int,
        type_: c_int,
        lb: c_double,
        ub: c_double,
    );
    fn glp_set_mat_row(
        P: *mut GlpProb,
        i: c_int,
        len: c_int,
        ind: *const c_int,
        val: *const c_double,
    );
}