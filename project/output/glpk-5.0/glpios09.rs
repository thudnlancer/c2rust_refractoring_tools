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
    fn glp_difftime(t1: libc::c_double, t0: libc::c_double) -> libc::c_double;
    fn glp_time() -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_copy_prob(dest: *mut glp_prob, prob: *mut glp_prob, names: i32);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_simplex(P: *mut glp_prob, parm: *const glp_smcp) -> i32;
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_get_status(P: *mut glp_prob) -> i32;
    fn glp_get_prim_stat(P: *mut glp_prob) -> i32;
    fn glp_get_dual_stat(P: *mut glp_prob) -> i32;
    fn glp_get_obj_val(P: *mut glp_prob) -> libc::c_double;
    fn glp_get_row_stat(P: *mut glp_prob, i: i32) -> i32;
    fn glp_get_row_dual(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_col_stat(P: *mut glp_prob, j: i32) -> i32;
    fn glp_get_col_prim(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_dual(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_kind(P: *mut glp_prob, j: i32) -> i32;
    fn glp_eval_tab_row(
        P: *mut glp_prob,
        k: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn glp_dual_rtest(
        P: *mut glp_prob,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
        dir: i32,
        eps: libc::c_double,
    ) -> i32;
    fn glp_ios_can_branch(T: *mut glp_tree, j: i32) -> i32;
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
pub struct glp_smcp {
    pub msg_lev: i32,
    pub meth: i32,
    pub pricing: i32,
    pub r_test: i32,
    pub tol_bnd: libc::c_double,
    pub tol_dj: libc::c_double,
    pub tol_piv: libc::c_double,
    pub obj_ll: libc::c_double,
    pub obj_ul: libc::c_double,
    pub it_lim: i32,
    pub tm_lim: i32,
    pub out_frq: i32,
    pub out_dly: i32,
    pub presolve: i32,
    pub excl: i32,
    pub shift: i32,
    pub aorn: i32,
    pub foo_bar: [libc::c_double; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub dn_cnt: *mut i32,
    pub dn_sum: *mut libc::c_double,
    pub up_cnt: *mut i32,
    pub up_sum: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_choose_var(
    mut T: *mut glp_tree,
    mut next: *mut i32,
) -> i32 {
    let mut j: i32 = 0;
    if (*(*T).parm).br_tech == 1 as i32 {
        j = branch_first(T, next);
    } else if (*(*T).parm).br_tech == 2 as i32 {
        j = branch_last(T, next);
    } else if (*(*T).parm).br_tech == 3 as i32 {
        j = branch_mostf(T, next);
    } else if (*(*T).parm).br_tech == 4 as i32 {
        j = branch_drtom(T, next);
    } else if (*(*T).parm).br_tech == 5 as i32 {
        j = _glp_ios_pcost_branch(T, next);
    } else {
        (T != T
            || {
                glp_assert_(
                    b"T != T\0" as *const u8 as *const i8,
                    b"draft/glpios09.c\0" as *const u8 as *const i8,
                    73 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    return j;
}
unsafe extern "C" fn branch_first(mut T: *mut glp_tree, mut _next: *mut i32) -> i32 {
    let mut j: i32 = 0;
    let mut next: i32 = 0;
    let mut beta: libc::c_double = 0.;
    j = 1 as i32;
    while j <= (*T).n {
        if *((*T).non_int).offset(j as isize) != 0 {
            break;
        }
        j += 1;
        j;
    }
    (1 as i32 <= j && j <= (*T).n
        || {
            glp_assert_(
                b"1 <= j && j <= T->n\0" as *const u8 as *const i8,
                b"draft/glpios09.c\0" as *const u8 as *const i8,
                93 as i32,
            );
            1 as i32 != 0
        }) as i32;
    beta = glp_get_col_prim((*T).mip, j);
    if beta - floor(beta) < ceil(beta) - beta {
        next = 1 as i32;
    } else {
        next = 2 as i32;
    }
    *_next = next;
    return j;
}
unsafe extern "C" fn branch_last(mut T: *mut glp_tree, mut _next: *mut i32) -> i32 {
    let mut j: i32 = 0;
    let mut next: i32 = 0;
    let mut beta: libc::c_double = 0.;
    j = (*T).n;
    while j >= 1 as i32 {
        if *((*T).non_int).offset(j as isize) != 0 {
            break;
        }
        j -= 1;
        j;
    }
    (1 as i32 <= j && j <= (*T).n
        || {
            glp_assert_(
                b"1 <= j && j <= T->n\0" as *const u8 as *const i8,
                b"draft/glpios09.c\0" as *const u8 as *const i8,
                120 as i32,
            );
            1 as i32 != 0
        }) as i32;
    beta = glp_get_col_prim((*T).mip, j);
    if beta - floor(beta) < ceil(beta) - beta {
        next = 1 as i32;
    } else {
        next = 2 as i32;
    }
    *_next = next;
    return j;
}
unsafe extern "C" fn branch_mostf(mut T: *mut glp_tree, mut _next: *mut i32) -> i32 {
    let mut j: i32 = 0;
    let mut jj: i32 = 0;
    let mut next: i32 = 0;
    let mut beta: libc::c_double = 0.;
    let mut most: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    jj = 0 as i32;
    most = 1.7976931348623157e+308f64;
    j = 1 as i32;
    while j <= (*T).n {
        if *((*T).non_int).offset(j as isize) != 0 {
            beta = glp_get_col_prim((*T).mip, j);
            temp = floor(beta) + 0.5f64;
            if most > fabs(beta - temp) {
                jj = j;
                most = fabs(beta - temp);
                if beta < temp {
                    next = 1 as i32;
                } else {
                    next = 2 as i32;
                }
            }
        }
        j += 1;
        j;
    }
    *_next = next;
    return jj;
}
unsafe extern "C" fn branch_drtom(mut T: *mut glp_tree, mut _next: *mut i32) -> i32 {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut m: i32 = (*mip).m;
    let mut n: i32 = (*mip).n;
    let mut non_int: *mut u8 = (*T).non_int;
    let mut j: i32 = 0;
    let mut jj: i32 = 0;
    let mut k: i32 = 0;
    let mut t: i32 = 0;
    let mut next: i32 = 0;
    let mut kase: i32 = 0;
    let mut len: i32 = 0;
    let mut stat: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut x: libc::c_double = 0.;
    let mut dk: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut delta_j: libc::c_double = 0.;
    let mut delta_k: libc::c_double = 0.;
    let mut delta_z: libc::c_double = 0.;
    let mut dz_dn: libc::c_double = 0.;
    let mut dz_up: libc::c_double = 0.;
    let mut dd_dn: libc::c_double = 0.;
    let mut dd_up: libc::c_double = 0.;
    let mut degrad: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    (glp_get_status(mip) == 5 as i32
        || {
            glp_assert_(
                b"glp_get_status(mip) == GLP_OPT\0" as *const u8 as *const i8,
                b"draft/glpios09.c\0" as *const u8 as *const i8,
                200 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    jj = 0 as i32;
    degrad = -1.0f64;
    j = 1 as i32;
    while j <= n {
        if !(*non_int.offset(j as isize) == 0) {
            x = glp_get_col_prim(mip, j);
            len = glp_eval_tab_row(mip, m + j, ind, val);
            kase = -(1 as i32);
            while kase <= 1 as i32 {
                k = glp_dual_rtest(
                    mip,
                    len,
                    ind as *const i32,
                    val as *const libc::c_double,
                    kase,
                    1e-9f64,
                );
                if k != 0 as i32 {
                    k = *ind.offset(k as isize);
                }
                if k == 0 as i32 {
                    delta_z = if (*(*T).mip).dir == 1 as i32 {
                        1.7976931348623157e+308f64
                    } else {
                        -1.7976931348623157e+308f64
                    };
                } else {
                    t = 1 as i32;
                    while t <= len {
                        if *ind.offset(t as isize) == k {
                            break;
                        }
                        t += 1;
                        t;
                    }
                    (1 as i32 <= t && t <= len
                        || {
                            glp_assert_(
                                b"1 <= t && t <= len\0" as *const u8 as *const i8,
                                b"draft/glpios09.c\0" as *const u8 as *const i8,
                                258 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    alfa = *val.offset(t as isize);
                    delta_j = (if kase < 0 as i32 { floor(x) } else { ceil(x) }) - x;
                    delta_k = delta_j / alfa;
                    if k > m && glp_get_col_kind(mip, k - m) != 1 as i32 {
                        if fabs(delta_k - floor(delta_k + 0.5f64)) > 1e-3f64 {
                            if delta_k > 0.0f64 {
                                delta_k = ceil(delta_k);
                            } else {
                                delta_k = floor(delta_k);
                            }
                        }
                    }
                    if k <= m {
                        stat = glp_get_row_stat(mip, k);
                        dk = glp_get_row_dual(mip, k);
                    } else {
                        stat = glp_get_col_stat(mip, k - m);
                        dk = glp_get_col_dual(mip, k - m);
                    }
                    match (*(*T).mip).dir {
                        1 => {
                            if stat == 2 as i32 && dk < 0.0f64
                                || stat == 3 as i32 && dk > 0.0f64 || stat == 4 as i32
                            {
                                dk = 0.0f64;
                            }
                        }
                        2 => {
                            if stat == 2 as i32 && dk > 0.0f64
                                || stat == 3 as i32 && dk < 0.0f64 || stat == 4 as i32
                            {
                                dk = 0.0f64;
                            }
                        }
                        _ => {
                            (T != T
                                || {
                                    glp_assert_(
                                        b"T != T\0" as *const u8 as *const i8,
                                        b"draft/glpios09.c\0" as *const u8 as *const i8,
                                        306 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                        }
                    }
                    delta_z = dk * delta_k;
                }
                match (*(*T).mip).dir {
                    1 => {
                        (delta_z >= 0.0f64
                            || {
                                glp_assert_(
                                    b"delta_z >= 0.0\0" as *const u8 as *const i8,
                                    b"draft/glpios09.c\0" as *const u8 as *const i8,
                                    318 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                    2 => {
                        (delta_z <= 0.0f64
                            || {
                                glp_assert_(
                                    b"delta_z <= 0.0\0" as *const u8 as *const i8,
                                    b"draft/glpios09.c\0" as *const u8 as *const i8,
                                    319 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                    _ => {
                        (T != T
                            || {
                                glp_assert_(
                                    b"T != T\0" as *const u8 as *const i8,
                                    b"draft/glpios09.c\0" as *const u8 as *const i8,
                                    320 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                }
                if kase < 0 as i32 {
                    dz_dn = delta_z;
                } else {
                    dz_up = delta_z;
                }
                kase += 2 as i32;
            }
            if degrad < fabs(dz_dn) || degrad < fabs(dz_up) {
                jj = j;
                if fabs(dz_dn) < fabs(dz_up) {
                    next = 1 as i32;
                    degrad = fabs(dz_up);
                } else {
                    next = 2 as i32;
                    degrad = fabs(dz_dn);
                }
                dd_dn = dz_dn;
                dd_up = dz_up;
                if degrad == 1.7976931348623157e+308f64 {
                    break;
                }
            }
        }
        j += 1;
        j;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    (1 as i32 <= jj && jj <= n
        || {
            glp_assert_(
                b"1 <= jj && jj <= n\0" as *const u8 as *const i8,
                b"draft/glpios09.c\0" as *const u8 as *const i8,
                361 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if degrad < 1e-6f64 * (1.0f64 + 0.001f64 * fabs((*mip).obj_val)) {
        jj = branch_mostf(T, &mut next);
    } else if (*(*T).parm).msg_lev >= 4 as i32 {
        glp_printf(
            b"branch_drtom: column %d chosen to branch on\n\0" as *const u8 as *const i8,
            jj,
        );
        if fabs(dd_dn) == 1.7976931348623157e+308f64 {
            glp_printf(
                b"branch_drtom: down-branch is infeasible\n\0" as *const u8 as *const i8,
            );
        } else {
            glp_printf(
                b"branch_drtom: down-branch bound is %.9e\n\0" as *const u8 as *const i8,
                glp_get_obj_val(mip) + dd_dn,
            );
        }
        if fabs(dd_up) == 1.7976931348623157e+308f64 {
            glp_printf(
                b"branch_drtom: up-branch   is infeasible\n\0" as *const u8 as *const i8,
            );
        } else {
            glp_printf(
                b"branch_drtom: up-branch   bound is %.9e\n\0" as *const u8 as *const i8,
                glp_get_obj_val(mip) + dd_up,
            );
        }
    }
    *_next = next;
    return jj;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_pcost_init(
    mut tree: *mut glp_tree,
) -> *mut libc::c_void {
    let mut csa: *mut csa = 0 as *mut csa;
    let mut n: i32 = (*tree).n;
    let mut j: i32 = 0;
    csa = glp_alloc(1 as i32, ::core::mem::size_of::<csa>() as u64 as i32) as *mut csa;
    (*csa).dn_cnt = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*csa).dn_sum = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    (*csa).up_cnt = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*csa).up_sum = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    j = 1 as i32;
    while j <= n {
        let ref mut fresh0 = *((*csa).up_cnt).offset(j as isize);
        *fresh0 = 0 as i32;
        *((*csa).dn_cnt).offset(j as isize) = *fresh0;
        let ref mut fresh1 = *((*csa).up_sum).offset(j as isize);
        *fresh1 = 0.0f64;
        *((*csa).dn_sum).offset(j as isize) = *fresh1;
        j += 1;
        j;
    }
    return csa as *mut libc::c_void;
}
unsafe extern "C" fn eval_degrad(
    mut P: *mut glp_prob,
    mut j: i32,
    mut bnd: libc::c_double,
) -> libc::c_double {
    let mut lp: *mut glp_prob = 0 as *mut glp_prob;
    let mut parm: glp_smcp = glp_smcp {
        msg_lev: 0,
        meth: 0,
        pricing: 0,
        r_test: 0,
        tol_bnd: 0.,
        tol_dj: 0.,
        tol_piv: 0.,
        obj_ll: 0.,
        obj_ul: 0.,
        it_lim: 0,
        tm_lim: 0,
        out_frq: 0,
        out_dly: 0,
        presolve: 0,
        excl: 0,
        shift: 0,
        aorn: 0,
        foo_bar: [0.; 33],
    };
    let mut ret: i32 = 0;
    let mut degrad: libc::c_double = 0.;
    (glp_get_status(P) == 5 as i32
        || {
            glp_assert_(
                b"glp_get_status(P) == GLP_OPT\0" as *const u8 as *const i8,
                b"draft/glpios09.c\0" as *const u8 as *const i8,
                432 as i32,
            );
            1 as i32 != 0
        }) as i32;
    lp = glp_create_prob();
    glp_copy_prob(lp, P, 0 as i32);
    glp_set_col_bnds(lp, j, 5 as i32, bnd, bnd);
    glp_init_smcp(&mut parm);
    parm.msg_lev = 0 as i32;
    parm.meth = 3 as i32;
    parm.it_lim = 30 as i32;
    parm.out_dly = 1000 as i32;
    parm.meth = 3 as i32;
    ret = glp_simplex(lp, &mut parm);
    if ret == 0 as i32 || ret == 0x8 as i32 {
        if glp_get_prim_stat(lp) == 4 as i32 {
            degrad = 1.7976931348623157e+308f64;
        } else if glp_get_dual_stat(lp) == 2 as i32 {
            if (*P).dir == 1 as i32 {
                degrad = (*lp).obj_val - (*P).obj_val;
            } else if (*P).dir == 2 as i32 {
                degrad = (*P).obj_val - (*lp).obj_val;
            } else {
                (P != P
                    || {
                        glp_assert_(
                            b"P != P\0" as *const u8 as *const i8,
                            b"draft/glpios09.c\0" as *const u8 as *const i8,
                            459 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if degrad < 1e-6f64 * (1.0f64 + 0.001f64 * fabs((*P).obj_val)) {
                degrad = 0.0f64;
            }
        } else {
            degrad = 0.0f64;
        }
    } else {
        degrad = 0.0f64;
    }
    glp_delete_prob(lp);
    return degrad;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_pcost_update(mut tree: *mut glp_tree) {
    let mut j: i32 = 0;
    let mut dx: libc::c_double = 0.;
    let mut dz: libc::c_double = 0.;
    let mut psi: libc::c_double = 0.;
    let mut csa: *mut csa = (*tree).pcost as *mut csa;
    (!csa.is_null()
        || {
            glp_assert_(
                b"csa != NULL\0" as *const u8 as *const i8,
                b"draft/glpios09.c\0" as *const u8 as *const i8,
                491 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!((*tree).curr).is_null()
        || {
            glp_assert_(
                b"tree->curr != NULL\0" as *const u8 as *const i8,
                b"draft/glpios09.c\0" as *const u8 as *const i8,
                492 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if !((*(*tree).curr).up).is_null() {
        j = (*(*(*tree).curr).up).br_var;
        (1 as i32 <= j && j <= (*tree).n
            || {
                glp_assert_(
                    b"1 <= j && j <= tree->n\0" as *const u8 as *const i8,
                    b"draft/glpios09.c\0" as *const u8 as *const i8,
                    498 as i32,
                );
                1 as i32 != 0
            }) as i32;
        dx = (**((*(*tree).mip).col).offset(j as isize)).prim
            - (*(*(*tree).curr).up).br_val;
        (dx != 0.0f64
            || {
                glp_assert_(
                    b"dx != 0.0\0" as *const u8 as *const i8,
                    b"draft/glpios09.c\0" as *const u8 as *const i8,
                    505 as i32,
                );
                1 as i32 != 0
            }) as i32;
        dz = (*(*tree).mip).obj_val - (*(*(*tree).curr).up).lp_obj;
        psi = fabs(dz / dx);
        if dx < 0.0f64 {
            let ref mut fresh2 = *((*csa).dn_cnt).offset(j as isize);
            *fresh2 += 1;
            *fresh2;
            *((*csa).dn_sum).offset(j as isize) += psi;
        } else {
            let ref mut fresh3 = *((*csa).up_cnt).offset(j as isize);
            *fresh3 += 1;
            *fresh3;
            *((*csa).up_sum).offset(j as isize) += psi;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_pcost_free(mut tree: *mut glp_tree) {
    let mut csa: *mut csa = (*tree).pcost as *mut csa;
    (!csa.is_null()
        || {
            glp_assert_(
                b"csa != NULL\0" as *const u8 as *const i8,
                b"draft/glpios09.c\0" as *const u8 as *const i8,
                528 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_free((*csa).dn_cnt as *mut libc::c_void);
    glp_free((*csa).dn_sum as *mut libc::c_void);
    glp_free((*csa).up_cnt as *mut libc::c_void);
    glp_free((*csa).up_sum as *mut libc::c_void);
    glp_free(csa as *mut libc::c_void);
    (*tree).pcost = 0 as *mut libc::c_void;
}
unsafe extern "C" fn eval_psi(
    mut T: *mut glp_tree,
    mut j: i32,
    mut brnch: i32,
) -> libc::c_double {
    let mut current_block: u64;
    let mut csa: *mut csa = (*T).pcost as *mut csa;
    let mut beta: libc::c_double = 0.;
    let mut degrad: libc::c_double = 0.;
    let mut psi: libc::c_double = 0.;
    (!csa.is_null()
        || {
            glp_assert_(
                b"csa != NULL\0" as *const u8 as *const i8,
                b"draft/glpios09.c\0" as *const u8 as *const i8,
                543 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= j && j <= (*T).n
        || {
            glp_assert_(
                b"1 <= j && j <= T->n\0" as *const u8 as *const i8,
                b"draft/glpios09.c\0" as *const u8 as *const i8,
                544 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if brnch == 1 as i32 {
        if *((*csa).dn_cnt).offset(j as isize) == 0 as i32 {
            beta = (**((*(*T).mip).col).offset(j as isize)).prim;
            degrad = eval_degrad((*T).mip, j, floor(beta));
            if degrad == 1.7976931348623157e+308f64 {
                psi = 1.7976931348623157e+308f64;
                current_block = 13263053202518799190;
            } else {
                *((*csa).dn_cnt).offset(j as isize) = 1 as i32;
                *((*csa).dn_sum).offset(j as isize) = degrad / (beta - floor(beta));
                current_block = 17216689946888361452;
            }
        } else {
            current_block = 17216689946888361452;
        }
        match current_block {
            13263053202518799190 => {}
            _ => {
                psi = *((*csa).dn_sum).offset(j as isize)
                    / *((*csa).dn_cnt).offset(j as isize) as libc::c_double;
            }
        }
    } else if brnch == 2 as i32 {
        if *((*csa).up_cnt).offset(j as isize) == 0 as i32 {
            beta = (**((*(*T).mip).col).offset(j as isize)).prim;
            degrad = eval_degrad((*T).mip, j, ceil(beta));
            if degrad == 1.7976931348623157e+308f64 {
                psi = 1.7976931348623157e+308f64;
                current_block = 13263053202518799190;
            } else {
                *((*csa).up_cnt).offset(j as isize) = 1 as i32;
                *((*csa).up_sum).offset(j as isize) = degrad / (ceil(beta) - beta);
                current_block = 1054647088692577877;
            }
        } else {
            current_block = 1054647088692577877;
        }
        match current_block {
            13263053202518799190 => {}
            _ => {
                psi = *((*csa).up_sum).offset(j as isize)
                    / *((*csa).up_cnt).offset(j as isize) as libc::c_double;
            }
        }
    } else {
        (brnch != brnch
            || {
                glp_assert_(
                    b"brnch != brnch\0" as *const u8 as *const i8,
                    b"draft/glpios09.c\0" as *const u8 as *const i8,
                    576 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    return psi;
}
unsafe extern "C" fn progress(mut T: *mut glp_tree) {
    let mut csa: *mut csa = (*T).pcost as *mut csa;
    let mut j: i32 = 0;
    let mut nv: i32 = 0 as i32;
    let mut ni: i32 = 0 as i32;
    j = 1 as i32;
    while j <= (*T).n {
        if glp_ios_can_branch(T, j) != 0 {
            nv += 1;
            nv;
            if *((*csa).dn_cnt).offset(j as isize) > 0 as i32
                && *((*csa).up_cnt).offset(j as isize) > 0 as i32
            {
                ni += 1;
                ni;
            }
        }
        j += 1;
        j;
    }
    glp_printf(
        b"Pseudocosts initialized for %d of %d variables\n\0" as *const u8 as *const i8,
        ni,
        nv,
    );
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_pcost_branch(
    mut T: *mut glp_tree,
    mut _next: *mut i32,
) -> i32 {
    let mut current_block: u64;
    let mut t: libc::c_double = glp_time();
    let mut j: i32 = 0;
    let mut jjj: i32 = 0;
    let mut sel: i32 = 0;
    let mut beta: libc::c_double = 0.;
    let mut psi: libc::c_double = 0.;
    let mut d1: libc::c_double = 0.;
    let mut d2: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut dmax: libc::c_double = 0.;
    if ((*T).pcost).is_null() {
        (*T).pcost = _glp_ios_pcost_init(T);
    }
    jjj = 0 as i32;
    dmax = -1.0f64;
    j = 1 as i32;
    loop {
        if !(j <= (*T).n) {
            current_block = 1109700713171191020;
            break;
        }
        if !(glp_ios_can_branch(T, j) == 0) {
            beta = (**((*(*T).mip).col).offset(j as isize)).prim;
            psi = eval_psi(T, j, 1 as i32);
            if psi == 1.7976931348623157e+308f64 {
                jjj = j;
                sel = 1 as i32;
                current_block = 18416197436858168928;
                break;
            } else {
                d1 = psi * (beta - floor(beta));
                psi = eval_psi(T, j, 2 as i32);
                if psi == 1.7976931348623157e+308f64 {
                    jjj = j;
                    sel = 2 as i32;
                    current_block = 18416197436858168928;
                    break;
                } else {
                    d2 = psi * (ceil(beta) - beta);
                    d = if d1 > d2 { d1 } else { d2 };
                    if dmax < d {
                        dmax = d;
                        jjj = j;
                        sel = if d1 <= d2 { 1 as i32 } else { 2 as i32 };
                    }
                    if (*(*T).parm).msg_lev >= 1 as i32 {
                        if glp_difftime(glp_time(), t) >= 10.0f64 {
                            progress(T);
                            t = glp_time();
                        }
                    }
                }
            }
        }
        j += 1;
        j;
    }
    match current_block {
        1109700713171191020 => {
            if dmax == 0.0f64 {
                jjj = branch_mostf(T, &mut sel);
            }
        }
        _ => {}
    }
    *_next = sel;
    return jjj;
}