use std::f64;
use std::ptr;

type GlpErrFunc = Option<unsafe extern "C" fn(*const libc::c_char, ...)>;

#[repr(C)]
pub struct GlpProb {
    pool: *mut libc::c_void,
    tree: *mut libc::c_void,
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
    r_tree: *mut libc::c_void,
    c_tree: *mut libc::c_void,
    valid: libc::c_int,
    head: *mut libc::c_int,
    bfd: *mut libc::c_void,
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
pub struct GlpRow {
    i: libc::c_int,
    name: *mut libc::c_char,
    node: *mut libc::c_void,
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

#[repr(C)]
pub struct GlpCol {
    j: libc::c_int,
    name: *mut libc::c_char,
    node: *mut libc::c_void,
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

fn min_row_aij(lp: &GlpProb, i: libc::c_int, scaled: libc::c_int) -> libc::c_double {
    assert!(1 <= i && i <= lp.m);
    let mut min_aij = 1.0;
    unsafe {
        let mut aij = (*lp.row.offset(i as isize)).ptr;
        while !aij.is_null() {
            let mut temp = (*aij).val.abs();
            if scaled != 0 {
                temp *= (*(*aij).row).rii * (*(*aij).col).sjj;
            }
            if (*aij).r_prev.is_null() || min_aij > temp {
                min_aij = temp;
            }
            aij = (*aij).r_next;
        }
    }
    min_aij
}

fn max_row_aij(lp: &GlpProb, i: libc::c_int, scaled: libc::c_int) -> libc::c_double {
    assert!(1 <= i && i <= lp.m);
    let mut max_aij = 1.0;
    unsafe {
        let mut aij = (*lp.row.offset(i as isize)).ptr;
        while !aij.is_null() {
            let mut temp = (*aij).val.abs();
            if scaled != 0 {
                temp *= (*(*aij).row).rii * (*(*aij).col).sjj;
            }
            if (*aij).r_prev.is_null() || max_aij < temp {
                max_aij = temp;
            }
            aij = (*aij).r_next;
        }
    }
    max_aij
}

fn min_col_aij(lp: &GlpProb, j: libc::c_int, scaled: libc::c_int) -> libc::c_double {
    assert!(1 <= j && j <= lp.n);
    let mut min_aij = 1.0;
    unsafe {
        let mut aij = (*lp.col.offset(j as isize)).ptr;
        while !aij.is_null() {
            let mut temp = (*aij).val.abs();
            if scaled != 0 {
                temp *= (*(*aij).row).rii * (*(*aij).col).sjj;
            }
            if (*aij).c_prev.is_null() || min_aij > temp {
                min_aij = temp;
            }
            aij = (*aij).c_next;
        }
    }
    min_aij
}

fn max_col_aij(lp: &GlpProb, j: libc::c_int, scaled: libc::c_int) -> libc::c_double {
    assert!(1 <= j && j <= lp.n);
    let mut max_aij = 1.0;
    unsafe {
        let mut aij = (*lp.col.offset(j as isize)).ptr;
        while !aij.is_null() {
            let mut temp = (*aij).val.abs();
            if scaled != 0 {
                temp *= (*(*aij).row).rii * (*(*aij).col).sjj;
            }
            if (*aij).c_prev.is_null() || max_aij < temp {
                max_aij = temp;
            }
            aij = (*aij).c_next;
        }
    }
    max_aij
}

fn min_mat_aij(lp: &GlpProb, scaled: libc::c_int) -> libc::c_double {
    let mut min_aij = 1.0;
    for i in 1..=lp.m {
        let temp = min_row_aij(lp, i, scaled);
        if i == 1 || min_aij > temp {
            min_aij = temp;
        }
    }
    min_aij
}

fn max_mat_aij(lp: &GlpProb, scaled: libc::c_int) -> libc::c_double {
    let mut max_aij = 1.0;
    for i in 1..=lp.m {
        let temp = max_row_aij(lp, i, scaled);
        if i == 1 || max_aij < temp {
            max_aij = temp;
        }
    }
    max_aij
}

fn eq_scaling(lp: &mut GlpProb, flag: libc::c_int) {
    assert!(flag == 0 || flag == 1);
    for pass in 0..=1 {
        if pass == flag {
            for i in 1..=lp.m {
                let temp = max_row_aij(lp, i, 1);
                unsafe {
                    glp_set_rii(lp, i, glp_get_rii(lp, i) / temp);
                }
            }
        } else {
            for j in 1..=lp.n {
                let temp = max_col_aij(lp, j, 1);
                unsafe {
                    glp_set_sjj(lp, j, glp_get_sjj(lp, j) / temp);
                }
            }
        }
    }
}

fn gm_scaling(lp: &mut GlpProb, flag: libc::c_int) {
    assert!(flag == 0 || flag == 1);
    for pass in 0..=1 {
        if pass == flag {
            for i in 1..=lp.m {
                let temp = min_row_aij(lp, i, 1) * max_row_aij(lp, i, 1);
                unsafe {
                    glp_set_rii(lp, i, glp_get_rii(lp, i) / temp.sqrt());
                }
            }
        } else {
            for j in 1..=lp.n {
                let temp = min_col_aij(lp, j, 1) * max_col_aij(lp, j, 1);
                unsafe {
                    glp_set_sjj(lp, j, glp_get_sjj(lp, j) / temp.sqrt());
                }
            }
        }
    }
}

fn max_row_ratio(lp: &GlpProb) -> libc::c_double {
    let mut ratio = 1.0;
    for i in 1..=lp.m {
        let temp = max_row_aij(lp, i, 1) / min_row_aij(lp, i, 1);
        if i == 1 || ratio < temp {
            ratio = temp;
        }
    }
    ratio
}

fn max_col_ratio(lp: &GlpProb) -> libc::c_double {
    let mut ratio = 1.0;
    for j in 1..=lp.n {
        let temp = max_col_aij(lp, j, 1) / min_col_aij(lp, j, 1);
        if j == 1 || ratio < temp {
            ratio = temp;
        }
    }
    ratio
}

fn gm_iterate(lp: &mut GlpProb, it_max: libc::c_int, tau: libc::c_double) {
    let flag = (max_row_ratio(lp) > max_col_ratio(lp)) as libc::c_int;
    let mut ratio = 0.0;
    for k in 1..=it_max {
        let r_old = ratio;
        ratio = max_mat_aij(lp, 1) / min_mat_aij(lp, 1);
        if k > 1 && ratio > tau * r_old {
            break;
        }
        gm_scaling(lp, flag);
    }
}

pub fn glp_scale_prob(lp: &mut GlpProb, flags: libc::c_int) {
    if flags & !(0x1 | 0x10 | 0x20 | 0x40 | 0x80) != 0 {
        panic!("Invalid scaling options");
    }
    let effective_flags = if flags & 0x80 != 0 {
        0x1 | 0x10 | 0x40
    } else {
        flags
    };
    scale_prob(lp, effective_flags);
}

fn scale_prob(lp: &mut GlpProb, flags: libc::c_int) {
    println!("Scaling...");
    unsafe {
        glp_unscale_prob(lp);
    }
    
    let min_aij = min_mat_aij(lp, 1);
    let max_aij = max_mat_aij(lp, 1);
    let ratio = max_aij / min_aij;
    println!(" A: min|aij| = {:.3e}  max|aij| = {:.3e}  ratio = {:.3e}", 
             min_aij, max_aij, ratio);

    if min_aij >= 0.10 && max_aij <= 10.0 {
        println!("Problem data seem to be well scaled");
        if flags & 0x40 == 0 {
            return;
        }
    }

    if flags & 0x1 != 0 {
        gm_iterate(lp, 15, 0.90);
        let min_aij = min_mat_aij(lp, 1);
        let max_aij = max_mat_aij(lp, 1);
        let ratio = max_aij / min_aij;
        println!("GM: min|aij| = {:.3e}  max|aij| = {:.3e}  ratio = {:.3e}", 
                 min_aij, max_aij, ratio);
    }

    if flags & 0x10 != 0 {
        eq_scaling(lp, (max_row_ratio(lp) > max_col_ratio(lp)) as libc::c_int);
        let min_aij = min_mat_aij(lp, 1);
        let max_aij = max_mat_aij(lp, 1);
        let ratio = max_aij / min_aij;
        println!("EQ: min|aij| = {:.3e}  max|aij| = {:.3e}  ratio = {:.3e}", 
                 min_aij, max_aij, ratio);
    }

    if flags & 0x20 != 0 {
        for i in 1..=lp.m {
            unsafe {
                glp_set_rii(lp, i, _glp_round2n(glp_get_rii(lp, i)));
            }
        }
        for j in 1..=lp.n {
            unsafe {
                glp_set_sjj(lp, j, _glp_round2n(glp_get_sjj(lp, j)));
            }
        }
        let min_aij = min_mat_aij(lp, 1);
        let max_aij = max_mat_aij(lp, 1);
        let ratio = max_aij / min_aij;
        println!("2N: min|aij| = {:.3e}  max|aij| = {:.3e}  ratio = {:.3e}", 
                 min_aij, max_aij, ratio);
    }
}