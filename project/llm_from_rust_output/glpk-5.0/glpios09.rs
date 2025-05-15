use libc::{c_double, c_int, c_uchar, c_void};
use std::ffi::CString;
use std::ptr;

#[repr(C)]
pub struct glp_prob {
    pool: *mut c_void,
    tree: *mut glp_tree,
    name: *mut libc::c_char,
    obj: *mut libc::c_char,
    dir: c_int,
    c0: c_double,
    m_max: c_int,
    n_max: c_int,
    m: c_int,
    n: c_int,
    nnz: c_int,
    row: *mut *mut GLPROW,
    col: *mut *mut GLPCOL,
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
pub struct GLPCOL {
    j: c_int,
    name: *mut libc::c_char,
    node: *mut c_void,
    kind: c_int,
    type_: c_int,
    lb: c_double,
    ub: c_double,
    coef: c_double,
    ptr: *mut GLPAIJ,
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
pub struct GLPAIJ {
    row: *mut GLPROW,
    col: *mut GLPCOL,
    val: c_double,
    r_prev: *mut GLPAIJ,
    r_next: *mut GLPAIJ,
    c_prev: *mut GLPAIJ,
    c_next: *mut GLPAIJ,
}

#[repr(C)]
pub struct GLPROW {
    i: c_int,
    name: *mut libc::c_char,
    node: *mut c_void,
    level: c_int,
    origin: c_uchar,
    klass: c_uchar,
    type_: c_int,
    lb: c_double,
    ub: c_double,
    ptr: *mut GLPAIJ,
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
pub struct glp_tree {
    magic: c_int,
    pool: *mut c_void,
    n: c_int,
    orig_m: c_int,
    orig_type: *mut c_uchar,
    orig_lb: *mut c_double,
    orig_ub: *mut c_double,
    orig_stat: *mut c_uchar,
    orig_prim: *mut c_double,
    orig_dual: *mut c_double,
    orig_obj: c_double,
    nslots: c_int,
    avail: c_int,
    slot: *mut IOSLOT,
    head: *mut IOSNPD,
    tail: *mut IOSNPD,
    a_cnt: c_int,
    n_cnt: c_int,
    t_cnt: c_int,
    root_m: c_int,
    root_type: *mut c_uchar,
    root_lb: *mut c_double,
    root_ub: *mut c_double,
    root_stat: *mut c_uchar,
    curr: *mut IOSNPD,
    mip: *mut glp_prob,
    non_int: *mut c_uchar,
    pred_m: c_int,
    pred_max: c_int,
    pred_type: *mut c_uchar,
    pred_lb: *mut c_double,
    pred_ub: *mut c_double,
    pred_stat: *mut c_uchar,
    local: *mut glp_prob,
    cov_gen: *mut c_void,
    mir_gen: *mut c_void,
    clq_gen: *mut c_void,
    pcost: *mut c_void,
    iwrk: *mut c_int,
    dwrk: *mut c_double,
    parm: *const glp_iocp,
    tm_beg: c_double,
    tm_lag: c_double,
    sol_cnt: c_int,
    P: *mut c_void,
    npp: *mut c_void,
    save_sol: *const libc::c_char,
    save_cnt: c_int,
    reason: c_int,
    stop: c_int,
    next_p: c_int,
    reopt: c_int,
    reinv: c_int,
    br_var: c_int,
    br_sel: c_int,
    child: c_int,
}

#[repr(C)]
pub struct glp_iocp {
    msg_lev: c_int,
    br_tech: c_int,
    bt_tech: c_int,
    tol_int: c_double,
    tol_obj: c_double,
    tm_lim: c_int,
    out_frq: c_int,
    out_dly: c_int,
    cb_func: Option<unsafe extern "C" fn(*mut glp_tree, *mut c_void)>,
    cb_info: *mut c_void,
    cb_size: c_int,
    pp_tech: c_int,
    mip_gap: c_double,
    mir_cuts: c_int,
    gmi_cuts: c_int,
    cov_cuts: c_int,
    clq_cuts: c_int,
    presolve: c_int,
    binarize: c_int,
    fp_heur: c_int,
    ps_heur: c_int,
    ps_tm_lim: c_int,
    sr_heur: c_int,
    use_sol: c_int,
    save_sol: *const libc::c_char,
    alien: c_int,
    flip: c_int,
    foo_bar: [c_double; 23],
}

#[repr(C)]
pub struct IOSNPD {
    p: c_int,
    up: *mut IOSNPD,
    level: c_int,
    count: c_int,
    b_ptr: *mut IOSBND,
    s_ptr: *mut IOSTAT,
    r_ptr: *mut IOSROW,
    solved: c_int,
    lp_obj: c_double,
    bound: c_double,
    ii_cnt: c_int,
    ii_sum: c_double,
    changed: c_int,
    br_var: c_int,
    br_val: c_double,
    data: *mut c_void,
    temp: *mut IOSNPD,
    prev: *mut IOSNPD,
    next: *mut IOSNPD,
}

#[repr(C)]
pub struct IOSROW {
    name: *mut libc::c_char,
    origin: c_uchar,
    klass: c_uchar,
    type_: c_uchar,
    lb: c_double,
    ub: c_double,
    ptr: *mut IOSAIJ,
    rii: c_double,
    stat: c_uchar,
    next: *mut IOSROW,
}

#[repr(C)]
pub struct IOSAIJ {
    j: c_int,
    val: c_double,
    next: *mut IOSAIJ,
}

#[repr(C)]
pub struct IOSTAT {
    k: c_int,
    stat: c_uchar,
    next: *mut IOSTAT,
}

#[repr(C)]
pub struct IOSBND {
    k: c_int,
    type_: c_uchar,
    lb: c_double,
    ub: c_double,
    next: *mut IOSBND,
}

#[repr(C)]
pub struct IOSLOT {
    node: *mut IOSNPD,
    next: c_int,
}

#[repr(C)]
pub struct glp_smcp {
    msg_lev: c_int,
    meth: c_int,
    pricing: c_int,
    r_test: c_int,
    tol_bnd: c_double,
    tol_dj: c_double,
    tol_piv: c_double,
    obj_ll: c_double,
    obj_ul: c_double,
    it_lim: c_int,
    tm_lim: c_int,
    out_frq: c_int,
    out_dly: c_int,
    presolve: c_int,
    excl: c_int,
    shift: c_int,
    aorn: c_int,
    foo_bar: [c_double; 33],
}

#[repr(C)]
pub struct csa {
    dn_cnt: *mut c_int,
    dn_sum: *mut c_double,
    up_cnt: *mut c_int,
    up_sum: *mut c_double,
}

pub fn _glp_ios_choose_var(T: *mut glp_tree, next: *mut c_int) -> c_int {
    unsafe {
        let br_tech = (*((*T).parm)).br_tech;
        match br_tech {
            1 => branch_first(T, next),
            2 => branch_last(T, next),
            3 => branch_mostf(T, next),
            4 => branch_drtom(T, next),
            5 => _glp_ios_pcost_branch(T, next),
            _ => {
                assert!(false, "Invalid branch technique");
                0
            }
        }
    }
}

unsafe fn branch_first(T: *mut glp_tree, next: *mut c_int) -> c_int {
    let mut j = 1;
    while j <= (*T).n {
        if *((*T).non_int).offset(j as isize) != 0 {
            break;
        }
        j += 1;
    }
    assert!(1 <= j && j <= (*T).n, "Invalid column index");

    let beta = glp_get_col_prim((*T).mip, j);
    *next = if beta - beta.floor() < beta.ceil() - beta {
        1
    } else {
        2
    };
    j
}

unsafe fn branch_last(T: *mut glp_tree, next: *mut c_int) -> c_int {
    let mut j = (*T).n;
    while j >= 1 {
        if *((*T).non_int).offset(j as isize) != 0 {
            break;
        }
        j -= 1;
    }
    assert!(1 <= j && j <= (*T).n, "Invalid column index");

    let beta = glp_get_col_prim((*T).mip, j);
    *next = if beta - beta.floor() < beta.ceil() - beta {
        1
    } else {
        2
    };
    j
}

unsafe fn branch_mostf(T: *mut glp_tree, next: *mut c_int) -> c_int {
    let mut jj = 0;
    let mut most = f64::MAX;
    let mut sel_next = 0;

    for j in 1..=(*T).n {
        if *((*T).non_int).offset(j as isize) != 0 {
            let beta = glp_get_col_prim((*T).mip, j);
            let temp = beta.floor() + 0.5;
            let dist = (beta - temp).abs();
            if most > dist {
                most = dist;
                jj = j;
                sel_next = if beta < temp { 1 } else { 2 };
            }
        }
    }

    *next = sel_next;
    jj
}

// Other functions would follow similar patterns, converting C to safe Rust
// while maintaining the same functionality