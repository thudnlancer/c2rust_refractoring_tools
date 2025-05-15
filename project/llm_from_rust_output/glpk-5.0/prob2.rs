use std::ffi::CStr;
use std::ptr;

#[repr(C)]
pub struct AVL;
#[repr(C)]
pub struct AVLNODE;
#[repr(C)]
pub struct BFD;
#[repr(C)]
pub struct DMP;
#[repr(C)]
pub struct glp_tree;

type GlpErrFunc = Option<unsafe extern "C" fn(*const libc::c_char, ...)>;

#[repr(C)]
pub struct GlpProb {
    pool: *mut DMP,
    tree: *mut glp_tree,
    name: *mut libc::c_char,
    obj: *mut libc::c_char,
    dir: libc::c_int,
    c0: libc::c_double,
    m_max: libc::c_int,
    n_max: libc::c_int,
    m: libc::c_int,
    n: libc::c_int,
    nnz: libc::c_int,
    row: *mut *mut GlpRow,
    col: *mut *mut GlpCol,
    r_tree: *mut AVL,
    c_tree: *mut AVL,
    valid: libc::c_int,
    head: *mut libc::c_int,
    bfd: *mut BFD,
    pbs_stat: libc::c_int,
    dbs_stat: libc::c_int,
    obj_val: libc::c_double,
    it_cnt: libc::c_int,
    some: libc::c_int,
    ipt_stat: libc::c_int,
    ipt_obj: libc::c_double,
    mip_stat: libc::c_int,
    mip_obj: libc::c_double,
}

#[repr(C)]
pub struct GlpCol {
    j: libc::c_int,
    name: *mut libc::c_char,
    node: *mut AVLNODE,
    kind: libc::c_int,
    type_: libc::c_int,
    lb: libc::c_double,
    ub: libc::c_double,
    coef: libc::c_double,
    ptr: *mut GlpAij,
    sjj: libc::c_double,
    stat: libc::c_int,
    bind: libc::c_int,
    prim: libc::c_double,
    dual: libc::c_double,
    pval: libc::c_double,
    dval: libc::c_double,
    mipx: libc::c_double,
}

#[repr(C)]
pub struct GlpAij {
    row: *mut GlpRow,
    col: *mut GlpCol,
    val: libc::c_double,
    r_prev: *mut GlpAij,
    r_next: *mut GlpAij,
    c_prev: *mut GlpAij,
    c_next: *mut GlpAij,
}

#[repr(C)]
pub struct GlpRow {
    i: libc::c_int,
    name: *mut libc::c_char,
    node: *mut AVLNODE,
    level: libc::c_int,
    origin: libc::c_uchar,
    klass: libc::c_uchar,
    type_: libc::c_int,
    lb: libc::c_double,
    ub: libc::c_double,
    ptr: *mut GlpAij,
    rii: libc::c_double,
    stat: libc::c_int,
    bind: libc::c_int,
    prim: libc::c_double,
    dual: libc::c_double,
    pval: libc::c_double,
    dval: libc::c_double,
    mipx: libc::c_double,
}

pub fn glp_get_prob_name(lp: &GlpProb) -> Option<&CStr> {
    unsafe { lp.name.as_ref().map(|p| CStr::from_ptr(p)) }
}

pub fn glp_get_obj_name(lp: &GlpProb) -> Option<&CStr> {
    unsafe { lp.obj.as_ref().map(|p| CStr::from_ptr(p)) }
}

pub fn glp_get_obj_dir(lp: &GlpProb) -> libc::c_int {
    lp.dir
}

pub fn glp_get_num_rows(lp: &GlpProb) -> libc::c_int {
    lp.m
}

pub fn glp_get_num_cols(lp: &GlpProb) -> libc::c_int {
    lp.n
}

pub fn glp_get_row_name(lp: &GlpProb, i: libc::c_int) -> Option<&CStr> {
    if !(1 <= i && i <= lp.m) {
        unsafe {
            let error_func = glp_error_.expect("glp_error_ is null");
            error_func(
                b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                146,
            );
        }
        return None;
    }
    unsafe { (*lp.row.offset(i as isize)).as_ref()?.name.as_ref().map(|p| CStr::from_ptr(p)) }
}

pub fn glp_get_col_name(lp: &GlpProb, j: libc::c_int) -> Option<&CStr> {
    if !(1 <= j && j <= lp.n) {
        unsafe {
            let error_func = glp_error_.expect("glp_error_ is null");
            error_func(
                b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                170,
            );
        }
        return None;
    }
    unsafe { (*lp.col.offset(j as isize)).as_ref()?.name.as_ref().map(|p| CStr::from_ptr(p)) }
}

pub fn glp_get_row_type(lp: &GlpProb, i: libc::c_int) -> libc::c_int {
    if !(1 <= i && i <= lp.m) {
        unsafe {
            let error_func = glp_error_.expect("glp_error_ is null");
            error_func(
                b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                198,
            );
        }
        return -1;
    }
    unsafe { (*lp.row.offset(i as isize)).as_ref().map_or(-1, |row| row.type_) }
}

pub fn glp_get_row_lb(lp: &GlpProb, i: libc::c_int) -> libc::c_double {
    if !(1 <= i && i <= lp.m) {
        unsafe {
            let error_func = glp_error_.expect("glp_error_ is null");
            error_func(
                b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                221,
            );
        }
        return 0.0;
    }
    unsafe {
        match (*lp.row.offset(i as isize)).as_ref().map(|row| row.type_) {
            Some(1) | Some(3) => -f64::INFINITY,
            Some(2) | Some(4) | Some(5) => (*lp.row.offset(i as isize)).as_ref().map(|row| row.lb).unwrap_or(0.0),
            _ => {
                glp_assert_(
                    b"lp != lp\0".as_ptr() as *const libc::c_char,
                    b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                    231,
                );
                0.0
            }
        }
    }
}

