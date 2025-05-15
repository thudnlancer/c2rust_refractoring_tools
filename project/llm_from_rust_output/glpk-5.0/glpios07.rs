use std::ffi::c_void;
use std::os::raw::{c_char, c_double, c_int, c_uchar};

extern "C" {
    fn fabs(x: c_double) -> c_double;
    fn glp_free(ptr: *mut c_void);
    fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
    fn glp_error_(file: *const c_char, line: c_int) -> Option<unsafe extern "C" fn(*const c_char, ...)>;
    fn glp_get_num_rows(P: *mut glp_prob) -> c_int;
    fn glp_get_num_cols(P: *mut glp_prob) -> c_int;
    fn glp_get_row_type(P: *mut glp_prob, i: c_int) -> c_int;
    fn glp_get_row_lb(P: *mut glp_prob, i: c_int) -> c_double;
    fn glp_get_row_ub(P: *mut glp_prob, i: c_int) -> c_double;
    fn glp_get_col_type(P: *mut glp_prob, j: c_int) -> c_int;
    fn glp_get_col_ub(P: *mut glp_prob, j: c_int) -> c_double;
    fn glp_get_mat_row(P: *mut glp_prob, i: c_int, ind: *mut c_int, val: *mut c_double) -> c_int;
    fn glp_get_col_kind(P: *mut glp_prob, j: c_int) -> c_int;
    fn glp_get_status(P: *mut glp_prob) -> c_int;
    fn glp_get_col_prim(P: *mut glp_prob, j: c_int) -> c_double;
    fn glp_get_col_lb(P: *mut glp_prob, j: c_int) -> c_double;
    fn glp_ios_add_row(
        T: *mut glp_tree,
        name: *const c_char,
        klass: c_int,
        flags: c_int,
        len: c_int,
        ind: *const c_int,
        val: *const c_double,
        type_: c_int,
        rhs: c_double,
    ) -> c_int;
}

