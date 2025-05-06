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
    pub type DMP;
    pub type glp_tree;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_npp_activate_row(npp: *mut NPP, row: *mut NPPROW);
    fn _glp_npp_deactivate_row(npp: *mut NPP, row: *mut NPPROW);
    fn _glp_npp_activate_col(npp: *mut NPP, col: *mut NPPCOL);
    fn _glp_npp_deactivate_col(npp: *mut NPP, col: *mut NPPCOL);
    fn _glp_npp_hidden_covering(npp: *mut NPP, row: *mut NPPROW) -> i32;
    fn _glp_npp_fixed_col(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_make_equality(npp: *mut NPP, p: *mut NPPROW) -> i32;
    fn _glp_npp_make_fixed(npp: *mut NPP, q: *mut NPPCOL) -> i32;
    fn _glp_npp_empty_row(npp: *mut NPP, p: *mut NPPROW) -> i32;
    fn _glp_npp_empty_col(npp: *mut NPP, q: *mut NPPCOL) -> i32;
    fn _glp_npp_eq_singlet(npp: *mut NPP, p: *mut NPPROW) -> i32;
    fn _glp_npp_implied_lower(npp: *mut NPP, q: *mut NPPCOL, l: libc::c_double) -> i32;
    fn _glp_npp_implied_upper(npp: *mut NPP, q: *mut NPPCOL, u: libc::c_double) -> i32;
    fn _glp_npp_ineq_singlet(npp: *mut NPP, p: *mut NPPROW) -> i32;
    fn _glp_npp_implied_slack(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_npp_implied_free(npp: *mut NPP, q: *mut NPPCOL) -> i32;
    fn _glp_npp_forcing_row(npp: *mut NPP, p: *mut NPPROW, at: i32) -> i32;
    fn _glp_npp_analyze_row(npp: *mut NPP, p: *mut NPPROW) -> i32;
    fn _glp_npp_inactive_bound(npp: *mut NPP, p: *mut NPPROW, which: i32);
    fn _glp_npp_implied_bounds(npp: *mut NPP, p: *mut NPPROW);
    fn _glp_npp_binarize_prob(npp: *mut NPP) -> i32;
    fn _glp_npp_hidden_packing(npp: *mut NPP, row: *mut NPPROW) -> i32;
    fn _glp_npp_free_row(npp: *mut NPP, p: *mut NPPROW);
    fn _glp_npp_reduce_ineq_coef(npp: *mut NPP, row: *mut NPPROW) -> i32;
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
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_clean_prob(mut npp: *mut NPP) {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut next_row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut next_col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut ret: i32 = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp5.c\0" as *const u8 as *const i8,
                59 as i32,
            );
            1 as i32 != 0
        }) as i32;
    row = (*npp).r_head;
    while !row.is_null() {
        next_row = (*row).next;
        if (*row).lb == -1.7976931348623157e+308f64
            && (*row).ub == 1.7976931348623157e+308f64
        {
            _glp_npp_free_row(npp, row);
        }
        row = next_row;
    }
    row = (*npp).r_head;
    while !row.is_null() {
        next_row = (*row).next;
        if (*row).lb != -1.7976931348623157e+308f64
            && (*row).ub != 1.7976931348623157e+308f64 && (*row).lb < (*row).ub
        {
            ret = _glp_npp_make_equality(npp, row);
            if !(ret == 0 as i32) {
                if !(ret == 1 as i32) {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const i8,
                                b"npp/npp5.c\0" as *const u8 as *const i8,
                                87 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
        }
        row = next_row;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        next_col = (*col).next;
        if (*col).lb == (*col).ub {
            _glp_npp_fixed_col(npp, col);
        }
        col = next_col;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        next_col = (*col).next;
        if (*col).lb != -1.7976931348623157e+308f64
            && (*col).ub != 1.7976931348623157e+308f64 && (*col).lb < (*col).ub
        {
            ret = _glp_npp_make_fixed(npp, col);
            if !(ret == 0 as i32) {
                if ret == 1 as i32 {
                    _glp_npp_fixed_col(npp, col);
                }
            }
        }
        col = next_col;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_process_row(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut hard: i32,
) -> i32 {
    let mut current_block: u64;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut next_aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut aaa: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ret: i32 = 0;
    (!((*row).lb == -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64)
        || {
            glp_assert_(
                b"!(row->lb == -DBL_MAX && row->ub == +DBL_MAX)\0" as *const u8
                    as *const i8,
                b"npp/npp5.c\0" as *const u8 as *const i8,
                175 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if ((*row).ptr).is_null() {
        ret = _glp_npp_empty_row(npp, row);
        if ret == 0 as i32 {
            return 0 as i32
        } else if ret == 1 as i32 {
            return 0xa as i32
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const i8,
                        b"npp/npp5.c\0" as *const u8 as *const i8,
                        192 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    if ((*(*row).ptr).r_next).is_null() {
        col = (*(*row).ptr).col;
        if (*row).lb == (*row).ub {
            ret = _glp_npp_eq_singlet(npp, row);
            if ret == 0 as i32 {
                aij = (*col).ptr;
                while !aij.is_null() {
                    _glp_npp_activate_row(npp, (*aij).row);
                    aij = (*aij).c_next;
                }
                _glp_npp_fixed_col(npp, col);
                return 0 as i32;
            } else if ret == 1 as i32 || ret == 2 as i32 {
                return 0xa as i32
            } else {
                (ret != ret
                    || {
                        glp_assert_(
                            b"ret != ret\0" as *const u8 as *const i8,
                            b"npp/npp5.c\0" as *const u8 as *const i8,
                            218 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        } else {
            ret = _glp_npp_ineq_singlet(npp, row);
            if 0 as i32 <= ret && ret <= 3 as i32 {
                _glp_npp_activate_col(npp, col);
                if ret >= 2 as i32 {
                    aij = (*col).ptr;
                    while !aij.is_null() {
                        _glp_npp_activate_row(npp, (*aij).row);
                        aij = (*aij).c_next;
                    }
                }
                if ret == 3 as i32 {
                    _glp_npp_fixed_col(npp, col);
                }
                return 0 as i32;
            } else if ret == 4 as i32 {
                return 0xa as i32
            } else {
                (ret != ret
                    || {
                        glp_assert_(
                            b"ret != ret\0" as *const u8 as *const i8,
                            b"npp/npp5.c\0" as *const u8 as *const i8,
                            253 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
    }
    ret = _glp_npp_analyze_row(npp, row);
    (0 as i32 <= ret && ret <= 0xff as i32
        || {
            glp_assert_(
                b"0x00 <= ret && ret <= 0xFF\0" as *const u8 as *const i8,
                b"npp/npp5.c\0" as *const u8 as *const i8,
                289 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if ret == 0x33 as i32 {
        return 0xa as i32;
    }
    if ret & 0xf as i32 == 0 as i32 {
        if (*row).lb != -1.7976931348623157e+308f64 {
            _glp_npp_inactive_bound(npp, row, 0 as i32);
        }
        current_block = 5159818223158340697;
    } else if ret & 0xf as i32 == 0x1 as i32 {
        current_block = 5159818223158340697;
    } else if ret & 0xf as i32 == 0x2 as i32 {
        if _glp_npp_forcing_row(npp, row, 0 as i32) == 0 as i32 {
            current_block = 7409962125402517174;
        } else {
            current_block = 5159818223158340697;
        }
    } else {
        (ret != ret
            || {
                glp_assert_(
                    b"ret != ret\0" as *const u8 as *const i8,
                    b"npp/npp5.c\0" as *const u8 as *const i8,
                    338 as i32,
                );
                1 as i32 != 0
            }) as i32;
        current_block = 5159818223158340697;
    }
    match current_block {
        5159818223158340697 => {
            if ret & 0xf0 as i32 == 0 as i32 {
                if (*row).ub != 1.7976931348623157e+308f64 {
                    _glp_npp_inactive_bound(npp, row, 1 as i32);
                }
                current_block = 15669289850109000831;
            } else if ret & 0xf0 as i32 == 0x10 as i32 {
                current_block = 15669289850109000831;
            } else if ret & 0xf0 as i32 == 0x20 as i32 {
                if _glp_npp_forcing_row(npp, row, 1 as i32) == 0 as i32 {
                    current_block = 7409962125402517174;
                } else {
                    current_block = 15669289850109000831;
                }
            } else {
                (ret != ret
                    || {
                        glp_assert_(
                            b"ret != ret\0" as *const u8 as *const i8,
                            b"npp/npp5.c\0" as *const u8 as *const i8,
                            362 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                current_block = 15669289850109000831;
            }
            match current_block {
                7409962125402517174 => {}
                _ => {
                    if (*row).lb == -1.7976931348623157e+308f64
                        && (*row).ub == 1.7976931348623157e+308f64
                    {
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            _glp_npp_activate_col(npp, (*aij).col);
                            aij = (*aij).r_next;
                        }
                        _glp_npp_free_row(npp, row);
                        return 0 as i32;
                    }
                    if (*npp).sol == 3 as i32 && hard != 0 {
                        if _glp_npp_improve_bounds(npp, row, 1 as i32) < 0 as i32 {
                            return 0xa as i32;
                        }
                    }
                    return 0 as i32;
                }
            }
        }
        _ => {}
    }
    aij = (*row).ptr;
    while !aij.is_null() {
        col = (*aij).col;
        next_aij = (*aij).r_next;
        aaa = (*col).ptr;
        while !aaa.is_null() {
            _glp_npp_activate_row(npp, (*aaa).row);
            aaa = (*aaa).c_next;
        }
        _glp_npp_fixed_col(npp, col);
        aij = next_aij;
    }
    _glp_npp_free_row(npp, row);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_improve_bounds(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut flag: i32,
) -> i32 {
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut next_aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut aaa: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut kase: i32 = 0;
    let mut ret: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    ((*npp).sol == 3 as i32
        || {
            glp_assert_(
                b"npp->sol == GLP_MIP\0" as *const u8 as *const i8,
                b"npp/npp5.c\0" as *const u8 as *const i8,
                424 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!((*row).lb == -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64)
        || {
            glp_assert_(
                b"!(row->lb == -DBL_MAX && row->ub == +DBL_MAX)\0" as *const u8
                    as *const i8,
                b"npp/npp5.c\0" as *const u8 as *const i8,
                426 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_npp_implied_bounds(npp, row);
    aij = (*row).ptr;
    while !aij.is_null() {
        col = (*aij).col;
        next_aij = (*aij).r_next;
        let mut current_block_24: u64;
        kase = 0 as i32;
        while kase <= 1 as i32 {
            lb = (*col).lb;
            ub = (*col).ub;
            if kase == 0 as i32 {
                if (*col).ll.ll == -1.7976931348623157e+308f64 {
                    current_block_24 = 10879442775620481940;
                } else {
                    ret = _glp_npp_implied_lower(npp, col, (*col).ll.ll);
                    current_block_24 = 3512920355445576850;
                }
            } else if (*col).uu.uu == 1.7976931348623157e+308f64 {
                current_block_24 = 10879442775620481940;
            } else {
                ret = _glp_npp_implied_upper(npp, col, (*col).uu.uu);
                current_block_24 = 3512920355445576850;
            }
            match current_block_24 {
                3512920355445576850 => {
                    if ret == 0 as i32 || ret == 1 as i32 {
                        (*col).lb = lb;
                        (*col).ub = ub;
                    } else if ret == 2 as i32 || ret == 3 as i32 {
                        count += 1;
                        count;
                        if flag != 0 {
                            aaa = (*col).ptr;
                            while !aaa.is_null() {
                                if (*aaa).row != row {
                                    _glp_npp_activate_row(npp, (*aaa).row);
                                }
                                aaa = (*aaa).c_next;
                            }
                        }
                        if ret == 3 as i32 {
                            _glp_npp_fixed_col(npp, col);
                            break;
                        }
                    } else if ret == 4 as i32 {
                        return -(1 as i32)
                    } else {
                        (ret != ret
                            || {
                                glp_assert_(
                                    b"ret != ret\0" as *const u8 as *const i8,
                                    b"npp/npp5.c\0" as *const u8 as *const i8,
                                    480 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                }
                _ => {}
            }
            kase += 1;
            kase;
        }
        aij = next_aij;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_process_col(
    mut npp: *mut NPP,
    mut col: *mut NPPCOL,
) -> i32 {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ret: i32 = 0;
    ((*col).lb < (*col).ub
        || {
            glp_assert_(
                b"col->lb < col->ub\0" as *const u8 as *const i8,
                b"npp/npp5.c\0" as *const u8 as *const i8,
                526 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if ((*col).ptr).is_null() {
        ret = _glp_npp_empty_col(npp, col);
        if ret == 0 as i32 {
            return 0 as i32
        } else if ret == 1 as i32 {
            return 0xb as i32
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const i8,
                        b"npp/npp5.c\0" as *const u8 as *const i8,
                        543 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    if ((*(*col).ptr).c_next).is_null() {
        row = (*(*col).ptr).row;
        let mut current_block_28: u64;
        if (*row).lb == (*row).ub {
            if (*col).is_int == 0 {
                current_block_28 = 12156029949716856391;
            } else {
                current_block_28 = 4775909272756257391;
            }
        } else if (*col).is_int == 0 {
            ret = _glp_npp_implied_free(npp, col);
            if ret == 0 as i32 {
                current_block_28 = 12156029949716856391;
            } else {
                if !(ret == 1 as i32) {
                    if ret == 2 as i32 {
                        return 0xb as i32;
                    }
                }
                current_block_28 = 4775909272756257391;
            }
        } else {
            current_block_28 = 4775909272756257391;
        }
        match current_block_28 {
            12156029949716856391 => {
                _glp_npp_implied_slack(npp, col);
                if (*row).lb == -1.7976931348623157e+308f64
                    && (*row).ub == 1.7976931348623157e+308f64
                {
                    aij = (*row).ptr;
                    while !aij.is_null() {
                        _glp_npp_activate_col(npp, (*aij).col);
                        aij = (*aij).r_next;
                    }
                    _glp_npp_free_row(npp, row);
                } else {
                    _glp_npp_activate_row(npp, row);
                }
                return 0 as i32;
            }
            _ => {}
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_process_prob(mut npp: *mut NPP, mut hard: i32) -> i32 {
    let mut current_block: u64;
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut processing: i32 = 0;
    let mut ret: i32 = 0;
    _glp_npp_clean_prob(npp);
    row = (*npp).r_head;
    while !row.is_null() {
        (*row).temp = 1 as i32;
        row = (*row).next;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        (*col).temp = 1 as i32;
        col = (*col).next;
    }
    processing = 1 as i32;
    's_41: loop {
        if !(processing != 0) {
            current_block = 1109700713171191020;
            break;
        }
        processing = 0 as i32;
        loop {
            row = (*npp).r_head;
            if row.is_null() || (*row).temp == 0 {
                break;
            }
            _glp_npp_deactivate_row(npp, row);
            ret = _glp_npp_process_row(npp, row, hard);
            if ret != 0 as i32 {
                current_block = 5180305035808669715;
                break 's_41;
            }
            processing = 1 as i32;
        }
        loop {
            col = (*npp).c_head;
            if col.is_null() || (*col).temp == 0 {
                break;
            }
            _glp_npp_deactivate_col(npp, col);
            ret = _glp_npp_process_col(npp, col);
            if ret != 0 as i32 {
                current_block = 5180305035808669715;
                break 's_41;
            }
            processing = 1 as i32;
        }
    }
    match current_block {
        1109700713171191020 => {
            if (*npp).sol == 3 as i32 && hard == 0 {
                row = (*npp).r_head;
                loop {
                    if row.is_null() {
                        current_block = 14359455889292382949;
                        break;
                    }
                    if _glp_npp_improve_bounds(npp, row, 0 as i32) < 0 as i32 {
                        ret = 0xa as i32;
                        current_block = 5180305035808669715;
                        break;
                    } else {
                        row = (*row).next;
                    }
                }
            } else {
                current_block = 14359455889292382949;
            }
            match current_block {
                5180305035808669715 => {}
                _ => {
                    ret = 0 as i32;
                }
            }
        }
        _ => {}
    }
    (ret == 0 as i32 || ret == 0xa as i32 || ret == 0xb as i32
        || {
            glp_assert_(
                b"ret == 0 || ret == GLP_ENOPFS || ret == GLP_ENODFS\0" as *const u8
                    as *const i8,
                b"npp/npp5.c\0" as *const u8 as *const i8,
                697 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_simplex(
    mut npp: *mut NPP,
    mut parm: *const glp_smcp,
) -> i32 {
    let mut ret: i32 = 0;
    ((*npp).sol == 1 as i32
        || {
            glp_assert_(
                b"npp->sol == GLP_SOL\0" as *const u8 as *const i8,
                b"npp/npp5.c\0" as *const u8 as *const i8,
                709 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (parm == parm
        || {
            glp_assert_(
                b"parm == parm\0" as *const u8 as *const i8,
                b"npp/npp5.c\0" as *const u8 as *const i8,
                710 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ret = _glp_npp_process_prob(npp, 0 as i32);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_integer(
    mut npp: *mut NPP,
    mut parm: *const glp_iocp,
) -> i32 {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut prev_row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut count: i32 = 0;
    let mut ret: i32 = 0;
    ((*npp).sol == 3 as i32
        || {
            glp_assert_(
                b"npp->sol == GLP_MIP\0" as *const u8 as *const i8,
                b"npp/npp5.c\0" as *const u8 as *const i8,
                723 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (parm == parm
        || {
            glp_assert_(
                b"parm == parm\0" as *const u8 as *const i8,
                b"npp/npp5.c\0" as *const u8 as *const i8,
                724 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ret = _glp_npp_process_prob(npp, 1 as i32);
    if !(ret != 0 as i32) {
        if (*parm).binarize != 0 {
            _glp_npp_binarize_prob(npp);
        }
        count = 0 as i32;
        row = (*npp).r_tail;
        while !row.is_null() {
            prev_row = (*row).prev;
            if !((*row).lb == -1.7976931348623157e+308f64
                && (*row).ub == 1.7976931348623157e+308f64)
            {
                if !((*row).lb == (*row).ub) {
                    if !(((*row).ptr).is_null() || ((*(*row).ptr).r_next).is_null()) {
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            col = (*aij).col;
                            if !((*col).is_int as i32 != 0 && (*col).lb == 0.0f64
                                && (*col).ub == 1.0f64)
                            {
                                break;
                            }
                            aij = (*aij).r_next;
                        }
                        if aij.is_null() {
                            count += _glp_npp_hidden_packing(npp, row);
                        }
                    }
                }
            }
            row = prev_row;
        }
        if count > 0 as i32 {
            glp_printf(
                b"%d hidden packing inequaliti(es) were detected\n\0" as *const u8
                    as *const i8,
                count,
            );
        }
        count = 0 as i32;
        row = (*npp).r_tail;
        while !row.is_null() {
            prev_row = (*row).prev;
            if !((*row).lb == -1.7976931348623157e+308f64
                && (*row).ub == 1.7976931348623157e+308f64)
            {
                if !((*row).lb == (*row).ub) {
                    if !(((*row).ptr).is_null() || ((*(*row).ptr).r_next).is_null()
                        || ((*(*(*row).ptr).r_next).r_next).is_null())
                    {
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            col = (*aij).col;
                            if !((*col).is_int as i32 != 0 && (*col).lb == 0.0f64
                                && (*col).ub == 1.0f64)
                            {
                                break;
                            }
                            aij = (*aij).r_next;
                        }
                        if aij.is_null() {
                            count += _glp_npp_hidden_covering(npp, row);
                        }
                    }
                }
            }
            row = prev_row;
        }
        if count > 0 as i32 {
            glp_printf(
                b"%d hidden covering inequaliti(es) were detected\n\0" as *const u8
                    as *const i8,
                count,
            );
        }
        count = 0 as i32;
        row = (*npp).r_tail;
        while !row.is_null() {
            prev_row = (*row).prev;
            if !((*row).lb == (*row).ub) {
                count += _glp_npp_reduce_ineq_coef(npp, row);
            }
            row = prev_row;
        }
        if count > 0 as i32 {
            glp_printf(
                b"%d constraint coefficient(s) were reduced\n\0" as *const u8
                    as *const i8,
                count,
            );
        }
        ret = 0 as i32;
    }
    return ret;
}