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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
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
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_choose_node(mut T: *mut glp_tree) -> i32 {
    let mut p: i32 = 0;
    if (*(*T).parm).bt_tech == 1 as i32 {
        (!((*T).tail).is_null()
            || {
                glp_assert_(
                    b"T->tail != NULL\0" as *const u8 as *const i8,
                    b"draft/glpios12.c\0" as *const u8 as *const i8,
                    54 as i32,
                );
                1 as i32 != 0
            }) as i32;
        p = (*(*T).tail).p;
    } else if (*(*T).parm).bt_tech == 2 as i32 {
        (!((*T).head).is_null()
            || {
                glp_assert_(
                    b"T->head != NULL\0" as *const u8 as *const i8,
                    b"draft/glpios12.c\0" as *const u8 as *const i8,
                    59 as i32,
                );
                1 as i32 != 0
            }) as i32;
        p = (*(*T).head).p;
    } else if (*(*T).parm).bt_tech == 3 as i32 {
        p = best_node(T);
    } else if (*(*T).parm).bt_tech == 4 as i32 {
        if (*(*T).mip).mip_stat == 1 as i32 {
            p = most_feas(T);
        } else {
            p = best_proj(T);
        }
    } else {
        (T != T
            || {
                glp_assert_(
                    b"T != T\0" as *const u8 as *const i8,
                    b"draft/glpios12.c\0" as *const u8 as *const i8,
                    77 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    return p;
}
unsafe extern "C" fn most_feas(mut T: *mut glp_tree) -> i32 {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut p: i32 = 0;
    let mut best: libc::c_double = 0.;
    p = 0 as i32;
    best = 1.7976931348623157e+308f64;
    node = (*T).head;
    while !node.is_null() {
        (!((*node).up).is_null()
            || {
                glp_assert_(
                    b"node->up != NULL\0" as *const u8 as *const i8,
                    b"draft/glpios12.c\0" as *const u8 as *const i8,
                    89 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if best > (*(*node).up).ii_sum {
            p = (*node).p;
            best = (*(*node).up).ii_sum;
        }
        node = (*node).next;
    }
    return p;
}
unsafe extern "C" fn best_proj(mut T: *mut glp_tree) -> i32 {
    let mut root: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut p: i32 = 0;
    let mut best: libc::c_double = 0.;
    let mut deg: libc::c_double = 0.;
    let mut obj: libc::c_double = 0.;
    ((*(*T).mip).mip_stat == 2 as i32
        || {
            glp_assert_(
                b"T->mip->mip_stat == GLP_FEAS\0" as *const u8 as *const i8,
                b"draft/glpios12.c\0" as *const u8 as *const i8,
                102 as i32,
            );
            1 as i32 != 0
        }) as i32;
    root = (*((*T).slot).offset(1 as i32 as isize)).node;
    (!root.is_null()
        || {
            glp_assert_(
                b"root != NULL\0" as *const u8 as *const i8,
                b"draft/glpios12.c\0" as *const u8 as *const i8,
                105 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*root).ii_sum > 0.0f64
        || {
            glp_assert_(
                b"root->ii_sum > 0.0\0" as *const u8 as *const i8,
                b"draft/glpios12.c\0" as *const u8 as *const i8,
                108 as i32,
            );
            1 as i32 != 0
        }) as i32;
    deg = ((*(*T).mip).mip_obj - (*root).bound) / (*root).ii_sum;
    p = 0 as i32;
    best = 1.7976931348623157e+308f64;
    node = (*T).head;
    while !node.is_null() {
        (!((*node).up).is_null()
            || {
                glp_assert_(
                    b"node->up != NULL\0" as *const u8 as *const i8,
                    b"draft/glpios12.c\0" as *const u8 as *const i8,
                    114 as i32,
                );
                1 as i32 != 0
            }) as i32;
        obj = (*(*node).up).bound + deg * (*(*node).up).ii_sum;
        if (*(*T).mip).dir == 2 as i32 {
            obj = -obj;
        }
        if best > obj {
            p = (*node).p;
            best = obj;
        }
        node = (*node).next;
    }
    return p;
}
unsafe extern "C" fn best_node(mut T: *mut glp_tree) -> i32 {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut best: *mut IOSNPD = 0 as *mut IOSNPD;
    let mut bound: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    match (*(*T).mip).dir {
        1 => {
            bound = 1.7976931348623157e+308f64;
            node = (*T).head;
            while !node.is_null() {
                if bound > (*node).bound {
                    bound = (*node).bound;
                }
                node = (*node).next;
            }
            (bound != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"bound != +DBL_MAX\0" as *const u8 as *const i8,
                        b"draft/glpios12.c\0" as *const u8 as *const i8,
                        135 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            eps = 1e-10f64 * (1.0f64 + fabs(bound));
            node = (*T).head;
            while !node.is_null() {
                if (*node).bound <= bound + eps {
                    (!((*node).up).is_null()
                        || {
                            glp_assert_(
                                b"node->up != NULL\0" as *const u8 as *const i8,
                                b"draft/glpios12.c\0" as *const u8 as *const i8,
                                139 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if best.is_null() || (*(*best).up).ii_sum > (*(*node).up).ii_sum {
                        best = node;
                    }
                }
                node = (*node).next;
            }
        }
        2 => {
            bound = -1.7976931348623157e+308f64;
            node = (*T).head;
            while !node.is_null() {
                if bound < (*node).bound {
                    bound = (*node).bound;
                }
                node = (*node).next;
            }
            (bound != -1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"bound != -DBL_MAX\0" as *const u8 as *const i8,
                        b"draft/glpios12.c\0" as *const u8 as *const i8,
                        153 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            eps = 1e-10f64 * (1.0f64 + fabs(bound));
            node = (*T).head;
            while !node.is_null() {
                if (*node).bound >= bound - eps {
                    (!((*node).up).is_null()
                        || {
                            glp_assert_(
                                b"node->up != NULL\0" as *const u8 as *const i8,
                                b"draft/glpios12.c\0" as *const u8 as *const i8,
                                157 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if best.is_null() || (*(*best).up).ii_sum > (*(*node).up).ii_sum {
                        best = node;
                    }
                }
                node = (*node).next;
            }
        }
        _ => {
            (T != T
                || {
                    glp_assert_(
                        b"T != T\0" as *const u8 as *const i8,
                        b"draft/glpios12.c\0" as *const u8 as *const i8,
                        168 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    (!best.is_null()
        || {
            glp_assert_(
                b"best != NULL\0" as *const u8 as *const i8,
                b"draft/glpios12.c\0" as *const u8 as *const i8,
                170 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return (*best).p;
}