#[repr(C)]
pub struct glp_prob {
    pub pool: *mut c_void,
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
    pub r_tree: *mut c_void,
    pub c_tree: *mut c_void,
    pub valid: c_int,
    pub head: *mut c_int,
    pub bfd: *mut c_void,
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
pub struct GLPCOL {
    pub j: c_int,
    pub name: *mut c_char,
    pub node: *mut c_void,
    pub kind: c_int,
    pub type_: c_int,
    pub lb: c_double,
    pub ub: c_double,
    pub coef: c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: c_double,
    pub stat: c_int,
    pub bind: c_int,
    pub prim: c_double,
    pub dual: c_double,
    pub pval: c_double,
    pub dval: c_double,
    pub mipx: c_double,
}

#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}

#[repr(C)]
pub struct GLPROW {
    pub i: c_int,
    pub name: *mut c_char,
    pub node: *mut c_void,
    pub level: c_int,
    pub origin: c_uchar,
    pub klass: c_uchar,
    pub type_: c_int,
    pub lb: c_double,
    pub ub: c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: c_double,
    pub stat: c_int,
    pub bind: c_int,
    pub prim: c_double,
    pub dual: c_double,
    pub pval: c_double,
    pub dval: c_double,
    pub mipx: c_double,
}

#[repr(C)]
pub struct glp_tree {
    pub magic: c_int,
    pub pool: *mut c_void,
    pub n: c_int,
    pub orig_m: c_int,
    pub orig_type: *mut c_uchar,
    pub orig_lb: *mut c_double,
    pub orig_ub: *mut c_double,
    pub orig_stat: *mut c_uchar,
    pub orig_prim: *mut c_double,
    pub orig_dual: *mut c_double,
    pub orig_obj: c_double,
    pub nslots: c_int,
    pub avail: c_int,
    pub slot: *mut c_void,
    pub head: *mut c_void,
    pub tail: *mut c_void,
    pub a_cnt: c_int,
    pub n_cnt: c_int,
    pub t_cnt: c_int,
    pub root_m: c_int,
    pub root_type: *mut c_uchar,
    pub root_lb: *mut c_double,
    pub root_ub: *mut c_double,
    pub root_stat: *mut c_uchar,
    pub curr: *mut c_void,
    pub mip: *mut glp_prob,
    pub non_int: *mut c_uchar,
    pub pred_m: c_int,
    pub pred_max: c_int,
    pub pred_type: *mut c_uchar,
    pub pred_lb: *mut c_double,
    pub pred_ub: *mut c_double,
    pub pred_stat: *mut c_uchar,
    pub local: *mut c_void,
    pub cov_gen: *mut c_void,
    pub mir_gen: *mut c_void,
    pub clq_gen: *mut c_void,
    pub pcost: *mut c_void,
    pub iwrk: *mut c_int,
    pub dwrk: *mut c_double,
    pub parm: *const glp_iocp,
    pub tm_beg: c_double,
    pub tm_lag: c_double,
    pub sol_cnt: c_int,
    pub P: *mut c_void,
    pub npp: *mut c_void,
    pub save_sol: *const c_char,
    pub save_cnt: c_int,
    pub reason: c_int,
    pub stop: c_int,
    pub next_p: c_int,
    pub reopt: c_int,
    pub reinv: c_int,
    pub br_var: c_int,
    pub br_sel: c_int,
    pub child: c_int,
}

#[repr(C)]
pub struct glp_iocp {
    pub msg_lev: c_int,
    pub br_tech: c_int,
    pub bt_tech: c_int,
    pub tol_int: c_double,
    pub tol_obj: c_double,
    pub tm_lim: c_int,
    pub out_frq: c_int,
    pub out_dly: c_int,
    pub cb_func: Option<unsafe extern "C" fn(*mut glp_tree, *mut c_void)>,
    pub cb_info: *mut c_void,
    pub cb_size: c_int,
    pub pp_tech: c_int,
    pub mip_gap: c_double,
    pub mir_cuts: c_int,
    pub gmi_cuts: c_int,
    pub cov_cuts: c_int,
    pub clq_cuts: c_int,
    pub presolve: c_int,
    pub binarize: c_int,
    pub fp_heur: c_int,
    pub ps_heur: c_int,
    pub ps_tm_lim: c_int,
    pub sr_heur: c_int,
    pub use_sol: c_int,
    pub save_sol: *const c_char,
    pub alien: c_int,
    pub flip: c_int,
    pub foo_bar: [c_double; 23],
}

fn cover2(
    n: c_int,
    a: *mut c_double,
    b: c_double,
    u: c_double,
    x: *mut c_double,
    y: c_double,
    cov: *mut c_int,
    _alfa: *mut c_double,
    _beta: *mut c_double,
) -> c_int {
    unsafe {
        let mut i: c_int = 0;
        let mut j: c_int = 0;
        let mut try_: c_int = 0;
        let mut ret: c_int = 0;
        let eps = 0.001 * (1.0 + fabs(b));
        let mut rmax = 0.001;
        
        i = 1;
        'outer: while i <= n {
            j = i + 1;
            while j <= n {
                try_ += 1;
                if try_ > 1000 {
                    break 'outer;
                }
                if *a.offset(i as isize) + *a.offset(j as isize) + y > b + eps {
                    let temp = *a.offset(i as isize) + *a.offset(j as isize) - b;
                    let alfa = 1.0 / (temp + u);
                    let beta = 2.0 - alfa * temp;
                    let temp = *x.offset(i as isize) + *x.offset(j as isize) + alfa * y - beta;
                    if rmax < temp {
                        rmax = temp;
                        *cov.offset(1) = i;
                        *cov.offset(2) = j;
                        *_alfa = alfa;
                        *_beta = beta;
                        ret = 1;
                    }
                }
                j += 1;
            }
            i += 1;
        }
        ret
    }
}

