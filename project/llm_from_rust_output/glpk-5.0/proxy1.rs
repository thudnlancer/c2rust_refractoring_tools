use std::ffi::CString;
use std::ptr;

#[repr(C)]
pub struct glp_prob {
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
    row: *mut *mut GLPROW,
    col: *mut *mut GLPCOL,
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
pub struct GLPCOL {
    j: libc::c_int,
    name: *mut libc::c_char,
    node: *mut AVLNODE,
    kind: libc::c_int,
    type_: libc::c_int,
    lb: libc::c_double,
    ub: libc::c_double,
    coef: libc::c_double,
    ptr: *mut GLPAIJ,
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
pub struct GLPAIJ {
    row: *mut GLPROW,
    col: *mut GLPCOL,
    val: libc::c_double,
    r_prev: *mut GLPAIJ,
    r_next: *mut GLPAIJ,
    c_prev: *mut GLPAIJ,
    c_next: *mut GLPAIJ,
}

#[repr(C)]
pub struct GLPROW {
    i: libc::c_int,
    name: *mut libc::c_char,
    node: *mut AVLNODE,
    level: libc::c_int,
    origin: libc::c_uchar,
    klass: libc::c_uchar,
    type_: libc::c_int,
    lb: libc::c_double,
    ub: libc::c_double,
    ptr: *mut GLPAIJ,
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
pub struct glp_tree {
    magic: libc::c_int,
    pool: *mut DMP,
    n: libc::c_int,
    orig_m: libc::c_int,
    orig_type: *mut libc::c_uchar,
    orig_lb: *mut libc::c_double,
    orig_ub: *mut libc::c_double,
    orig_stat: *mut libc::c_uchar,
    orig_prim: *mut libc::c_double,
    orig_dual: *mut libc::c_double,
    orig_obj: libc::c_double,
    nslots: libc::c_int,
    avail: libc::c_int,
    slot: *mut IOSLOT,
    head: *mut IOSNPD,
    tail: *mut IOSNPD,
    a_cnt: libc::c_int,
    n_cnt: libc::c_int,
    t_cnt: libc::c_int,
    root_m: libc::c_int,
    root_type: *mut libc::c_uchar,
    root_lb: *mut libc::c_double,
    root_ub: *mut libc::c_double,
    root_stat: *mut libc::c_uchar,
    curr: *mut IOSNPD,
    mip: *mut glp_prob,
    non_int: *mut libc::c_uchar,
    pred_m: libc::c_int,
    pred_max: libc::c_int,
    pred_type: *mut libc::c_uchar,
    pred_lb: *mut libc::c_double,
    pred_ub: *mut libc::c_double,
    pred_stat: *mut libc::c_uchar,
    local: *mut IOSPOOL,
    cov_gen: *mut glp_cov,
    mir_gen: *mut glp_mir,
    clq_gen: *mut glp_cfg,
    pcost: *mut libc::c_void,
    iwrk: *mut libc::c_int,
    dwrk: *mut libc::c_double,
    parm: *const glp_iocp,
    tm_beg: libc::c_double,
    tm_lag: libc::c_double,
    sol_cnt: libc::c_int,
    P: *mut libc::c_void,
    npp: *mut libc::c_void,
    save_sol: *const libc::c_char,
    save_cnt: libc::c_int,
    reason: libc::c_int,
    stop: libc::c_int,
    next_p: libc::c_int,
    reopt: libc::c_int,
    reinv: libc::c_int,
    br_var: libc::c_int,
    br_sel: libc::c_int,
    child: libc::c_int,
}

#[repr(C)]
pub struct glp_iocp {
    msg_lev: libc::c_int,
    br_tech: libc::c_int,
    bt_tech: libc::c_int,
    tol_int: libc::c_double,
    tol_obj: libc::c_double,
    tm_lim: libc::c_int,
    out_frq: libc::c_int,
    out_dly: libc::c_int,
    cb_func: Option<unsafe extern "C" fn(*mut glp_tree, *mut libc::c_void)>,
    cb_info: *mut libc::c_void,
    cb_size: libc::c_int,
    pp_tech: libc::c_int,
    mip_gap: libc::c_double,
    mir_cuts: libc::c_int,
    gmi_cuts: libc::c_int,
    cov_cuts: libc::c_int,
    clq_cuts: libc::c_int,
    presolve: libc::c_int,
    binarize: libc::c_int,
    fp_heur: libc::c_int,
    ps_heur: libc::c_int,
    ps_tm_lim: libc::c_int,
    sr_heur: libc::c_int,
    use_sol: libc::c_int,
    save_sol: *const libc::c_char,
    alien: libc::c_int,
    flip: libc::c_int,
    foo_bar: [libc::c_double; 23],
}

pub type IOSPOOL = glp_prob;

#[repr(C)]
pub struct IOSNPD {
    p: libc::c_int,
    up: *mut IOSNPD,
    level: libc::c_int,
    count: libc::c_int,
    b_ptr: *mut IOSBND,
    s_ptr: *mut IOSTAT,
    r_ptr: *mut IOSROW,
    solved: libc::c_int,
    lp_obj: libc::c_double,
    bound: libc::c_double,
    ii_cnt: libc::c_int,
    ii_sum: libc::c_double,
    changed: libc::c_int,
    br_var: libc::c_int,
    br_val: libc::c_double,
    data: *mut libc::c_void,
    temp: *mut IOSNPD,
    prev: *mut IOSNPD,
    next: *mut IOSNPD,
}

#[repr(C)]
pub struct IOSROW {
    name: *mut libc::c_char,
    origin: libc::c_uchar,
    klass: libc::c_uchar,
    type_: libc::c_uchar,
    lb: libc::c_double,
    ub: libc::c_double,
    ptr: *mut IOSAIJ,
    rii: libc::c_double,
    stat: libc::c_uchar,
    next: *mut IOSROW,
}

#[repr(C)]
pub struct IOSAIJ {
    j: libc::c_int,
    val: libc::c_double,
    next: *mut IOSAIJ,
}

#[repr(C)]
pub struct IOSTAT {
    k: libc::c_int,
    stat: libc::c_uchar,
    next: *mut IOSTAT,
}

#[repr(C)]
pub struct IOSBND {
    k: libc::c_int,
    type_: libc::c_uchar,
    lb: libc::c_double,
    ub: libc::c_double,
    next: *mut IOSBND,
}

#[repr(C)]
pub struct IOSLOT {
    node: *mut IOSNPD,
    next: libc::c_int,
}

#[no_mangle]
pub extern "C" fn _glp_ios_proxy_heur(T: *mut glp_tree) {
    unsafe {
        if (*T).curr.is_null() || (*(*T).curr).level != 0 || (*(*T).curr).solved != 1 {
            return;
        }

        let prob = glp_create_prob();
        if prob.is_null() {
            return;
        }

        glp_copy_prob(prob, (*T).mip, 0);

        let n = (*prob).n;
        let mut xstar = Vec::with_capacity((n + 1) as usize);
        xstar.resize((n + 1) as usize, 0.0);

        let status = if (*(*T).mip).mip_stat != 2 {
            let mut zstar = 0.0;
            _glp_proxy(
                prob,
                &mut zstar,
                xstar.as_mut_ptr(),
                ptr::null(),
                0.0,
                (*(*T).parm).ps_tm_lim,
                1,
            )
        } else {
            let mut xinit = Vec::with_capacity((n + 1) as usize);
            for j in 1..=n {
                xinit.push((*((*(*T).mip).col.offset(j as isize))).mipx);
            }
            let mut zstar = 0.0;
            let status = _glp_proxy(
                prob,
                &mut zstar,
                xstar.as_mut_ptr(),
                xinit.as_ptr(),
                0.0,
                (*(*T).parm).ps_tm_lim,
                1,
            );
            status
        };

        if status == 0 {
            let mut ae_max = 0.0;
            let mut re_max = 0.0;
            let mut ae_ind = 0;
            let mut re_ind = 0;

            glp_copy_prob(prob, (*T).mip, 0);
            for j in 1..=n {
                (*((*prob).col.offset(j as isize))).mipx = xstar[j as usize];
            }

            for i in 1..=(*prob).m {
                let row = *((*prob).row).offset(i as isize);
                (*row).mipx = 0.0;
                let mut aij = (*row).ptr;
                while !aij.is_null() {
                    (*row).mipx += (*aij).val * (*(*aij).col).mipx;
                    aij = (*aij).r_next;
                }
            }

            glp_check_kkt(
                prob,
                3,
                1,
                &mut ae_max,
                &mut ae_ind,
                &mut re_max,
                &mut re_ind,
            );
            let feas1 = (re_max <= 1e-6) as libc::c_int;

            glp_check_kkt(
                prob,
                3,
                2,
                &mut ae_max,
                &mut ae_ind,
                &mut re_max,
                &mut re_ind,
            );
            let feas2 = (re_max <= 1e-6) as libc::c_int;

            if feas1 != 0 && feas2 != 0 {
                glp_ios_heur_sol(T, xstar.as_ptr());
            } else {
                let msg = CString::new("WARNING: PROXY HEURISTIC REPORTED WRONG SOLUTION; SOLUTION REJECTED\n").unwrap();
                glp_printf(msg.as_ptr());
            }
        }

        glp_delete_prob(prob);
    }
}