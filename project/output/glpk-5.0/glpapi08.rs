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
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn _glp_get_env_ptr() -> *mut ENV;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_get_mat_col(
        P: *mut glp_prob,
        j: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn glp_scale_prob(P: *mut glp_prob, flags: i32);
    fn _glp_ipm_solve(P: *mut glp_prob, parm: *const glp_iptcp) -> i32;
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
    fn _glp_npp_free_row(npp: *mut NPP, p: *mut NPPROW);
    fn _glp_npp_geq_row(npp: *mut NPP, p: *mut NPPROW);
    fn _glp_npp_leq_row(npp: *mut NPP, p: *mut NPPROW);
    fn _glp_npp_free_col(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_lbnd_col(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_ubnd_col(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_dbnd_col(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_fixed_col(npp: *mut NPP, q: *mut NPPCOL);
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
pub struct glp_iptcp {
    pub msg_lev: i32,
    pub ord_alg: i32,
    pub foo_bar: [libc::c_double; 48],
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
unsafe extern "C" fn transform(mut npp: *mut NPP) {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut prev_row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut prev_col: *mut NPPCOL = 0 as *mut NPPCOL;
    row = (*npp).r_tail;
    while !row.is_null() {
        prev_row = (*row).prev;
        if (*row).lb == -1.7976931348623157e+308f64
            && (*row).ub == 1.7976931348623157e+308f64
        {
            _glp_npp_free_row(npp, row);
        } else if (*row).lb == -1.7976931348623157e+308f64 {
            _glp_npp_leq_row(npp, row);
        } else if (*row).ub == 1.7976931348623157e+308f64 {
            _glp_npp_geq_row(npp, row);
        } else if (*row).lb != (*row).ub {
            if fabs((*row).lb) < fabs((*row).ub) {
                _glp_npp_geq_row(npp, row);
            } else {
                _glp_npp_leq_row(npp, row);
            }
        }
        row = prev_row;
    }
    col = (*npp).c_tail;
    while !col.is_null() {
        prev_col = (*col).prev;
        if (*col).lb == -1.7976931348623157e+308f64
            && (*col).ub == 1.7976931348623157e+308f64
        {
            _glp_npp_free_col(npp, col);
        } else if (*col).lb == -1.7976931348623157e+308f64 {
            _glp_npp_ubnd_col(npp, col);
        } else if (*col).ub == 1.7976931348623157e+308f64 {
            if (*col).lb != 0.0f64 {
                _glp_npp_lbnd_col(npp, col);
            }
        } else if (*col).lb != (*col).ub {
            if fabs((*col).lb) < fabs((*col).ub) {
                if (*col).lb != 0.0f64 {
                    _glp_npp_lbnd_col(npp, col);
                }
            } else {
                _glp_npp_ubnd_col(npp, col);
            }
            _glp_npp_dbnd_col(npp, col);
        } else {
            _glp_npp_fixed_col(npp, col);
        }
        col = prev_col;
    }
    row = (*npp).r_head;
    while !row.is_null() {
        ((*row).lb == (*row).ub
            || {
                glp_assert_(
                    b"row->lb == row->ub\0" as *const u8 as *const i8,
                    b"draft/glpapi08.c\0" as *const u8 as *const i8,
                    111 as i32,
                );
                1 as i32 != 0
            }) as i32;
        row = (*row).next;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        ((*col).lb == 0.0f64 && (*col).ub == 1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"col->lb == 0.0 && col->ub == +DBL_MAX\0" as *const u8 as *const i8,
                    b"draft/glpapi08.c\0" as *const u8 as *const i8,
                    113 as i32,
                );
                1 as i32 != 0
            }) as i32;
        col = (*col).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_interior(
    mut P: *mut glp_prob,
    mut parm: *const glp_iptcp,
) -> i32 {
    let mut current_block: u64;
    let mut _parm: glp_iptcp = glp_iptcp {
        msg_lev: 0,
        ord_alg: 0,
        foo_bar: [0.; 48],
    };
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut npp: *mut NPP = 0 as *mut NPP;
    let mut prob: *mut glp_prob = 0 as *mut glp_prob;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ret: i32 = 0;
    if parm.is_null() {
        glp_init_iptcp(&mut _parm);
        parm = &mut _parm;
    }
    if !((*parm).msg_lev == 0 as i32 || (*parm).msg_lev == 1 as i32
        || (*parm).msg_lev == 2 as i32 || (*parm).msg_lev == 3 as i32)
    {
        (glp_error_(b"draft/glpapi08.c\0" as *const u8 as *const i8, 131 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_interior: msg_lev = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).msg_lev,
        );
    }
    if !((*parm).ord_alg == 0 as i32 || (*parm).ord_alg == 1 as i32
        || (*parm).ord_alg == 2 as i32 || (*parm).ord_alg == 3 as i32)
    {
        (glp_error_(b"draft/glpapi08.c\0" as *const u8 as *const i8, 137 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_interior: ord_alg = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            (*parm).ord_alg,
        );
    }
    (*P).ipt_stat = 1 as i32;
    (*P).ipt_obj = 0.0f64;
    i = 1 as i32;
    loop {
        if !(i <= (*P).m) {
            current_block = 7976072742316086414;
            break;
        }
        row = *((*P).row).offset(i as isize);
        if (*row).type_0 == 4 as i32 && (*row).lb >= (*row).ub {
            if (*parm).msg_lev >= 1 as i32 {
                glp_printf(
                    b"glp_interior: row %d: lb = %g, ub = %g; incorrect bounds\n\0"
                        as *const u8 as *const i8,
                    i,
                    (*row).lb,
                    (*row).ub,
                );
            }
            ret = 0x4 as i32;
            current_block = 12728352836523215188;
            break;
        } else {
            i += 1;
            i;
        }
    }
    match current_block {
        7976072742316086414 => {
            j = 1 as i32;
            loop {
                if !(j <= (*P).n) {
                    current_block = 5783071609795492627;
                    break;
                }
                col = *((*P).col).offset(j as isize);
                if (*col).type_0 == 4 as i32 && (*col).lb >= (*col).ub {
                    if (*parm).msg_lev >= 1 as i32 {
                        glp_printf(
                            b"glp_interior: column %d: lb = %g, ub = %g; incorrect bounds\n\0"
                                as *const u8 as *const i8,
                            j,
                            (*col).lb,
                            (*col).ub,
                        );
                    }
                    ret = 0x4 as i32;
                    current_block = 12728352836523215188;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
            match current_block {
                12728352836523215188 => {}
                _ => {
                    if (*parm).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"Original LP has %d row(s), %d column(s), and %d non-zero(s)\n\0"
                                as *const u8 as *const i8,
                            (*P).m,
                            (*P).n,
                            (*P).nnz,
                        );
                    }
                    npp = _glp_npp_create_wksp();
                    _glp_npp_load_prob(npp, P, 0 as i32, 2 as i32, 1 as i32);
                    transform(npp);
                    prob = glp_create_prob();
                    _glp_npp_build_prob(npp, prob);
                    if (*parm).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"Working LP has %d row(s), %d column(s), and %d non-zero(s)\n\0"
                                as *const u8 as *const i8,
                            (*prob).m,
                            (*prob).n,
                            (*prob).nnz,
                        );
                    }
                    if !((*prob).m > 0 as i32 && (*prob).n > 0 as i32) {
                        if (*parm).msg_lev >= 1 as i32 {
                            glp_printf(
                                b"glp_interior: unable to solve empty problem\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                        ret = 0x5 as i32;
                    } else {
                        let mut env: *mut ENV = _glp_get_env_ptr();
                        let mut term_out: i32 = (*env).term_out;
                        (*env).term_out = 0 as i32;
                        glp_scale_prob(prob, 0x10 as i32);
                        (*env).term_out = term_out;
                        if (*parm).msg_lev >= 2 as i32 && (*prob).m >= 200 as i32 {
                            let mut len: i32 = 0;
                            let mut cnt: i32 = 0 as i32;
                            j = 1 as i32;
                            while j <= (*prob).n {
                                len = glp_get_mat_col(
                                    prob,
                                    j,
                                    0 as *mut i32,
                                    0 as *mut libc::c_double,
                                );
                                if len as libc::c_double
                                    >= 0.20f64 * (*prob).m as libc::c_double
                                {
                                    cnt += 1;
                                    cnt;
                                }
                                j += 1;
                                j;
                            }
                            if cnt == 1 as i32 {
                                glp_printf(
                                    b"WARNING: PROBLEM HAS ONE DENSE COLUMN\n\0" as *const u8
                                        as *const i8,
                                );
                            } else if cnt > 0 as i32 {
                                glp_printf(
                                    b"WARNING: PROBLEM HAS %d DENSE COLUMNS\n\0" as *const u8
                                        as *const i8,
                                    cnt,
                                );
                            }
                        }
                        ret = _glp_ipm_solve(prob, parm);
                        _glp_npp_postprocess(npp, prob);
                        _glp_npp_unload_sol(npp, P);
                    }
                }
            }
        }
        _ => {}
    }
    if !npp.is_null() {
        _glp_npp_delete_wksp(npp);
    }
    if !prob.is_null() {
        glp_delete_prob(prob);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_init_iptcp(mut parm: *mut glp_iptcp) {
    (*parm).msg_lev = 3 as i32;
    (*parm).ord_alg = 2 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ipt_status(mut lp: *mut glp_prob) -> i32 {
    let mut ipt_stat: i32 = (*lp).ipt_stat;
    return ipt_stat;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ipt_obj_val(mut lp: *mut glp_prob) -> libc::c_double {
    let mut z: libc::c_double = 0.;
    z = (*lp).ipt_obj;
    return z;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ipt_row_prim(
    mut lp: *mut glp_prob,
    mut i: i32,
) -> libc::c_double {
    let mut pval: libc::c_double = 0.;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"draft/glpapi08.c\0" as *const u8 as *const i8, 303 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ipt_row_prim: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    pval = (**((*lp).row).offset(i as isize)).pval;
    return pval;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ipt_row_dual(
    mut lp: *mut glp_prob,
    mut i: i32,
) -> libc::c_double {
    let mut dval: libc::c_double = 0.;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"draft/glpapi08.c\0" as *const u8 as *const i8, 328 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ipt_row_dual: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    dval = (**((*lp).row).offset(i as isize)).dval;
    return dval;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ipt_col_prim(
    mut lp: *mut glp_prob,
    mut j: i32,
) -> libc::c_double {
    let mut pval: libc::c_double = 0.;
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"draft/glpapi08.c\0" as *const u8 as *const i8, 353 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ipt_col_prim: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    pval = (**((*lp).col).offset(j as isize)).pval;
    return pval;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ipt_col_dual(
    mut lp: *mut glp_prob,
    mut j: i32,
) -> libc::c_double {
    let mut dval: libc::c_double = 0.;
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"draft/glpapi08.c\0" as *const u8 as *const i8, 378 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ipt_col_dual: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    dval = (**((*lp).col).offset(j as isize)).dval;
    return dval;
}