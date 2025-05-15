use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_double, c_uchar};

pub type GlpErrFunc = Option<unsafe extern "C" fn(*const c_char, ...)>;

#[derive(Debug)]
pub struct GlpProb {
    pool: *mut DMP,
    tree: *mut glp_tree,
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
    r_tree: *mut AVL,
    c_tree: *mut AVL,
    valid: c_int,
    head: *mut c_int,
    bfd: *mut BFD,
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

#[derive(Debug)]
pub struct GlpCol {
    j: c_int,
    name: *mut c_char,
    node: *mut AVLNODE,
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

#[derive(Debug)]
pub struct GlpAij {
    row: *mut GlpRow,
    col: *mut GlpCol,
    val: c_double,
    r_prev: *mut GlpAij,
    r_next: *mut GlpAij,
    c_prev: *mut GlpAij,
    c_next: *mut GlpAij,
}

#[derive(Debug)]
pub struct GlpRow {
    i: c_int,
    name: *mut c_char,
    node: *mut AVLNODE,
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

pub fn glp_set_rii(lp: &mut GlpProb, i: c_int, rii: c_double) -> Result<(), String> {
    if !(1 <= i && i <= lp.m) {
        return Err(format!("glp_set_rii: i = {}; row number out of range", i));
    }
    if rii <= 0.0 {
        return Err(format!("glp_set_rii: i = {}; rii = {}; invalid scale factor", i, rii));
    }

    unsafe {
        if lp.valid != 0 && (*lp.row.offset(i as isize)).rii != rii {
            let mut aij = (*lp.row.offset(i as isize)).ptr;
            while !aij.is_null() {
                if (*(*aij).col).stat == 1 {
                    lp.valid = 0;
                    break;
                }
                aij = (*aij).r_next;
            }
        }
        (*lp.row.offset(i as isize)).rii = rii;
    }
    Ok(())
}

pub fn glp_set_sjj(lp: &mut GlpProb, j: c_int, sjj: c_double) -> Result<(), String> {
    if !(1 <= j && j <= lp.n) {
        return Err(format!("glp_set_sjj: j = {}; column number out of range", j));
    }
    if sjj <= 0.0 {
        return Err(format!("glp_set_sjj: j = {}; sjj = {}; invalid scale factor", j, sjj));
    }

    unsafe {
        if lp.valid != 0 && (*lp.col.offset(j as isize)).sjj != sjj && (*lp.col.offset(j as isize)).stat == 1 {
            lp.valid = 0;
        }
        (*lp.col.offset(j as isize)).sjj = sjj;
    }
    Ok(())
}

pub fn glp_get_rii(lp: &GlpProb, i: c_int) -> Result<c_double, String> {
    if !(1 <= i && i <= lp.m) {
        return Err(format!("glp_get_rii: i = {}; row number out of range", i));
    }
    unsafe { Ok((*lp.row.offset(i as isize)).rii) }
}

pub fn glp_get_sjj(lp: &GlpProb, j: c_int) -> Result<c_double, String> {
    if !(1 <= j && j <= lp.n) {
        return Err(format!("glp_get_sjj: j = {}; column number out of range", j));
    }
    unsafe { Ok((*lp.col.offset(j as isize)).sjj) }
}

pub fn glp_unscale_prob(lp: &mut GlpProb) -> Result<(), String> {
    let m = lp.m;
    let n = lp.n;

    for i in 1..=m {
        glp_set_rii(lp, i, 1.0)?;
    }

    for j in 1..=n {
        glp_set_sjj(lp, j, 1.0)?;
    }

    Ok(())
}

// Opaque types for FFI compatibility
pub enum DMP {}
pub enum glp_tree {}
pub enum AVL {}
pub enum AVLNODE {}
pub enum BFD {}