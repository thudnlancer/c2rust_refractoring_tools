use std::os::raw::{c_int, c_double, c_char, c_void};
use std::ptr;
use std::mem;
use std::f64;

#[repr(C)]
struct FVS {
    n: c_int,
    nnz: c_int,
    ind: *mut c_int,
    vec: *mut c_double,
}

#[repr(C)]
struct SPXLP {
    m: c_int,
    n: c_int,
    nnz: c_int,
    A_ptr: *mut c_int,
    A_ind: *mut c_int,
    A_val: *mut c_double,
    b: *mut c_double,
    c: *mut c_double,
    l: *mut c_double,
    u: *mut c_double,
    head: *mut c_int,
    flag: *mut c_char,
    valid: c_int,
    bfd: *mut c_void,
}

#[repr(C)]
struct SPXAT {
    ptr: *mut c_int,
    ind: *mut c_int,
    val: *mut c_double,
    work: *mut c_double,
}

#[repr(C)]
struct SPXNT {
    ptr: *mut c_int,
    len: *mut c_int,
    ind: *mut c_int,
    val: *mut c_double,
}

#[repr(C)]
struct SPYSE {
    valid: c_int,
    refsp: *mut c_char,
    gamma: *mut c_double,
    work: *mut c_double,
    u: FVS,
}

#[repr(C)]
struct SPYBP {
    j: c_int,
    teta: c_double,
    dz: c_double,
}

#[repr(C)]
struct csa {
    lp: *mut SPXLP,
    dir: c_int,
    fz: c_double,
    orig_b: *mut c_double,
    orig_c: *mut c_double,
    orig_l: *mut c_double,
    orig_u: *mut c_double,
    at: *mut SPXAT,
    nt: *mut SPXNT,
    phase: c_int,
    beta: *mut c_double,
    beta_st: c_int,
    d: *mut c_double,
    d_st: c_int,
    se: *mut SPYSE,
    r: FVS,
    p: c_int,
    trow: FVS,
    bp: *mut SPYBP,
    q: c_int,
    tcol: FVS,
    work: *mut c_double,
    work1: *mut c_double,
    p_stat: c_int,
    d_stat: c_int,
    msg_lev: c_int,
    dualp: c_int,
    r_test: c_int,
    tol_bnd: c_double,
    tol_bnd1: c_double,
    tol_dj: c_double,
    tol_dj1: c_double,
    tol_piv: c_double,
    obj_lim: c_double,
    it_lim: c_int,
    tm_lim: c_int,
    out_frq: c_int,
    out_dly: c_int,
    tm_beg: c_double,
    it_beg: c_int,
    it_cnt: c_int,
    it_dpy: c_int,
    tm_dpy: c_double,
    inv_cnt: c_int,
    degen: c_int,
    ns_cnt: c_int,
    ls_cnt: c_int,
}