pub fn glp_get_row_ub(lp: &GlpProb, i: libc::c_int) -> libc::c_double {
    if !(1 <= i && i <= lp.m) {
        unsafe {
            let error_func = glp_error_.expect("glp_error_ is null");
            error_func(
                b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                254,
            );
        }
        return 0.0;
    }
    unsafe {
        match (*lp.row.offset(i as isize)).as_ref().map(|row| row.type_) {
            Some(1) | Some(2) => f64::INFINITY,
            Some(3) | Some(4) | Some(5) => (*lp.row.offset(i as isize)).as_ref().map(|row| row.ub).unwrap_or(0.0),
            _ => {
                glp_assert_(
                    b"lp != lp\0".as_ptr() as *const libc::c_char,
                    b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                    264,
                );
                0.0
            }
        }
    }
}

pub fn glp_get_col_type(lp: &GlpProb, j: libc::c_int) -> libc::c_int {
    if !(1 <= j && j <= lp.n) {
        unsafe {
            let error_func = glp_error_.expect("glp_error_ is null");
            error_func(
                b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                291,
            );
        }
        return -1;
    }
    unsafe { (*lp.col.offset(j as isize)).as_ref().map_or(-1, |col| col.type_) }
}

pub fn glp_get_col_lb(lp: &GlpProb, j: libc::c_int) -> libc::c_double {
    if !(1 <= j && j <= lp.n) {
        unsafe {
            let error_func = glp_error_.expect("glp_error_ is null");
            error_func(
                b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                314,
            );
        }
        return 0.0;
    }
    unsafe {
        match (*lp.col.offset(j as isize)).as_ref().map(|col| col.type_) {
            Some(1) | Some(3) => -f64::INFINITY,
            Some(2) | Some(4) | Some(5) => (*lp.col.offset(j as isize)).as_ref().map(|col| col.lb).unwrap_or(0.0),
            _ => {
                glp_assert_(
                    b"lp != lp\0".as_ptr() as *const libc::c_char,
                    b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                    325,
                );
                0.0
            }
        }
    }
}

pub fn glp_get_col_ub(lp: &GlpProb, j: libc::c_int) -> libc::c_double {
    if !(1 <= j && j <= lp.n) {
        unsafe {
            let error_func = glp_error_.expect("glp_error_ is null");
            error_func(
                b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                348,
            );
        }
        return 0.0;
    }
    unsafe {
        match (*lp.col.offset(j as isize)).as_ref().map(|col| col.type_) {
            Some(1) | Some(2) => f64::INFINITY,
            Some(3) | Some(4) | Some(5) => (*lp.col.offset(j as isize)).as_ref().map(|col| col.ub).unwrap_or(0.0),
            _ => {
                glp_assert_(
                    b"lp != lp\0".as_ptr() as *const libc::c_char,
                    b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                    359,
                );
                0.0
            }
        }
    }
}

pub fn glp_get_obj_coef(lp: &GlpProb, j: libc::c_int) -> libc::c_double {
    if !(0 <= j && j <= lp.n) {
        unsafe {
            let error_func = glp_error_.expect("glp_error_ is null");
            error_func(
                b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                383,
            );
        }
        return 0.0;
    }
    if j == 0 {
        lp.c0
    } else {
        unsafe { (*lp.col.offset(j as isize)).as_ref().map_or(0.0, |col| col.coef) }
    }
}

pub fn glp_get_num_nz(lp: &GlpProb) -> libc::c_int {
    lp.nnz
}

pub fn glp_get_mat_row(
    lp: &GlpProb,
    i: libc::c_int,
    ind: &mut [libc::c_int],
    val: &mut [libc::c_double],
) -> libc::c_int {
    if !(1 <= i && i <= lp.m) {
        unsafe {
            let error_func = glp_error_.expect("glp_error_ is null");
            error_func(
                b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                436,
            );
        }
        return 0;
    }

    let mut len = 0;
    unsafe {
        let mut aij = (*lp.row.offset(i as isize)).as_ref().and_then(|row| row.ptr.as_ref());
        while let Some(a) = aij {
            len += 1;
            if len <= ind.len() {
                ind[len - 1] = (*a.col).j;
            }
            if len <= val.len() {
                val[len - 1] = a.val;
            }
            aij = a.r_next.as_ref();
        }
    }
    len
}

pub fn glp_get_mat_col(
    lp: &GlpProb,
    j: libc::c_int,
    ind: &mut [libc::c_int],
    val: &mut [libc::c_double],
) -> libc::c_int {
    if !(1 <= j && j <= lp.n) {
        unsafe {
            let error_func = glp_error_.expect("glp_error_ is null");
            error_func(
                b"api/prob2.c\0".as_ptr() as *const libc::c_char,
                477,
            );
        }
        return 0;
    }

    let mut len = 0;
    unsafe {
        let mut aij = (*lp.col.offset(j as isize)).as_ref().and_then(|col| col.ptr.as_ref());
        while let Some(a) = aij {
            len += 1;
            if len <= ind.len() {
                ind[len - 1] = (*a.row).i;
            }
            if len <= val.len() {
                val[len - 1] = a.val;
            }
            aij = a.c_next.as_ref();
        }
    }
    len
}

extern "C" {
    fn glp_assert_(expr: *const libc::c_char, file: *const libc::c_char, line: libc::c_int);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> GlpErrFunc;
}