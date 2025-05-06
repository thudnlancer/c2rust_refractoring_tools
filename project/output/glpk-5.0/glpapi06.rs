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
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_get_env_ptr() -> *mut ENV;
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_scale_prob(P: *mut glp_prob, flags: i32);
    fn glp_adv_basis(P: *mut glp_prob, flags: i32);
    fn glp_factorize(P: *mut glp_prob) -> i32;
    fn glp_bf_exists(P: *mut glp_prob) -> i32;
    fn glp_set_bfcp(P: *mut glp_prob, parm: *const glp_bfcp);
    fn glp_get_bfcp(P: *mut glp_prob, parm: *mut glp_bfcp);
    fn glp_version() -> *const i8;
    fn _glp_npp_create_wksp() -> *mut NPP;
    fn _glp_npp_load_prob(
        npp: *mut NPP,
        orig: *mut glp_prob,
        names: i32,
        sol: i32,
        scaling: i32,
    );
    fn _glp_npp_build_prob(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_postprocess(npp: *mut NPP, prob: *mut glp_prob);
    fn _glp_npp_unload_sol(npp: *mut NPP, orig: *mut glp_prob);
    fn _glp_npp_delete_wksp(npp: *mut NPP);
    fn _glp_npp_simplex(npp: *mut NPP, parm: *const glp_smcp) -> i32;
    fn _glp_spx_primal(P: *mut glp_prob, parm: *const glp_smcp) -> i32;
    fn _glp_spy_dual(P: *mut glp_prob, parm: *const glp_smcp) -> i32;
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENV {
    pub self_0: *mut ENV,
    pub term_buf: *mut i8,
    pub term_out: i32,
    pub term_hook: Option<unsafe extern "C" fn(*mut libc::c_void, *const i8) -> i32>,
    pub term_info: *mut libc::c_void,
    pub tee_file: *mut FILE,
    pub err_st: i32,
    pub err_file: *const i8,
    pub err_line: i32,
    pub err_hook: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_info: *mut libc::c_void,
    pub err_buf: *mut i8,
    pub mem_limit: size_t,
    pub mem_ptr: *mut MBD,
    pub mem_count: i32,
    pub mem_cpeak: i32,
    pub mem_total: size_t,
    pub mem_tpeak: size_t,
    pub gmp_pool: *mut libc::c_void,
    pub gmp_size: i32,
    pub gmp_work: *mut libc::c_ushort,
    pub h_odbc: *mut libc::c_void,
    pub h_mysql: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBD {
    pub size: size_t,
    pub self_0: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_bfcp {
    pub msg_lev: i32,
    pub type_0: i32,
    pub lu_size: i32,
    pub piv_tol: libc::c_double,
    pub piv_lim: i32,
    pub suhl: i32,
    pub eps_tol: libc::c_double,
    pub max_gro: libc::c_double,
    pub nfs_max: i32,
    pub upd_tol: libc::c_double,
    pub nrs_max: i32,
    pub rs_size: i32,
    pub foo_bar: [libc::c_double; 38],
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
pub struct glp_prep {
    pub orig_dir: i32,
    pub orig_m: i32,
    pub orig_n: i32,
    pub orig_nnz: i32,
    pub pool: *mut DMP,
    pub name: *mut i8,
    pub obj: *mut i8,
    pub c0: libc::c_double,
    pub nrows: i32,
    pub ncols: i32,
    pub r_head: *mut NPPROW,
    pub r_tail: *mut NPPROW,
    pub c_head: *mut NPPCOL,
    pub c_tail: *mut NPPCOL,
    pub stack: *mut DMP,
    pub top: *mut NPPTSE,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub row_ref: *mut i32,
    pub col_ref: *mut i32,
    pub sol: i32,
    pub scaling: i32,
    pub p_stat: i32,
    pub d_stat: i32,
    pub t_stat: i32,
    pub i_stat: i32,
    pub r_stat: *mut i8,
    pub c_stat: *mut i8,
    pub r_pi: *mut libc::c_double,
    pub c_value: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPTSE {
    pub func: Option<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32>,
    pub info: *mut libc::c_void,
    pub link: *mut NPPTSE,
}
pub type NPP = glp_prep;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPCOL {
    pub j: i32,
    pub name: *mut i8,
    pub is_int: i8,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: i32,
    pub ll: C2RustUnnamed_0,
    pub uu: C2RustUnnamed,
    pub prev: *mut NPPCOL,
    pub next: *mut NPPCOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub uu: libc::c_double,
    pub neg: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ll: libc::c_double,
    pub pos: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPAIJ {
    pub row: *mut NPPROW,
    pub col: *mut NPPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut NPPAIJ,
    pub r_next: *mut NPPAIJ,
    pub c_prev: *mut NPPAIJ,
    pub c_next: *mut NPPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPROW {
    pub i: i32,
    pub name: *mut i8,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: i32,
    pub prev: *mut NPPROW,
    pub next: *mut NPPROW,
}
unsafe extern "C" fn trivial_lp(mut P: *mut glp_prob, mut parm: *const glp_smcp) {
    let mut current_block: u64;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p_infeas: libc::c_double = 0.;
    let mut d_infeas: libc::c_double = 0.;
    let mut zeta: libc::c_double = 0.;
    (*P).valid = 0 as i32;
    (*P).dbs_stat = 2 as i32;
    (*P).pbs_stat = (*P).dbs_stat;
    (*P).obj_val = (*P).c0;
    (*P).some = 0 as i32;
    d_infeas = 0.0f64;
    p_infeas = d_infeas;
    i = 1 as i32;
    while i <= (*P).m {
        row = *((*P).row).offset(i as isize);
        (*row).stat = 1 as i32;
        (*row).dual = 0.0f64;
        (*row).prim = (*row).dual;
        if (*row).type_0 == 2 as i32 || (*row).type_0 == 4 as i32
            || (*row).type_0 == 5 as i32
        {
            if (*row).lb > (*parm).tol_bnd {
                (*P).pbs_stat = 4 as i32;
                if (*P).some == 0 as i32 && (*parm).meth != 1 as i32 {
                    (*P).some = i;
                }
            }
            if p_infeas < (*row).lb {
                p_infeas = (*row).lb;
            }
        }
        if (*row).type_0 == 3 as i32 || (*row).type_0 == 4 as i32
            || (*row).type_0 == 5 as i32
        {
            if (*row).ub < -(*parm).tol_bnd {
                (*P).pbs_stat = 4 as i32;
                if (*P).some == 0 as i32 && (*parm).meth != 1 as i32 {
                    (*P).some = i;
                }
            }
            if p_infeas < -(*row).ub {
                p_infeas = -(*row).ub;
            }
        }
        i += 1;
        i;
    }
    zeta = 1.0f64;
    j = 1 as i32;
    while j <= (*P).n {
        col = *((*P).col).offset(j as isize);
        if zeta < fabs((*col).coef) {
            zeta = fabs((*col).coef);
        }
        j += 1;
        j;
    }
    zeta = (if (*P).dir == 1 as i32 { 1.0f64 } else { -1.0f64 }) / zeta;
    j = 1 as i32;
    while j <= (*P).n {
        col = *((*P).col).offset(j as isize);
        if (*col).type_0 == 1 as i32 {
            (*col).stat = 4 as i32;
            (*col).prim = 0.0f64;
        } else {
            if (*col).type_0 == 2 as i32 {
                current_block = 17210280960103470250;
            } else {
                if (*col).type_0 == 3 as i32 {
                    current_block = 7754779993781549577;
                } else if (*col).type_0 == 4 as i32 {
                    if zeta * (*col).coef > 0.0f64 {
                        current_block = 17210280960103470250;
                    } else if zeta * (*col).coef < 0.0f64 {
                        current_block = 7754779993781549577;
                    } else if fabs((*col).lb) <= fabs((*col).ub) {
                        current_block = 17210280960103470250;
                    } else {
                        current_block = 7754779993781549577;
                    }
                } else {
                    if (*col).type_0 == 5 as i32 {
                        (*col).stat = 5 as i32;
                        (*col).prim = (*col).lb;
                    }
                    current_block = 16799951812150840583;
                }
                match current_block {
                    16799951812150840583 => {}
                    17210280960103470250 => {}
                    _ => {
                        (*col).stat = 3 as i32;
                        (*col).prim = (*col).ub;
                        current_block = 16799951812150840583;
                    }
                }
            }
            match current_block {
                16799951812150840583 => {}
                _ => {
                    (*col).stat = 2 as i32;
                    (*col).prim = (*col).lb;
                }
            }
        }
        (*col).dual = (*col).coef;
        (*P).obj_val += (*col).coef * (*col).prim;
        if (*col).type_0 == 1 as i32 || (*col).type_0 == 2 as i32 {
            if zeta * (*col).dual < -(*parm).tol_dj {
                (*P).dbs_stat = 4 as i32;
                if (*P).some == 0 as i32 && (*parm).meth == 1 as i32 {
                    (*P).some = (*P).m + j;
                }
            }
            if d_infeas < -zeta * (*col).dual {
                d_infeas = -zeta * (*col).dual;
            }
        }
        if (*col).type_0 == 1 as i32 || (*col).type_0 == 3 as i32 {
            if zeta * (*col).dual > (*parm).tol_dj {
                (*P).dbs_stat = 4 as i32;
                if (*P).some == 0 as i32 && (*parm).meth == 1 as i32 {
                    (*P).some = (*P).m + j;
                }
            }
            if d_infeas < zeta * (*col).dual {
                d_infeas = zeta * (*col).dual;
            }
        }
        j += 1;
        j;
    }
    if (*parm).msg_lev >= 2 as i32 && (*parm).out_dly == 0 as i32 {
        glp_printf(
            b"~%6d: obj = %17.9e  infeas = %10.3e\n\0" as *const u8 as *const i8,
            (*P).it_cnt,
            (*P).obj_val,
            if (*parm).meth == 1 as i32 { p_infeas } else { d_infeas },
        );
    }
    if (*parm).msg_lev >= 3 as i32 && (*parm).out_dly == 0 as i32 {
        if (*P).pbs_stat == 2 as i32 && (*P).dbs_stat == 2 as i32 {
            glp_printf(b"OPTIMAL SOLUTION FOUND\n\0" as *const u8 as *const i8);
        } else if (*P).pbs_stat == 4 as i32 {
            glp_printf(
                b"PROBLEM HAS NO FEASIBLE SOLUTION\n\0" as *const u8 as *const i8,
            );
        } else if (*parm).meth == 1 as i32 {
            glp_printf(b"PROBLEM HAS UNBOUNDED SOLUTION\n\0" as *const u8 as *const i8);
        } else {
            glp_printf(
                b"PROBLEM HAS NO DUAL FEASIBLE SOLUTION\n\0" as *const u8 as *const i8,
            );
        }
    }
}
unsafe extern "C" fn solve_lp(mut P: *mut glp_prob, mut parm: *const glp_smcp) -> i32 {
    let mut current_block: u64;
    let mut ret: i32 = 0;
    if glp_bf_exists(P) == 0 {
        ret = glp_factorize(P);
        if !(ret == 0 as i32) {
            if ret == 0x1 as i32 {
                if (*parm).msg_lev >= 1 as i32 {
                    glp_printf(
                        b"glp_simplex: initial basis is invalid\n\0" as *const u8
                            as *const i8,
                    );
                }
            } else if ret == 0x2 as i32 {
                if (*parm).msg_lev >= 1 as i32 {
                    glp_printf(
                        b"glp_simplex: initial basis is singular\n\0" as *const u8
                            as *const i8,
                    );
                }
            } else if ret == 0x3 as i32 {
                if (*parm).msg_lev >= 1 as i32 {
                    glp_printf(
                        b"glp_simplex: initial basis is ill-conditioned\n\0" as *const u8
                            as *const i8,
                    );
                }
            } else {
                (ret != ret
                    || {
                        glp_assert_(
                            b"ret != ret\0" as *const u8 as *const i8,
                            b"draft/glpapi06.c\0" as *const u8 as *const i8,
                            241 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
        if ret != 0 as i32 {
            current_block = 17285591988595213416;
        } else {
            current_block = 12039483399334584727;
        }
    } else {
        current_block = 12039483399334584727;
    }
    match current_block {
        12039483399334584727 => {
            if (*parm).meth == 1 as i32 {
                ret = _glp_spx_primal(P, parm);
            } else if (*parm).meth == 2 as i32 {
                ret = _glp_spy_dual(P, parm);
                if ret == 0x5 as i32 && (*P).valid != 0 {
                    ret = _glp_spx_primal(P, parm);
                }
            } else if (*parm).meth == 3 as i32 {
                ret = _glp_spy_dual(P, parm);
            } else {
                (parm != parm
                    || {
                        glp_assert_(
                            b"parm != parm\0" as *const u8 as *const i8,
                            b"draft/glpapi06.c\0" as *const u8 as *const i8,
                            254 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn preprocess_and_solve_lp(
    mut P: *mut glp_prob,
    mut parm: *const glp_smcp,
) -> i32 {
    let mut current_block: u64;
    let mut npp: *mut NPP = 0 as *mut NPP;
    let mut lp: *mut glp_prob = 0 as *mut glp_prob;
    let mut bfcp: glp_bfcp = glp_bfcp {
        msg_lev: 0,
        type_0: 0,
        lu_size: 0,
        piv_tol: 0.,
        piv_lim: 0,
        suhl: 0,
        eps_tol: 0.,
        max_gro: 0.,
        nfs_max: 0,
        upd_tol: 0.,
        nrs_max: 0,
        rs_size: 0,
        foo_bar: [0.; 38],
    };
    let mut ret: i32 = 0;
    if (*parm).msg_lev >= 3 as i32 {
        glp_printf(b"Preprocessing...\n\0" as *const u8 as *const i8);
    }
    npp = _glp_npp_create_wksp();
    _glp_npp_load_prob(npp, P, 0 as i32, 1 as i32, 0 as i32);
    ret = _glp_npp_simplex(npp, parm);
    if !(ret == 0 as i32) {
        if ret == 0xa as i32 {
            if (*parm).msg_lev >= 3 as i32 {
                glp_printf(
                    b"PROBLEM HAS NO PRIMAL FEASIBLE SOLUTION\n\0" as *const u8
                        as *const i8,
                );
            }
        } else if ret == 0xb as i32 {
            if (*parm).msg_lev >= 3 as i32 {
                glp_printf(
                    b"PROBLEM HAS NO DUAL FEASIBLE SOLUTION\n\0" as *const u8
                        as *const i8,
                );
            }
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const i8,
                        b"draft/glpapi06.c\0" as *const u8 as *const i8,
                        283 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    if !(ret != 0 as i32) {
        lp = glp_create_prob();
        _glp_npp_build_prob(npp, lp);
        if (*lp).m == 0 as i32 && (*lp).n == 0 as i32 {
            (*lp).dbs_stat = 2 as i32;
            (*lp).pbs_stat = (*lp).dbs_stat;
            (*lp).obj_val = (*lp).c0;
            if (*parm).msg_lev >= 2 as i32 && (*parm).out_dly == 0 as i32 {
                glp_printf(
                    b"~%6d: obj = %17.9e  infeas = %10.3e\n\0" as *const u8 as *const i8,
                    (*P).it_cnt,
                    (*lp).obj_val,
                    0.0f64,
                );
            }
            if (*parm).msg_lev >= 3 as i32 {
                glp_printf(
                    b"OPTIMAL SOLUTION FOUND BY LP PREPROCESSOR\n\0" as *const u8
                        as *const i8,
                );
            }
            current_block = 10869362242983964752;
        } else {
            if (*parm).msg_lev >= 3 as i32 {
                glp_printf(
                    b"%d row%s, %d column%s, %d non-zero%s\n\0" as *const u8
                        as *const i8,
                    (*lp).m,
                    if (*lp).m == 1 as i32 {
                        b"\0" as *const u8 as *const i8
                    } else {
                        b"s\0" as *const u8 as *const i8
                    },
                    (*lp).n,
                    if (*lp).n == 1 as i32 {
                        b"\0" as *const u8 as *const i8
                    } else {
                        b"s\0" as *const u8 as *const i8
                    },
                    (*lp).nnz,
                    if (*lp).nnz == 1 as i32 {
                        b"\0" as *const u8 as *const i8
                    } else {
                        b"s\0" as *const u8 as *const i8
                    },
                );
            }
            glp_get_bfcp(P, &mut bfcp);
            glp_set_bfcp(lp, &mut bfcp);
            let mut env: *mut ENV = _glp_get_env_ptr();
            let mut term_out: i32 = (*env).term_out;
            if term_out == 0 || (*parm).msg_lev < 3 as i32 {
                (*env).term_out = 0 as i32;
            } else {
                (*env).term_out = 1 as i32;
            }
            glp_scale_prob(lp, 0x80 as i32);
            (*env).term_out = term_out;
            let mut env_0: *mut ENV = _glp_get_env_ptr();
            let mut term_out_0: i32 = (*env_0).term_out;
            if term_out_0 == 0 || (*parm).msg_lev < 3 as i32 {
                (*env_0).term_out = 0 as i32;
            } else {
                (*env_0).term_out = 1 as i32;
            }
            glp_adv_basis(lp, 0 as i32);
            (*env_0).term_out = term_out_0;
            (*lp).it_cnt = (*P).it_cnt;
            ret = solve_lp(lp, parm);
            (*P).it_cnt = (*lp).it_cnt;
            if !(ret == 0 as i32 && (*lp).pbs_stat == 2 as i32
                && (*lp).dbs_stat == 2 as i32)
            {
                if (*parm).msg_lev >= 1 as i32 {
                    glp_printf(
                        b"glp_simplex: unable to recover undefined or non-optimal solution\n\0"
                            as *const u8 as *const i8,
                    );
                }
                if ret == 0 as i32 {
                    if (*lp).pbs_stat == 4 as i32 {
                        ret = 0xa as i32;
                    } else if (*lp).dbs_stat == 4 as i32 {
                        ret = 0xb as i32;
                    } else {
                        (lp != lp
                            || {
                                glp_assert_(
                                    b"lp != lp\0" as *const u8 as *const i8,
                                    b"draft/glpapi06.c\0" as *const u8 as *const i8,
                                    345 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                }
                current_block = 3279715233734680527;
            } else {
                current_block = 10869362242983964752;
            }
        }
        match current_block {
            3279715233734680527 => {}
            _ => {
                _glp_npp_postprocess(npp, lp);
                glp_delete_prob(lp);
                lp = 0 as *mut glp_prob;
                _glp_npp_unload_sol(npp, P);
                ret = 0 as i32;
            }
        }
    }
    if !lp.is_null() {
        glp_delete_prob(lp);
    }
    _glp_npp_delete_wksp(npp);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_simplex(
    mut P: *mut glp_prob,
    mut parm: *const glp_smcp,
) -> i32 {
    let mut current_block: u64;
    let mut _parm: glp_smcp = glp_smcp {
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
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ret: i32 = 0;
    if !((*P).tree).is_null() && (*(*P).tree).reason != 0 as i32 {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 374 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_simplex: operation not allowed\n\0" as *const u8 as *const i8);
    }
    if parm.is_null() {
        parm = &mut _parm;
        glp_init_smcp(parm as *mut glp_smcp);
    }
    if !((*parm).msg_lev == 0 as i32 || (*parm).msg_lev == 1 as i32
        || (*parm).msg_lev == 2 as i32 || (*parm).msg_lev == 3 as i32
        || (*parm).msg_lev == 4 as i32)
    {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 383 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: msg_lev = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).msg_lev,
        );
    }
    if !((*parm).meth == 1 as i32 || (*parm).meth == 2 as i32
        || (*parm).meth == 3 as i32)
    {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 388 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: meth = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).meth,
        );
    }
    if !((*parm).pricing == 0x11 as i32 || (*parm).pricing == 0x22 as i32) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 392 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: pricing = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).pricing,
        );
    }
    if !((*parm).r_test == 0x11 as i32 || (*parm).r_test == 0x33 as i32
        || (*parm).r_test == 0x22 as i32)
    {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 399 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: r_test = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).r_test,
        );
    }
    if !(0.0f64 < (*parm).tol_bnd && (*parm).tol_bnd < 1.0f64) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 402 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: tol_bnd = %g; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).tol_bnd,
        );
    }
    if !(0.0f64 < (*parm).tol_dj && (*parm).tol_dj < 1.0f64) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 405 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: tol_dj = %g; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).tol_dj,
        );
    }
    if !(0.0f64 < (*parm).tol_piv && (*parm).tol_piv < 1.0f64) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 408 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: tol_piv = %g; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).tol_piv,
        );
    }
    if (*parm).it_lim < 0 as i32 {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 411 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: it_lim = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).it_lim,
        );
    }
    if (*parm).tm_lim < 0 as i32 {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 414 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: tm_lim = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).tm_lim,
        );
    }
    if (*parm).out_frq < 0 as i32 {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 421 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: out_frq = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).out_frq,
        );
    }
    if (*parm).out_dly < 0 as i32 {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 424 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: out_dly = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).out_dly,
        );
    }
    if !((*parm).presolve == 1 as i32 || (*parm).presolve == 0 as i32) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 427 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: presolve = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).presolve,
        );
    }
    if !((*parm).excl == 1 as i32 || (*parm).excl == 0 as i32) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 431 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: excl = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).excl,
        );
    }
    if !((*parm).shift == 1 as i32 || (*parm).shift == 0 as i32) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 434 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: shift = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).shift,
        );
    }
    if !((*parm).aorn == 1 as i32 || (*parm).aorn == 2 as i32) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 437 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_simplex: aorn = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).aorn,
        );
    }
    (*P).dbs_stat = 1 as i32;
    (*P).pbs_stat = (*P).dbs_stat;
    (*P).obj_val = 0.0f64;
    (*P).some = 0 as i32;
    i = 1 as i32;
    loop {
        if !(i <= (*P).m) {
            current_block = 1836292691772056875;
            break;
        }
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        if (*row).type_0 == 4 as i32 && (*row).lb >= (*row).ub {
            if (*parm).msg_lev >= 1 as i32 {
                glp_printf(
                    b"glp_simplex: row %d: lb = %g, ub = %g; incorrect bounds\n\0"
                        as *const u8 as *const i8,
                    i,
                    (*row).lb,
                    (*row).ub,
                );
            }
            ret = 0x4 as i32;
            current_block = 8007126901599900252;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        1836292691772056875 => {
            j = 1 as i32;
            loop {
                if !(j <= (*P).n) {
                    current_block = 10758786907990354186;
                    break;
                }
                let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
                if (*col).type_0 == 4 as i32 && (*col).lb >= (*col).ub {
                    if (*parm).msg_lev >= 1 as i32 {
                        glp_printf(
                            b"glp_simplex: column %d: lb = %g, ub = %g; incorrect bounds\n\0"
                                as *const u8 as *const i8,
                            j,
                            (*col).lb,
                            (*col).ub,
                        );
                    }
                    ret = 0x4 as i32;
                    current_block = 8007126901599900252;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            match current_block {
                8007126901599900252 => {}
                _ => {
                    if (*parm).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"GLPK Simplex Optimizer %s\n\0" as *const u8 as *const i8,
                            glp_version(),
                        );
                        glp_printf(
                            b"%d row%s, %d column%s, %d non-zero%s\n\0" as *const u8
                                as *const i8,
                            (*P).m,
                            if (*P).m == 1 as i32 {
                                b"\0" as *const u8 as *const i8
                            } else {
                                b"s\0" as *const u8 as *const i8
                            },
                            (*P).n,
                            if (*P).n == 1 as i32 {
                                b"\0" as *const u8 as *const i8
                            } else {
                                b"s\0" as *const u8 as *const i8
                            },
                            (*P).nnz,
                            if (*P).nnz == 1 as i32 {
                                b"\0" as *const u8 as *const i8
                            } else {
                                b"s\0" as *const u8 as *const i8
                            },
                        );
                    }
                    if (*P).nnz == 0 as i32 {
                        trivial_lp(P, parm);
                        ret = 0 as i32;
                    } else if (*parm).presolve == 0 {
                        ret = solve_lp(P, parm);
                    } else {
                        ret = preprocess_and_solve_lp(P, parm);
                    }
                }
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_init_smcp(mut parm: *mut glp_smcp) {
    (*parm).msg_lev = 3 as i32;
    (*parm).meth = 1 as i32;
    (*parm).pricing = 0x22 as i32;
    (*parm).r_test = 0x22 as i32;
    (*parm).tol_bnd = 1e-7f64;
    (*parm).tol_dj = 1e-7f64;
    (*parm).tol_piv = 1e-9f64;
    (*parm).obj_ll = -1.7976931348623157e+308f64;
    (*parm).obj_ul = 1.7976931348623157e+308f64;
    (*parm).it_lim = 2147483647 as i32;
    (*parm).tm_lim = 2147483647 as i32;
    (*parm).out_frq = 5000 as i32;
    (*parm).out_dly = 0 as i32;
    (*parm).presolve = 0 as i32;
    (*parm).excl = 1 as i32;
    (*parm).shift = 1 as i32;
    (*parm).aorn = 2 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_status(mut lp: *mut glp_prob) -> i32 {
    let mut status: i32 = 0;
    status = glp_get_prim_stat(lp);
    match status {
        2 => {
            match glp_get_dual_stat(lp) {
                2 => {
                    status = 5 as i32;
                }
                4 => {
                    status = 6 as i32;
                }
                1 | 3 => {
                    status = status;
                }
                _ => {
                    (lp != lp
                        || {
                            glp_assert_(
                                b"lp != lp\0" as *const u8 as *const i8,
                                b"draft/glpapi06.c\0" as *const u8 as *const i8,
                                568 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
        }
        1 | 3 | 4 => {
            status = status;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const i8,
                        b"draft/glpapi06.c\0" as *const u8 as *const i8,
                        577 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_prim_stat(mut lp: *mut glp_prob) -> i32 {
    let mut pbs_stat: i32 = (*lp).pbs_stat;
    return pbs_stat;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_dual_stat(mut lp: *mut glp_prob) -> i32 {
    let mut dbs_stat: i32 = (*lp).dbs_stat;
    return dbs_stat;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_obj_val(mut lp: *mut glp_prob) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    z = (*lp).obj_val;
    return z;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_row_stat(mut lp: *mut glp_prob, mut i: i32) -> i32 {
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 674 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_row_stat: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    return (**((*lp).row).offset(i as isize)).stat;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_row_prim(
    mut lp: *mut glp_prob,
    mut i: i32,
) -> libc::c_double {
    let mut prim: libc::c_double = 0.;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 697 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_row_prim: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    prim = (**((*lp).row).offset(i as isize)).prim;
    return prim;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_row_dual(
    mut lp: *mut glp_prob,
    mut i: i32,
) -> libc::c_double {
    let mut dual: libc::c_double = 0.;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 722 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_row_dual: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    dual = (**((*lp).row).offset(i as isize)).dual;
    return dual;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_col_stat(mut lp: *mut glp_prob, mut j: i32) -> i32 {
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 751 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_col_stat: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    return (**((*lp).col).offset(j as isize)).stat;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_col_prim(
    mut lp: *mut glp_prob,
    mut j: i32,
) -> libc::c_double {
    let mut prim: libc::c_double = 0.;
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 774 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_col_prim: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    prim = (**((*lp).col).offset(j as isize)).prim;
    return prim;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_col_dual(
    mut lp: *mut glp_prob,
    mut j: i32,
) -> libc::c_double {
    let mut dual: libc::c_double = 0.;
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"draft/glpapi06.c\0" as *const u8 as *const i8, 799 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_col_dual: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    dual = (**((*lp).col).offset(j as isize)).dual;
    return dual;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_unbnd_ray(mut lp: *mut glp_prob) -> i32 {
    let mut k: i32 = 0;
    k = (*lp).some;
    (k >= 0 as i32
        || {
            glp_assert_(
                b"k >= 0\0" as *const u8 as *const i8,
                b"draft/glpapi06.c\0" as *const u8 as *const i8,
                837 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if k > (*lp).m + (*lp).n {
        k = 0 as i32;
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_it_cnt(mut P: *mut glp_prob) -> i32 {
    return (*P).it_cnt;
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_it_cnt(mut P: *mut glp_prob, mut it_cnt: i32) {
    (*P).it_cnt = it_cnt;
}