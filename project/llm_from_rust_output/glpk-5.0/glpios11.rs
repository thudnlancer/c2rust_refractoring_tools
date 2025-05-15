use std::cmp::Ordering;
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int, c_uchar, c_void};
use std::ptr;

#[repr(C)]
pub struct BFD {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct AVL {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct AVLNODE {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct glp_cfg {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct glp_mir {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct glp_cov {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct DMP {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct glp_prob {
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
    row: *mut *mut GLPROW,
    col: *mut *mut GLPCOL,
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

#[repr(C)]
pub struct GLPCOL {
    j: c_int,
    name: *mut c_char,
    node: *mut AVLNODE,
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
    name: *mut c_char,
    node: *mut AVLNODE,
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
    pool: *mut DMP,
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
    local: *mut IOSPOOL,
    cov_gen: *mut glp_cov,
    mir_gen: *mut glp_mir,
    clq_gen: *mut glp_cfg,
    pcost: *mut c_void,
    iwrk: *mut c_int,
    dwrk: *mut c_double,
    parm: *const glp_iocp,
    tm_beg: c_double,
    tm_lag: c_double,
    sol_cnt: c_int,
    P: *mut c_void,
    npp: *mut c_void,
    save_sol: *const c_char,
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
    save_sol: *const c_char,
    alien: c_int,
    flip: c_int,
    foo_bar: [c_double; 23],
}

pub type IOSPOOL = glp_prob;

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
    name: *mut c_char,
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

pub type IOSCUT = GLPROW;

#[repr(C)]
struct Info {
    cut: *mut IOSCUT,
    flag: c_char,
    eff: c_double,
    deg: c_double,
}

fn fcmp(a: &Info, b: &Info) -> Ordering {
    if a.deg == 0.0 && b.deg == 0.0 {
        b.eff.partial_cmp(&a.eff).unwrap_or(Ordering::Equal)
    } else {
        b.deg.partial_cmp(&a.deg).unwrap_or(Ordering::Equal)
    }
}

#[no_mangle]
pub extern "C" fn _glp_ios_process_cuts(T: *mut glp_tree) {
    unsafe {
        assert!(!(*T).curr.is_null(), "T->curr != NULL");
        let pool = (*T).local;
        assert!(!pool.is_null(), "pool != NULL");
        assert!((*pool).m > 0, "pool->m > 0");

        let mut info_vec = Vec::with_capacity((*pool).m as usize + 1);
        let mut ind_vec = Vec::with_capacity((*T).n as usize + 1);
        let mut val_vec = Vec::with_capacity((*T).n as usize + 1);
        let mut work_vec = Vec::with_capacity((*T).n as usize + 1);

        work_vec.resize((*T).n as usize + 1, 0.0);

        for k in 1..=(*pool).m {
            let cut = *((*pool).row.add(k as usize));
            info_vec.push(Info {
                cut,
                flag: 0,
                eff: 0.0,
                deg: 0.0,
            });
        }

        for (k, info) in info_vec.iter_mut().enumerate().skip(1) {
            let cut = info.cut;
            let mut len = 0;
            let mut temp = 0.0;

            let mut aij = (*cut).ptr;
            while !aij.is_null() {
                len += 1;
                ind_vec[len] = (*(*aij).col).j;
                val_vec[len] = (*aij).val;
                temp += (*aij).val * (*aij).val;
                aij = (*aij).r_next;
            }

            if temp < 2.2204460492503131e-16 * 2.2204460492503131e-16 {
                temp = 2.2204460492503131e-16;
            }

            let rhs = match (*cut).type_ {
                2 => (*cut).lb,
                3 => (*cut).ub,
                _ => panic!("Invalid cut type"),
            };

            let mut dy = 0.0;
            let mut dz = 0.0;
            let ret = _glp_analyze_row(
                (*T).mip,
                len,
                ind_vec.as_ptr(),
                val_vec.as_ptr(),
                (*cut).type_,
                rhs,
                1e-9,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut dy,
                &mut dz,
            );

            match ret {
                0 => {
                    info.eff = dy.abs() / temp.sqrt();
                    info.deg = if (*(*T).mip).dir == 1 {
                        if dz < 0.0 { 0.0 } else { dz }
                    } else {
                        if dz > 0.0 { 0.0 } else { -dz }
                    };
                }
                1 => {
                    info.deg = 0.0;
                    info.eff = 0.0;
                }
                2 => {
                    info.eff = 1.0;
                    info.deg = std::f64::MAX;
                }
                _ => panic!("Invalid return value from _glp_analyze_row"),
            }

            if info.deg < 0.01 {
                info.deg = 0.0;
            }
        }

        info_vec[1..].sort_by(fcmp);

        let max_cuts = if (*(*T).curr).level == 0 { 90 } else { 10 }.min((*pool).m);

        for k in 1..=max_cuts {
            if info_vec[k].deg >= 0.01 || info_vec[k].eff >= 0.01 {
                let mut parallel = false;
                for kk in 1..k {
                    if info_vec[kk].flag != 0 {
                        if parallel(info_vec[k].cut, info_vec[kk].cut, work_vec.as_mut_ptr()) > 0.90 {
                            parallel = true;
                            break;
                        }
                    }
                }

                if !parallel {
                    let cut = info_vec[k].cut;
                    info_vec[k].flag = 1;

                    let i = glp_add_rows((*T).mip, 1);
                    if !(*cut).name.is_null() {
                        glp_set_row_name((*T).mip, i, (*cut).name);
                    }

                    assert_eq!(
                        (*(*(*T).mip).row.add(i as usize)).origin as c_int,
                        2,
                        "T->mip->row[i]->origin == GLP_RF_CUT"
                    );

                    (*(*(*T).mip).row.add(i as usize)).klass = (*cut).klass;

                    let mut len = 0;
                    let mut aij = (*cut).ptr;
                    while !aij.is_null() {
                        len += 1;
                        ind_vec[len] = (*(*aij).col).j;
                        val_vec[len] = (*aij).val;
                        aij = (*aij).r_next;
                    }

                    glp_set_mat_row(
                        (*T).mip,
                        i,
                        len,
                        ind_vec.as_ptr(),
                        val_vec.as_ptr(),
                    );

                    let rhs = match (*cut).type_ {
                        2 => (*cut).lb,
                        3 => (*cut).ub,
                        _ => panic!("Invalid cut type"),
                    };

                    glp_set_row_bnds((*T).mip, i, (*cut).type_, rhs, rhs);
                }
            }
        }
    }
}

unsafe fn parallel(a: *mut IOSCUT, b: *mut IOSCUT, work: *mut c_double) -> c_double {
    let mut s = 0.0;
    let mut sa = 0.0;
    let mut sb = 0.0;

    let mut aij = (*a).ptr;
    while !aij.is_null() {
        *work.add((*(*aij).col).j as usize) = (*aij).val;
        sa += (*aij).val * (*aij).val;
        aij = (*aij).r_next;
    }

    let mut aij = (*b).ptr;
    while !aij.is_null() {
        s += *work.add((*(*aij).col).j as usize) * (*aij).val;
        sb += (*aij).val * (*aij).val;
        aij = (*aij).r_next;
    }

    let mut aij = (*a).ptr;
    while !aij.is_null() {
        *work.add((*(*aij).col).j as usize) = 0.0;
        aij = (*aij).r_next;
    }

    let temp = sa.sqrt() * sb.sqrt();
    if temp < 2.2204460492503131e-16 * 2.2204460492503131e-16 {
        2.2204460492503131e-16
    } else {
        s / temp
    }
}