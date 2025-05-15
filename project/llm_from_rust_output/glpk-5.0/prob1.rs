use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_uchar, c_ushort, c_void};
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IOSPOOL {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut c_char,
    pub obj: *mut c_char,
    pub dir: c_int,
    pub c0: c_double,
    pub m_max: c_int,
    pub n_max: c_int,
    pub m: c_int,
    pub n: c_int,
    pub nnz: c_int,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: c_int,
    pub head: *mut c_int,
    pub bfd: *mut BFD,
    pub pbs_stat: c_int,
    pub dbs_stat: c_int,
    pub obj_val: c_double,
    pub it_cnt: c_int,
    pub some: c_int,
    pub ipt_stat: c_int,
    pub ipt_obj: c_double,
    pub mip_stat: c_int,
    pub mip_obj: c_double,
}

#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct IOSAIJ {
    j: c_int,
    val: c_double,
    next: *mut IOSAIJ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IOSTAT {
    k: c_int,
    stat: c_uchar,
    next: *mut IOSTAT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IOSBND {
    k: c_int,
    type_: c_uchar,
    lb: c_double,
    ub: c_double,
    next: *mut IOSBND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IOSLOT {
    node: *mut IOSNPD,
    next: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct glp_bfcp {
    msg_lev: c_int,
    type_: c_int,
    lu_size: c_int,
    piv_tol: c_double,
    piv_lim: c_int,
    suhl: c_int,
    eps_tol: c_double,
    max_gro: c_double,
    nfs_max: c_int,
    upd_tol: c_double,
    nrs_max: c_int,
    rs_size: c_int,
    foo_bar: [c_double; 38],
}

type AVL = c_void;
type AVLNODE = c_void;
type BFD = c_void;
type DMP = c_void;
type glp_cfg = c_void;
type glp_cov = c_void;
type glp_mir = c_void;

const _ISalnum: u32 = 8;
const _ISpunct: u32 = 4;
const _IScntrl: u32 = 2;
const _ISblank: u32 = 1;
const _ISgraph: u32 = 32768;
const _ISprint: u32 = 16384;
const _ISspace: u32 = 8192;
const _ISxdigit: u32 = 4096;
const _ISdigit: u32 = 2048;
const _ISalpha: u32 = 1024;
const _ISlower: u32 = 512;
const _ISupper: u32 = 256;

type glp_errfunc = Option<unsafe extern "C" fn(*const c_char, ...)>;

extern "C" {
    fn __ctype_b_loc() -> *mut *const c_ushort;
    fn fabs(_: c_double) -> c_double;
    fn glp_free(ptr: *mut c_void);
    fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
    fn glp_error_(file: *const c_char, line: c_int) -> glp_errfunc;
    fn strlen(_: *const c_char) -> usize;
    fn memcpy(_: *mut c_void, _: *const c_void, _: usize) -> *mut c_void;
    fn strcpy(_: *mut c_char, _: *const c_char) -> *mut c_char;
    fn glp_get_mat_col(
        P: *mut glp_prob,
        j: c_int,
        ind: *mut c_int,
        val: *mut c_double,
    ) -> c_int;
    fn glp_get_bfcp(P: *mut glp_prob, parm: *mut glp_bfcp);
    fn glp_set_bfcp(P: *mut glp_prob, parm: *const glp_bfcp);
    fn _glp_avl_insert_node(tree: *mut AVL, key: *const c_void) -> *mut AVLNODE;
    fn _glp_avl_set_node_link(node: *mut AVLNODE, link: *mut c_void);
    fn _glp_avl_delete_tree(tree: *mut AVL);
    fn _glp_avl_delete_node(tree: *mut AVL, node: *mut AVLNODE);
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut c_void, size: c_int);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: c_int) -> *mut c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_bfd_delete_it(bfd: *mut BFD);
    fn _glp_dmp_delete_pool(pool: *mut DMP);
}

unsafe fn create_prob(lp: *mut glp_prob) {
    (*lp).pool = _glp_dmp_create_pool();
    (*lp).tree = ptr::null_mut();
    (*lp).name = ptr::null_mut();
    (*lp).obj = ptr::null_mut();
    (*lp).dir = 1;
    (*lp).c0 = 0.0;
    (*lp).m_max = 100;
    (*lp).n_max = 200;
    (*lp).n = 0;
    (*lp).m = (*lp).n;
    (*lp).nnz = 0;
    (*lp).row = glp_alloc(
        1 + (*lp).m_max,
        std::mem::size_of::<*mut GLPROW>() as c_int,
    ) as *mut *mut GLPROW;
    (*lp).col = glp_alloc(
        1 + (*lp).n_max,
        std::mem::size_of::<*mut GLPCOL>() as c_int,
    ) as *mut *mut GLPCOL;
    (*lp).c_tree = ptr::null_mut();
    (*lp).r_tree = (*lp).c_tree;
    (*lp).valid = 0;
    (*lp).head = glp_alloc(
        1 + (*lp).m_max,
        std::mem::size_of::<c_int>() as c_int,
    ) as *mut c_int;
    (*lp).bfd = ptr::null_mut();
    (*lp).dbs_stat = 1;
    (*lp).pbs_stat = (*lp).dbs_stat;
    (*lp).obj_val = 0.0;
    (*lp).it_cnt = 0;
    (*lp).some = 0;
    (*lp).ipt_stat = 1;
    (*lp).ipt_obj = 0.0;
    (*lp).mip_stat = 1;
    (*lp).mip_obj = 0.0;
}

#[no_mangle]
pub unsafe extern "C" fn glp_create_prob() -> *mut glp_prob {
    let lp = glp_alloc(1, std::mem::size_of::<glp_prob>() as c_int) as *mut glp_prob;
    create_prob(lp);
    lp
}

#[no_mangle]
pub unsafe extern "C" fn glp_set_prob_name(lp: *mut glp_prob, name: *const c_char) {
    let tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 {
        (glp_error_(
            b"api/prob1.c\0".as_ptr() as *const c_char,
            135,
        ))
        .expect("non-null function pointer")(
            b"glp_set_prob_name: operation not allowed\n\0".as_ptr() as *const c_char,
        );
    }

    if !(*lp).name.is_null() {
        _glp_dmp_free_atom(
            (*lp).pool,
            (*lp).name as *mut c_void,
            (strlen((*lp).name) + 1,
        );
        (*lp).name = ptr::null_mut();
    }

    if !(name.is_null() || *name == 0) {
        let mut k = 0;
        while *name.offset(k) != 0 {
            if k == 256 {
                (glp_error_(
                    b"api/prob1.c\0".as_ptr() as *const c_char,
                    144,
                ))
                .expect("non-null function pointer")(
                    b"glp_set_prob_name: problem name too long\n\0".as_ptr() as *const c_char,
                );
            }
            if *(*__ctype_b_loc()).offset(*name.offset(k) as isize) as c_int & _IScntrl as c_int != 0 {
                (glp_error_(
                    b"api/prob1.c\0".as_ptr() as *const c_char,
                    146,
                ))
                .expect("non-null function pointer")(
                    b"glp_set_prob_name: problem name contains invalid character(s)\n\0"
                        .as_ptr() as *const c_char,
                );
            }
            k += 1;
        }
        (*lp).name = _glp_dmp_get_atom((*lp).pool, (strlen(name) + 1) as c_int) as *mut c_char;
        strcpy((*lp).name, name);
    }
}

// 其他函数类似转换...

unsafe fn delete_prob(lp: *mut glp_prob) {
    _glp_dmp_delete_pool((*lp).pool);
    assert!((*lp).tree.is_null(), "lp->tree should be NULL");
    glp_free((*lp).row as *mut c_void);
    glp_free((*lp).col as *mut c_void);
    if !(*lp).r_tree.is_null() {
        _glp_avl_delete_tree((*lp).r_tree);
    }