fn cover3(
    n: c_int,
    a: *mut c_double,
    b: c_double,
    u: c_double,
    x: *mut c_double,
    y: c_double,
    cov: *mut c_int,
    _alfa: *mut c_double,
    _beta: *mut c_double,
) -> c_int {
    unsafe {
        let mut i: c_int = 0;
        let mut j: c_int = 0;
        let mut k: c_int = 0;
        let mut try_: c_int = 0;
        let mut ret: c_int = 0;
        let eps = 0.001 * (1.0 + fabs(b));
        let mut rmax = 0.001;
        
        i = 1;
        'outer: while i <= n {
            j = i + 1;
            while j <= n {
                k = j + 1;
                while k <= n {
                    try_ += 1;
                    if try_ > 1000 {
                        break 'outer;
                    }
                    if *a.offset(i as isize) + *a.offset(j as isize) + *a.offset(k as isize) + y > b + eps {
                        let temp = *a.offset(i as isize) + *a.offset(j as isize) + *a.offset(k as isize) - b;
                        let alfa = 1.0 / (temp + u);
                        let beta = 3.0 - alfa * temp;
                        let temp = *x.offset(i as isize) + *x.offset(j as isize) + *x.offset(k as isize) + alfa * y - beta;
                        if rmax < temp {
                            rmax = temp;
                            *cov.offset(1) = i;
                            *cov.offset(2) = j;
                            *cov.offset(3) = k;
                            *_alfa = alfa;
                            *_beta = beta;
                            ret = 1;
                        }
                    }
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
        ret
    }
}

fn cover4(
    n: c_int,
    a: *mut c_double,
    b: c_double,
    u: c_double,
    x: *mut c_double,
    y: c_double,
    cov: *mut c_int,
    _alfa: *mut c_double,
    _beta: *mut c_double,
) -> c_int {
    unsafe {
        let mut i: c_int = 0;
        let mut j: c_int = 0;
        let mut k: c_int = 0;
        let mut l: c_int = 0;
        let mut try_: c_int = 0;
        let mut ret: c_int = 0;
        let eps = 0.001 * (1.0 + fabs(b));
        let mut rmax = 0.001;
        
        i = 1;
        'outer: while i <= n {
            j = i + 1;
            while j <= n {
                k = j + 1;
                while k <= n {
                    l = k + 1;
                    while l <= n {
                        try_ += 1;
                        if try_ > 1000 {
                            break 'outer;
                        }
                        if *a.offset(i as isize) + *a.offset(j as isize) + *a.offset(k as isize) + *a.offset(l as isize) + y > b + eps {
                            let temp = *a.offset(i as isize) + *a.offset(j as isize) + *a.offset(k as isize) + *a.offset(l as isize) - b;
                            let alfa = 1.0 / (temp + u);
                            let beta = 4.0 - alfa * temp;
                            let temp = *x.offset(i as isize) + *x.offset(j as isize) + *x.offset(k as isize) + *x.offset(l as isize) + alfa * y - beta;
                            if rmax < temp {
                                rmax = temp;
                                *cov.offset(1) = i;
                                *cov.offset(2) = j;
                                *cov.offset(3) = k;
                                *cov.offset(4) = l;
                                *_alfa = alfa;
                                *_beta = beta;
                                ret = 1;
                            }
                        }
                        l += 1;
                    }
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
        ret
    }
}

fn cover(
    n: c_int,
    a: *mut c_double,
    b: c_double,
    u: c_double,
    x: *mut c_double,
    y: c_double,
    cov: *mut c_int,
    alfa: *mut c_double,
    beta: *mut c_double,
) -> c_int {
    unsafe {
        assert!(n >= 2);
        for j in 1..=n {
            assert!(*a.offset(j as isize) > 0.0);
        }
        assert!(b > -1e-5);
        assert!(u >= 0.0);
        for j in 1..=n {
            assert!(0.0 <= *x.offset(j as isize) && *x.offset(j as isize) <= 1.0);
        }
        assert!(0.0 <= y && y <= u);

        if cover2(n, a, b, u, x, y, cov, alfa, beta) != 0 {
            return 2;
        }
        if cover3(n, a, b, u, x, y, cov, alfa, beta) != 0 {
            return 3;
        }
        if cover4(n, a, b, u, x, y, cov, alfa, beta) != 0 {
            return 4;
        }
        0
    }
}

fn lpx_cover_cut(
    lp: *mut glp_prob,
    len: c_int,
    ind: *mut c_int,
    val: *mut c_double,
    work: *mut c_double,
) -> c_int {
    unsafe {
        let mut cov = [0; 5];
        let mut newlen = 0;
        let mut nb = 0;
        let mut r = 0;
        let mut f_min = 0.0;
        let mut f_max = 0.0;
        let mut alfa = 0.0;
        let mut beta = 0.0;
        let mut u = 0.0;
        let x = work;
        let mut y = 0.0;

        newlen = 0;
        for k in 1..=len {
            let j = *ind.offset(k as isize);
            if glp_get_col_type(lp, j) == 5 {
                *val.offset(0) -= *val.offset(k as isize) * glp_get_col_lb(lp, j);
            } else {
                newlen += 1;
                *ind.offset(newlen as isize) = *ind.offset(k as isize);
                *val.offset(newlen as isize) = *val.offset(k as isize);
            }
        }
        let len = newlen;

        nb = 0;
        for k in 