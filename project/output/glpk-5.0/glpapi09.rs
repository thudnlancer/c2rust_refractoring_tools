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
    pub type BFD;
    pub type AVL;
    pub type AVLNODE;
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    pub type DMP;
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_scale_prob(P: *mut glp_prob, flags: i32);
    fn glp_adv_basis(P: *mut glp_prob, flags: i32);
    fn glp_simplex(P: *mut glp_prob, parm: *const glp_smcp) -> i32;
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_get_status(P: *mut glp_prob) -> i32;
    fn glp_get_bfcp(P: *mut glp_prob, parm: *mut glp_bfcp);
    fn glp_set_bfcp(P: *mut glp_prob, parm: *const glp_bfcp);
    fn glp_version() -> *const i8;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn floor(_: libc::c_double) -> libc::c_double;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn _glp_get_env_ptr() -> *mut ENV;
    fn _glp_ios_create_tree(mip: *mut glp_prob, parm: *const glp_iocp) -> *mut glp_tree;
    fn _glp_ios_delete_tree(tree: *mut glp_tree);
    fn _glp_ios_driver(tree: *mut glp_tree) -> i32;
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
    fn _glp_npp_integer(npp: *mut NPP, parm: *const glp_iocp) -> i32;
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
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
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
pub type FILE = _IO_FILE;
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
pub type __off64_t = i64;
pub type _IO_lock_t = ();
pub type __off_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_col_kind(
    mut mip: *mut glp_prob,
    mut j: i32,
    mut kind: i32,
) {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    if !(1 as i32 <= j && j <= (*mip).n) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 48 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_col_kind: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    col = *((*mip).col).offset(j as isize);
    match kind {
        1 => {
            (*col).kind = 1 as i32;
        }
        2 => {
            (*col).kind = 2 as i32;
        }
        3 => {
            (*col).kind = 2 as i32;
            if !((*col).type_0 == 4 as i32 && (*col).lb == 0.0f64 && (*col).ub == 1.0f64)
            {
                glp_set_col_bnds(mip, j, 4 as i32, 0.0f64, 1.0f64);
            }
        }
        _ => {
            (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 64 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_col_kind: j = %d; kind = %d; invalid column kind\n\0"
                    as *const u8 as *const i8,
                j,
                kind,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_col_kind(mut mip: *mut glp_prob, mut j: i32) -> i32 {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut kind: i32 = 0;
    if !(1 as i32 <= j && j <= (*mip).n) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 92 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_col_kind: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    col = *((*mip).col).offset(j as isize);
    kind = (*col).kind;
    match kind {
        1 => {}
        2 => {
            if (*col).type_0 == 4 as i32 && (*col).lb == 0.0f64 && (*col).ub == 1.0f64 {
                kind = 3 as i32;
            }
        }
        _ => {
            (kind != kind
                || {
                    glp_assert_(
                        b"kind != kind\0" as *const u8 as *const i8,
                        b"draft/glpapi09.c\0" as *const u8 as *const i8,
                        104 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return kind;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_num_int(mut mip: *mut glp_prob) -> i32 {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut j: i32 = 0;
    let mut count: i32 = 0 as i32;
    j = 1 as i32;
    while j <= (*mip).n {
        col = *((*mip).col).offset(j as isize);
        if (*col).kind == 2 as i32 {
            count += 1;
            count;
        }
        j += 1;
        j;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_num_bin(mut mip: *mut glp_prob) -> i32 {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut j: i32 = 0;
    let mut count: i32 = 0 as i32;
    j = 1 as i32;
    while j <= (*mip).n {
        col = *((*mip).col).offset(j as isize);
        if (*col).kind == 2 as i32 && (*col).type_0 == 4 as i32 && (*col).lb == 0.0f64
            && (*col).ub == 1.0f64
        {
            count += 1;
            count;
        }
        j += 1;
        j;
    }
    return count;
}
unsafe extern "C" fn solve_mip(
    mut P: *mut glp_prob,
    mut parm: *const glp_iocp,
    mut P0: *mut glp_prob,
    mut npp: *mut NPP,
) -> i32 {
    let mut T: *mut glp_tree = 0 as *mut glp_tree;
    let mut ret: i32 = 0;
    if glp_get_status(P) != 5 as i32 {
        if (*parm).msg_lev >= 1 as i32 {
            glp_printf(
                b"glp_intopt: optimal basis to initial LP relaxation not provided\n\0"
                    as *const u8 as *const i8,
            );
        }
        ret = 0xc as i32;
    } else {
        if (*parm).msg_lev >= 3 as i32 {
            glp_printf(b"Integer optimization begins...\n\0" as *const u8 as *const i8);
        }
        T = _glp_ios_create_tree(P, parm);
        (*T).P = P0 as *mut libc::c_void;
        (*T).npp = npp as *mut libc::c_void;
        ret = _glp_ios_driver(T);
        _glp_ios_delete_tree(T);
        if ret == 0 as i32 {
            if (*P).mip_stat == 2 as i32 {
                if (*parm).msg_lev >= 3 as i32 {
                    glp_printf(
                        b"INTEGER OPTIMAL SOLUTION FOUND\n\0" as *const u8 as *const i8,
                    );
                }
                (*P).mip_stat = 5 as i32;
            } else {
                if (*parm).msg_lev >= 3 as i32 {
                    glp_printf(
                        b"PROBLEM HAS NO INTEGER FEASIBLE SOLUTION\n\0" as *const u8
                            as *const i8,
                    );
                }
                (*P).mip_stat = 4 as i32;
            }
        } else if ret == 0xe as i32 {
            if (*parm).msg_lev >= 3 as i32 {
                glp_printf(
                    b"RELATIVE MIP GAP TOLERANCE REACHED; SEARCH TERMINATED\n\0"
                        as *const u8 as *const i8,
                );
            }
        } else if ret == 0x9 as i32 {
            if (*parm).msg_lev >= 3 as i32 {
                glp_printf(
                    b"TIME LIMIT EXCEEDED; SEARCH TERMINATED\n\0" as *const u8
                        as *const i8,
                );
            }
        } else if ret == 0x5 as i32 {
            if (*parm).msg_lev >= 1 as i32 {
                glp_printf(
                    b"glp_intopt: cannot solve current LP relaxation\n\0" as *const u8
                        as *const i8,
                );
            }
        } else if ret == 0xd as i32 {
            if (*parm).msg_lev >= 3 as i32 {
                glp_printf(
                    b"SEARCH TERMINATED BY APPLICATION\n\0" as *const u8 as *const i8,
                );
            }
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const i8,
                        b"draft/glpapi09.c\0" as *const u8 as *const i8,
                        281 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return ret;
}
unsafe extern "C" fn preprocess_and_solve_mip(
    mut P: *mut glp_prob,
    mut parm: *const glp_iocp,
) -> i32 {
    let mut current_block: u64;
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut term_out: i32 = (*env).term_out;
    let mut npp: *mut NPP = 0 as *mut NPP;
    let mut mip: *mut glp_prob = 0 as *mut glp_prob;
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
    let mut smcp: glp_smcp = glp_smcp {
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
    if (*parm).msg_lev >= 3 as i32 {
        glp_printf(b"Preprocessing...\n\0" as *const u8 as *const i8);
    }
    npp = _glp_npp_create_wksp();
    _glp_npp_load_prob(npp, P, 0 as i32, 3 as i32, 0 as i32);
    if term_out == 0 || (*parm).msg_lev < 3 as i32 {
        (*env).term_out = 0 as i32;
    } else {
        (*env).term_out = 1 as i32;
    }
    ret = _glp_npp_integer(npp, parm);
    (*env).term_out = term_out;
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
                    b"LP RELAXATION HAS NO DUAL FEASIBLE SOLUTION\n\0" as *const u8
                        as *const i8,
                );
            }
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const i8,
                        b"draft/glpapi09.c\0" as *const u8 as *const i8,
                        318 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    if !(ret != 0 as i32) {
        mip = glp_create_prob();
        _glp_npp_build_prob(npp, mip);
        if (*mip).m == 0 as i32 && (*mip).n == 0 as i32 {
            (*mip).mip_stat = 5 as i32;
            (*mip).mip_obj = (*mip).c0;
            if (*parm).msg_lev >= 3 as i32 {
                glp_printf(
                    b"Objective value = %17.9e\n\0" as *const u8 as *const i8,
                    (*mip).mip_obj,
                );
                glp_printf(
                    b"INTEGER OPTIMAL SOLUTION FOUND BY MIP PREPROCESSOR\n\0"
                        as *const u8 as *const i8,
                );
            }
            current_block = 6855399523897028225;
        } else {
            if (*parm).msg_lev >= 3 as i32 {
                let mut ni: i32 = glp_get_num_int(mip);
                let mut nb: i32 = glp_get_num_bin(mip);
                let mut s: [i8; 50] = [0; 50];
                glp_printf(
                    b"%d row%s, %d column%s, %d non-zero%s\n\0" as *const u8
                        as *const i8,
                    (*mip).m,
                    if (*mip).m == 1 as i32 {
                        b"\0" as *const u8 as *const i8
                    } else {
                        b"s\0" as *const u8 as *const i8
                    },
                    (*mip).n,
                    if (*mip).n == 1 as i32 {
                        b"\0" as *const u8 as *const i8
                    } else {
                        b"s\0" as *const u8 as *const i8
                    },
                    (*mip).nnz,
                    if (*mip).nnz == 1 as i32 {
                        b"\0" as *const u8 as *const i8
                    } else {
                        b"s\0" as *const u8 as *const i8
                    },
                );
                if nb == 0 as i32 {
                    strcpy(s.as_mut_ptr(), b"none of\0" as *const u8 as *const i8);
                } else if ni == 1 as i32 && nb == 1 as i32 {
                    strcpy(s.as_mut_ptr(), b"\0" as *const u8 as *const i8);
                } else if nb == 1 as i32 {
                    strcpy(s.as_mut_ptr(), b"one of\0" as *const u8 as *const i8);
                } else if nb == ni {
                    strcpy(s.as_mut_ptr(), b"all of\0" as *const u8 as *const i8);
                } else {
                    sprintf(s.as_mut_ptr(), b"%d of\0" as *const u8 as *const i8, nb);
                }
                glp_printf(
                    b"%d integer variable%s, %s which %s binary\n\0" as *const u8
                        as *const i8,
                    ni,
                    if ni == 1 as i32 {
                        b"\0" as *const u8 as *const i8
                    } else {
                        b"s\0" as *const u8 as *const i8
                    },
                    s.as_mut_ptr(),
                    if nb == 1 as i32 {
                        b"is\0" as *const u8 as *const i8
                    } else {
                        b"are\0" as *const u8 as *const i8
                    },
                );
            }
            glp_get_bfcp(P, &mut bfcp);
            glp_set_bfcp(mip, &mut bfcp);
            if term_out == 0 || (*parm).msg_lev < 3 as i32 {
                (*env).term_out = 0 as i32;
            } else {
                (*env).term_out = 1 as i32;
            }
            glp_scale_prob(mip, 0x1 as i32 | 0x10 as i32 | 0x20 as i32 | 0x40 as i32);
            (*env).term_out = term_out;
            if term_out == 0 || (*parm).msg_lev < 3 as i32 {
                (*env).term_out = 0 as i32;
            } else {
                (*env).term_out = 1 as i32;
            }
            glp_adv_basis(mip, 0 as i32);
            (*env).term_out = term_out;
            if (*parm).msg_lev >= 3 as i32 {
                glp_printf(b"Solving LP relaxation...\n\0" as *const u8 as *const i8);
            }
            glp_init_smcp(&mut smcp);
            smcp.msg_lev = (*parm).msg_lev;
            smcp.tm_lim = (*parm).tm_lim;
            (*mip).it_cnt = (*P).it_cnt;
            ret = glp_simplex(mip, &mut smcp);
            (*P).it_cnt = (*mip).it_cnt;
            if ret == 0x9 as i32 {
                current_block = 10038430210359095033;
            } else if ret != 0 as i32 {
                if (*parm).msg_lev >= 1 as i32 {
                    glp_printf(
                        b"glp_intopt: cannot solve LP relaxation\n\0" as *const u8
                            as *const i8,
                    );
                }
                ret = 0x5 as i32;
                current_block = 10038430210359095033;
            } else {
                ret = glp_get_status(mip);
                if ret == 5 as i32 {
                    ret = 0 as i32;
                } else if ret == 4 as i32 {
                    ret = 0xa as i32;
                } else if ret == 6 as i32 {
                    ret = 0xb as i32;
                } else {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const i8,
                                b"draft/glpapi09.c\0" as *const u8 as *const i8,
                                401 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
                if ret != 0 as i32 {
                    current_block = 10038430210359095033;
                } else {
                    (*mip).it_cnt = (*P).it_cnt;
                    if (*parm).use_sol != 0 {
                        (*mip).mip_stat = (*P).mip_stat;
                        (*mip).mip_obj = (*P).mip_obj;
                    }
                    ret = solve_mip(mip, parm, P, npp);
                    (*P).it_cnt = (*mip).it_cnt;
                    if !((*mip).mip_stat == 5 as i32 || (*mip).mip_stat == 2 as i32) {
                        (*P).mip_stat = (*mip).mip_stat;
                        current_block = 10038430210359095033;
                    } else {
                        current_block = 6855399523897028225;
                    }
                }
            }
        }
        match current_block {
            10038430210359095033 => {}
            _ => {
                _glp_npp_postprocess(npp, mip);
                glp_delete_prob(mip);
                mip = 0 as *mut glp_prob;
                _glp_npp_unload_sol(npp, P);
            }
        }
    }
    if !mip.is_null() {
        glp_delete_prob(mip);
    }
    _glp_npp_delete_wksp(npp);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_intopt1(
    mut P: *mut glp_prob,
    mut parm: *const glp_iocp,
) -> i32 {
    (P == P
        || {
            glp_assert_(
                b"P == P\0" as *const u8 as *const i8,
                b"draft/glpapi09.c\0" as *const u8 as *const i8,
                435 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (parm == parm
        || {
            glp_assert_(
                b"parm == parm\0" as *const u8 as *const i8,
                b"draft/glpapi09.c\0" as *const u8 as *const i8,
                436 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_printf(
        b"glp_intopt: no alien solver is available\n\0" as *const u8 as *const i8,
    );
    return 0x5 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_intopt(
    mut P: *mut glp_prob,
    mut parm: *const glp_iocp,
) -> i32 {
    let mut current_block: u64;
    let mut _parm: glp_iocp = glp_iocp {
        msg_lev: 0,
        br_tech: 0,
        bt_tech: 0,
        tol_int: 0.,
        tol_obj: 0.,
        tm_lim: 0,
        out_frq: 0,
        out_dly: 0,
        cb_func: None,
        cb_info: 0 as *mut libc::c_void,
        cb_size: 0,
        pp_tech: 0,
        mip_gap: 0.,
        mir_cuts: 0,
        gmi_cuts: 0,
        cov_cuts: 0,
        clq_cuts: 0,
        presolve: 0,
        binarize: 0,
        fp_heur: 0,
        ps_heur: 0,
        ps_tm_lim: 0,
        sr_heur: 0,
        use_sol: 0,
        save_sol: 0 as *const i8,
        alien: 0,
        flip: 0,
        foo_bar: [0.; 23],
    };
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ret: i32 = 0;
    if !((*P).tree).is_null() {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 452 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_intopt: operation not allowed\n\0" as *const u8 as *const i8);
    }
    if parm.is_null() {
        parm = &mut _parm;
        glp_init_iocp(parm as *mut glp_iocp);
    }
    if !((*parm).msg_lev == 0 as i32 || (*parm).msg_lev == 1 as i32
        || (*parm).msg_lev == 2 as i32 || (*parm).msg_lev == 3 as i32
        || (*parm).msg_lev == 4 as i32)
    {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 461 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: msg_lev = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).msg_lev,
        );
    }
    if !((*parm).br_tech == 1 as i32 || (*parm).br_tech == 2 as i32
        || (*parm).br_tech == 3 as i32 || (*parm).br_tech == 4 as i32
        || (*parm).br_tech == 5 as i32)
    {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 468 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: br_tech = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).br_tech,
        );
    }
    if !((*parm).bt_tech == 1 as i32 || (*parm).bt_tech == 2 as i32
        || (*parm).bt_tech == 3 as i32 || (*parm).bt_tech == 4 as i32)
    {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 474 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: bt_tech = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).bt_tech,
        );
    }
    if !(0.0f64 < (*parm).tol_int && (*parm).tol_int < 1.0f64) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 477 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: tol_int = %g; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).tol_int,
        );
    }
    if !(0.0f64 < (*parm).tol_obj && (*parm).tol_obj < 1.0f64) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 480 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: tol_obj = %g; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).tol_obj,
        );
    }
    if (*parm).tm_lim < 0 as i32 {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 483 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: tm_lim = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).tm_lim,
        );
    }
    if (*parm).out_frq < 0 as i32 {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 486 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: out_frq = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).out_frq,
        );
    }
    if (*parm).out_dly < 0 as i32 {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 489 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: out_dly = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).out_dly,
        );
    }
    if !(0 as i32 <= (*parm).cb_size && (*parm).cb_size <= 256 as i32) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 492 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: cb_size = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).cb_size,
        );
    }
    if !((*parm).pp_tech == 0 as i32 || (*parm).pp_tech == 1 as i32
        || (*parm).pp_tech == 2 as i32)
    {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 497 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: pp_tech = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).pp_tech,
        );
    }
    if (*parm).mip_gap < 0.0f64 {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 500 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: mip_gap = %g; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).mip_gap,
        );
    }
    if !((*parm).mir_cuts == 1 as i32 || (*parm).mir_cuts == 0 as i32) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 503 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: mir_cuts = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).mir_cuts,
        );
    }
    if !((*parm).gmi_cuts == 1 as i32 || (*parm).gmi_cuts == 0 as i32) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 506 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: gmi_cuts = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).gmi_cuts,
        );
    }
    if !((*parm).cov_cuts == 1 as i32 || (*parm).cov_cuts == 0 as i32) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 509 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: cov_cuts = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).cov_cuts,
        );
    }
    if !((*parm).clq_cuts == 1 as i32 || (*parm).clq_cuts == 0 as i32) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 512 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: clq_cuts = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).clq_cuts,
        );
    }
    if !((*parm).presolve == 1 as i32 || (*parm).presolve == 0 as i32) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 515 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: presolve = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).presolve,
        );
    }
    if !((*parm).binarize == 1 as i32 || (*parm).binarize == 0 as i32) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 518 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: binarize = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).binarize,
        );
    }
    if !((*parm).fp_heur == 1 as i32 || (*parm).fp_heur == 0 as i32) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 521 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: fp_heur = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).fp_heur,
        );
    }
    if !((*parm).alien == 1 as i32 || (*parm).alien == 0 as i32) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 525 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_intopt: alien = %d; invalid parameter\n\0" as *const u8 as *const i8,
            (*parm).alien,
        );
    }
    if (*parm).use_sol == 0 {
        (*P).mip_stat = 1 as i32;
    }
    if (*P).mip_stat == 4 as i32 {
        (*P).mip_stat = 1 as i32;
    }
    if (*P).mip_stat == 1 as i32 {
        (*P).mip_obj = 0.0f64;
    } else if (*P).mip_stat == 5 as i32 {
        (*P).mip_stat = 2 as i32;
    }
    i = 1 as i32;
    loop {
        if !(i <= (*P).m) {
            current_block = 10150597327160359210;
            break;
        }
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        if (*row).type_0 == 4 as i32 && (*row).lb >= (*row).ub {
            if (*parm).msg_lev >= 1 as i32 {
                glp_printf(
                    b"glp_intopt: row %d: lb = %g, ub = %g; incorrect bounds\n\0"
                        as *const u8 as *const i8,
                    i,
                    (*row).lb,
                    (*row).ub,
                );
            }
            ret = 0x4 as i32;
            current_block = 6122598547576472723;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        10150597327160359210 => {
            j = 1 as i32;
            loop {
                if !(j <= (*P).n) {
                    current_block = 7018308795614528254;
                    break;
                }
                let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
                if (*col).type_0 == 4 as i32 && (*col).lb >= (*col).ub {
                    if (*parm).msg_lev >= 1 as i32 {
                        glp_printf(
                            b"glp_intopt: column %d: lb = %g, ub = %g; incorrect bounds\n\0"
                                as *const u8 as *const i8,
                            j,
                            (*col).lb,
                            (*col).ub,
                        );
                    }
                    ret = 0x4 as i32;
                    current_block = 6122598547576472723;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            match current_block {
                6122598547576472723 => {}
                _ => {
                    j = 1 as i32;
                    loop {
                        if !(j <= (*P).n) {
                            current_block = 7158658067966855297;
                            break;
                        }
                        let mut col_0: *mut GLPCOL = *((*P).col).offset(j as isize);
                        if !((*col_0).kind != 2 as i32) {
                            if (*col_0).type_0 == 2 as i32 || (*col_0).type_0 == 4 as i32
                            {
                                if (*col_0).lb != floor((*col_0).lb) {
                                    if (*parm).msg_lev >= 1 as i32 {
                                        glp_printf(
                                            b"glp_intopt: integer column %d has non-integer lower bound %g\n\0"
                                                as *const u8 as *const i8,
                                            j,
                                            (*col_0).lb,
                                        );
                                    }
                                    ret = 0x4 as i32;
                                    current_block = 6122598547576472723;
                                    break;
                                }
                            }
                            if (*col_0).type_0 == 3 as i32 || (*col_0).type_0 == 4 as i32
                            {
                                if (*col_0).ub != floor((*col_0).ub) {
                                    if (*parm).msg_lev >= 1 as i32 {
                                        glp_printf(
                                            b"glp_intopt: integer column %d has non-integer upper bound %g\n\0"
                                                as *const u8 as *const i8,
                                            j,
                                            (*col_0).ub,
                                        );
                                    }
                                    ret = 0x4 as i32;
                                    current_block = 6122598547576472723;
                                    break;
                                }
                            }
                            if (*col_0).type_0 == 5 as i32 {
                                if (*col_0).lb != floor((*col_0).lb) {
                                    if (*parm).msg_lev >= 1 as i32 {
                                        glp_printf(
                                            b"glp_intopt: integer column %d has non-integer fixed value %g\n\0"
                                                as *const u8 as *const i8,
                                            j,
                                            (*col_0).lb,
                                        );
                                    }
                                    ret = 0x4 as i32;
                                    current_block = 6122598547576472723;
                                    break;
                                }
                            }
                        }
                        j += 1;
                        j;
                    }
                    match current_block {
                        6122598547576472723 => {}
                        _ => {
                            if (*parm).msg_lev >= 3 as i32 {
                                let mut ni: i32 = glp_get_num_int(P);
                                let mut nb: i32 = glp_get_num_bin(P);
                                let mut s: [i8; 50] = [0; 50];
                                glp_printf(
                                    b"GLPK Integer Optimizer %s\n\0" as *const u8 as *const i8,
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
                                if nb == 0 as i32 {
                                    strcpy(
                                        s.as_mut_ptr(),
                                        b"none of\0" as *const u8 as *const i8,
                                    );
                                } else if ni == 1 as i32 && nb == 1 as i32 {
                                    strcpy(s.as_mut_ptr(), b"\0" as *const u8 as *const i8);
                                } else if nb == 1 as i32 {
                                    strcpy(
                                        s.as_mut_ptr(),
                                        b"one of\0" as *const u8 as *const i8,
                                    );
                                } else if nb == ni {
                                    strcpy(
                                        s.as_mut_ptr(),
                                        b"all of\0" as *const u8 as *const i8,
                                    );
                                } else {
                                    sprintf(
                                        s.as_mut_ptr(),
                                        b"%d of\0" as *const u8 as *const i8,
                                        nb,
                                    );
                                }
                                glp_printf(
                                    b"%d integer variable%s, %s which %s binary\n\0"
                                        as *const u8 as *const i8,
                                    ni,
                                    if ni == 1 as i32 {
                                        b"\0" as *const u8 as *const i8
                                    } else {
                                        b"s\0" as *const u8 as *const i8
                                    },
                                    s.as_mut_ptr(),
                                    if nb == 1 as i32 {
                                        b"is\0" as *const u8 as *const i8
                                    } else {
                                        b"are\0" as *const u8 as *const i8
                                    },
                                );
                            }
                            if (*parm).alien != 0 {
                                ret = _glp_intopt1(P, parm);
                            } else {
                                if (*parm).presolve == 0 {
                                    ret = solve_mip(P, parm, P, 0 as *mut NPP);
                                } else {
                                    ret = preprocess_and_solve_mip(P, parm);
                                }
                                if ret == 0xa as i32 {
                                    (*P).mip_stat = 4 as i32;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_init_iocp(mut parm: *mut glp_iocp) {
    (*parm).msg_lev = 3 as i32;
    (*parm).br_tech = 4 as i32;
    (*parm).bt_tech = 3 as i32;
    (*parm).tol_int = 1e-5f64;
    (*parm).tol_obj = 1e-7f64;
    (*parm).tm_lim = 2147483647 as i32;
    (*parm).out_frq = 5000 as i32;
    (*parm).out_dly = 10000 as i32;
    (*parm).cb_func = None;
    (*parm).cb_info = 0 as *mut libc::c_void;
    (*parm).cb_size = 0 as i32;
    (*parm).pp_tech = 2 as i32;
    (*parm).mip_gap = 0.0f64;
    (*parm).mir_cuts = 0 as i32;
    (*parm).gmi_cuts = 0 as i32;
    (*parm).cov_cuts = 0 as i32;
    (*parm).clq_cuts = 0 as i32;
    (*parm).presolve = 0 as i32;
    (*parm).binarize = 0 as i32;
    (*parm).fp_heur = 0 as i32;
    (*parm).ps_heur = 0 as i32;
    (*parm).ps_tm_lim = 60000 as i32;
    (*parm).sr_heur = 1 as i32;
    (*parm).use_sol = 0 as i32;
    (*parm).save_sol = 0 as *const i8;
    (*parm).alien = 0 as i32;
    (*parm).flip = 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_mip_status(mut mip: *mut glp_prob) -> i32 {
    let mut mip_stat: i32 = (*mip).mip_stat;
    return mip_stat;
}
#[no_mangle]
pub unsafe extern "C" fn glp_mip_obj_val(mut mip: *mut glp_prob) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    z = (*mip).mip_obj;
    return z;
}
#[no_mangle]
pub unsafe extern "C" fn glp_mip_row_val(
    mut mip: *mut glp_prob,
    mut i: i32,
) -> libc::c_double {
    let mut mipx: libc::c_double = 0.;
    if !(1 as i32 <= i && i <= (*mip).m) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 763 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mip_row_val: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    mipx = (**((*mip).row).offset(i as isize)).mipx;
    return mipx;
}
#[no_mangle]
pub unsafe extern "C" fn glp_mip_col_val(
    mut mip: *mut glp_prob,
    mut j: i32,
) -> libc::c_double {
    let mut mipx: libc::c_double = 0.;
    if !(1 as i32 <= j && j <= (*mip).n) {
        (glp_error_(b"draft/glpapi09.c\0" as *const u8 as *const i8, 788 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mip_col_val: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    mipx = (**((*mip).col).offset(j as isize)).mipx;
    return mipx;
}