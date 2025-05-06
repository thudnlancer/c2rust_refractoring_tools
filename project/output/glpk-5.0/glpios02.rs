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
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_get_row_lb(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_row_ub(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_col_lb(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_ub(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_row_stat(P: *mut glp_prob, i: i32) -> i32;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct f_info {
    pub j_min: i32,
    pub j_max: i32,
    pub f_min: libc::c_double,
    pub f_max: libc::c_double,
}
unsafe extern "C" fn prepare_row_info(
    mut n: i32,
    mut a: *const libc::c_double,
    mut l: *const libc::c_double,
    mut u: *const libc::c_double,
    mut f: *mut f_info,
) {
    let mut j: i32 = 0;
    let mut j_min: i32 = 0;
    let mut j_max: i32 = 0;
    let mut f_min: libc::c_double = 0.;
    let mut f_max: libc::c_double = 0.;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"draft/glpios02.c\0" as *const u8 as *const i8,
                104 as i32,
            );
            1 as i32 != 0
        }) as i32;
    f_min = 0.0f64;
    j_min = 0 as i32;
    j = 1 as i32;
    while j <= n {
        if *a.offset(j as isize) > 0.0f64 {
            if *l.offset(j as isize) == -1.7976931348623157e+308f64 {
                if j_min == 0 as i32 {
                    j_min = j;
                } else {
                    f_min = -1.7976931348623157e+308f64;
                    j_min = 0 as i32;
                    break;
                }
            } else {
                f_min += *a.offset(j as isize) * *l.offset(j as isize);
            }
        } else if *a.offset(j as isize) < 0.0f64 {
            if *u.offset(j as isize) == 1.7976931348623157e+308f64 {
                if j_min == 0 as i32 {
                    j_min = j;
                } else {
                    f_min = -1.7976931348623157e+308f64;
                    j_min = 0 as i32;
                    break;
                }
            } else {
                f_min += *a.offset(j as isize) * *u.offset(j as isize);
            }
        } else {
            (a != a
                || {
                    glp_assert_(
                        b"a != a\0" as *const u8 as *const i8,
                        b"draft/glpios02.c\0" as *const u8 as *const i8,
                        133 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        j += 1;
        j;
    }
    (*f).f_min = f_min;
    (*f).j_min = j_min;
    f_max = 0.0f64;
    j_max = 0 as i32;
    j = 1 as i32;
    while j <= n {
        if *a.offset(j as isize) > 0.0f64 {
            if *u.offset(j as isize) == 1.7976931348623157e+308f64 {
                if j_max == 0 as i32 {
                    j_max = j;
                } else {
                    f_max = 1.7976931348623157e+308f64;
                    j_max = 0 as i32;
                    break;
                }
            } else {
                f_max += *a.offset(j as isize) * *u.offset(j as isize);
            }
        } else if *a.offset(j as isize) < 0.0f64 {
            if *l.offset(j as isize) == -1.7976931348623157e+308f64 {
                if j_max == 0 as i32 {
                    j_max = j;
                } else {
                    f_max = 1.7976931348623157e+308f64;
                    j_max = 0 as i32;
                    break;
                }
            } else {
                f_max += *a.offset(j as isize) * *l.offset(j as isize);
            }
        } else {
            (a != a
                || {
                    glp_assert_(
                        b"a != a\0" as *const u8 as *const i8,
                        b"draft/glpios02.c\0" as *const u8 as *const i8,
                        164 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        j += 1;
        j;
    }
    (*f).f_max = f_max;
    (*f).j_max = j_max;
}
unsafe extern "C" fn row_implied_bounds(
    mut f: *const f_info,
    mut LL: *mut libc::c_double,
    mut UU: *mut libc::c_double,
) {
    *LL = if (*f).j_min == 0 as i32 { (*f).f_min } else { -1.7976931348623157e+308f64 };
    *UU = if (*f).j_max == 0 as i32 { (*f).f_max } else { 1.7976931348623157e+308f64 };
}
unsafe extern "C" fn col_implied_bounds(
    mut f: *const f_info,
    mut n: i32,
    mut a: *const libc::c_double,
    mut L: libc::c_double,
    mut U: libc::c_double,
    mut l: *const libc::c_double,
    mut u: *const libc::c_double,
    mut k: i32,
    mut ll: *mut libc::c_double,
    mut uu: *mut libc::c_double,
) {
    let mut ilb: libc::c_double = 0.;
    let mut iub: libc::c_double = 0.;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"draft/glpios02.c\0" as *const u8 as *const i8,
                299 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= k && k <= n
        || {
            glp_assert_(
                b"1 <= k && k <= n\0" as *const u8 as *const i8,
                b"draft/glpios02.c\0" as *const u8 as *const i8,
                300 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if L == -1.7976931348623157e+308f64 || (*f).f_max == 1.7976931348623157e+308f64 {
        ilb = -1.7976931348623157e+308f64;
    } else if (*f).j_max == 0 as i32 {
        if *a.offset(k as isize) > 0.0f64 {
            (*u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] != +DBL_MAX\0" as *const u8 as *const i8,
                        b"draft/glpios02.c\0" as *const u8 as *const i8,
                        306 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            ilb = L - ((*f).f_max - *a.offset(k as isize) * *u.offset(k as isize));
        } else if *a.offset(k as isize) < 0.0f64 {
            (*l.offset(k as isize) != -1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != -DBL_MAX\0" as *const u8 as *const i8,
                        b"draft/glpios02.c\0" as *const u8 as *const i8,
                        310 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            ilb = L - ((*f).f_max - *a.offset(k as isize) * *l.offset(k as isize));
        } else {
            (a != a
                || {
                    glp_assert_(
                        b"a != a\0" as *const u8 as *const i8,
                        b"draft/glpios02.c\0" as *const u8 as *const i8,
                        314 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    } else if (*f).j_max == k {
        ilb = L - (*f).f_max;
    } else {
        ilb = -1.7976931348623157e+308f64;
    }
    if U == 1.7976931348623157e+308f64 || (*f).f_min == -1.7976931348623157e+308f64 {
        iub = 1.7976931348623157e+308f64;
    } else if (*f).j_min == 0 as i32 {
        if *a.offset(k as isize) > 0.0f64 {
            (*l.offset(k as isize) != -1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != -DBL_MAX\0" as *const u8 as *const i8,
                        b"draft/glpios02.c\0" as *const u8 as *const i8,
                        325 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            iub = U - ((*f).f_min - *a.offset(k as isize) * *l.offset(k as isize));
        } else if *a.offset(k as isize) < 0.0f64 {
            (*u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] != +DBL_MAX\0" as *const u8 as *const i8,
                        b"draft/glpios02.c\0" as *const u8 as *const i8,
                        329 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            iub = U - ((*f).f_min - *a.offset(k as isize) * *u.offset(k as isize));
        } else {
            (a != a
                || {
                    glp_assert_(
                        b"a != a\0" as *const u8 as *const i8,
                        b"draft/glpios02.c\0" as *const u8 as *const i8,
                        333 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    } else if (*f).j_min == k {
        iub = U - (*f).f_min;
    } else {
        iub = 1.7976931348623157e+308f64;
    }
    if fabs(*a.offset(k as isize)) < 1e-6f64 {
        *ll = -1.7976931348623157e+308f64;
        *uu = 1.7976931348623157e+308f64;
    } else if *a.offset(k as isize) > 0.0f64 {
        *ll = if ilb == -1.7976931348623157e+308f64 {
            -1.7976931348623157e+308f64
        } else {
            ilb / *a.offset(k as isize)
        };
        *uu = if iub == 1.7976931348623157e+308f64 {
            1.7976931348623157e+308f64
        } else {
            iub / *a.offset(k as isize)
        };
    } else if *a.offset(k as isize) < 0.0f64 {
        *ll = if iub == 1.7976931348623157e+308f64 {
            -1.7976931348623157e+308f64
        } else {
            iub / *a.offset(k as isize)
        };
        *uu = if ilb == -1.7976931348623157e+308f64 {
            1.7976931348623157e+308f64
        } else {
            ilb / *a.offset(k as isize)
        };
    } else {
        (a != a
            || {
                glp_assert_(
                    b"a != a\0" as *const u8 as *const i8,
                    b"draft/glpios02.c\0" as *const u8 as *const i8,
                    357 as i32,
                );
                1 as i32 != 0
            }) as i32;
    };
}
unsafe extern "C" fn check_row_bounds(
    mut f: *const f_info,
    mut L_: *mut libc::c_double,
    mut U_: *mut libc::c_double,
) -> i32 {
    let mut current_block: u64;
    let mut ret: i32 = 0 as i32;
    let mut L: libc::c_double = *L_;
    let mut U: libc::c_double = *U_;
    let mut LL: libc::c_double = 0.;
    let mut UU: libc::c_double = 0.;
    row_implied_bounds(f, &mut LL, &mut UU);
    if L != -1.7976931348623157e+308f64 {
        let mut eps: libc::c_double = 1e-3f64 * (1.0f64 + fabs(L));
        if UU < L - eps {
            ret = 1 as i32;
            current_block = 11399325336817285152;
        } else {
            current_block = 820271813250567934;
        }
    } else {
        current_block = 820271813250567934;
    }
    match current_block {
        820271813250567934 => {
            if U != 1.7976931348623157e+308f64 {
                let mut eps_0: libc::c_double = 1e-3f64 * (1.0f64 + fabs(U));
                if LL > U + eps_0 {
                    ret = 1 as i32;
                    current_block = 11399325336817285152;
                } else {
                    current_block = 13513818773234778473;
                }
            } else {
                current_block = 13513818773234778473;
            }
            match current_block {
                11399325336817285152 => {}
                _ => {
                    if L != -1.7976931348623157e+308f64 {
                        let mut eps_1: libc::c_double = 1e-12f64 * (1.0f64 + fabs(L));
                        if LL > L - eps_1 {
                            *L_ = -1.7976931348623157e+308f64;
                        }
                    }
                    if U != 1.7976931348623157e+308f64 {
                        let mut eps_2: libc::c_double = 1e-12f64 * (1.0f64 + fabs(U));
                        if UU < U + eps_2 {
                            *U_ = 1.7976931348623157e+308f64;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn check_col_bounds(
    mut f: *const f_info,
    mut n: i32,
    mut a: *const libc::c_double,
    mut L: libc::c_double,
    mut U: libc::c_double,
    mut l: *const libc::c_double,
    mut u: *const libc::c_double,
    mut flag: i32,
    mut j: i32,
    mut _lj: *mut libc::c_double,
    mut _uj: *mut libc::c_double,
) -> i32 {
    let mut current_block: u64;
    let mut ret: i32 = 0 as i32;
    let mut lj: libc::c_double = 0.;
    let mut uj: libc::c_double = 0.;
    let mut ll: libc::c_double = 0.;
    let mut uu: libc::c_double = 0.;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"draft/glpios02.c\0" as *const u8 as *const i8,
                457 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= j && j <= n
        || {
            glp_assert_(
                b"1 <= j && j <= n\0" as *const u8 as *const i8,
                b"draft/glpios02.c\0" as *const u8 as *const i8,
                458 as i32,
            );
            1 as i32 != 0
        }) as i32;
    lj = *l.offset(j as isize);
    uj = *u.offset(j as isize);
    col_implied_bounds(f, n, a, L, U, l, u, j, &mut ll, &mut uu);
    if flag != 0 {
        if ll != -1.7976931348623157e+308f64 {
            ll = if ll - floor(ll) < 1e-3f64 { floor(ll) } else { ceil(ll) };
        }
        if uu != 1.7976931348623157e+308f64 {
            uu = if ceil(uu) - uu < 1e-3f64 { ceil(uu) } else { floor(uu) };
        }
    }
    if lj != -1.7976931348623157e+308f64 {
        let mut eps: libc::c_double = 1e-3f64 * (1.0f64 + fabs(lj));
        if uu < lj - eps {
            ret = 1 as i32;
            current_block = 12301347527267834071;
        } else {
            current_block = 12209867499936983673;
        }
    } else {
        current_block = 12209867499936983673;
    }
    match current_block {
        12209867499936983673 => {
            if uj != 1.7976931348623157e+308f64 {
                let mut eps_0: libc::c_double = 1e-3f64 * (1.0f64 + fabs(uj));
                if ll > uj + eps_0 {
                    ret = 1 as i32;
                    current_block = 12301347527267834071;
                } else {
                    current_block = 17407779659766490442;
                }
            } else {
                current_block = 17407779659766490442;
            }
            match current_block {
                12301347527267834071 => {}
                _ => {
                    if ll != -1.7976931348623157e+308f64 {
                        let mut eps_1: libc::c_double = 1e-3f64 * (1.0f64 + fabs(ll));
                        if lj < ll - eps_1 {
                            lj = ll;
                        }
                    }
                    if uu != 1.7976931348623157e+308f64 {
                        let mut eps_2: libc::c_double = 1e-3f64 * (1.0f64 + fabs(uu));
                        if uj > uu + eps_2 {
                            uj = uu;
                        }
                    }
                    if !(lj == -1.7976931348623157e+308f64
                        || uj == 1.7976931348623157e+308f64)
                    {
                        let mut t1: libc::c_double = fabs(lj);
                        let mut t2: libc::c_double = fabs(uj);
                        let mut eps_3: libc::c_double = 1e-10f64
                            * (1.0f64 + (if t1 <= t2 { t1 } else { t2 }));
                        if lj > uj - eps_3 {
                            if lj == *l.offset(j as isize) {
                                uj = lj;
                            } else if uj == *u.offset(j as isize) {
                                lj = uj;
                            } else if t1 <= t2 {
                                uj = lj;
                            } else {
                                lj = uj;
                            }
                        }
                    }
                    *_lj = lj;
                    *_uj = uj;
                }
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn check_efficiency(
    mut flag: i32,
    mut l: libc::c_double,
    mut u: libc::c_double,
    mut ll: libc::c_double,
    mut uu: libc::c_double,
) -> i32 {
    let mut eff: i32 = 0 as i32;
    if l < ll {
        if flag != 0 || l == -1.7976931348623157e+308f64 {
            eff += 1;
            eff;
        } else {
            let mut r: libc::c_double = 0.;
            if u == 1.7976931348623157e+308f64 {
                r = 1.0f64 + fabs(l);
            } else {
                r = 1.0f64 + (u - l);
            }
            if ll - l >= 0.25f64 * r {
                eff += 1;
                eff;
            }
        }
    }
    if u > uu {
        if flag != 0 || u == 1.7976931348623157e+308f64 {
            eff += 1;
            eff;
        } else {
            let mut r_0: libc::c_double = 0.;
            if l == -1.7976931348623157e+308f64 {
                r_0 = 1.0f64 + fabs(u);
            } else {
                r_0 = 1.0f64 + (u - l);
            }
            if u - uu >= 0.25f64 * r_0 {
                eff += 1;
                eff;
            }
        }
    }
    return eff;
}
unsafe extern "C" fn basic_preprocessing(
    mut mip: *mut glp_prob,
    mut L: *mut libc::c_double,
    mut U: *mut libc::c_double,
    mut l: *mut libc::c_double,
    mut u: *mut libc::c_double,
    mut nrs: i32,
    mut num: *const i32,
    mut max_pass: i32,
) -> i32 {
    let mut m: i32 = (*mip).m;
    let mut n: i32 = (*mip).n;
    let mut f: f_info = f_info {
        j_min: 0,
        j_max: 0,
        f_min: 0.,
        f_max: 0.,
    };
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    let mut size: i32 = 0;
    let mut ret: i32 = 0 as i32;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut list: *mut i32 = 0 as *mut i32;
    let mut mark: *mut i32 = 0 as *mut i32;
    let mut pass: *mut i32 = 0 as *mut i32;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut lb: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ub: *mut libc::c_double = 0 as *mut libc::c_double;
    (0 as i32 <= nrs && nrs <= m + 1 as i32
        || {
            glp_assert_(
                b"0 <= nrs && nrs <= m+1\0" as *const u8 as *const i8,
                b"draft/glpios02.c\0" as *const u8 as *const i8,
                608 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (max_pass > 0 as i32
        || {
            glp_assert_(
                b"max_pass > 0\0" as *const u8 as *const i8,
                b"draft/glpios02.c\0" as *const u8 as *const i8,
                609 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    list = glp_alloc(
        1 as i32 + m + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    mark = glp_alloc(
        1 as i32 + m + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    memset(
        &mut *mark.offset(0 as i32 as isize) as *mut i32 as *mut libc::c_void,
        0 as i32,
        ((m + 1 as i32) as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    pass = glp_alloc(
        1 as i32 + m + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    memset(
        &mut *pass.offset(0 as i32 as isize) as *mut i32 as *mut libc::c_void,
        0 as i32,
        ((m + 1 as i32) as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    val = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    lb = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    ub = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    size = 0 as i32;
    k = 1 as i32;
    while k <= nrs {
        i = *num.offset(k as isize);
        (0 as i32 <= i && i <= m
            || {
                glp_assert_(
                    b"0 <= i && i <= m\0" as *const u8 as *const i8,
                    b"draft/glpios02.c\0" as *const u8 as *const i8,
                    624 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*mark.offset(i as isize) == 0
            || {
                glp_assert_(
                    b"!mark[i]\0" as *const u8 as *const i8,
                    b"draft/glpios02.c\0" as *const u8 as *const i8,
                    626 as i32,
                );
                1 as i32 != 0
            }) as i32;
        size += 1;
        *list.offset(size as isize) = i;
        *mark.offset(i as isize) = 1 as i32;
        k += 1;
        k;
    }
    (size == nrs
        || {
            glp_assert_(
                b"size == nrs\0" as *const u8 as *const i8,
                b"draft/glpios02.c\0" as *const u8 as *const i8,
                629 as i32,
            );
            1 as i32 != 0
        }) as i32;
    's_79: while size > 0 as i32 {
        let fresh0 = size;
        size = size - 1;
        i = *list.offset(fresh0 as isize);
        *mark.offset(i as isize) = 0 as i32;
        let ref mut fresh1 = *pass.offset(i as isize);
        *fresh1 += 1;
        *fresh1;
        if *L.offset(i as isize) == -1.7976931348623157e+308f64
            && *U.offset(i as isize) == 1.7976931348623157e+308f64
        {
            continue;
        }
        len = 0 as i32;
        if i == 0 as i32 {
            j = 1 as i32;
            while j <= n {
                let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
                if (*col).coef != 0.0f64 {
                    len += 1;
                    len;
                    *ind.offset(len as isize) = j;
                    *val.offset(len as isize) = (*col).coef;
                }
                j += 1;
                j;
            }
        } else {
            let mut row: *mut GLPROW = *((*mip).row).offset(i as isize);
            let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
            aij = (*row).ptr;
            while !aij.is_null() {
                len += 1;
                len;
                *ind.offset(len as isize) = (*(*aij).col).j;
                *val.offset(len as isize) = (*aij).val;
                aij = (*aij).r_next;
            }
        }
        k = 1 as i32;
        while k <= len {
            j = *ind.offset(k as isize);
            *lb.offset(k as isize) = *l.offset(j as isize);
            *ub.offset(k as isize) = *u.offset(j as isize);
            k += 1;
            k;
        }
        prepare_row_info(
            len,
            val as *const libc::c_double,
            lb as *const libc::c_double,
            ub as *const libc::c_double,
            &mut f,
        );
        if check_row_bounds(
            &mut f,
            &mut *L.offset(i as isize),
            &mut *U.offset(i as isize),
        ) != 0
        {
            ret = 1 as i32;
            break;
        } else {
            if *L.offset(i as isize) == -1.7976931348623157e+308f64
                && *U.offset(i as isize) == 1.7976931348623157e+308f64
            {
                continue;
            }
            k = 1 as i32;
            while k <= len {
                let mut col_0: *mut GLPCOL = 0 as *mut GLPCOL;
                let mut flag: i32 = 0;
                let mut eff: i32 = 0;
                let mut ll: libc::c_double = 0.;
                let mut uu: libc::c_double = 0.;
                j = *ind.offset(k as isize);
                col_0 = *((*mip).col).offset(j as isize);
                flag = ((*col_0).kind != 1 as i32) as i32;
                if check_col_bounds(
                    &mut f,
                    len,
                    val as *const libc::c_double,
                    *L.offset(i as isize),
                    *U.offset(i as isize),
                    lb as *const libc::c_double,
                    ub as *const libc::c_double,
                    flag,
                    k,
                    &mut ll,
                    &mut uu,
                ) != 0
                {
                    ret = 1 as i32;
                    break 's_79;
                } else {
                    eff = check_efficiency(
                        flag,
                        *l.offset(j as isize),
                        *u.offset(j as isize),
                        ll,
                        uu,
                    );
                    *l.offset(j as isize) = ll;
                    *u.offset(j as isize) = uu;
                    if eff > 0 as i32 {
                        let mut aij_0: *mut GLPAIJ = 0 as *mut GLPAIJ;
                        aij_0 = (*col_0).ptr;
                        while !aij_0.is_null() {
                            let mut ii: i32 = (*(*aij_0).row).i;
                            if !(*pass.offset(ii as isize) >= max_pass) {
                                if !(*L.offset(ii as isize) == -1.7976931348623157e+308f64
                                    && *U.offset(ii as isize) == 1.7976931348623157e+308f64)
                                {
                                    if *mark.offset(ii as isize) == 0 as i32 {
                                        (size <= m
                                            || {
                                                glp_assert_(
                                                    b"size <= m\0" as *const u8 as *const i8,
                                                    b"draft/glpios02.c\0" as *const u8 as *const i8,
                                                    699 as i32,
                                                );
                                                1 as i32 != 0
                                            }) as i32;
                                        size += 1;
                                        *list.offset(size as isize) = ii;
                                        *mark.offset(ii as isize) = 1 as i32;
                                    }
                                }
                            }
                            aij_0 = (*aij_0).c_next;
                        }
                    }
                    k += 1;
                    k;
                }
            }
        }
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(list as *mut libc::c_void);
    glp_free(mark as *mut libc::c_void);
    glp_free(pass as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(lb as *mut libc::c_void);
    glp_free(ub as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_preprocess_node(
    mut tree: *mut glp_tree,
    mut max_pass: i32,
) -> i32 {
    let mut mip: *mut glp_prob = (*tree).mip;
    let mut m: i32 = (*mip).m;
    let mut n: i32 = (*mip).n;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut nrs: i32 = 0;
    let mut num: *mut i32 = 0 as *mut i32;
    let mut ret: i32 = 0 as i32;
    let mut L: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut U: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut l: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut u: *mut libc::c_double = 0 as *mut libc::c_double;
    (!((*tree).curr).is_null()
        || {
            glp_assert_(
                b"tree->curr != NULL\0" as *const u8 as *const i8,
                b"draft/glpios02.c\0" as *const u8 as *const i8,
                744 as i32,
            );
            1 as i32 != 0
        }) as i32;
    L = glp_alloc(1 as i32 + m, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    U = glp_alloc(1 as i32 + m, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    match (*mip).mip_stat {
        1 => {
            *L.offset(0 as i32 as isize) = -1.7976931348623157e+308f64;
            *U.offset(0 as i32 as isize) = 1.7976931348623157e+308f64;
        }
        2 => {
            match (*mip).dir {
                1 => {
                    *L.offset(0 as i32 as isize) = -1.7976931348623157e+308f64;
                    *U.offset(0 as i32 as isize) = (*mip).mip_obj - (*mip).c0;
                }
                2 => {
                    *L.offset(0 as i32 as isize) = (*mip).mip_obj - (*mip).c0;
                    *U.offset(0 as i32 as isize) = 1.7976931348623157e+308f64;
                }
                _ => {
                    (mip != mip
                        || {
                            glp_assert_(
                                b"mip != mip\0" as *const u8 as *const i8,
                                b"draft/glpios02.c\0" as *const u8 as *const i8,
                                761 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
        }
        _ => {
            (mip != mip
                || {
                    glp_assert_(
                        b"mip != mip\0" as *const u8 as *const i8,
                        b"draft/glpios02.c\0" as *const u8 as *const i8,
                        765 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    i = 1 as i32;
    while i <= m {
        *L.offset(i as isize) = glp_get_row_lb(mip, i);
        *U.offset(i as isize) = glp_get_row_ub(mip, i);
        i += 1;
        i;
    }
    l = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    u = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    j = 1 as i32;
    while j <= n {
        *l.offset(j as isize) = glp_get_col_lb(mip, j);
        *u.offset(j as isize) = glp_get_col_ub(mip, j);
        j += 1;
        j;
    }
    nrs = m + 1 as i32;
    num = glp_alloc(1 as i32 + nrs, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    i = 1 as i32;
    while i <= nrs {
        *num.offset(i as isize) = i - 1 as i32;
        i += 1;
        i;
    }
    if basic_preprocessing(mip, L, U, l, u, nrs, num as *const i32, max_pass) != 0 {
        ret = 1 as i32;
    } else {
        i = 1 as i32;
        while i <= m {
            if glp_get_row_stat(mip, i) == 1 as i32 {
                if *L.offset(i as isize) == -1.7976931348623157e+308f64
                    && *U.offset(i as isize) == 1.7976931348623157e+308f64
                {
                    glp_set_row_bnds(mip, i, 1 as i32, 0.0f64, 0.0f64);
                } else if *U.offset(i as isize) == 1.7976931348623157e+308f64 {
                    glp_set_row_bnds(mip, i, 2 as i32, *L.offset(i as isize), 0.0f64);
                } else if *L.offset(i as isize) == -1.7976931348623157e+308f64 {
                    glp_set_row_bnds(mip, i, 3 as i32, 0.0f64, *U.offset(i as isize));
                }
            }
            i += 1;
            i;
        }
        j = 1 as i32;
        while j <= n {
            let mut type_0: i32 = 0;
            if *l.offset(j as isize) == -1.7976931348623157e+308f64
                && *u.offset(j as isize) == 1.7976931348623157e+308f64
            {
                type_0 = 1 as i32;
            } else if *u.offset(j as isize) == 1.7976931348623157e+308f64 {
                type_0 = 2 as i32;
            } else if *l.offset(j as isize) == -1.7976931348623157e+308f64 {
                type_0 = 3 as i32;
            } else if *l.offset(j as isize) != *u.offset(j as isize) {
                type_0 = 4 as i32;
            } else {
                type_0 = 5 as i32;
            }
            glp_set_col_bnds(
                mip,
                j,
                type_0,
                *l.offset(j as isize),
                *u.offset(j as isize),
            );
            j += 1;
            j;
        }
    }
    glp_free(L as *mut libc::c_void);
    glp_free(U as *mut libc::c_void);
    glp_free(l as *mut libc::c_void);
    glp_free(u as *mut libc::c_void);
    glp_free(num as *mut libc::c_void);
    return ret;
}