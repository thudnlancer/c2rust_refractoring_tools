#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_get_num_rows(P: *mut glp_prob) -> i32;
    fn glp_get_num_cols(P: *mut glp_prob) -> i32;
    fn glp_get_row_type(P: *mut glp_prob, i: i32) -> i32;
    fn glp_get_row_lb(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_row_ub(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_col_type(P: *mut glp_prob, j: i32) -> i32;
    fn glp_get_col_ub(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_mat_row(
        P: *mut glp_prob,
        i: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn glp_get_col_kind(P: *mut glp_prob, j: i32) -> i32;
    fn glp_get_status(P: *mut glp_prob) -> i32;
    fn glp_get_col_prim(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_lb(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_ios_add_row(
        T: *mut glp_tree,
        name: *const i8,
        klass: i32,
        flags: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
        type_0: i32,
        rhs: libc::c_double,
    ) -> i32;
}
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prob {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut i8,
    pub obj: *mut i8,
    pub dir: i32,
    pub c0: libc::c_double,
    pub m_max: i32,
    pub n_max: i32,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: i32,
    pub head: *mut i32,
    pub bfd: *mut BFD,
    pub pbs_stat: i32,
    pub dbs_stat: i32,
    pub obj_val: libc::c_double,
    pub it_cnt: i32,
    pub some: i32,
    pub ipt_stat: i32,
    pub ipt_obj: libc::c_double,
    pub mip_stat: i32,
    pub mip_obj: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPCOL {
    pub j: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub kind: i32,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPROW {
    pub i: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub level: i32,
    pub origin: u8,
    pub klass: u8,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_tree {
    pub magic: i32,
    pub pool: *mut DMP,
    pub n: i32,
    pub orig_m: i32,
    pub orig_type: *mut u8,
    pub orig_lb: *mut libc::c_double,
    pub orig_ub: *mut libc::c_double,
    pub orig_stat: *mut u8,
    pub orig_prim: *mut libc::c_double,
    pub orig_dual: *mut libc::c_double,
    pub orig_obj: libc::c_double,
    pub nslots: i32,
    pub avail: i32,
    pub slot: *mut IOSLOT,
    pub head: *mut IOSNPD,
    pub tail: *mut IOSNPD,
    pub a_cnt: i32,
    pub n_cnt: i32,
    pub t_cnt: i32,
    pub root_m: i32,
    pub root_type: *mut u8,
    pub root_lb: *mut libc::c_double,
    pub root_ub: *mut libc::c_double,
    pub root_stat: *mut u8,
    pub curr: *mut IOSNPD,
    pub mip: *mut glp_prob,
    pub non_int: *mut u8,
    pub pred_m: i32,
    pub pred_max: i32,
    pub pred_type: *mut u8,
    pub pred_lb: *mut libc::c_double,
    pub pred_ub: *mut libc::c_double,
    pub pred_stat: *mut u8,
    pub local: *mut IOSPOOL,
    pub cov_gen: *mut glp_cov,
    pub mir_gen: *mut glp_mir,
    pub clq_gen: *mut glp_cfg,
    pub pcost: *mut libc::c_void,
    pub iwrk: *mut i32,
    pub dwrk: *mut libc::c_double,
    pub parm: *const glp_iocp,
    pub tm_beg: libc::c_double,
    pub tm_lag: libc::c_double,
    pub sol_cnt: i32,
    pub P: *mut libc::c_void,
    pub npp: *mut libc::c_void,
    pub save_sol: *const i8,
    pub save_cnt: i32,
    pub reason: i32,
    pub stop: i32,
    pub next_p: i32,
    pub reopt: i32,
    pub reinv: i32,
    pub br_var: i32,
    pub br_sel: i32,
    pub child: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_iocp {
    pub msg_lev: i32,
    pub br_tech: i32,
    pub bt_tech: i32,
    pub tol_int: libc::c_double,
    pub tol_obj: libc::c_double,
    pub tm_lim: i32,
    pub out_frq: i32,
    pub out_dly: i32,
    pub cb_func: Option<unsafe extern "C" fn(*mut glp_tree, *mut libc::c_void) -> ()>,
    pub cb_info: *mut libc::c_void,
    pub cb_size: i32,
    pub pp_tech: i32,
    pub mip_gap: libc::c_double,
    pub mir_cuts: i32,
    pub gmi_cuts: i32,
    pub cov_cuts: i32,
    pub clq_cuts: i32,
    pub presolve: i32,
    pub binarize: i32,
    pub fp_heur: i32,
    pub ps_heur: i32,
    pub ps_tm_lim: i32,
    pub sr_heur: i32,
    pub use_sol: i32,
    pub save_sol: *const i8,
    pub alien: i32,
    pub flip: i32,
    pub foo_bar: [libc::c_double; 23],
}
pub type IOSPOOL = glp_prob;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSNPD {
    pub p: i32,
    pub up: *mut IOSNPD,
    pub level: i32,
    pub count: i32,
    pub b_ptr: *mut IOSBND,
    pub s_ptr: *mut IOSTAT,
    pub r_ptr: *mut IOSROW,
    pub solved: i32,
    pub lp_obj: libc::c_double,
    pub bound: libc::c_double,
    pub ii_cnt: i32,
    pub ii_sum: libc::c_double,
    pub changed: i32,
    pub br_var: i32,
    pub br_val: libc::c_double,
    pub data: *mut libc::c_void,
    pub temp: *mut IOSNPD,
    pub prev: *mut IOSNPD,
    pub next: *mut IOSNPD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSROW {
    pub name: *mut i8,
    pub origin: u8,
    pub klass: u8,
    pub type_0: u8,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut IOSAIJ,
    pub rii: libc::c_double,
    pub stat: u8,
    pub next: *mut IOSROW,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSAIJ {
    pub j: i32,
    pub val: libc::c_double,
    pub next: *mut IOSAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSTAT {
    pub k: i32,
    pub stat: u8,
    pub next: *mut IOSTAT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSBND {
    pub k: i32,
    pub type_0: u8,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub next: *mut IOSBND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSLOT {
    pub node: *mut IOSNPD,
    pub next: i32,
}
unsafe extern "C" fn cover2(
    mut n: i32,
    mut a: *mut libc::c_double,
    mut b: libc::c_double,
    mut u: libc::c_double,
    mut x: *mut libc::c_double,
    mut y: libc::c_double,
    mut cov: *mut i32,
    mut _alfa: *mut libc::c_double,
    mut _beta: *mut libc::c_double,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut try_0: i32 = 0 as i32;
    let mut ret: i32 = 0 as i32;
    let mut eps: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut rmax: libc::c_double = 0.001f64;
    eps = 0.001f64 * (1.0f64 + fabs(b));
    i = 0 as i32 + 1 as i32;
    's_9: while i <= n {
        j = i + 1 as i32;
        while j <= n {
            try_0 += 1;
            try_0;
            if try_0 > 1000 as i32 {
                break 's_9;
            }
            if *a.offset(i as isize) + *a.offset(j as isize) + y > b + eps {
                temp = *a.offset(i as isize) + *a.offset(j as isize) - b;
                alfa = 1.0f64 / (temp + u);
                beta = 2.0f64 - alfa * temp;
                temp = *x.offset(i as isize) + *x.offset(j as isize) + alfa * y - beta;
                if rmax < temp {
                    rmax = temp;
                    *cov.offset(1 as i32 as isize) = i;
                    *cov.offset(2 as i32 as isize) = j;
                    *_alfa = alfa;
                    *_beta = beta;
                    ret = 1 as i32;
                }
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return ret;
}
unsafe extern "C" fn cover3(
    mut n: i32,
    mut a: *mut libc::c_double,
    mut b: libc::c_double,
    mut u: libc::c_double,
    mut x: *mut libc::c_double,
    mut y: libc::c_double,
    mut cov: *mut i32,
    mut _alfa: *mut libc::c_double,
    mut _beta: *mut libc::c_double,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut try_0: i32 = 0 as i32;
    let mut ret: i32 = 0 as i32;
    let mut eps: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut rmax: libc::c_double = 0.001f64;
    eps = 0.001f64 * (1.0f64 + fabs(b));
    i = 0 as i32 + 1 as i32;
    's_9: while i <= n {
        j = i + 1 as i32;
        while j <= n {
            k = j + 1 as i32;
            while k <= n {
                try_0 += 1;
                try_0;
                if try_0 > 1000 as i32 {
                    break 's_9;
                }
                if *a.offset(i as isize) + *a.offset(j as isize) + *a.offset(k as isize)
                    + y > b + eps
                {
                    temp = *a.offset(i as isize) + *a.offset(j as isize)
                        + *a.offset(k as isize) - b;
                    alfa = 1.0f64 / (temp + u);
                    beta = 3.0f64 - alfa * temp;
                    temp = *x.offset(i as isize) + *x.offset(j as isize)
                        + *x.offset(k as isize) + alfa * y - beta;
                    if rmax < temp {
                        rmax = temp;
                        *cov.offset(1 as i32 as isize) = i;
                        *cov.offset(2 as i32 as isize) = j;
                        *cov.offset(3 as i32 as isize) = k;
                        *_alfa = alfa;
                        *_beta = beta;
                        ret = 1 as i32;
                    }
                }
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return ret;
}
unsafe extern "C" fn cover4(
    mut n: i32,
    mut a: *mut libc::c_double,
    mut b: libc::c_double,
    mut u: libc::c_double,
    mut x: *mut libc::c_double,
    mut y: libc::c_double,
    mut cov: *mut i32,
    mut _alfa: *mut libc::c_double,
    mut _beta: *mut libc::c_double,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut try_0: i32 = 0 as i32;
    let mut ret: i32 = 0 as i32;
    let mut eps: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut rmax: libc::c_double = 0.001f64;
    eps = 0.001f64 * (1.0f64 + fabs(b));
    i = 0 as i32 + 1 as i32;
    's_9: while i <= n {
        j = i + 1 as i32;
        while j <= n {
            k = j + 1 as i32;
            while k <= n {
                l = k + 1 as i32;
                while l <= n {
                    try_0 += 1;
                    try_0;
                    if try_0 > 1000 as i32 {
                        break 's_9;
                    }
                    if *a.offset(i as isize) + *a.offset(j as isize)
                        + *a.offset(k as isize) + *a.offset(l as isize) + y > b + eps
                    {
                        temp = *a.offset(i as isize) + *a.offset(j as isize)
                            + *a.offset(k as isize) + *a.offset(l as isize) - b;
                        alfa = 1.0f64 / (temp + u);
                        beta = 4.0f64 - alfa * temp;
                        temp = *x.offset(i as isize) + *x.offset(j as isize)
                            + *x.offset(k as isize) + *x.offset(l as isize) + alfa * y
                            - beta;
                        if rmax < temp {
                            rmax = temp;
                            *cov.offset(1 as i32 as isize) = i;
                            *cov.offset(2 as i32 as isize) = j;
                            *cov.offset(3 as i32 as isize) = k;
                            *cov.offset(4 as i32 as isize) = l;
                            *_alfa = alfa;
                            *_beta = beta;
                            ret = 1 as i32;
                        }
                    }
                    l += 1;
                    l;
                }
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return ret;
}
unsafe extern "C" fn cover(
    mut n: i32,
    mut a: *mut libc::c_double,
    mut b: libc::c_double,
    mut u: libc::c_double,
    mut x: *mut libc::c_double,
    mut y: libc::c_double,
    mut cov: *mut i32,
    mut alfa: *mut libc::c_double,
    mut beta: *mut libc::c_double,
) -> i32 {
    let mut j: i32 = 0;
    (n >= 2 as i32
        || {
            glp_assert_(
                b"n >= 2\0" as *const u8 as *const i8,
                b"draft/glpios07.c\0" as *const u8 as *const i8,
                261 as i32,
            );
            1 as i32 != 0
        }) as i32;
    j = 1 as i32;
    while j <= n {
        (*a.offset(j as isize) > 0.0f64
            || {
                glp_assert_(
                    b"a[j] > 0.0\0" as *const u8 as *const i8,
                    b"draft/glpios07.c\0" as *const u8 as *const i8,
                    262 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j += 1;
        j;
    }
    (b > -1e-5f64
        || {
            glp_assert_(
                b"b > -1e-5\0" as *const u8 as *const i8,
                b"draft/glpios07.c\0" as *const u8 as *const i8,
                264 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (u >= 0.0f64
        || {
            glp_assert_(
                b"u >= 0.0\0" as *const u8 as *const i8,
                b"draft/glpios07.c\0" as *const u8 as *const i8,
                268 as i32,
            );
            1 as i32 != 0
        }) as i32;
    j = 1 as i32;
    while j <= n {
        (0.0f64 <= *x.offset(j as isize) && *x.offset(j as isize) <= 1.0f64
            || {
                glp_assert_(
                    b"0.0 <= x[j] && x[j] <= 1.0\0" as *const u8 as *const i8,
                    b"draft/glpios07.c\0" as *const u8 as *const i8,
                    269 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j += 1;
        j;
    }
    (0.0f64 <= y && y <= u
        || {
            glp_assert_(
                b"0.0 <= y && y <= u\0" as *const u8 as *const i8,
                b"draft/glpios07.c\0" as *const u8 as *const i8,
                270 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if cover2(n, a, b, u, x, y, cov, alfa, beta) != 0 {
        return 2 as i32;
    }
    if cover3(n, a, b, u, x, y, cov, alfa, beta) != 0 {
        return 3 as i32;
    }
    if cover4(n, a, b, u, x, y, cov, alfa, beta) != 0 {
        return 4 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn lpx_cover_cut(
    mut lp: *mut glp_prob,
    mut len: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
    mut work: *mut libc::c_double,
) -> i32 {
    let mut cov: [i32; 5] = [0; 5];
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut nb: i32 = 0;
    let mut newlen: i32 = 0;
    let mut r: i32 = 0;
    let mut f_min: libc::c_double = 0.;
    let mut f_max: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut x: *mut libc::c_double = work;
    let mut y: libc::c_double = 0.;
    newlen = 0 as i32;
    k = 1 as i32;
    while k <= len {
        j = *ind.offset(k as isize);
        if glp_get_col_type(lp, j) == 5 as i32 {
            *val.offset(0 as i32 as isize)
                -= *val.offset(k as isize) * glp_get_col_lb(lp, j);
        } else {
            newlen += 1;
            newlen;
            *ind.offset(newlen as isize) = *ind.offset(k as isize);
            *val.offset(newlen as isize) = *val.offset(k as isize);
        }
        k += 1;
        k;
    }
    len = newlen;
    nb = 0 as i32;
    k = 1 as i32;
    while k <= len {
        j = *ind.offset(k as isize);
        if glp_get_col_kind(lp, j) == 3 as i32 {
            let mut ind_k: i32 = 0;
            let mut val_k: libc::c_double = 0.;
            nb += 1;
            nb;
            ind_k = *ind.offset(nb as isize);
            val_k = *val.offset(nb as isize);
            *ind.offset(nb as isize) = *ind.offset(k as isize);
            *val.offset(nb as isize) = *val.offset(k as isize);
            *ind.offset(k as isize) = ind_k;
            *val.offset(k as isize) = val_k;
        }
        k += 1;
        k;
    }
    if nb < 2 as i32 {
        return 0 as i32;
    }
    f_max = 0.0f64;
    f_min = f_max;
    k = nb + 1 as i32;
    while k <= len {
        j = *ind.offset(k as isize);
        if glp_get_col_type(lp, j) != 4 as i32 {
            return 0 as i32;
        }
        if *val.offset(k as isize) > 0.0f64 {
            f_min += *val.offset(k as isize) * glp_get_col_lb(lp, j);
            f_max += *val.offset(k as isize) * glp_get_col_ub(lp, j);
        } else {
            f_min += *val.offset(k as isize) * glp_get_col_ub(lp, j);
            f_max += *val.offset(k as isize) * glp_get_col_lb(lp, j);
        }
        k += 1;
        k;
    }
    u = f_max - f_min;
    y = 0.0f64;
    k = nb + 1 as i32;
    while k <= len {
        j = *ind.offset(k as isize);
        y += *val.offset(k as isize) * glp_get_col_prim(lp, j);
        k += 1;
        k;
    }
    y -= f_min;
    if y < 0.0f64 {
        y = 0.0f64;
    }
    if y > u {
        y = u;
    }
    *val.offset(0 as i32 as isize) -= f_min;
    k = 1 as i32;
    while k <= nb {
        j = *ind.offset(k as isize);
        *x.offset(k as isize) = glp_get_col_prim(lp, j);
        if *x.offset(k as isize) < 0.0f64 {
            *x.offset(k as isize) = 0.0f64;
        }
        if *x.offset(k as isize) > 1.0f64 {
            *x.offset(k as isize) = 1.0f64;
        }
        k += 1;
        k;
    }
    k = 1 as i32;
    while k <= nb {
        if *val.offset(k as isize) < 0.0f64 {
            *ind.offset(k as isize) = -*ind.offset(k as isize);
            *val.offset(k as isize) = -*val.offset(k as isize);
            *val.offset(0 as i32 as isize) += *val.offset(k as isize);
            *x.offset(k as isize) = 1.0f64 - *x.offset(k as isize);
        }
        k += 1;
        k;
    }
    r = cover(
        nb,
        val,
        *val.offset(0 as i32 as isize),
        u,
        x,
        y,
        cov.as_mut_ptr(),
        &mut alfa,
        &mut beta,
    );
    if r == 0 as i32 {
        return 0 as i32;
    }
    (2 as i32 <= r && r <= 4 as i32
        || {
            glp_assert_(
                b"2 <= r && r <= 4\0" as *const u8 as *const i8,
                b"draft/glpios07.c\0" as *const u8 as *const i8,
                407 as i32,
            );
            1 as i32 != 0
        }) as i32;
    *ind.offset(0 as i32 as isize) = 0 as i32;
    *val.offset(0 as i32 as isize) = beta;
    j = 1 as i32;
    while j <= r {
        cov[j as usize] = *ind.offset(cov[j as usize] as isize);
        j += 1;
        j;
    }
    (r <= nb
        || {
            glp_assert_(
                b"r <= nb\0" as *const u8 as *const i8,
                b"draft/glpios07.c\0" as *const u8 as *const i8,
                416 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = 1 as i32;
    while k <= r {
        if cov[k as usize] > 0 as i32 {
            *ind.offset(k as isize) = cov[k as usize];
            *val.offset(k as isize) = 1.0f64;
        } else {
            *ind.offset(k as isize) = -cov[k as usize];
            *val.offset(k as isize) = -1.0f64;
            *val.offset(0 as i32 as isize) -= 1.0f64;
        }
        k += 1;
        k;
    }
    k = nb + 1 as i32;
    while k <= len {
        r += 1;
        r;
        *ind.offset(r as isize) = *ind.offset(k as isize);
        *val.offset(r as isize) = alfa * *val.offset(k as isize);
        k += 1;
        k;
    }
    *val.offset(0 as i32 as isize) += alfa * f_min;
    (r <= len
        || {
            glp_assert_(
                b"r <= len\0" as *const u8 as *const i8,
                b"draft/glpios07.c\0" as *const u8 as *const i8,
                435 as i32,
            );
            1 as i32 != 0
        }) as i32;
    len = r;
    return len;
}
unsafe extern "C" fn lpx_eval_row(
    mut lp: *mut glp_prob,
    mut len: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> libc::c_double {
    let mut n: i32 = glp_get_num_cols(lp);
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut sum: libc::c_double = 0.0f64;
    if len < 0 as i32 {
        (glp_error_(b"draft/glpios07.c\0" as *const u8 as *const i8, 475 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"lpx_eval_row: len = %d; invalid row length\n\0" as *const u8 as *const i8,
            len,
        );
    }
    k = 1 as i32;
    while k <= len {
        j = *ind.offset(k as isize);
        if !(1 as i32 <= j && j <= n) {
            (glp_error_(b"draft/glpios07.c\0" as *const u8 as *const i8, 479 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"lpx_eval_row: j = %d; column number out of range\n\0" as *const u8
                    as *const i8,
                j,
            );
        }
        sum += *val.offset(k as isize) * glp_get_col_prim(lp, j);
        k += 1;
        k;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_cov_gen(mut tree: *mut glp_tree) {
    let mut prob: *mut glp_prob = (*tree).mip;
    let mut m: i32 = glp_get_num_rows(prob);
    let mut n: i32 = glp_get_num_cols(prob);
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut type_0: i32 = 0;
    let mut kase: i32 = 0;
    let mut len: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut r: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    (glp_get_status(prob) == 5 as i32
        || {
            glp_assert_(
                b"glp_get_status(prob) == GLP_OPT\0" as *const u8 as *const i8,
                b"draft/glpios07.c\0" as *const u8 as *const i8,
                507 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    work = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    i = 1 as i32;
    while i <= m {
        let mut current_block_16: u64;
        kase = 1 as i32;
        while kase <= 2 as i32 {
            type_0 = glp_get_row_type(prob, i);
            if kase == 1 as i32 {
                if !(type_0 == 3 as i32 || type_0 == 4 as i32) {
                    current_block_16 = 2473556513754201174;
                } else {
                    len = glp_get_mat_row(prob, i, ind, val);
                    *val.offset(0 as i32 as isize) = glp_get_row_ub(prob, i);
                    current_block_16 = 17833034027772472439;
                }
            } else if !(type_0 == 2 as i32 || type_0 == 4 as i32) {
                current_block_16 = 2473556513754201174;
            } else {
                len = glp_get_mat_row(prob, i, ind, val);
                k = 1 as i32;
                while k <= len {
                    *val.offset(k as isize) = -*val.offset(k as isize);
                    k += 1;
                    k;
                }
                *val.offset(0 as i32 as isize) = -glp_get_row_lb(prob, i);
                current_block_16 = 17833034027772472439;
            }
            match current_block_16 {
                17833034027772472439 => {
                    len = lpx_cover_cut(prob, len, ind, val, work);
                    if !(len == 0 as i32) {
                        r = lpx_eval_row(prob, len, ind, val)
                            - *val.offset(0 as i32 as isize);
                        if !(r < 1e-3f64) {
                            glp_ios_add_row(
                                tree,
                                0 as *const i8,
                                3 as i32,
                                0 as i32,
                                len,
                                ind as *const i32,
                                val as *const libc::c_double,
                                3 as i32,
                                *val.offset(0 as i32 as isize),
                            );
                        }
                    }
                }
                _ => {}
            }
            kase += 1;
            kase;
        }
        i += 1;
        i;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(work as *mut libc::c_void);
}