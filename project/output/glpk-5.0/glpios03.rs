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
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_difftime(t1: libc::c_double, t0: libc::c_double) -> libc::c_double;
    fn glp_time() -> libc::c_double;
    fn glp_mem_usage(
        count: *mut i32,
        cpeak: *mut i32,
        total: *mut size_t,
        tpeak: *mut size_t,
    );
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_cov_gen1(P: *mut glp_prob, cov: *mut glp_cov, pool: *mut glp_prob);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn _glp_dmp_in_use(pool: *mut DMP) -> size_t;
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_add_cols(P: *mut glp_prob, ncs: i32) -> i32;
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_del_rows(P: *mut glp_prob, nrs: i32, num: *const i32);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_get_mat_row(
        P: *mut glp_prob,
        i: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn glp_factorize(P: *mut glp_prob) -> i32;
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
    fn glp_ios_heur_sol(T: *mut glp_tree, x: *const libc::c_double) -> i32;
    fn glp_gmi_gen(P: *mut glp_prob, pool: *mut glp_prob, max_cuts: i32) -> i32;
    fn glp_cov_init(P: *mut glp_prob) -> *mut glp_cov;
    fn _glp_ios_choose_node(T: *mut glp_tree) -> i32;
    fn _glp_ios_feas_pump(T: *mut glp_tree);
    fn _glp_ios_proxy_heur(T: *mut glp_tree);
    fn _glp_ios_process_cuts(T: *mut glp_tree);
    fn _glp_ios_pcost_update(tree: *mut glp_tree);
    fn _glp_ios_choose_var(T: *mut glp_tree, next: *mut i32) -> i32;
    fn _glp_ios_preprocess_node(tree: *mut glp_tree, max_pass: i32) -> i32;
    fn _glp_ios_process_sol(T: *mut glp_tree);
    fn _glp_ios_clear_pool(tree: *mut glp_tree, pool: *mut IOSPOOL);
    fn _glp_ios_solve_node(tree: *mut glp_tree) -> i32;
    fn _glp_ios_relative_gap(tree: *mut glp_tree) -> libc::c_double;
    fn _glp_ios_best_node(tree: *mut glp_tree) -> i32;
    fn _glp_ios_is_hopeful(tree: *mut glp_tree, bound: libc::c_double) -> i32;
    fn _glp_ios_round_bound(
        tree: *mut glp_tree,
        bound: libc::c_double,
    ) -> libc::c_double;
    fn _glp_ios_eval_degrad(
        tree: *mut glp_tree,
        j: i32,
        dn: *mut libc::c_double,
        up: *mut libc::c_double,
    );
    fn _glp_ios_delete_node(tree: *mut glp_tree, p: i32);
    fn _glp_ios_clone_node(tree: *mut glp_tree, p: i32, nnn: i32, ref_0: *mut i32);
    fn _glp_ios_freeze_node(tree: *mut glp_tree);
    fn _glp_ios_revive_node(tree: *mut glp_tree, p: i32);
    fn glp_cov_free(cov: *mut glp_cov);
    fn glp_mir_init(P: *mut glp_prob) -> *mut glp_mir;
    fn glp_mir_gen(P: *mut glp_prob, mir: *mut glp_mir, pool: *mut glp_prob) -> i32;
    fn glp_mir_free(mir: *mut glp_mir);
    fn glp_cfg_init(P: *mut glp_prob) -> *mut glp_cfg;
    fn glp_cfg_free(G: *mut glp_cfg);
    fn glp_clq_cut(
        P: *mut glp_prob,
        G: *mut glp_cfg,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
}
pub type size_t = u64;
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
unsafe extern "C" fn show_progress(mut T: *mut glp_tree, mut bingo: i32) {
    let mut p: i32 = 0;
    let mut temp: libc::c_double = 0.;
    let mut best_mip: [i8; 50] = [0; 50];
    let mut best_bound: [i8; 50] = [0; 50];
    let mut rho: *mut i8 = 0 as *mut i8;
    let mut rel_gap: [i8; 50] = [0; 50];
    if (*(*T).mip).mip_stat == 2 as i32 {
        sprintf(
            best_mip.as_mut_ptr(),
            b"%17.9e\0" as *const u8 as *const i8,
            (*(*T).mip).mip_obj,
        );
    } else {
        sprintf(
            best_mip.as_mut_ptr(),
            b"%17s\0" as *const u8 as *const i8,
            b"not found yet\0" as *const u8 as *const i8,
        );
    }
    p = _glp_ios_best_node(T);
    if p == 0 as i32 {
        sprintf(
            best_bound.as_mut_ptr(),
            b"%17s\0" as *const u8 as *const i8,
            b"tree is empty\0" as *const u8 as *const i8,
        );
    } else {
        temp = (*(*((*T).slot).offset(p as isize)).node).bound;
        if temp == -1.7976931348623157e+308f64 {
            sprintf(
                best_bound.as_mut_ptr(),
                b"%17s\0" as *const u8 as *const i8,
                b"-inf\0" as *const u8 as *const i8,
            );
        } else if temp == 1.7976931348623157e+308f64 {
            sprintf(
                best_bound.as_mut_ptr(),
                b"%17s\0" as *const u8 as *const i8,
                b"+inf\0" as *const u8 as *const i8,
            );
        } else {
            if fabs(temp) < 1e-9f64 {
                temp = 0 as i32 as libc::c_double;
            }
            sprintf(
                best_bound.as_mut_ptr(),
                b"%17.9e\0" as *const u8 as *const i8,
                temp,
            );
        }
    }
    if (*(*T).mip).dir == 1 as i32 {
        rho = b">=\0" as *const u8 as *const i8 as *mut i8;
    } else if (*(*T).mip).dir == 2 as i32 {
        rho = b"<=\0" as *const u8 as *const i8 as *mut i8;
    } else {
        (T != T
            || {
                glp_assert_(
                    b"T != T\0" as *const u8 as *const i8,
                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                    83 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    temp = _glp_ios_relative_gap(T);
    if temp == 0.0f64 {
        sprintf(rel_gap.as_mut_ptr(), b"  0.0%%\0" as *const u8 as *const i8);
    } else if temp < 0.001f64 {
        sprintf(rel_gap.as_mut_ptr(), b"< 0.1%%\0" as *const u8 as *const i8);
    } else if temp <= 9.999f64 {
        sprintf(
            rel_gap.as_mut_ptr(),
            b"%5.1f%%\0" as *const u8 as *const i8,
            100.0f64 * temp,
        );
    } else {
        sprintf(
            rel_gap.as_mut_ptr(),
            b"%6s\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
    glp_printf(
        b"+%6d: %s %s %s %s %s (%d; %d)\n\0" as *const u8 as *const i8,
        (*(*T).mip).it_cnt,
        if bingo != 0 {
            b">>>>>\0" as *const u8 as *const i8
        } else {
            b"mip =\0" as *const u8 as *const i8
        },
        best_mip.as_mut_ptr(),
        rho,
        best_bound.as_mut_ptr(),
        rel_gap.as_mut_ptr(),
        (*T).a_cnt,
        (*T).t_cnt - (*T).n_cnt,
    );
    (*T).tm_lag = glp_time();
}
unsafe extern "C" fn is_branch_hopeful(mut T: *mut glp_tree, mut p: i32) -> i32 {
    (1 as i32 <= p && p <= (*T).nslots
        || {
            glp_assert_(
                b"1 <= p && p <= T->nslots\0" as *const u8 as *const i8,
                b"draft/glpios03.c\0" as *const u8 as *const i8,
                117 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!((*((*T).slot).offset(p as isize)).node).is_null()
        || {
            glp_assert_(
                b"T->slot[p].node != NULL\0" as *const u8 as *const i8,
                b"draft/glpios03.c\0" as *const u8 as *const i8,
                118 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return _glp_ios_is_hopeful(T, (*(*((*T).slot).offset(p as isize)).node).bound);
}
unsafe extern "C" fn check_integrality(mut T: *mut glp_tree) {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut j: i32 = 0;
    let mut type_0: i32 = 0;
    let mut ii_cnt: i32 = 0 as i32;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut temp1: libc::c_double = 0.;
    let mut temp2: libc::c_double = 0.;
    let mut ii_sum: libc::c_double = 0.0f64;
    let mut current_block_15: u64;
    j = 1 as i32;
    while j <= (*mip).n {
        let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
        *((*T).non_int).offset(j as isize) = 0 as i32 as u8;
        if !((*col).kind != 2 as i32) {
            if !((*col).stat != 1 as i32) {
                type_0 = (*col).type_0;
                lb = (*col).lb;
                ub = (*col).ub;
                x = (*col).prim;
                if type_0 == 2 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32 {
                    temp1 = lb - (*(*T).parm).tol_int;
                    temp2 = lb + (*(*T).parm).tol_int;
                    if temp1 <= x && x <= temp2 {
                        current_block_15 = 6239978542346980191;
                    } else if x < lb {
                        current_block_15 = 6239978542346980191;
                    } else {
                        current_block_15 = 2968425633554183086;
                    }
                } else {
                    current_block_15 = 2968425633554183086;
                }
                match current_block_15 {
                    6239978542346980191 => {}
                    _ => {
                        if type_0 == 3 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32
                        {
                            temp1 = ub - (*(*T).parm).tol_int;
                            temp2 = ub + (*(*T).parm).tol_int;
                            if temp1 <= x && x <= temp2 {
                                current_block_15 = 6239978542346980191;
                            } else if x > ub {
                                current_block_15 = 6239978542346980191;
                            } else {
                                current_block_15 = 8831408221741692167;
                            }
                        } else {
                            current_block_15 = 8831408221741692167;
                        }
                        match current_block_15 {
                            6239978542346980191 => {}
                            _ => {
                                temp1 = floor(x + 0.5f64) - (*(*T).parm).tol_int;
                                temp2 = floor(x + 0.5f64) + (*(*T).parm).tol_int;
                                if !(temp1 <= x && x <= temp2) {
                                    *((*T).non_int).offset(j as isize) = 1 as i32 as u8;
                                    ii_cnt += 1;
                                    ii_cnt;
                                    temp1 = x - floor(x);
                                    temp2 = ceil(x) - x;
                                    (temp1 > 0.0f64 && temp2 > 0.0f64
                                        || {
                                            glp_assert_(
                                                b"temp1 > 0.0 && temp2 > 0.0\0" as *const u8 as *const i8,
                                                b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                206 as i32,
                                            );
                                            1 as i32 != 0
                                        }) as i32;
                                    ii_sum += if temp1 <= temp2 { temp1 } else { temp2 };
                                }
                            }
                        }
                    }
                }
            }
        }
        j += 1;
        j;
    }
    (!((*T).curr).is_null()
        || {
            glp_assert_(
                b"T->curr != NULL\0" as *const u8 as *const i8,
                b"draft/glpios03.c\0" as *const u8 as *const i8,
                210 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*(*T).curr).ii_cnt = ii_cnt;
    (*(*T).curr).ii_sum = ii_sum;
    if (*(*T).parm).msg_lev >= 4 as i32 {
        if ii_cnt == 0 as i32 {
            glp_printf(b"There are no fractional columns\n\0" as *const u8 as *const i8);
        } else if ii_cnt == 1 as i32 {
            glp_printf(
                b"There is one fractional column, integer infeasibility is %.3e\n\0"
                    as *const u8 as *const i8,
                ii_sum,
            );
        } else {
            glp_printf(
                b"There are %d fractional columns, integer infeasibility is %.3e\n\0"
                    as *const u8 as *const i8,
                ii_cnt,
                ii_sum,
            );
        }
    }
}
unsafe extern "C" fn record_solution(mut T: *mut glp_tree) {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    (*mip).mip_stat = 2 as i32;
    (*mip).mip_obj = (*mip).obj_val;
    i = 1 as i32;
    while i <= (*mip).m {
        let mut row: *mut GLPROW = *((*mip).row).offset(i as isize);
        (*row).mipx = (*row).prim;
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= (*mip).n {
        let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
        if (*col).kind == 1 as i32 {
            (*col).mipx = (*col).prim;
        } else if (*col).kind == 2 as i32 {
            (*col).mipx = floor((*col).prim + 0.5f64);
        } else {
            (col != col
                || {
                    glp_assert_(
                        b"col != col\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        252 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        j += 1;
        j;
    }
    (*T).sol_cnt += 1;
    (*T).sol_cnt;
}
unsafe extern "C" fn fix_by_red_cost(mut T: *mut glp_tree) {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut j: i32 = 0;
    let mut stat: i32 = 0;
    let mut fixed: i32 = 0 as i32;
    let mut obj: libc::c_double = 0.;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut dj: libc::c_double = 0.;
    ((*(*T).mip).mip_stat == 2 as i32
        || {
            glp_assert_(
                b"T->mip->mip_stat == GLP_FEAS\0" as *const u8 as *const i8,
                b"draft/glpios03.c\0" as *const u8 as *const i8,
                271 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*mip).pbs_stat == 2 as i32 && (*mip).dbs_stat == 2 as i32
        || {
            glp_assert_(
                b"mip->pbs_stat == GLP_FEAS && mip->dbs_stat == GLP_FEAS\0" as *const u8
                    as *const i8,
                b"draft/glpios03.c\0" as *const u8 as *const i8,
                273 as i32,
            );
            1 as i32 != 0
        }) as i32;
    obj = (*mip).obj_val;
    j = 1 as i32;
    while j <= (*mip).n {
        let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
        if !((*col).kind != 2 as i32) {
            lb = (*col).lb;
            ub = (*col).ub;
            stat = (*col).stat;
            dj = (*col).dual;
            match (*mip).dir {
                1 => {
                    if stat == 2 as i32 {
                        if dj < 0.0f64 {
                            dj = 0.0f64;
                        }
                        if obj + dj >= (*mip).mip_obj {
                            glp_set_col_bnds(mip, j, 5 as i32, lb, lb);
                            fixed += 1;
                            fixed;
                        }
                    } else if stat == 3 as i32 {
                        if dj > 0.0f64 {
                            dj = 0.0f64;
                        }
                        if obj - dj >= (*mip).mip_obj {
                            glp_set_col_bnds(mip, j, 5 as i32, ub, ub);
                            fixed += 1;
                            fixed;
                        }
                    }
                }
                2 => {
                    if stat == 2 as i32 {
                        if dj > 0.0f64 {
                            dj = 0.0f64;
                        }
                        if obj + dj <= (*mip).mip_obj {
                            glp_set_col_bnds(mip, j, 5 as i32, lb, lb);
                            fixed += 1;
                            fixed;
                        }
                    } else if stat == 3 as i32 {
                        if dj < 0.0f64 {
                            dj = 0.0f64;
                        }
                        if obj - dj <= (*mip).mip_obj {
                            glp_set_col_bnds(mip, j, 5 as i32, ub, ub);
                            fixed += 1;
                            fixed;
                        }
                    }
                }
                _ => {
                    (T != T
                        || {
                            glp_assert_(
                                b"T != T\0" as *const u8 as *const i8,
                                b"draft/glpios03.c\0" as *const u8 as *const i8,
                                318 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
        }
        j += 1;
        j;
    }
    if (*(*T).parm).msg_lev >= 4 as i32 {
        if !(fixed == 0 as i32) {
            if fixed == 1 as i32 {
                glp_printf(
                    b"One column has been fixed by reduced cost\n\0" as *const u8
                        as *const i8,
                );
            } else {
                glp_printf(
                    b"%d columns have been fixed by reduced costs\n\0" as *const u8
                        as *const i8,
                    fixed,
                );
            }
        }
    }
    ((*mip).pbs_stat == 2 as i32 && (*mip).dbs_stat == 2 as i32
        || {
            glp_assert_(
                b"mip->pbs_stat == GLP_FEAS && mip->dbs_stat == GLP_FEAS\0" as *const u8
                    as *const i8,
                b"draft/glpios03.c\0" as *const u8 as *const i8,
                332 as i32,
            );
            1 as i32 != 0
        }) as i32;
}
unsafe extern "C" fn branch_on(mut T: *mut glp_tree, mut j: i32, mut next: i32) -> i32 {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut m: i32 = (*mip).m;
    let mut n: i32 = (*mip).n;
    let mut type_0: i32 = 0;
    let mut dn_type: i32 = 0;
    let mut up_type: i32 = 0;
    let mut dn_bad: i32 = 0;
    let mut up_bad: i32 = 0;
    let mut p: i32 = 0;
    let mut ret: i32 = 0;
    let mut clone: [i32; 3] = [0; 3];
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut new_ub: libc::c_double = 0.;
    let mut new_lb: libc::c_double = 0.;
    let mut dn_lp: libc::c_double = 0.;
    let mut up_lp: libc::c_double = 0.;
    let mut dn_bnd: libc::c_double = 0.;
    let mut up_bnd: libc::c_double = 0.;
    (1 as i32 <= j && j <= n
        || {
            glp_assert_(
                b"1 <= j && j <= n\0" as *const u8 as *const i8,
                b"draft/glpios03.c\0" as *const u8 as *const i8,
                371 as i32,
            );
            1 as i32 != 0
        }) as i32;
    type_0 = (**((*mip).col).offset(j as isize)).type_0;
    lb = (**((*mip).col).offset(j as isize)).lb;
    ub = (**((*mip).col).offset(j as isize)).ub;
    beta = (**((*mip).col).offset(j as isize)).prim;
    new_ub = floor(beta);
    new_lb = ceil(beta);
    match type_0 {
        1 => {
            dn_type = 3 as i32;
            up_type = 2 as i32;
        }
        2 => {
            (lb <= new_ub
                || {
                    glp_assert_(
                        b"lb <= new_ub\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        385 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            dn_type = if lb == new_ub { 5 as i32 } else { 4 as i32 };
            (lb + 1.0f64 <= new_lb
                || {
                    glp_assert_(
                        b"lb + 1.0 <= new_lb\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        387 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            up_type = 2 as i32;
        }
        3 => {
            (new_ub <= ub - 1.0f64
                || {
                    glp_assert_(
                        b"new_ub <= ub - 1.0\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        391 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            dn_type = 3 as i32;
            (new_lb <= ub
                || {
                    glp_assert_(
                        b"new_lb <= ub\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        393 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            up_type = if new_lb == ub { 5 as i32 } else { 4 as i32 };
        }
        4 => {
            (lb <= new_ub && new_ub <= ub - 1.0f64
                || {
                    glp_assert_(
                        b"lb <= new_ub && new_ub <= ub - 1.0\0" as *const u8
                            as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        397 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            dn_type = if lb == new_ub { 5 as i32 } else { 4 as i32 };
            (lb + 1.0f64 <= new_lb && new_lb <= ub
                || {
                    glp_assert_(
                        b"lb + 1.0 <= new_lb && new_lb <= ub\0" as *const u8
                            as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        399 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            up_type = if new_lb == ub { 5 as i32 } else { 4 as i32 };
        }
        _ => {
            (type_0 != type_0
                || {
                    glp_assert_(
                        b"type != type\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        403 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    _glp_ios_eval_degrad(T, j, &mut dn_lp, &mut up_lp);
    dn_bnd = _glp_ios_round_bound(T, dn_lp);
    up_bnd = _glp_ios_round_bound(T, up_lp);
    dn_bad = (_glp_ios_is_hopeful(T, dn_bnd) == 0) as i32;
    up_bad = (_glp_ios_is_hopeful(T, up_bnd) == 0) as i32;
    if dn_bad != 0 && up_bad != 0 {
        if (*(*T).parm).msg_lev >= 4 as i32 {
            glp_printf(
                b"Both down- and up-branches are hopeless\n\0" as *const u8 as *const i8,
            );
        }
        ret = 2 as i32;
    } else if up_bad != 0 {
        if (*(*T).parm).msg_lev >= 4 as i32 {
            glp_printf(b"Up-branch is hopeless\n\0" as *const u8 as *const i8);
        }
        glp_set_col_bnds(mip, j, dn_type, lb, new_ub);
        (*(*T).curr).lp_obj = dn_lp;
        if (*mip).dir == 1 as i32 {
            if (*(*T).curr).bound < dn_bnd {
                (*(*T).curr).bound = dn_bnd;
            }
        } else if (*mip).dir == 2 as i32 {
            if (*(*T).curr).bound > dn_bnd {
                (*(*T).curr).bound = dn_bnd;
            }
        } else {
            (mip != mip
                || {
                    glp_assert_(
                        b"mip != mip\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        433 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        ret = 1 as i32;
    } else if dn_bad != 0 {
        if (*(*T).parm).msg_lev >= 4 as i32 {
            glp_printf(b"Down-branch is hopeless\n\0" as *const u8 as *const i8);
        }
        glp_set_col_bnds(mip, j, up_type, new_lb, ub);
        (*(*T).curr).lp_obj = up_lp;
        if (*mip).dir == 1 as i32 {
            if (*(*T).curr).bound < up_bnd {
                (*(*T).curr).bound = up_bnd;
            }
        } else if (*mip).dir == 2 as i32 {
            if (*(*T).curr).bound > up_bnd {
                (*(*T).curr).bound = up_bnd;
            }
        } else {
            (mip != mip
                || {
                    glp_assert_(
                        b"mip != mip\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        451 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        ret = 1 as i32;
    } else {
        if (*(*T).parm).msg_lev >= 4 as i32 {
            glp_printf(
                b"Branching on column %d, primal value is %.9e\n\0" as *const u8
                    as *const i8,
                j,
                beta,
            );
        }
        (!((*T).curr).is_null()
            || {
                glp_assert_(
                    b"T->curr != NULL\0" as *const u8 as *const i8,
                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                    460 as i32,
                );
                1 as i32 != 0
            }) as i32;
        p = (*(*T).curr).p;
        (*(*T).curr).br_var = j;
        (*(*T).curr).br_val = beta;
        _glp_ios_freeze_node(T);
        _glp_ios_clone_node(T, p, 2 as i32, clone.as_mut_ptr());
        if (*(*T).parm).msg_lev >= 4 as i32 {
            glp_printf(
                b"Node %d begins down branch, node %d begins up branch \n\0" as *const u8
                    as *const i8,
                clone[1 as i32 as usize],
                clone[2 as i32 as usize],
            );
        }
        node = (*((*T).slot).offset(clone[1 as i32 as usize] as isize)).node;
        (!node.is_null()
            || {
                glp_assert_(
                    b"node != NULL\0" as *const u8 as *const i8,
                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                    474 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (!((*node).up).is_null()
            || {
                glp_assert_(
                    b"node->up != NULL\0" as *const u8 as *const i8,
                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                    475 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (((*node).b_ptr).is_null()
            || {
                glp_assert_(
                    b"node->b_ptr == NULL\0" as *const u8 as *const i8,
                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                    476 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*node).b_ptr = _glp_dmp_get_atom(
            (*T).pool,
            ::core::mem::size_of::<IOSBND>() as u64 as i32,
        ) as *mut IOSBND;
        (*(*node).b_ptr).k = m + j;
        (*(*node).b_ptr).type_0 = dn_type as u8;
        (*(*node).b_ptr).lb = lb;
        (*(*node).b_ptr).ub = new_ub;
        (*(*node).b_ptr).next = 0 as *mut IOSBND;
        (*node).lp_obj = dn_lp;
        if (*mip).dir == 1 as i32 {
            if (*node).bound < dn_bnd {
                (*node).bound = dn_bnd;
            }
        } else if (*mip).dir == 2 as i32 {
            if (*node).bound > dn_bnd {
                (*node).bound = dn_bnd;
            }
        } else {
            (mip != mip
                || {
                    glp_assert_(
                        b"mip != mip\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        493 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        node = (*((*T).slot).offset(clone[2 as i32 as usize] as isize)).node;
        (!node.is_null()
            || {
                glp_assert_(
                    b"node != NULL\0" as *const u8 as *const i8,
                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                    496 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (!((*node).up).is_null()
            || {
                glp_assert_(
                    b"node->up != NULL\0" as *const u8 as *const i8,
                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                    497 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (((*node).b_ptr).is_null()
            || {
                glp_assert_(
                    b"node->b_ptr == NULL\0" as *const u8 as *const i8,
                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                    498 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*node).b_ptr = _glp_dmp_get_atom(
            (*T).pool,
            ::core::mem::size_of::<IOSBND>() as u64 as i32,
        ) as *mut IOSBND;
        (*(*node).b_ptr).k = m + j;
        (*(*node).b_ptr).type_0 = up_type as u8;
        (*(*node).b_ptr).lb = new_lb;
        (*(*node).b_ptr).ub = ub;
        (*(*node).b_ptr).next = 0 as *mut IOSBND;
        (*node).lp_obj = up_lp;
        if (*mip).dir == 1 as i32 {
            if (*node).bound < up_bnd {
                (*node).bound = up_bnd;
            }
        } else if (*mip).dir == 2 as i32 {
            if (*node).bound > up_bnd {
                (*node).bound = up_bnd;
            }
        } else {
            (mip != mip
                || {
                    glp_assert_(
                        b"mip != mip\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        515 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        ((*T).child == 0 as i32
            || {
                glp_assert_(
                    b"T->child == 0\0" as *const u8 as *const i8,
                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                    517 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if next == 0 as i32 {
            (*T).child = 0 as i32;
        } else if next == 1 as i32 {
            (*T).child = clone[1 as i32 as usize];
        } else if next == 2 as i32 {
            (*T).child = clone[2 as i32 as usize];
        } else {
            (next != next
                || {
                    glp_assert_(
                        b"next != next\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        525 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        ret = 0 as i32;
    }
    return ret;
}
unsafe extern "C" fn cleanup_the_tree(mut T: *mut glp_tree) {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut next_node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut count: i32 = 0 as i32;
    ((*(*T).mip).mip_stat == 2 as i32
        || {
            glp_assert_(
                b"T->mip->mip_stat == GLP_FEAS\0" as *const u8 as *const i8,
                b"draft/glpios03.c\0" as *const u8 as *const i8,
                543 as i32,
            );
            1 as i32 != 0
        }) as i32;
    node = (*T).head;
    while !node.is_null() {
        next_node = (*node).next;
        if is_branch_hopeful(T, (*node).p) == 0 {
            _glp_ios_delete_node(T, (*node).p);
            count += 1;
            count;
        }
        node = next_node;
    }
    if (*(*T).parm).msg_lev >= 4 as i32 {
        if count == 1 as i32 {
            glp_printf(
                b"One hopeless branch has been pruned\n\0" as *const u8 as *const i8,
            );
        } else if count > 1 as i32 {
            glp_printf(
                b"%d hopeless branches have been pruned\n\0" as *const u8 as *const i8,
                count,
            );
        }
    }
}
unsafe extern "C" fn round_heur(mut T: *mut glp_tree) -> i32 {
    let mut current_block: u64;
    let mut P: *mut glp_prob = (*T).mip;
    let mut n: i32 = (*P).n;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ret: i32 = 0;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    x = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    j = 1 as i32;
    loop {
        if !(j <= n) {
            current_block = 17965632435239708295;
            break;
        }
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        if (*col).kind == 2 as i32 {
            *x.offset(j as isize) = floor((*col).prim + 0.5f64);
        } else if (*col).type_0 == 5 as i32 {
            *x.offset(j as isize) = (*col).prim;
        } else {
            ret = 3 as i32;
            current_block = 12234934137130331946;
            break;
        }
        j += 1;
        j;
    }
    match current_block {
        17965632435239708295 => {
            i = 1 as i32;
            loop {
                if !(i <= (*T).orig_m) {
                    current_block = 6669252993407410313;
                    break;
                }
                let mut type_0: i32 = *((*T).orig_type).offset(i as isize) as i32;
                let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
                let mut sum: libc::c_double = 0.;
                if !(type_0 == 1 as i32) {
                    sum = 0.0f64;
                    aij = (**((*P).row).offset(i as isize)).ptr;
                    while !aij.is_null() {
                        sum += (*aij).val * *x.offset((*(*aij).col).j as isize);
                        aij = (*aij).r_next;
                    }
                    if type_0 == 2 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32 {
                        if sum < *((*T).orig_lb).offset(i as isize) - 1e-9f64 {
                            ret = 2 as i32;
                            current_block = 12234934137130331946;
                            break;
                        }
                    }
                    if type_0 == 3 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32 {
                        if sum > *((*T).orig_ub).offset(i as isize) + 1e-9f64 {
                            ret = 2 as i32;
                            current_block = 12234934137130331946;
                            break;
                        }
                    }
                }
                i += 1;
                i;
            }
            match current_block {
                12234934137130331946 => {}
                _ => {
                    if glp_ios_heur_sol(T, x as *const libc::c_double) == 0 as i32 {
                        ret = 0 as i32;
                    } else {
                        ret = 1 as i32;
                    }
                }
            }
        }
        _ => {}
    }
    glp_free(x as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn gmi_gen(mut T: *mut glp_tree) {
    let mut P: *mut glp_prob = 0 as *mut glp_prob;
    let mut pool: *mut glp_prob = 0 as *mut glp_prob;
    P = (*T).mip;
    pool = glp_create_prob();
    glp_add_cols(pool, (*P).n);
    glp_gmi_gen(P, pool, 50 as i32);
    if (*pool).m > 0 as i32 {
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        let mut ind: *mut i32 = 0 as *mut i32;
        let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
        ind = glp_alloc(1 as i32 + (*P).n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        val = glp_alloc(
            1 as i32 + (*P).n,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        i = 1 as i32;
        while i <= (*pool).m {
            len = glp_get_mat_row(pool, i, ind, val);
            glp_ios_add_row(
                T,
                0 as *const i8,
                1 as i32,
                0 as i32,
                len,
                ind as *const i32,
                val as *const libc::c_double,
                2 as i32,
                (**((*pool).row).offset(i as isize)).lb,
            );
            i += 1;
            i;
        }
        glp_free(ind as *mut libc::c_void);
        glp_free(val as *mut libc::c_void);
    }
    glp_delete_prob(pool);
}
unsafe extern "C" fn cov_gen(mut T: *mut glp_tree) {
    let mut P: *mut glp_prob = 0 as *mut glp_prob;
    let mut pool: *mut glp_prob = 0 as *mut glp_prob;
    if ((*T).cov_gen).is_null() {
        return;
    }
    P = (*T).mip;
    pool = glp_create_prob();
    glp_add_cols(pool, (*P).n);
    glp_cov_gen1(P, (*T).cov_gen, pool);
    if (*pool).m > 0 as i32 {
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        let mut ind: *mut i32 = 0 as *mut i32;
        let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
        ind = glp_alloc(1 as i32 + (*P).n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        val = glp_alloc(
            1 as i32 + (*P).n,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        i = 1 as i32;
        while i <= (*pool).m {
            len = glp_get_mat_row(pool, i, ind, val);
            glp_ios_add_row(
                T,
                0 as *const i8,
                3 as i32,
                0 as i32,
                len,
                ind as *const i32,
                val as *const libc::c_double,
                3 as i32,
                (**((*pool).row).offset(i as isize)).ub,
            );
            i += 1;
            i;
        }
        glp_free(ind as *mut libc::c_void);
        glp_free(val as *mut libc::c_void);
    }
    glp_delete_prob(pool);
}
unsafe extern "C" fn mir_gen(mut T: *mut glp_tree) {
    let mut P: *mut glp_prob = 0 as *mut glp_prob;
    let mut pool: *mut glp_prob = 0 as *mut glp_prob;
    P = (*T).mip;
    pool = glp_create_prob();
    glp_add_cols(pool, (*P).n);
    glp_mir_gen(P, (*T).mir_gen, pool);
    if (*pool).m > 0 as i32 {
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        let mut ind: *mut i32 = 0 as *mut i32;
        let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
        ind = glp_alloc(1 as i32 + (*P).n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        val = glp_alloc(
            1 as i32 + (*P).n,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        i = 1 as i32;
        while i <= (*pool).m {
            len = glp_get_mat_row(pool, i, ind, val);
            glp_ios_add_row(
                T,
                0 as *const i8,
                2 as i32,
                0 as i32,
                len,
                ind as *const i32,
                val as *const libc::c_double,
                3 as i32,
                (**((*pool).row).offset(i as isize)).ub,
            );
            i += 1;
            i;
        }
        glp_free(ind as *mut libc::c_void);
        glp_free(val as *mut libc::c_void);
    }
    glp_delete_prob(pool);
}
unsafe extern "C" fn clq_gen(mut T: *mut glp_tree, mut G: *mut glp_cfg) {
    let mut P: *mut glp_prob = (*T).mip;
    let mut n: i32 = (*P).n;
    let mut len: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    len = glp_clq_cut((*T).mip, G, ind, val);
    if len > 0 as i32 {
        glp_ios_add_row(
            T,
            0 as *const i8,
            4 as i32,
            0 as i32,
            len,
            ind as *const i32,
            val as *const libc::c_double,
            3 as i32,
            *val.offset(0 as i32 as isize),
        );
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
}
unsafe extern "C" fn generate_cuts(mut T: *mut glp_tree) {
    if (*(*T).parm).mir_cuts == 1 as i32 || (*(*T).parm).gmi_cuts == 1 as i32
        || (*(*T).parm).cov_cuts == 1 as i32 || (*(*T).parm).clq_cuts == 1 as i32
    {
        let mut i: i32 = 0;
        let mut max_cuts: i32 = 0;
        let mut added_cuts: i32 = 0;
        max_cuts = (*T).n;
        if max_cuts < 1000 as i32 {
            max_cuts = 1000 as i32;
        }
        added_cuts = 0 as i32;
        i = (*T).orig_m + 1 as i32;
        while i <= (*(*T).mip).m {
            if (**((*(*T).mip).row).offset(i as isize)).origin as i32 == 2 as i32 {
                added_cuts += 1;
                added_cuts;
            }
            i += 1;
            i;
        }
        if !(added_cuts >= max_cuts) {
            if (*(*T).parm).gmi_cuts == 1 as i32 {
                if (*(*T).curr).changed < 7 as i32 {
                    gmi_gen(T);
                }
            }
            if (*(*T).parm).mir_cuts == 1 as i32 {
                (!((*T).mir_gen).is_null()
                    || {
                        glp_assert_(
                            b"T->mir_gen != NULL\0" as *const u8 as *const i8,
                            b"draft/glpios03.c\0" as *const u8 as *const i8,
                            766 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                mir_gen(T);
            }
            if (*(*T).parm).cov_cuts == 1 as i32 {
                cov_gen(T);
            }
            if (*(*T).parm).clq_cuts == 1 as i32 {
                if !((*T).clq_gen).is_null() {
                    if (*(*T).curr).level == 0 as i32
                        && (*(*T).curr).changed < 500 as i32
                        || (*(*T).curr).level > 0 as i32
                            && (*(*T).curr).changed < 50 as i32
                    {
                        clq_gen(T, (*T).clq_gen);
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn remove_cuts(mut T: *mut glp_tree) {
    let mut i: i32 = 0;
    let mut cnt: i32 = 0 as i32;
    let mut num: *mut i32 = 0 as *mut i32;
    (!((*T).curr).is_null()
        || {
            glp_assert_(
                b"T->curr != NULL\0" as *const u8 as *const i8,
                b"draft/glpios03.c\0" as *const u8 as *const i8,
                806 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = (*T).orig_m + 1 as i32;
    while i <= (*(*T).mip).m {
        if (**((*(*T).mip).row).offset(i as isize)).origin as i32 == 2 as i32
            && (**((*(*T).mip).row).offset(i as isize)).level == (*(*T).curr).level
            && (**((*(*T).mip).row).offset(i as isize)).stat == 1 as i32
        {
            if num.is_null() {
                num = glp_alloc(
                    1 as i32 + (*(*T).mip).m,
                    ::core::mem::size_of::<i32>() as u64 as i32,
                ) as *mut i32;
            }
            cnt += 1;
            *num.offset(cnt as isize) = i;
        }
        i += 1;
        i;
    }
    if cnt > 0 as i32 {
        glp_del_rows((*T).mip, cnt, num as *const i32);
        glp_free(num as *mut libc::c_void);
        (glp_factorize((*T).mip) == 0 as i32
            || {
                glp_assert_(
                    b"glp_factorize(T->mip) == 0\0" as *const u8 as *const i8,
                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                    822 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
}
unsafe extern "C" fn display_cut_info(mut T: *mut glp_tree) {
    let mut mip: *mut glp_prob = (*T).mip;
    let mut i: i32 = 0;
    let mut gmi: i32 = 0 as i32;
    let mut mir: i32 = 0 as i32;
    let mut cov: i32 = 0 as i32;
    let mut clq: i32 = 0 as i32;
    let mut app: i32 = 0 as i32;
    i = (*mip).m;
    while i > 0 as i32 {
        let mut row: *mut GLPROW = 0 as *mut GLPROW;
        row = *((*mip).row).offset(i as isize);
        if (*row).origin as i32 == 2 as i32 {
            if (*row).klass as i32 == 1 as i32 {
                gmi += 1;
                gmi;
            } else if (*row).klass as i32 == 2 as i32 {
                mir += 1;
                mir;
            } else if (*row).klass as i32 == 3 as i32 {
                cov += 1;
                cov;
            } else if (*row).klass as i32 == 4 as i32 {
                clq += 1;
                clq;
            } else {
                app += 1;
                app;
            }
        }
        i -= 1;
        i;
    }
    (!((*T).curr).is_null()
        || {
            glp_assert_(
                b"T->curr != NULL\0" as *const u8 as *const i8,
                b"draft/glpios03.c\0" as *const u8 as *const i8,
                849 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if gmi + mir + cov + clq + app > 0 as i32 {
        glp_printf(b"Cuts on level %d:\0" as *const u8 as *const i8, (*(*T).curr).level);
        if gmi > 0 as i32 {
            glp_printf(b" gmi = %d;\0" as *const u8 as *const i8, gmi);
        }
        if mir > 0 as i32 {
            glp_printf(b" mir = %d;\0" as *const u8 as *const i8, mir);
        }
        if cov > 0 as i32 {
            glp_printf(b" cov = %d;\0" as *const u8 as *const i8, cov);
        }
        if clq > 0 as i32 {
            glp_printf(b" clq = %d;\0" as *const u8 as *const i8, clq);
        }
        if app > 0 as i32 {
            glp_printf(b" app = %d;\0" as *const u8 as *const i8, app);
        }
        glp_printf(b"\n\0" as *const u8 as *const i8);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_driver(mut T: *mut glp_tree) -> i32 {
    let mut current_block: u64;
    let mut p: i32 = 0;
    let mut curr_p: i32 = 0;
    let mut p_stat: i32 = 0;
    let mut d_stat: i32 = 0;
    let mut ret: i32 = 0;
    let mut pred_p: i32 = 0 as i32;
    let mut bad_cut: i32 = 0;
    let mut old_obj: libc::c_double = 0.;
    let mut ttt: libc::c_double = (*T).tm_beg;
    let mut root_done: i32 = 0 as i32;
    if (*((*T).parm as *mut glp_iocp)).flip != 0 {
        if (*(*T).parm).msg_lev >= 3 as i32 {
            glp_printf(
                b"Long-step dual simplex will be used\n\0" as *const u8 as *const i8,
            );
        }
    }
    '_loop: loop {
        (((*T).curr).is_null()
            || {
                glp_assert_(
                    b"T->curr == NULL\0" as *const u8 as *const i8,
                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                    930 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if ((*T).head).is_null() {
            if (*(*T).parm).msg_lev >= 4 as i32 {
                glp_printf(b"Active list is empty!\n\0" as *const u8 as *const i8);
            }
            (_glp_dmp_in_use((*T).pool) == 0 as i32 as u64
                || {
                    glp_assert_(
                        b"dmp_in_use(T->pool) == 0\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        938 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            ret = 0 as i32;
            break;
        } else {
            ((*T).next_p == 0 as i32
                || {
                    glp_assert_(
                        b"T->next_p == 0\0" as *const u8 as *const i8,
                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                        944 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if ((*(*T).parm).cb_func).is_some() {
                ((*T).reason == 0 as i32
                    || {
                        glp_assert_(
                            b"T->reason == 0\0" as *const u8 as *const i8,
                            b"draft/glpios03.c\0" as *const u8 as *const i8,
                            947 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*T).reason = 0x6 as i32;
                ((*(*T).parm).cb_func)
                    .expect("non-null function pointer")(T, (*(*T).parm).cb_info);
                (*T).reason = 0 as i32;
                if (*T).stop != 0 {
                    ret = 0xd as i32;
                    break;
                }
            }
            if !((*T).next_p != 0 as i32) {
                if (*T).a_cnt == 1 as i32 {
                    (((*(*T).head).next).is_null()
                        || {
                            glp_assert_(
                                b"T->head->next == NULL\0" as *const u8 as *const i8,
                                b"draft/glpios03.c\0" as *const u8 as *const i8,
                                962 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    (*T).next_p = (*(*T).head).p;
                } else if (*T).child != 0 as i32 {
                    (*T).next_p = (*T).child;
                } else {
                    (*T).next_p = _glp_ios_choose_node(T);
                }
            }
            _glp_ios_revive_node(T, (*T).next_p);
            (*T).child = 0 as i32;
            (*T).next_p = (*T).child;
            if !((*(*T).curr).up).is_null() && (*(*(*T).curr).up).p != pred_p {
                pred_p = 0 as i32;
            }
            p = (*(*T).curr).p;
            if (*(*T).parm).msg_lev >= 4 as i32 {
                glp_printf(
                    b"------------------------------------------------------------------------\n\0"
                        as *const u8 as *const i8,
                );
                glp_printf(
                    b"Processing node %d at level %d\n\0" as *const u8 as *const i8,
                    p,
                    (*(*T).curr).level,
                );
            }
            if p == 1 as i32 {
                if (*(*T).parm).sr_heur == 0 as i32 {
                    if (*(*T).parm).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"Simple rounding heuristic disabled\n\0" as *const u8
                                as *const i8,
                        );
                    }
                }
            }
            if p == 1 as i32 {
                if (*(*T).parm).gmi_cuts == 1 as i32 {
                    if (*(*T).parm).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"Gomory's cuts enabled\n\0" as *const u8 as *const i8,
                        );
                    }
                }
                if (*(*T).parm).mir_cuts == 1 as i32 {
                    if (*(*T).parm).msg_lev >= 3 as i32 {
                        glp_printf(b"MIR cuts enabled\n\0" as *const u8 as *const i8);
                    }
                    (((*T).mir_gen).is_null()
                        || {
                            glp_assert_(
                                b"T->mir_gen == NULL\0" as *const u8 as *const i8,
                                b"draft/glpios03.c\0" as *const u8 as *const i8,
                                1009 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    (*T).mir_gen = glp_mir_init((*T).mip);
                }
                if (*(*T).parm).cov_cuts == 1 as i32 {
                    if (*(*T).parm).msg_lev >= 3 as i32 {
                        glp_printf(b"Cover cuts enabled\n\0" as *const u8 as *const i8);
                    }
                    (((*T).cov_gen).is_null()
                        || {
                            glp_assert_(
                                b"T->cov_gen == NULL\0" as *const u8 as *const i8,
                                b"draft/glpios03.c\0" as *const u8 as *const i8,
                                1020 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    (*T).cov_gen = glp_cov_init((*T).mip);
                }
                if (*(*T).parm).clq_cuts == 1 as i32 {
                    (((*T).clq_gen).is_null()
                        || {
                            glp_assert_(
                                b"T->clq_gen == NULL\0" as *const u8 as *const i8,
                                b"draft/glpios03.c\0" as *const u8 as *const i8,
                                1025 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if (*(*T).parm).msg_lev >= 3 as i32 {
                        glp_printf(b"Clique cuts enabled\n\0" as *const u8 as *const i8);
                    }
                    (*T).clq_gen = glp_cfg_init((*T).mip);
                }
            }
            bad_cut = 0 as i32;
            loop {
                if (*(*T).parm).msg_lev >= 4 as i32
                    || (*(*T).parm).msg_lev >= 2 as i32
                        && ((*(*T).parm).out_frq - 1 as i32) as libc::c_double
                            <= 1000.0f64 * glp_difftime(glp_time(), (*T).tm_lag)
                {
                    show_progress(T, 0 as i32);
                }
                if (*(*T).parm).msg_lev >= 3 as i32
                    && glp_difftime(glp_time(), ttt) >= 60.0f64
                {
                    let mut total: size_t = 0;
                    glp_mem_usage(
                        0 as *mut i32,
                        0 as *mut i32,
                        &mut total,
                        0 as *mut size_t,
                    );
                    glp_printf(
                        b"Time used: %.1f secs.  Memory used: %.1f Mb.\n\0" as *const u8
                            as *const i8,
                        glp_difftime(glp_time(), (*T).tm_beg),
                        total as libc::c_double / 1048576.0f64,
                    );
                    ttt = glp_time();
                }
                if (*(*T).parm).mip_gap > 0.0f64
                    && _glp_ios_relative_gap(T) <= (*(*T).parm).mip_gap
                {
                    if (*(*T).parm).msg_lev >= 4 as i32 {
                        glp_printf(
                            b"Relative gap tolerance reached; search terminated \n\0"
                                as *const u8 as *const i8,
                        );
                    }
                    ret = 0xe as i32;
                    break '_loop;
                } else if (*(*T).parm).tm_lim < 2147483647 as i32
                    && ((*(*T).parm).tm_lim - 1 as i32) as libc::c_double
                        <= 1000.0f64 * glp_difftime(glp_time(), (*T).tm_beg)
                {
                    if (*(*T).parm).msg_lev >= 4 as i32 {
                        glp_printf(
                            b"Time limit exhausted; search terminated\n\0" as *const u8
                                as *const i8,
                        );
                    }
                    ret = 0x9 as i32;
                    break '_loop;
                } else {
                    if ((*(*T).parm).cb_func).is_some() {
                        ((*T).reason == 0 as i32
                            || {
                                glp_assert_(
                                    b"T->reason == 0\0" as *const u8 as *const i8,
                                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                                    1084 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        (*T).reason = 0x7 as i32;
                        ((*(*T).parm).cb_func)
                            .expect(
                                "non-null function pointer",
                            )(T, (*(*T).parm).cb_info);
                        (*T).reason = 0 as i32;
                        if (*T).stop != 0 {
                            ret = 0xd as i32;
                            break '_loop;
                        }
                    }
                    if !((*(*T).parm).pp_tech == 0 as i32) {
                        if (*(*T).parm).pp_tech == 1 as i32 {
                            if root_done == 0 {
                                if _glp_ios_preprocess_node(T, 100 as i32) != 0 {
                                    current_block = 9037502348538308815;
                                    break;
                                }
                            }
                        } else if (*(*T).parm).pp_tech == 2 as i32 {
                            if _glp_ios_preprocess_node(
                                T,
                                if root_done == 0 { 100 as i32 } else { 10 as i32 },
                            ) != 0
                            {
                                current_block = 9037502348538308815;
                                break;
                            }
                        } else {
                            (T != T
                                || {
                                    glp_assert_(
                                        b"T != T\0" as *const u8 as *const i8,
                                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                                        1115 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                        }
                    }
                    if is_branch_hopeful(T, p) == 0 {
                        glp_printf(
                            b"*** not tested yet ***\n\0" as *const u8 as *const i8,
                        );
                        current_block = 9037502348538308815;
                        break;
                    } else {
                        if (*(*T).parm).msg_lev >= 4 as i32 {
                            glp_printf(
                                b"Solving LP relaxation...\n\0" as *const u8 as *const i8,
                            );
                        }
                        ret = _glp_ios_solve_node(T);
                        if ret == 0x9 as i32 {
                            break '_loop;
                        }
                        if !(ret == 0 as i32 || ret == 0x6 as i32 || ret == 0x7 as i32) {
                            if (*(*T).parm).msg_lev >= 1 as i32 {
                                glp_printf(
                                    b"ios_driver: unable to solve current LP relaxation; glp_simplex returned %d\n\0"
                                        as *const u8 as *const i8,
                                    ret,
                                );
                            }
                            ret = 0x5 as i32;
                            break '_loop;
                        } else {
                            p_stat = (*(*T).mip).pbs_stat;
                            d_stat = (*(*T).mip).dbs_stat;
                            if p_stat == 2 as i32 && d_stat == 2 as i32 {
                                if (*(*T).parm).msg_lev >= 4 as i32 {
                                    glp_printf(
                                        b"Found optimal solution to LP relaxation\n\0" as *const u8
                                            as *const i8,
                                    );
                                }
                            } else if d_stat == 4 as i32 {
                                if (*(*T).parm).msg_lev >= 1 as i32 {
                                    glp_printf(
                                        b"ios_driver: current LP relaxation has no dual feasible solution\n\0"
                                            as *const u8 as *const i8,
                                    );
                                }
                                ret = 0x5 as i32;
                                break '_loop;
                            } else if p_stat == 3 as i32 && d_stat == 2 as i32 {
                                ((*(*T).mip).mip_stat == 2 as i32
                                    || {
                                        glp_assert_(
                                            b"T->mip->mip_stat == GLP_FEAS\0" as *const u8 as *const i8,
                                            b"draft/glpios03.c\0" as *const u8 as *const i8,
                                            1155 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                                if (*(*T).parm).msg_lev >= 4 as i32 {
                                    glp_printf(
                                        b"LP relaxation has no solution better than incumbent objective value\n\0"
                                            as *const u8 as *const i8,
                                    );
                                }
                                current_block = 9037502348538308815;
                                break;
                            } else if p_stat == 4 as i32 {
                                if (*(*T).parm).msg_lev >= 4 as i32 {
                                    glp_printf(
                                        b"LP relaxation has no feasible solution\n\0" as *const u8
                                            as *const i8,
                                    );
                                }
                                current_block = 9037502348538308815;
                                break;
                            } else {
                                ((*T).mip != (*T).mip
                                    || {
                                        glp_assert_(
                                            b"T->mip != T->mip\0" as *const u8 as *const i8,
                                            b"draft/glpios03.c\0" as *const u8 as *const i8,
                                            1171 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                            }
                            (p_stat == 2 as i32 && d_stat == 2 as i32
                                || {
                                    glp_assert_(
                                        b"p_stat == GLP_FEAS && d_stat == GLP_FEAS\0" as *const u8
                                            as *const i8,
                                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                                        1175 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            (!((*T).curr).is_null()
                                || {
                                    glp_assert_(
                                        b"T->curr != NULL\0" as *const u8 as *const i8,
                                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                                        1176 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            (*(*T).curr).lp_obj = (*(*T).mip).obj_val;
                            let mut bound: libc::c_double = (*(*T).mip).obj_val;
                            bound = _glp_ios_round_bound(T, bound);
                            if (*(*T).mip).dir == 1 as i32 {
                                if (*(*T).curr).bound < bound {
                                    (*(*T).curr).bound = bound;
                                }
                            } else if (*(*T).mip).dir == 2 as i32 {
                                if (*(*T).curr).bound > bound {
                                    (*(*T).curr).bound = bound;
                                }
                            } else {
                                ((*T).mip != (*T).mip
                                    || {
                                        glp_assert_(
                                            b"T->mip != T->mip\0" as *const u8 as *const i8,
                                            b"draft/glpios03.c\0" as *const u8 as *const i8,
                                            1193 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                            }
                            if (*(*T).parm).msg_lev >= 4 as i32 {
                                glp_printf(
                                    b"Local bound is %.9e\n\0" as *const u8 as *const i8,
                                    bound,
                                );
                            }
                            if is_branch_hopeful(T, p) == 0 {
                                if (*(*T).parm).msg_lev >= 4 as i32 {
                                    glp_printf(
                                        b"Current branch is hopeless and can be pruned\n\0"
                                            as *const u8 as *const i8,
                                    );
                                }
                                current_block = 9037502348538308815;
                                break;
                            } else {
                                ((*T).reopt == 0 as i32
                                    || {
                                        glp_assert_(
                                            b"T->reopt == 0\0" as *const u8 as *const i8,
                                            b"draft/glpios03.c\0" as *const u8 as *const i8,
                                            1207 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                                ((*T).reinv == 0 as i32
                                    || {
                                        glp_assert_(
                                            b"T->reinv == 0\0" as *const u8 as *const i8,
                                            b"draft/glpios03.c\0" as *const u8 as *const i8,
                                            1208 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                                if ((*(*T).parm).cb_func).is_some() {
                                    ((*T).reason == 0 as i32
                                        || {
                                            glp_assert_(
                                                b"T->reason == 0\0" as *const u8 as *const i8,
                                                b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                1210 as i32,
                                            );
                                            1 as i32 != 0
                                        }) as i32;
                                    (*T).reason = 0x1 as i32;
                                    ((*(*T).parm).cb_func)
                                        .expect(
                                            "non-null function pointer",
                                        )(T, (*(*T).parm).cb_info);
                                    (*T).reason = 0 as i32;
                                    if (*T).stop != 0 {
                                        ret = 0xd as i32;
                                        break '_loop;
                                    } else if (*T).reopt != 0 {
                                        (*T).reinv = 0 as i32;
                                        (*T).reopt = (*T).reinv;
                                        continue;
                                    } else if (*T).reinv != 0 {
                                        (*T).reinv = 0 as i32;
                                        (glp_factorize((*T).mip) == 0 as i32
                                            || {
                                                glp_assert_(
                                                    b"glp_factorize(T->mip) == 0\0" as *const u8 as *const i8,
                                                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                    1227 as i32,
                                                );
                                                1 as i32 != 0
                                            }) as i32;
                                    }
                                }
                                check_integrality(T);
                                if (*(*T).curr).ii_cnt == 0 as i32 {
                                    if (*(*T).parm).msg_lev >= 4 as i32 {
                                        glp_printf(
                                            b"New integer feasible solution found\n\0" as *const u8
                                                as *const i8,
                                        );
                                    }
                                    if (*(*T).parm).msg_lev >= 3 as i32 {
                                        display_cut_info(T);
                                    }
                                    record_solution(T);
                                    if (*(*T).parm).msg_lev >= 2 as i32 {
                                        show_progress(T, 1 as i32);
                                    }
                                    _glp_ios_process_sol(T);
                                    if ((*(*T).parm).cb_func).is_some() {
                                        current_block = 4871270227279186910;
                                        break;
                                    } else {
                                        current_block = 8880031775101799352;
                                        break;
                                    }
                                } else {
                                    if (*(*T).mip).mip_stat == 2 as i32 {
                                        fix_by_red_cost(T);
                                    }
                                    if ((*(*T).parm).cb_func).is_some() {
                                        ((*T).reason == 0 as i32
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const i8,
                                                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                    1269 as i32,
                                                );
                                                1 as i32 != 0
                                            }) as i32;
                                        (*T).reason = 0x3 as i32;
                                        ((*(*T).parm).cb_func)
                                            .expect(
                                                "non-null function pointer",
                                            )(T, (*(*T).parm).cb_info);
                                        (*T).reason = 0 as i32;
                                        if (*T).stop != 0 {
                                            ret = 0xd as i32;
                                            break '_loop;
                                        } else if is_branch_hopeful(T, p) == 0 {
                                            if (*(*T).parm).msg_lev >= 4 as i32 {
                                                glp_printf(
                                                    b"Current branch became hopeless and can be pruned\n\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                            current_block = 9037502348538308815;
                                            break;
                                        }
                                    }
                                    if (*(*T).parm).fp_heur != 0 && root_done == 0 {
                                        ((*T).reason == 0 as i32
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const i8,
                                                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                    1291 as i32,
                                                );
                                                1 as i32 != 0
                                            }) as i32;
                                        (*T).reason = 0x3 as i32;
                                        _glp_ios_feas_pump(T);
                                        (*T).reason = 0 as i32;
                                        if is_branch_hopeful(T, p) == 0 {
                                            if (*(*T).parm).msg_lev >= 4 as i32 {
                                                glp_printf(
                                                    b"Current branch became hopeless and can be pruned\n\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                            current_block = 9037502348538308815;
                                            break;
                                        }
                                    }
                                    if (*(*T).parm).ps_heur != 0 && root_done == 0 {
                                        ((*T).reason == 0 as i32
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const i8,
                                                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                    1310 as i32,
                                                );
                                                1 as i32 != 0
                                            }) as i32;
                                        (*T).reason = 0x3 as i32;
                                        _glp_ios_proxy_heur(T);
                                        (*T).reason = 0 as i32;
                                        if is_branch_hopeful(T, p) == 0 {
                                            if (*(*T).parm).msg_lev >= 4 as i32 {
                                                glp_printf(
                                                    b"Current branch became hopeless and can be pruned\n\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                            current_block = 9037502348538308815;
                                            break;
                                        }
                                    }
                                    if (*(*T).parm).sr_heur != 0 {
                                        ((*T).reason == 0 as i32
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const i8,
                                                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                    1326 as i32,
                                                );
                                                1 as i32 != 0
                                            }) as i32;
                                        (*T).reason = 0x3 as i32;
                                        round_heur(T);
                                        (*T).reason = 0 as i32;
                                        if is_branch_hopeful(T, p) == 0 {
                                            if (*(*T).parm).msg_lev >= 4 as i32 {
                                                glp_printf(
                                                    b"Current branch became hopeless and can be pruned\n\0"
                                                        as *const u8 as *const i8,
                                                );
                                            }
                                            current_block = 9037502348538308815;
                                            break;
                                        }
                                    }
                                    (!((*T).local).is_null()
                                        || {
                                            glp_assert_(
                                                b"T->local != NULL\0" as *const u8 as *const i8,
                                                b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                1340 as i32,
                                            );
                                            1 as i32 != 0
                                        }) as i32;
                                    ((*(*T).local).m == 0 as i32
                                        || {
                                            glp_assert_(
                                                b"T->local->m == 0\0" as *const u8 as *const i8,
                                                b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                1342 as i32,
                                            );
                                            1 as i32 != 0
                                        }) as i32;
                                    if ((*(*T).parm).cb_func).is_some() {
                                        ((*T).reason == 0 as i32
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const i8,
                                                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                    1350 as i32,
                                                );
                                                1 as i32 != 0
                                            }) as i32;
                                        (*T).reason = 0x4 as i32;
                                        ((*(*T).parm).cb_func)
                                            .expect(
                                                "non-null function pointer",
                                            )(T, (*(*T).parm).cb_info);
                                        (*T).reason = 0 as i32;
                                        if (*T).stop != 0 {
                                            ret = 0xd as i32;
                                            break '_loop;
                                        }
                                    }
                                    if (*(*T).curr).changed > 0 as i32 {
                                        let mut degrad: libc::c_double = fabs(
                                            (*(*T).curr).lp_obj - old_obj,
                                        );
                                        if degrad < 1e-4f64 * (1.0f64 + fabs(old_obj)) {
                                            bad_cut += 1;
                                            bad_cut;
                                        } else {
                                            bad_cut = 0 as i32;
                                        }
                                    }
                                    old_obj = (*(*T).curr).lp_obj;
                                    if bad_cut == 0 as i32
                                        || root_done == 0 && bad_cut <= 3 as i32
                                    {
                                        if root_done == 0 || pred_p == 0 as i32 {
                                            ((*T).reason == 0 as i32
                                                || {
                                                    glp_assert_(
                                                        b"T->reason == 0\0" as *const u8 as *const i8,
                                                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                        1384 as i32,
                                                    );
                                                    1 as i32 != 0
                                                }) as i32;
                                            (*T).reason = 0x4 as i32;
                                            generate_cuts(T);
                                            (*T).reason = 0 as i32;
                                        }
                                    }
                                    if (*(*T).local).m > 0 as i32 {
                                        ((*T).reason == 0 as i32
                                            || {
                                                glp_assert_(
                                                    b"T->reason == 0\0" as *const u8 as *const i8,
                                                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                    1396 as i32,
                                                );
                                                1 as i32 != 0
                                            }) as i32;
                                        (*T).reason = 0x4 as i32;
                                        _glp_ios_process_cuts(T);
                                        (*T).reason = 0 as i32;
                                    }
                                    _glp_ios_clear_pool(T, (*T).local);
                                    if (*T).reopt != 0 {
                                        (*T).reopt = 0 as i32;
                                        (*(*T).curr).changed += 1;
                                        (*(*T).curr).changed;
                                    } else {
                                        remove_cuts(T);
                                        if (*(*T).parm).msg_lev >= 3 as i32 && root_done == 0 {
                                            display_cut_info(T);
                                        }
                                        if root_done == 0 {
                                            root_done = 1 as i32;
                                        }
                                        if !((*T).pcost).is_null() {
                                            _glp_ios_pcost_update(T);
                                        }
                                        ((*T).br_var == 0 as i32
                                            || {
                                                glp_assert_(
                                                    b"T->br_var == 0\0" as *const u8 as *const i8,
                                                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                    1424 as i32,
                                                );
                                                1 as i32 != 0
                                            }) as i32;
                                        ((*T).br_sel == 0 as i32
                                            || {
                                                glp_assert_(
                                                    b"T->br_sel == 0\0" as *const u8 as *const i8,
                                                    b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                    1425 as i32,
                                                );
                                                1 as i32 != 0
                                            }) as i32;
                                        if ((*(*T).parm).cb_func).is_some() {
                                            ((*T).reason == 0 as i32
                                                || {
                                                    glp_assert_(
                                                        b"T->reason == 0\0" as *const u8 as *const i8,
                                                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                        1428 as i32,
                                                    );
                                                    1 as i32 != 0
                                                }) as i32;
                                            ((*T).br_var == 0 as i32
                                                || {
                                                    glp_assert_(
                                                        b"T->br_var == 0\0" as *const u8 as *const i8,
                                                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                        1429 as i32,
                                                    );
                                                    1 as i32 != 0
                                                }) as i32;
                                            ((*T).br_sel == 0 as i32
                                                || {
                                                    glp_assert_(
                                                        b"T->br_sel == 0\0" as *const u8 as *const i8,
                                                        b"draft/glpios03.c\0" as *const u8 as *const i8,
                                                        1430 as i32,
                                                    );
                                                    1 as i32 != 0
                                                }) as i32;
                                            (*T).reason = 0x5 as i32;
                                            ((*(*T).parm).cb_func)
                                                .expect(
                                                    "non-null function pointer",
                                                )(T, (*(*T).parm).cb_info);
                                            (*T).reason = 0 as i32;
                                            if (*T).stop != 0 {
                                                ret = 0xd as i32;
                                                break '_loop;
                                            }
                                        }
                                        if (*T).br_var == 0 as i32 {
                                            (*T).br_var = _glp_ios_choose_var(T, &mut (*T).br_sel);
                                        }
                                        curr_p = (*(*T).curr).p;
                                        ret = branch_on(T, (*T).br_var, (*T).br_sel);
                                        (*T).br_sel = 0 as i32;
                                        (*T).br_var = (*T).br_sel;
                                        if ret == 0 as i32 {
                                            pred_p = curr_p;
                                            continue '_loop;
                                        } else if ret == 1 as i32 {
                                            (*(*T).curr).changed = 0 as i32;
                                            (*(*T).curr).solved = (*(*T).curr).changed;
                                        } else if ret == 2 as i32 {
                                            current_block = 9037502348538308815;
                                            break;
                                        } else {
                                            current_block = 3818209998506676277;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                4871270227279186910 => {
                    ((*T).reason == 0 as i32
                        || {
                            glp_assert_(
                                b"T->reason == 0\0" as *const u8 as *const i8,
                                b"draft/glpios03.c\0" as *const u8 as *const i8,
                                1247 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    (*T).reason = 0x2 as i32;
                    ((*(*T).parm).cb_func)
                        .expect("non-null function pointer")(T, (*(*T).parm).cb_info);
                    (*T).reason = 0 as i32;
                    if (*T).stop != 0 {
                        ret = 0xd as i32;
                        break;
                    } else {
                        current_block = 8880031775101799352;
                    }
                }
                3818209998506676277 => {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const i8,
                                b"draft/glpios03.c\0" as *const u8 as *const i8,
                                1469 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    current_block = 9037502348538308815;
                }
                _ => {}
            }
            match current_block {
                8880031775101799352 => {}
                _ => {}
            }
            if (*(*T).parm).msg_lev >= 4 as i32 {
                glp_printf(b"Node %d fathomed\n\0" as *const u8 as *const i8, p);
            }
            _glp_ios_freeze_node(T);
            _glp_ios_delete_node(T, p);
            if (*(*T).mip).mip_stat == 2 as i32 {
                cleanup_the_tree(T);
            }
            pred_p = 0 as i32;
        }
    }
    if (*(*T).parm).msg_lev >= 2 as i32 {
        show_progress(T, 0 as i32);
    }
    if !((*T).mir_gen).is_null() {
        glp_mir_free((*T).mir_gen);
        (*T).mir_gen = 0 as *mut glp_mir;
    }
    if !((*T).cov_gen).is_null() {
        glp_cov_free((*T).cov_gen);
        (*T).cov_gen = 0 as *mut glp_cov;
    }
    if !((*T).clq_gen).is_null() {
        glp_cfg_free((*T).clq_gen);
        (*T).clq_gen = 0 as *mut glp_cfg;
    }
    return ret;
}