#[repr(C)]
struct glp_prob {
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
    row: *mut *mut c_void,
    col: *mut *mut c_void,
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
struct glp_smcp {
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

extern "C" {
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
    fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    fn glp_free(ptr: *mut c_void);
    fn glp_time() -> c_double;
    fn glp_difftime(t1: c_double, t0: c_double) -> c_double;
    fn glp_printf(fmt: *const c_char, ...);
    fn _glp_fvs_alloc_vec(x: *mut FVS, n: c_int);
    fn _glp_fvs_gather_vec(x: *mut FVS, eps: c_double);
    fn _glp_fvs_adjust_vec(x: *mut FVS, eps: c_double);
    fn _glp_fvs_free_vec(x: *mut FVS);
    fn _glp_bfd_condest(bfd: *mut c_void) -> c_double;
    fn _glp_bfd_get_count(bfd: *mut c_void) -> c_int;
    fn _glp_spx_factorize(lp: *mut SPXLP) -> c_int;
    fn _glp_spx_eval_beta(lp: *mut SPXLP, beta: *mut c_double);
    fn _glp_spx_eval_obj(lp: *mut SPXLP, beta: *const c_double) -> c_double;
    fn _glp_spx_eval_pi(lp: *mut SPXLP, pi: *mut c_double);
    fn _glp_spx_eval_dj(lp: *mut SPXLP, pi: *const c_double, j: c_int) -> c_double;
    fn _glp_spx_eval_tcol(lp: *mut SPXLP, j: c_int, tcol: *mut c_double);
    fn _glp_spx_eval_rho(lp: *mut SPXLP, i: c_int, rho: *mut c_double);
    fn _glp_spx_update_beta_s(
        lp: *mut SPXLP,
        beta: *mut c_double,
        p: c_int,
        p_flag: c_int,
        q: c_int,
        tcol: *const FVS,
    );
    fn _glp_spx_build_at(lp: *mut SPXLP, at: *mut SPXAT);
    fn _glp_spx_update_d_s(
        lp: *mut SPXLP,
        d: *mut c_double,
        p: c_int,
        q: c_int,
        trow: *const FVS,
        tcol: *const FVS,
    ) -> c_double;
    fn _glp_spx_alloc_at(lp: *mut SPXLP, at: *mut SPXAT);
    fn _glp_spx_change_basis(lp: *mut SPXLP, p: c_int, p_flag: c_int, q: c_int);
    fn _glp_spx_update_invb(lp: *mut SPXLP, i: c_int, k: c_int) -> c_int;
    fn _glp_spx_eval_trow1(
        lp: *mut SPXLP,
        at: *mut SPXAT,
        rho: *const c_double,
        trow: *mut c_double,
    );
    fn _glp_spx_free_at(lp: *mut SPXLP, at: *mut SPXAT);
    fn _glp_spx_alloc_nt(lp: *mut SPXLP, nt: *mut SPXNT);
    fn _glp_spx_init_nt(lp: *mut SPXLP, nt: *mut SPXNT);
    fn _glp_spx_build_nt(lp: *mut SPXLP, nt: *mut SPXNT);
    fn _glp_spx_update_nt(lp: *mut SPXLP, nt: *mut SPXNT, p: c_int, q: c_int);
    fn _glp_spx_nt_prod(
        lp: *mut SPXLP,
        nt: *mut SPXNT,
        y: *mut c_double,
        ign: c_int,
        s: c_double,
        x: *const c_double,
    );
    fn _glp_spx_free_nt(lp: *mut SPXLP, nt: *mut SPXNT);
    fn _glp_spx_init_lp(lp: *mut SPXLP, P: *mut glp_prob, excl: c_int);
    fn _glp_spx_alloc_lp(lp: *mut SPXLP);
    fn _glp_spx_build_lp(
        lp: *mut SPXLP,
        P: *mut glp_prob,
        excl: c_int,
        shift: c_int,
        map: *mut c_int,
    );
    fn _glp_spx_build_basis(lp: *mut SPXLP, P: *mut glp_prob, map: *const c_int);
    fn _glp_spx_store_basis(
        lp: *mut SPXLP,
        P: *mut glp_prob,
        map: *const c_int,
        daeh: *mut c_int,
    );
    fn _glp_spx_store_sol(
        lp: *mut SPXLP,
        P: *mut glp_prob,
        shift: c_int,
        map: *const c_int,
        daeh: *const c_int,
        beta: *const c_double,
        pi: *const c_double,
        d: *const c_double,
    );
    fn _glp_spx_free_lp(lp: *mut SPXLP);
    fn _glp_spy_chuzc_std(
        lp: *mut SPXLP,
        d: *const c_double,
        r: c_double,
        trow: *const c_double,
        tol_piv: c_double,
        tol: c_double,
        tol1: c_double,
    ) -> c_int;
    fn _glp_spy_chuzc_harris(
        lp: *mut SPXLP,
        d: *const c_double,
        r: c_double,
        trow: *const c_double,
        tol_piv: c_double,
        tol: c_double,
        tol1: c_double,
    ) -> c_int;
    fn _glp_spy_ls_eval_bp(
        lp: *mut SPXLP,
        d: *const c_double,
        r: c_double,
        trow: *const c_double,
        tol_piv: c_double,
        bp: *mut SPYBP,
    ) -> c_int;
    fn _glp_spy_ls_select_bp(
        lp: *mut SPXLP,
        trow: *const c_double,
        nbp: c_int,
        bp: *mut SPYBP,
        num: c_int,
        slope: *mut c_double,
        teta_lim: c_double,
    ) -> c_int;
    fn _glp_spy_chuzr_sel(
        lp: *mut SPXLP,
        beta: *const c_double,
        tol: c_double,
        tol1: c_double,
        list: *mut c_int,
    ) -> c_int;
    fn _glp_spy_chuzr_std(
        lp: *mut SPXLP,
        beta: *const c_double,
        num: c_int,
        list: *const c_int,
    ) -> c_int;
    fn _glp_spy_alloc_se(lp: *mut SPXLP, se: *mut SPYSE);
    fn _glp_spy_reset_refsp(lp: *mut SPXLP, se: *mut SPYSE);
    fn _glp_spy_chuzr_pse(
        lp: *mut SPXLP,
        se: *mut SPYSE,
        beta: *const c_double,
        num: c_int,
        list: *const c_int,
    ) -> c_int;
    fn _glp_spy_update_gamma_s(
        lp: *mut SPXLP,
        se: *mut SPYSE,
        p: c_int,
        q: c_int,
        trow: *const FVS,
        tcol: *const FVS,
    ) -> c_double;
    fn _glp_spy_free_se(lp: *mut SPXLP, se: *mut SPYSE);
}

#[no_mangle]
pub unsafe extern "C" fn _glp_spy_dual(P: *mut glp_prob, parm: *const glp_smcp) -> c_int {
    let mut csa_: csa = mem::zeroed();
    let mut csa = &mut csa_ as *mut csa;
    let mut lp: SPXLP = mem::zeroed();
    let mut at: SPXAT = mem::zeroed();
    let mut nt: SPXNT = mem::zeroed();
    let mut se: SPYSE = mem::zeroed();
    let mut ret: c_int = 0;
    let mut map: *mut c_int = ptr::null_mut();
    let mut daeh: *mut c_int = ptr::null_mut();
    let mut i: c_int = 0;
    let mut j: c_int = 0;
    let mut k: c_int = 0;

    (*csa).lp = &mut lp;
    _glp_spx_init_lp((*csa).lp, P, (*parm).excl);
    _glp_spx_alloc_lp((*csa).lp);
    
    map = glp_alloc(
        1 + (*P).m + (*P).n,
        mem::size_of::<c_int>() as c_int,
    ) as *mut c_int;
    
    _glp_spx_build_lp((*csa).lp, P, (*parm).excl, (*parm).shift, map);
    _glp_spx_build_basis((*csa).lp, P, map);

    match (*P).dir {
        1 => (*csa).dir = 1,
        2 => (*csa).dir = -1,
        _ => glp_assert_(
            b"P != P\0".as_ptr() as *const c_char,
            b"simplex/spydual.c\0".as_ptr() as *const c_char,
            1859,
        ),
    }

    (*csa).fz = 0.0;
    k = 1;
    while k <= (*(*csa).lp).n {
        let t = f64::abs(*((*(*csa).lp).c.offset(k as isize));
        if (*csa).fz < t {
            (*csa).fz = t;
        }
        k += 1;
    }

    if (*csa).fz <= 1000.0 {
        (*csa).fz = 1.0;
    } else {
        (*csa).fz /= 1000.0;
    }

    k = 0;
    while k <= (*(*csa).lp).n {
        *((*(*csa).lp).c.offset(k as isize)) /= (*csa).fz;
        k += 1;
    }

    (*csa).orig_b = glp_alloc(
        1 + (*(*csa).lp).m,
        mem::size_of::<c_double>() as c_int,
    ) as *mut c_double;
    
    ptr::copy_nonoverlapping(
        (*(*csa).lp).b,
        (*csa).orig_b,
        (1 + (*(*csa).lp).m) as usize,
    );

    (*csa).orig_c = glp_alloc(
        1 + (*(*csa).lp).n,
        mem::size_of::<c_double>() as c_int,
    ) as *mut c_double;
    
    ptr::copy_nonoverlapping(
        (*(*csa).lp).c,
        (*csa).orig_c,
        (1 + (*(*csa).lp).n) as usize,
    );

    (*csa).orig_l = glp_alloc(
        1 + (*(*csa).lp).n,
        mem::size_of::<c_double>() as c_int,
    ) as *mut c_double;
    
    ptr::copy_nonoverlapping(
        (*(*csa).lp).l,
        (*csa).orig_l,
        (1 + (*(*csa).lp).n) as usize,
    );

    (*csa).orig_u = glp_alloc(
        1 + (*(*csa).lp).n,
        mem::size_of::<c_double>() as c_int,
    ) as *mut c_double;
    
    ptr::copy_nonoverlapping(
        (*(*csa).lp).u,
        (*csa).orig_u,
        (1 + (*(*csa).lp).n) as usize,
    );

    match