use std::os::raw::{c_int, c_double, c_char, c_void};
use std::ptr;
use std::ffi::CString;
use std::mem;
use std::cmp::Ordering;
use std::fmt;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GLPCOL {
    pub j: c_int,
    pub name: *mut c_char,
    pub node: *mut AVLNODE,
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub struct GLPROW {
    pub i: c_int,
    pub name: *mut c_char,
    pub node: *mut AVLNODE,
    pub level: c_int,
    pub origin: u8,
    pub klass: u8,
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
#[derive(Debug, Clone, Copy)]
pub struct glp_prob {
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
#[derive(Debug, Clone, Copy)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct glp_tree {
    pub magic: c_int,
    pub pool: *mut DMP,
    pub n: c_int,
    pub orig_m: c_int,
    pub orig_type: *mut u8,
    pub orig_lb: *mut c_double,
    pub orig_ub: *mut c_double,
    pub orig_stat: *mut u8,
    pub orig_prim: *mut c_double,
    pub orig_dual: *mut c_double,
    pub orig_obj: c_double,
    pub nslots: c_int,
    pub avail: c_int,
    pub slot: *mut IOSLOT,
    pub head: *mut IOSNPD,
    pub tail: *mut IOSNPD,
    pub a_cnt: c_int,
    pub n_cnt: c_int,
    pub t_cnt: c_int,
    pub root_m: c_int,
    pub root_type: *mut u8,
    pub root_lb: *mut c_double,
    pub root_ub: *mut c_double,
    pub root_stat: *mut u8,
    pub curr: *mut IOSNPD,
    pub mip: *mut glp_prob,
    pub non_int: *mut u8,
    pub pred_m: c_int,
    pub pred_max: c_int,
    pub pred_type: *mut u8,
    pub pred_lb: *mut c_double,
    pub pred_ub: *mut c_double,
    pub pred_stat: *mut u8,
    pub local: *mut IOSPOOL,
    pub cov_gen: *mut glp_cov,
    pub mir_gen: *mut glp_mir,
    pub clq_gen: *mut glp_cfg,
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
#[derive(Debug, Clone, Copy)]
pub struct glp_bfcp {
    pub msg_lev: c_int,
    pub type_: c_int,
    pub lu_size: c_int,
    pub piv_tol: c_double,
    pub piv_lim: c_int,
    pub suhl: c_int,
    pub eps_tol: c_double,
    pub max_gro: c_double,
    pub nfs_max: c_int,
    pub upd_tol: c_double,
    pub nrs_max: c_int,
    pub rs_size: c_int,
    pub foo_bar: [c_double; 38],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct glp_smcp {
    pub msg_lev: c_int,
    pub meth: c_int,
    pub pricing: c_int,
    pub r_test: c_int,
    pub tol_bnd: c_double,
    pub tol_dj: c_double,
    pub tol_piv: c_double,
    pub obj_ll: c_double,
    pub obj_ul: c_double,
    pub it_lim: c_int,
    pub tm_lim: c_int,
    pub out_frq: c_int,
    pub out_dly: c_int,
    pub presolve: c_int,
    pub excl: c_int,
    pub shift: c_int,
    pub aorn: c_int,
    pub foo_bar: [c_double; 33],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct glp_prep {
    pub orig_dir: c_int,
    pub orig_m: c_int,
    pub orig_n: c_int,
    pub orig_nnz: c_int,
    pub pool: *mut DMP,
    pub name: *mut c_char,
    pub obj: *mut c_char,
    pub c0: c_double,
    pub nrows: c_int,
    pub ncols: c_int,
    pub r_head: *mut NPPROW,
    pub r_tail: *mut NPPROW,
    pub c_head: *mut NPPCOL,
    pub c_tail: *mut NPPCOL,
    pub stack: *mut DMP,
    pub top: *mut NPPTSE,
    pub m: c_int,
    pub n: c_int,
    pub nnz: c_int,
    pub row_ref: *mut c_int,
    pub col_ref: *mut c_int,
    pub sol: c_int,
    pub scaling: c_int,
    pub p_stat: c_int,
    pub d_stat: c_int,
    pub t_stat: c_int,
    pub i_stat: c_int,
    pub r_stat: *mut c_char,
    pub c_stat: *mut c_char,
    pub r_pi: *mut c_double,
    pub c_value: *mut c_double,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NPPTSE {
    pub func: Option<unsafe extern "C" fn(*mut NPP, *mut c_void) -> c_int>,
    pub info: *mut c_void,
    pub link: *mut NPPTSE,
}

pub type NPP = glp_prep;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NPPCOL {
    pub j: c_int,
    pub name: *mut c_char,
    pub is_int: i8,
    pub lb: c_double,
    pub ub: c_double,
    pub coef: c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: c_int,
    pub ll: C2RustUnnamed_0,
    pub uu: C2RustUnnamed,
    pub prev: *mut NPPCOL,
    pub next: *mut NPPCOL,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union C2RustUnnamed {
    pub uu: c_double,
    pub neg: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union C2RustUnnamed_0 {
    pub ll: c_double,
    pub pos: c_int,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NPPAIJ {
    pub row: *mut NPPROW,
    pub col: *mut NPPCOL,
    pub val: c_double,
    pub r_prev: *mut NPPAIJ,
    pub r_next: *mut NPPAIJ,
    pub c_prev: *mut NPPAIJ,
    pub c_next: *mut NPPAIJ,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NPPROW {
    pub i: c_int,
    pub name: *mut c_char,
    pub lb: c_double,
    pub ub: c_double,
    pub ptr: *mut NPPAIJ,
    pub temp: c_int,
    pub prev: *mut NPPROW,
    pub next: *mut NPPROW,
}

pub type glp_errfunc = Option<unsafe extern "C" fn(*const c_char, ...)>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ENV {
    pub self_: *mut ENV,
    pub term_buf: *mut c_char,
    pub term_out: c_int,
    pub term_hook: Option<unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int>,
    pub term_info: *mut c_void,
    pub tee_file: *mut FILE,
    pub err_st: c_int,
    pub err_file: *const c_char,
    pub err_line: c_int,
    pub err_hook: Option<unsafe extern "C" fn(*mut c_void)>,
    pub err_info: *mut c_void,
    pub err_buf: *mut c_char,
    pub mem_limit: usize,
    pub mem_ptr: *mut MBD,
    pub mem_count: c_int,
    pub mem_cpeak: c_int,
    pub mem_total: usize,
    pub mem_tpeak: usize,
    pub gmp_pool: *mut c_void,
    pub gmp_size: c_int,
    pub gmp_work: *mut u16,
    pub h_odbc: *mut c_void,
    pub h_mysql: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MBD {
    pub size: usize,
    pub self_: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
}

pub type FILE = _IO_FILE;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct _IO_FILE {
    pub _flags: c_int,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: c_int,
    pub _flags2: c_int,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [c_char; 1],
    pub _lock: *mut c_void,
    pub _offset: i64,
    pub __pad1: *mut c_void,
    pub __pad2: *mut c_void,
    pub __pad3: *mut c_void,
    pub __pad4: *mut c_void,
    pub __pad5: usize,
    pub _mode: c_int,
    pub _unused2: [c_char; 20],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn glp_set_col_kind(mip: *mut glp_prob, j: c_int, kind: c_int) {
    let col = (*mip).col.offset(j as isize);
    match kind {
        1 => (*col).kind = 1,
        2 => (*col).kind = 2,
        3 => {
            (*col).kind = 2;
            if !((*col).type_ == 4 && (*col).lb == 0.0 && (*col).ub == 1.0) {
                glp_set_col_bnds(mip, j, 4, 0.0, 1.0);
            }
        }
        _ => {
            let msg = format!("glp_set_col_kind: j = {}; kind = {}; invalid column kind\n", j, kind);
            let msg_c = CString::new(msg).unwrap();
            glp_error_(msg_c.as_ptr(), 64);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn glp_get_col_kind(mip: *mut glp_prob, j: c_int) -> c_int {
    let col = (*mip).col.offset(j as isize);
    let kind = (*col).kind;
    match kind {
        1 => kind,
        2 => {
            if (*col).type_ == 4 && (*col).lb == 0.0 && (*col).ub == 1.0 {
                3
            } else {
                kind
            }
        }
        _ => {
            let msg = format!("glp_get_col_kind: j = {}; column number out of range\n", j);
            let msg_c = CString::new(msg).unwrap();
            glp_error_(msg_c.as_ptr(), 92);
            kind
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn glp_get_num_int(mip: *mut glp_prob) -> c_int {
    let mut count = 0;
    for j in 1..=(*mip).n {
        let col = (*mip).col.offset(j as isize);
        if (*col).kind == 2 {
            count += 1;
        }
    }
    count
}

#[no_mangle]
pub unsafe extern "C" fn glp_get_num_bin(mip: *mut glp_prob) -> c_int {
    let mut count = 0;
    for j in 1..=(*mip).n {
        let col = (*mip).col.offset(j as isize);
        if (*col).kind == 2 && (*col).type_ == 4 && (*col).lb == 0.0 && (*col).ub == 1.0 {
            count += 1;
        }
    }
    count
}

#[no_mangle]
pub unsafe extern "