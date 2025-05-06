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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: i32);
    fn _glp_npp_del_aij(npp: *mut NPP, aij: *mut NPPAIJ);
    fn _glp_npp_del_col(npp: *mut NPP, col: *mut NPPCOL);
    fn _glp_npp_del_row(npp: *mut NPP, row: *mut NPPROW);
    fn _glp_npp_push_tse(
        npp: *mut NPP,
        func: Option<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32>,
        size: i32,
    ) -> *mut libc::c_void;
    fn _glp_npp_row_nnz(npp: *mut NPP, row: *mut NPPROW) -> i32;
    fn _glp_npp_add_aij(
        npp: *mut NPP,
        row: *mut NPPROW,
        col: *mut NPPCOL,
        val: libc::c_double,
    ) -> *mut NPPAIJ;
    fn _glp_npp_add_col(npp: *mut NPP) -> *mut NPPCOL;
    fn _glp_npp_add_row(npp: *mut NPP) -> *mut NPPROW;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sat_fixed_col {
    pub q: i32,
    pub s: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPLIT {
    pub col: *mut NPPCOL,
    pub neg: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPLSE {
    pub lit: NPPLIT,
    pub next: *mut NPPLSE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NPPSED {
    pub x: NPPLIT,
    pub y: NPPLIT,
    pub z: NPPLIT,
    pub s: *mut NPPCOL,
    pub c: *mut NPPCOL,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_free_row(mut npp: *mut NPP, mut p: *mut NPPROW) {
    ((*p).lb == -1.7976931348623157e+308f64 && (*p).ub == 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->lb == -DBL_MAX && p->ub == +DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                38 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_npp_del_row(npp, p);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_fixed_col(
    mut npp: *mut NPP,
    mut q: *mut NPPCOL,
) -> i32 {
    let mut info: *mut sat_fixed_col = 0 as *mut sat_fixed_col;
    let mut i: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut temp: i32 = 0;
    ((*q).lb == (*q).ub
        || {
            glp_assert_(
                b"q->lb == q->ub\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                97 as i32,
            );
            1 as i32 != 0
        }) as i32;
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_sat_fixed_col as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32,
        ),
        ::core::mem::size_of::<sat_fixed_col>() as u64 as i32,
    ) as *mut sat_fixed_col;
    (*info).q = (*q).j;
    (*info).s = (*q).lb as i32;
    ((*info).s as libc::c_double == (*q).lb
        || {
            glp_assert_(
                b"(double)info->s == q->lb\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                103 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if !((*info).s == 0 as i32) {
        aij = (*q).ptr;
        while !aij.is_null() {
            i = (*aij).row;
            if (*i).lb != -1.7976931348623157e+308f64 {
                (*i).lb -= (*aij).val * (*info).s as libc::c_double;
                temp = (*i).lb as i32;
                if temp as libc::c_double != (*i).lb {
                    return 1 as i32;
                }
            }
            if (*i).ub != 1.7976931348623157e+308f64 {
                (*i).ub -= (*aij).val * (*info).s as libc::c_double;
                temp = (*i).ub as i32;
                if temp as libc::c_double != (*i).ub {
                    return 2 as i32;
                }
            }
            aij = (*aij).c_next;
        }
    }
    _glp_npp_del_col(npp, q);
    return 0 as i32;
}
unsafe extern "C" fn rcv_sat_fixed_col(
    mut npp: *mut NPP,
    mut info_: *mut libc::c_void,
) -> i32 {
    let mut info: *mut sat_fixed_col = info_ as *mut sat_fixed_col;
    *((*npp).c_value).offset((*info).q as isize) = (*info).s as libc::c_double;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_is_bin_comb(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                144 as i32,
            );
            1 as i32 != 0
        }) as i32;
    aij = (*row).ptr;
    while !aij.is_null() {
        if !((*aij).val == 1.0f64 || (*aij).val == -1.0f64) {
            return 0 as i32;
        }
        col = (*aij).col;
        if !((*col).is_int as i32 != 0 && (*col).lb == 0.0f64 && (*col).ub == 1.0f64) {
            return 0 as i32;
        }
        aij = (*aij).r_next;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_num_pos_coef(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut num: i32 = 0 as i32;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                164 as i32,
            );
            1 as i32 != 0
        }) as i32;
    aij = (*row).ptr;
    while !aij.is_null() {
        if (*aij).val > 0.0f64 {
            num += 1;
            num;
        }
        aij = (*aij).r_next;
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_num_neg_coef(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut num: i32 = 0 as i32;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                181 as i32,
            );
            1 as i32 != 0
        }) as i32;
    aij = (*row).ptr;
    while !aij.is_null() {
        if (*aij).val < 0.0f64 {
            num += 1;
            num;
        }
        aij = (*aij).r_next;
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_is_cover_ineq(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                223 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*row).lb != -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64
    {
        if _glp_npp_sat_is_bin_comb(npp, row) != 0 {
            if (*row).lb
                == 1.0f64 - _glp_npp_sat_num_neg_coef(npp, row) as libc::c_double
            {
                return 1 as i32;
            }
        }
    } else if (*row).lb == -1.7976931348623157e+308f64
        && (*row).ub != 1.7976931348623157e+308f64
    {
        if _glp_npp_sat_is_bin_comb(npp, row) != 0 {
            if (*row).ub
                == _glp_npp_sat_num_pos_coef(npp, row) as libc::c_double - 1.0f64
            {
                return 2 as i32;
            }
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_is_pack_ineq(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                282 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*row).lb == -1.7976931348623157e+308f64
        && (*row).ub != 1.7976931348623157e+308f64
    {
        if _glp_npp_sat_is_bin_comb(npp, row) != 0 {
            if (*row).ub
                == 1.0f64 - _glp_npp_sat_num_neg_coef(npp, row) as libc::c_double
            {
                return 1 as i32;
            }
        }
    } else if (*row).lb != -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64
    {
        if _glp_npp_sat_is_bin_comb(npp, row) != 0 {
            if (*row).lb
                == _glp_npp_sat_num_pos_coef(npp, row) as libc::c_double - 1.0f64
            {
                return 2 as i32;
            }
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_is_partn_eq(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                341 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*row).lb == (*row).ub {
        if _glp_npp_sat_is_bin_comb(npp, row) != 0 {
            if (*row).lb
                == 1.0f64 - _glp_npp_sat_num_neg_coef(npp, row) as libc::c_double
            {
                return 1 as i32;
            }
            if (*row).ub
                == _glp_npp_sat_num_pos_coef(npp, row) as libc::c_double - 1.0f64
            {
                return 2 as i32;
            }
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_reverse_row(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut temp: i32 = 0;
    let mut ret: i32 = 0 as i32;
    let mut old_lb: libc::c_double = 0.;
    let mut old_ub: libc::c_double = 0.;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                379 as i32,
            );
            1 as i32 != 0
        }) as i32;
    aij = (*row).ptr;
    while !aij.is_null() {
        (*aij).val = -(*aij).val;
        temp = (*aij).val as i32;
        if temp as libc::c_double != (*aij).val {
            ret = 1 as i32;
        }
        aij = (*aij).r_next;
    }
    old_lb = (*row).lb;
    old_ub = (*row).ub;
    if old_ub == 1.7976931348623157e+308f64 {
        (*row).lb = -1.7976931348623157e+308f64;
    } else {
        (*row).lb = -old_ub;
        temp = (*row).lb as i32;
        if temp as libc::c_double != (*row).lb {
            ret = 2 as i32;
        }
    }
    if old_lb == -1.7976931348623157e+308f64 {
        (*row).ub = 1.7976931348623157e+308f64;
    } else {
        (*row).ub = -old_lb;
        temp = (*row).ub as i32;
        if temp as libc::c_double != (*row).ub {
            ret = 3 as i32;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_split_pack(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut nlit: i32,
) -> *mut NPPROW {
    let mut rrr: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut k: i32 = 0;
    (_glp_npp_sat_is_pack_ineq(npp, row) == 1 as i32
        || {
            glp_assert_(
                b"npp_sat_is_pack_ineq(npp, row) == 1\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                439 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((0 as i32) < nlit && nlit < _glp_npp_row_nnz(npp, row)
        || {
            glp_assert_(
                b"0 < nlit && nlit < npp_row_nnz(npp, row)\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                442 as i32,
            );
            1 as i32 != 0
        }) as i32;
    rrr = _glp_npp_add_row(npp);
    (*rrr).lb = -1.7976931348623157e+308f64;
    (*rrr).ub = 1.0f64;
    k = 1 as i32;
    while k <= nlit {
        aij = (*row).ptr;
        (!aij.is_null()
            || {
                glp_assert_(
                    b"aij != NULL\0" as *const u8 as *const i8,
                    b"npp/npp6.c\0" as *const u8 as *const i8,
                    450 as i32,
                );
                1 as i32 != 0
            }) as i32;
        _glp_npp_add_aij(npp, rrr, (*aij).col, (*aij).val);
        if (*aij).val < 0.0f64 {
            (*rrr).ub -= 1.0f64;
            (*row).ub += 1.0f64;
        }
        _glp_npp_del_aij(npp, aij);
        k += 1;
        k;
    }
    col = _glp_npp_add_col(npp);
    (*col).is_int = 1 as i32 as i8;
    (*col).lb = 0.0f64;
    (*col).ub = 1.0f64;
    _glp_npp_add_aij(npp, rrr, col, -1.0f64);
    (*rrr).ub -= 1.0f64;
    _glp_npp_add_aij(npp, row, col, 1.0f64);
    return rrr;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_encode_pack(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) {
    let mut rrr: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut aik: *mut NPPAIJ = 0 as *mut NPPAIJ;
    (_glp_npp_sat_is_pack_ineq(npp, row) == 1 as i32
        || {
            glp_assert_(
                b"npp_sat_is_pack_ineq(npp, row) == 1\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                496 as i32,
            );
            1 as i32 != 0
        }) as i32;
    aij = (*row).ptr;
    while !aij.is_null() {
        aik = (*aij).r_next;
        while !aik.is_null() {
            rrr = _glp_npp_add_row(npp);
            (*rrr).lb = -1.7976931348623157e+308f64;
            (*rrr).ub = 1.0f64;
            _glp_npp_add_aij(npp, rrr, (*aij).col, (*aij).val);
            if (*aij).val < 0.0f64 {
                (*rrr).ub -= 1.0f64;
            }
            _glp_npp_add_aij(npp, rrr, (*aik).col, (*aik).val);
            if (*aik).val < 0.0f64 {
                (*rrr).ub -= 1.0f64;
            }
            _glp_npp_sat_reverse_row(npp, rrr);
            (_glp_npp_sat_is_cover_ineq(npp, rrr) == 1 as i32
                || {
                    glp_assert_(
                        b"npp_sat_is_cover_ineq(npp, rrr) == 1\0" as *const u8
                            as *const i8,
                        b"npp/npp6.c\0" as *const u8 as *const i8,
                        513 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            aik = (*aik).r_next;
        }
        aij = (*aij).r_next;
    }
    _glp_npp_del_row(npp, row);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_encode_sum2(
    mut npp: *mut NPP,
    mut set: *mut NPPLSE,
    mut sed: *mut NPPSED,
) {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut s: i32 = 0;
    let mut c: i32 = 0;
    (!set.is_null()
        || {
            glp_assert_(
                b"set != NULL\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                543 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!((*set).next).is_null()
        || {
            glp_assert_(
                b"set->next != NULL\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                544 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (((*(*set).next).next).is_null()
        || {
            glp_assert_(
                b"set->next->next == NULL\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                545 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*sed).x = (*set).lit;
    ((*sed).x.neg == 0 as i32 || (*sed).x.neg == 1 as i32
        || {
            glp_assert_(
                b"sed->x.neg == 0 || sed->x.neg == 1\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                547 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*sed).y = (*(*set).next).lit;
    ((*sed).y.neg == 0 as i32 || (*sed).y.neg == 1 as i32
        || {
            glp_assert_(
                b"sed->y.neg == 0 || sed->y.neg == 1\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                549 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*sed).z.col = 0 as *mut NPPCOL;
    (*sed).z.neg = 0 as i32;
    (*sed).s = _glp_npp_add_col(npp);
    (*(*sed).s).is_int = 1 as i32 as i8;
    (*(*sed).s).lb = 0.0f64;
    (*(*sed).s).ub = 1.0f64;
    x = 0 as i32;
    while x <= 1 as i32 {
        y = 0 as i32;
        while y <= 1 as i32 {
            s = 0 as i32;
            while s <= 1 as i32 {
                if (x + y) % 2 as i32 != s {
                    row = _glp_npp_add_row(npp);
                    (*row).lb = 1.0f64;
                    (*row).ub = 1.7976931348623157e+308f64;
                    if x == (*sed).x.neg {
                        _glp_npp_add_aij(npp, row, (*sed).x.col, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).x.col, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                    if y == (*sed).y.neg {
                        _glp_npp_add_aij(npp, row, (*sed).y.col, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).y.col, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                    if s == 0 as i32 {
                        _glp_npp_add_aij(npp, row, (*sed).s, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).s, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                }
                s += 1;
                s;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    (*sed).c = _glp_npp_add_col(npp);
    (*(*sed).c).is_int = 1 as i32 as i8;
    (*(*sed).c).lb = 0.0f64;
    (*(*sed).c).ub = 1.0f64;
    x = 0 as i32;
    while x <= 1 as i32 {
        y = 0 as i32;
        while y <= 1 as i32 {
            c = 0 as i32;
            while c <= 1 as i32 {
                if (x + y) / 2 as i32 != c {
                    row = _glp_npp_add_row(npp);
                    (*row).lb = 1.0f64;
                    (*row).ub = 1.7976931348623157e+308f64;
                    if x == (*sed).x.neg {
                        _glp_npp_add_aij(npp, row, (*sed).x.col, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).x.col, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                    if y == (*sed).y.neg {
                        _glp_npp_add_aij(npp, row, (*sed).y.col, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).y.col, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                    if c == 0 as i32 {
                        _glp_npp_add_aij(npp, row, (*sed).c, 1.0f64);
                    } else {
                        _glp_npp_add_aij(npp, row, (*sed).c, -1.0f64);
                        (*row).lb -= 1.0f64;
                    }
                }
                c += 1;
                c;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_encode_sum3(
    mut npp: *mut NPP,
    mut set: *mut NPPLSE,
    mut sed: *mut NPPSED,
) {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;
    let mut s: i32 = 0;
    let mut c: i32 = 0;
    (!set.is_null()
        || {
            glp_assert_(
                b"set != NULL\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                642 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!((*set).next).is_null()
        || {
            glp_assert_(
                b"set->next != NULL\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                643 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!((*(*set).next).next).is_null()
        || {
            glp_assert_(
                b"set->next->next != NULL\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                644 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*sed).x = (*set).lit;
    ((*sed).x.neg == 0 as i32 || (*sed).x.neg == 1 as i32
        || {
            glp_assert_(
                b"sed->x.neg == 0 || sed->x.neg == 1\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                646 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*sed).y = (*(*set).next).lit;
    ((*sed).y.neg == 0 as i32 || (*sed).y.neg == 1 as i32
        || {
            glp_assert_(
                b"sed->y.neg == 0 || sed->y.neg == 1\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                648 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*sed).z = (*(*(*set).next).next).lit;
    ((*sed).z.neg == 0 as i32 || (*sed).z.neg == 1 as i32
        || {
            glp_assert_(
                b"sed->z.neg == 0 || sed->z.neg == 1\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                650 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*sed).s = _glp_npp_add_col(npp);
    (*(*sed).s).is_int = 1 as i32 as i8;
    (*(*sed).s).lb = 0.0f64;
    (*(*sed).s).ub = 1.0f64;
    x = 0 as i32;
    while x <= 1 as i32 {
        y = 0 as i32;
        while y <= 1 as i32 {
            z = 0 as i32;
            while z <= 1 as i32 {
                s = 0 as i32;
                while s <= 1 as i32 {
                    if (x + y + z) % 2 as i32 != s {
                        row = _glp_npp_add_row(npp);
                        (*row).lb = 1.0f64;
                        (*row).ub = 1.7976931348623157e+308f64;
                        if x == (*sed).x.neg {
                            _glp_npp_add_aij(npp, row, (*sed).x.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).x.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if y == (*sed).y.neg {
                            _glp_npp_add_aij(npp, row, (*sed).y.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).y.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if z == (*sed).z.neg {
                            _glp_npp_add_aij(npp, row, (*sed).z.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).z.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if s == 0 as i32 {
                            _glp_npp_add_aij(npp, row, (*sed).s, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).s, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                    }
                    s += 1;
                    s;
                }
                z += 1;
                z;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    (*sed).c = _glp_npp_add_col(npp);
    (*(*sed).c).is_int = 1 as i32 as i8;
    (*(*sed).c).lb = 0.0f64;
    (*(*sed).c).ub = 1.0f64;
    x = 0 as i32;
    while x <= 1 as i32 {
        y = 0 as i32;
        while y <= 1 as i32 {
            z = 0 as i32;
            while z <= 1 as i32 {
                c = 0 as i32;
                while c <= 1 as i32 {
                    if (x + y + z) / 2 as i32 != c {
                        row = _glp_npp_add_row(npp);
                        (*row).lb = 1.0f64;
                        (*row).ub = 1.7976931348623157e+308f64;
                        if x == (*sed).x.neg {
                            _glp_npp_add_aij(npp, row, (*sed).x.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).x.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if y == (*sed).y.neg {
                            _glp_npp_add_aij(npp, row, (*sed).y.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).y.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if z == (*sed).z.neg {
                            _glp_npp_add_aij(npp, row, (*sed).z.col, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).z.col, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                        if c == 0 as i32 {
                            _glp_npp_add_aij(npp, row, (*sed).c, 1.0f64);
                        } else {
                            _glp_npp_add_aij(npp, row, (*sed).c, -1.0f64);
                            (*row).lb -= 1.0f64;
                        }
                    }
                    c += 1;
                    c;
                }
                z += 1;
                z;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
}
unsafe extern "C" fn remove_lse(
    mut npp: *mut NPP,
    mut set: *mut NPPLSE,
    mut col: *mut NPPCOL,
) -> *mut NPPLSE {
    let mut lse: *mut NPPLSE = 0 as *mut NPPLSE;
    let mut prev: *mut NPPLSE = 0 as *mut NPPLSE;
    lse = set;
    while !lse.is_null() {
        if (*lse).lit.col == col {
            break;
        }
        prev = lse;
        lse = (*lse).next;
    }
    (!lse.is_null()
        || {
            glp_assert_(
                b"lse != NULL\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                825 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if prev.is_null() {
        set = (*lse).next;
    } else {
        (*prev).next = (*lse).next;
    }
    _glp_dmp_free_atom(
        (*npp).pool,
        lse as *mut libc::c_void,
        ::core::mem::size_of::<NPPLSE>() as u64 as i32,
    );
    return set;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_encode_sum_ax(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut y: *mut NPPLIT,
) -> i32 {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut set: [*mut NPPLSE; 32] = [0 as *mut NPPLSE; 32];
    let mut lse: *mut NPPLSE = 0 as *mut NPPLSE;
    let mut sed: NPPSED = NPPSED {
        x: NPPLIT {
            col: 0 as *mut NPPCOL,
            neg: 0,
        },
        y: NPPLIT {
            col: 0 as *mut NPPCOL,
            neg: 0,
        },
        z: NPPLIT {
            col: 0 as *mut NPPCOL,
            neg: 0,
        },
        s: 0 as *mut NPPCOL,
        c: 0 as *mut NPPCOL,
    };
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut temp: i32 = 0;
    let mut sum: libc::c_double = 0.;
    sum = 0.0f64;
    aij = (*row).ptr;
    while !aij.is_null() {
        sum += fabs((*aij).val);
        aij = (*aij).r_next;
    }
    temp = sum as i32;
    if temp as libc::c_double != sum {
        return -(1 as i32);
    }
    n = 0 as i32;
    while temp > 0 as i32 {
        n += 1;
        n;
        temp >>= 1 as i32;
    }
    (0 as i32 <= n && n <= 31 as i32
        || {
            glp_assert_(
                b"0 <= n && n <= NBIT_MAX\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                849 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = 1 as i32;
    while k <= n {
        set[k as usize] = 0 as *mut NPPLSE;
        k += 1;
        k;
    }
    aij = (*row).ptr;
    while !aij.is_null() {
        temp = fabs((*aij).val) as i32;
        (temp as libc::c_double == fabs((*aij).val)
            || {
                glp_assert_(
                    b"(int)temp == fabs(aij->val)\0" as *const u8 as *const i8,
                    b"npp/npp6.c\0" as *const u8 as *const i8,
                    856 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k = 1 as i32;
        while temp > 0 as i32 {
            if temp & 1 as i32 != 0 {
                (k <= n
                    || {
                        glp_assert_(
                            b"k <= n\0" as *const u8 as *const i8,
                            b"npp/npp6.c\0" as *const u8 as *const i8,
                            859 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                lse = _glp_dmp_get_atom(
                    (*npp).pool,
                    ::core::mem::size_of::<NPPLSE>() as u64 as i32,
                ) as *mut NPPLSE;
                (*lse).lit.col = (*aij).col;
                (*lse).lit.neg = if (*aij).val > 0.0f64 { 0 as i32 } else { 1 as i32 };
                (*lse).next = set[k as usize];
                set[k as usize] = lse;
            }
            k += 1;
            k;
            temp >>= 1 as i32;
        }
        aij = (*aij).r_next;
    }
    k = 1 as i32;
    while k <= n {
        loop {
            if (set[k as usize]).is_null() {
                let ref mut fresh0 = (*y.offset(k as isize)).col;
                *fresh0 = 0 as *mut NPPCOL;
                (*y.offset(k as isize)).neg = 0 as i32;
                break;
            } else if ((*set[k as usize]).next).is_null() {
                *y.offset(k as isize) = (*set[k as usize]).lit;
                _glp_dmp_free_atom(
                    (*npp).pool,
                    set[k as usize] as *mut libc::c_void,
                    ::core::mem::size_of::<NPPLSE>() as u64 as i32,
                );
                break;
            } else {
                if ((*(*set[k as usize]).next).next).is_null() {
                    _glp_npp_sat_encode_sum2(npp, set[k as usize], &mut sed);
                } else {
                    _glp_npp_sat_encode_sum3(npp, set[k as usize], &mut sed);
                    set[k as usize] = remove_lse(npp, set[k as usize], sed.z.col);
                }
                set[k as usize] = remove_lse(npp, set[k as usize], sed.y.col);
                set[k as usize] = remove_lse(npp, set[k as usize], sed.x.col);
                lse = _glp_dmp_get_atom(
                    (*npp).pool,
                    ::core::mem::size_of::<NPPLSE>() as u64 as i32,
                ) as *mut NPPLSE;
                (*lse).lit.col = sed.s;
                (*lse).lit.neg = 0 as i32;
                (*lse).next = set[k as usize];
                set[k as usize] = lse;
                (k < n
                    || {
                        glp_assert_(
                            b"k < n\0" as *const u8 as *const i8,
                            b"npp/npp6.c\0" as *const u8 as *const i8,
                            907 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                lse = _glp_dmp_get_atom(
                    (*npp).pool,
                    ::core::mem::size_of::<NPPLSE>() as u64 as i32,
                ) as *mut NPPLSE;
                (*lse).lit.col = sed.c;
                (*lse).lit.neg = 0 as i32;
                (*lse).next = set[(k + 1 as i32) as usize];
                set[(k + 1 as i32) as usize] = lse;
            }
        }
        k += 1;
        k;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_normalize_clause(
    mut npp: *mut NPP,
    mut size: i32,
    mut lit: *mut NPPLIT,
) -> i32 {
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut new_size: i32 = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                931 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (size >= 0 as i32
        || {
            glp_assert_(
                b"size >= 0\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                932 as i32,
            );
            1 as i32 != 0
        }) as i32;
    new_size = 0 as i32;
    k = 1 as i32;
    while k <= size {
        let mut current_block_8: u64;
        j = 1 as i32;
        loop {
            if !(j <= new_size) {
                current_block_8 = 13109137661213826276;
                break;
            }
            if (*lit.offset(k as isize)).col == (*lit.offset(j as isize)).col {
                if (*lit.offset(k as isize)).neg == (*lit.offset(j as isize)).neg {
                    current_block_8 = 5399440093318478209;
                    break;
                }
                return -(1 as i32);
            } else {
                j += 1;
                j;
            }
        }
        match current_block_8 {
            13109137661213826276 => {
                new_size += 1;
                *lit.offset(new_size as isize) = *lit.offset(k as isize);
            }
            _ => {}
        }
        k += 1;
        k;
    }
    return new_size;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_encode_clause(
    mut npp: *mut NPP,
    mut size: i32,
    mut lit: *mut NPPLIT,
) -> *mut NPPROW {
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut k: i32 = 0;
    (size >= 1 as i32
        || {
            glp_assert_(
                b"size >= 1\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                980 as i32,
            );
            1 as i32 != 0
        }) as i32;
    row = _glp_npp_add_row(npp);
    (*row).lb = 1.0f64;
    (*row).ub = 1.7976931348623157e+308f64;
    k = 1 as i32;
    while k <= size {
        (!((*lit.offset(k as isize)).col).is_null()
            || {
                glp_assert_(
                    b"lit[k].col != NULL\0" as *const u8 as *const i8,
                    b"npp/npp6.c\0" as *const u8 as *const i8,
                    984 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if (*lit.offset(k as isize)).neg == 0 as i32 {
            _glp_npp_add_aij(npp, row, (*lit.offset(k as isize)).col, 1.0f64);
        } else if (*lit.offset(k as isize)).neg == 1 as i32 {
            _glp_npp_add_aij(npp, row, (*lit.offset(k as isize)).col, -1.0f64);
            (*row).lb -= 1.0f64;
        } else {
            (lit != lit
                || {
                    glp_assert_(
                        b"lit != lit\0" as *const u8 as *const i8,
                        b"npp/npp6.c\0" as *const u8 as *const i8,
                        992 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        k += 1;
        k;
    }
    return row;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_encode_geq(
    mut npp: *mut NPP,
    mut n: i32,
    mut y: *mut NPPLIT,
    mut rhs: i32,
) -> i32 {
    let mut current_block: u64;
    let mut lit: [NPPLIT; 32] = [NPPLIT {
        col: 0 as *mut NPPCOL,
        neg: 0,
    }; 32];
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut size: i32 = 0;
    let mut temp: i32 = 0;
    let mut b: [i32; 32] = [0; 32];
    (0 as i32 <= n && n <= 31 as i32
        || {
            glp_assert_(
                b"0 <= n && n <= NBIT_MAX\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                1059 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if rhs < 0 as i32 {
        return 0 as i32;
    }
    k = 1 as i32;
    temp = rhs;
    while k <= n {
        b[k as usize] = temp & 1 as i32;
        k += 1;
        k;
        temp >>= 1 as i32;
    }
    if temp != 0 as i32 {
        return 1 as i32;
    }
    k = 1 as i32;
    while k <= n {
        size = 0 as i32;
        if !(b[k as usize] == 0 as i32) {
            if ((*y.offset(k as isize)).col).is_null() {
                ((*y.offset(k as isize)).neg == 0 as i32
                    || {
                        glp_assert_(
                            b"y[k].neg == 0\0" as *const u8 as *const i8,
                            b"npp/npp6.c\0" as *const u8 as *const i8,
                            1081 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            } else {
                size += 1;
                lit[size as usize] = *y.offset(k as isize);
            }
            j = k + 1 as i32;
            loop {
                if !(j <= n) {
                    current_block = 16203760046146113240;
                    break;
                }
                if ((*y.offset(j as isize)).col).is_null() {
                    ((*y.offset(j as isize)).neg == 0 as i32
                        || {
                            glp_assert_(
                                b"y[j].neg == 0\0" as *const u8 as *const i8,
                                b"npp/npp6.c\0" as *const u8 as *const i8,
                                1090 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if !(b[j as usize] == 0 as i32) {
                        current_block = 14523784380283086299;
                        break;
                    }
                } else {
                    size += 1;
                    lit[size as usize] = *y.offset(j as isize);
                    if b[j as usize] != 0 as i32 {
                        lit[size as usize].neg = 1 as i32 - lit[size as usize].neg;
                    }
                }
                j += 1;
                j;
            }
            match current_block {
                14523784380283086299 => {}
                _ => {
                    size = _glp_npp_sat_normalize_clause(npp, size, lit.as_mut_ptr());
                    if !(size < 0 as i32) {
                        if size == 0 as i32 {
                            return 2 as i32;
                        }
                        _glp_npp_sat_encode_clause(npp, size, lit.as_mut_ptr());
                    }
                }
            }
        }
        k += 1;
        k;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_encode_leq(
    mut npp: *mut NPP,
    mut n: i32,
    mut y: *mut NPPLIT,
    mut rhs: i32,
) -> i32 {
    let mut current_block: u64;
    let mut lit: [NPPLIT; 32] = [NPPLIT {
        col: 0 as *mut NPPCOL,
        neg: 0,
    }; 32];
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut size: i32 = 0;
    let mut temp: i32 = 0;
    let mut b: [i32; 32] = [0; 32];
    (0 as i32 <= n && n <= 31 as i32
        || {
            glp_assert_(
                b"0 <= n && n <= NBIT_MAX\0" as *const u8 as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                1186 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if rhs < 0 as i32 {
        return 1 as i32;
    }
    k = 1 as i32;
    temp = rhs;
    while k <= n {
        b[k as usize] = temp & 1 as i32;
        k += 1;
        k;
        temp >>= 1 as i32;
    }
    if temp != 0 as i32 {
        return 0 as i32;
    }
    k = 1 as i32;
    while k <= n {
        size = 0 as i32;
        if !(b[k as usize] == 1 as i32) {
            if ((*y.offset(k as isize)).col).is_null() {
                ((*y.offset(k as isize)).neg == 0 as i32
                    || {
                        glp_assert_(
                            b"y[k].neg == 0\0" as *const u8 as *const i8,
                            b"npp/npp6.c\0" as *const u8 as *const i8,
                            1208 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            } else {
                size += 1;
                lit[size as usize] = *y.offset(k as isize);
                lit[size as usize].neg = 1 as i32 - lit[size as usize].neg;
                j = k + 1 as i32;
                loop {
                    if !(j <= n) {
                        current_block = 7172762164747879670;
                        break;
                    }
                    if ((*y.offset(j as isize)).col).is_null() {
                        ((*y.offset(j as isize)).neg == 0 as i32
                            || {
                                glp_assert_(
                                    b"y[j].neg == 0\0" as *const u8 as *const i8,
                                    b"npp/npp6.c\0" as *const u8 as *const i8,
                                    1219 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        if !(b[j as usize] == 0 as i32) {
                            current_block = 14523784380283086299;
                            break;
                        }
                    } else {
                        size += 1;
                        lit[size as usize] = *y.offset(j as isize);
                        if b[j as usize] != 0 as i32 {
                            lit[size as usize].neg = 1 as i32 - lit[size as usize].neg;
                        }
                    }
                    j += 1;
                    j;
                }
                match current_block {
                    14523784380283086299 => {}
                    _ => {
                        size = _glp_npp_sat_normalize_clause(
                            npp,
                            size,
                            lit.as_mut_ptr(),
                        );
                        if !(size < 0 as i32) {
                            if size == 0 as i32 {
                                return 2 as i32;
                            }
                            _glp_npp_sat_encode_clause(npp, size, lit.as_mut_ptr());
                        }
                    }
                }
            }
        }
        k += 1;
        k;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_encode_row(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut y: [NPPLIT; 32] = [NPPLIT {
        col: 0 as *mut NPPCOL,
        neg: 0,
    }; 32];
    let mut n: i32 = 0;
    let mut rhs: i32 = 0;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    (!((*row).lb == -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64)
        || {
            glp_assert_(
                b"!(row->lb == -DBL_MAX && row->ub == +DBL_MAX)\0" as *const u8
                    as *const i8,
                b"npp/npp6.c\0" as *const u8 as *const i8,
                1327 as i32,
            );
            1 as i32 != 0
        }) as i32;
    lb = (*row).lb;
    ub = (*row).ub;
    aij = (*row).ptr;
    while !aij.is_null() {
        if (*aij).val < 0.0f64 {
            if lb != -1.7976931348623157e+308f64 {
                lb -= (*aij).val;
            }
            if ub != -1.7976931348623157e+308f64 {
                ub -= (*aij).val;
            }
        }
        aij = (*aij).r_next;
    }
    n = _glp_npp_sat_encode_sum_ax(npp, row, y.as_mut_ptr());
    if n < 0 as i32 {
        return 2 as i32;
    }
    if lb != -1.7976931348623157e+308f64 {
        rhs = lb as i32;
        if rhs as libc::c_double != lb {
            return 2 as i32;
        }
        if _glp_npp_sat_encode_geq(npp, n, y.as_mut_ptr(), rhs) != 0 as i32 {
            return 1 as i32;
        }
    }
    if ub != 1.7976931348623157e+308f64 {
        rhs = ub as i32;
        if rhs as libc::c_double != ub {
            return 2 as i32;
        }
        if _glp_npp_sat_encode_leq(npp, n, y.as_mut_ptr(), rhs) != 0 as i32 {
            return 1 as i32;
        }
    }
    _glp_npp_del_row(npp, row);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_sat_encode_prob(mut npp: *mut NPP) -> i32 {
    let mut current_block: u64;
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut next_row: *mut NPPROW = 0 as *mut NPPROW;
    let mut prev_row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut next_col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut cover: i32 = 0 as i32;
    let mut pack: i32 = 0 as i32;
    let mut partn: i32 = 0 as i32;
    let mut ret: i32 = 0;
    row = (*npp).r_head;
    while !row.is_null() {
        next_row = (*row).next;
        if (*row).lb == -1.7976931348623157e+308f64
            && (*row).ub == 1.7976931348623157e+308f64
        {
            _glp_npp_sat_free_row(npp, row);
        }
        row = next_row;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        next_col = (*col).next;
        if (*col).lb == (*col).ub {
            (_glp_npp_sat_fixed_col(npp, col) == 0 as i32
                || {
                    glp_assert_(
                        b"npp_sat_fixed_col(npp, col) == 0\0" as *const u8 as *const i8,
                        b"npp/npp6.c\0" as *const u8 as *const i8,
                        1394 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        col = next_col;
    }
    col = (*npp).c_head;
    while !col.is_null() {
        ((*col).is_int as i32 != 0 && (*col).lb == 0.0f64 && (*col).ub == 1.0f64
            || {
                glp_assert_(
                    b"col->is_int && col->lb == 0.0 && col->ub == 1.0\0" as *const u8
                        as *const i8,
                    b"npp/npp6.c\0" as *const u8 as *const i8,
                    1398 as i32,
                );
                1 as i32 != 0
            }) as i32;
        col = (*col).next;
    }
    row = (*npp).r_tail;
    loop {
        if row.is_null() {
            current_block = 2520131295878969859;
            break;
        }
        prev_row = (*row).prev;
        ret = _glp_npp_sat_is_cover_ineq(npp, row);
        if ret != 0 as i32 {
            cover += 1;
            cover;
            if ret == 2 as i32 {
                (_glp_npp_sat_reverse_row(npp, row) == 0 as i32
                    || {
                        glp_assert_(
                            b"npp_sat_reverse_row(npp, row) == 0\0" as *const u8
                                as *const i8,
                            b"npp/npp6.c\0" as *const u8 as *const i8,
                            1411 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                ret = _glp_npp_sat_is_cover_ineq(npp, row);
            }
            (ret == 1 as i32
                || {
                    glp_assert_(
                        b"ret == 1\0" as *const u8 as *const i8,
                        b"npp/npp6.c\0" as *const u8 as *const i8,
                        1414 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        } else {
            ret = _glp_npp_sat_is_partn_eq(npp, row);
            if ret != 0 as i32 {
                let mut cov: *mut NPPROW = 0 as *mut NPPROW;
                let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
                partn += 1;
                partn;
                if ret == 2 as i32 {
                    (_glp_npp_sat_reverse_row(npp, row) == 0 as i32
                        || {
                            glp_assert_(
                                b"npp_sat_reverse_row(npp, row) == 0\0" as *const u8
                                    as *const i8,
                                b"npp/npp6.c\0" as *const u8 as *const i8,
                                1425 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    ret = _glp_npp_sat_is_partn_eq(npp, row);
                }
                (ret == 1 as i32
                    || {
                        glp_assert_(
                            b"ret == 1\0" as *const u8 as *const i8,
                            b"npp/npp6.c\0" as *const u8 as *const i8,
                            1428 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                cov = _glp_npp_add_row(npp);
                (*cov).lb = (*row).lb;
                (*cov).ub = 1.7976931348623157e+308f64;
                aij = (*row).ptr;
                while !aij.is_null() {
                    _glp_npp_add_aij(npp, cov, (*aij).col, (*aij).val);
                    aij = (*aij).r_next;
                }
                (_glp_npp_sat_is_cover_ineq(npp, cov) == 1 as i32
                    || {
                        glp_assert_(
                            b"npp_sat_is_cover_ineq(npp, cov) == 1\0" as *const u8
                                as *const i8,
                            b"npp/npp6.c\0" as *const u8 as *const i8,
                            1435 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*row).lb = -1.7976931348623157e+308f64;
                (_glp_npp_sat_is_pack_ineq(npp, row) == 1 as i32
                    || {
                        glp_assert_(
                            b"npp_sat_is_pack_ineq(npp, row) == 1\0" as *const u8
                                as *const i8,
                            b"npp/npp6.c\0" as *const u8 as *const i8,
                            1439 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                pack -= 1;
                pack;
            }
            ret = _glp_npp_sat_is_pack_ineq(npp, row);
            if ret != 0 as i32 {
                let mut rrr: *mut NPPROW = 0 as *mut NPPROW;
                let mut nlit: i32 = 0;
                let mut desired_nlit: i32 = 4 as i32;
                pack += 1;
                pack;
                if ret == 2 as i32 {
                    (_glp_npp_sat_reverse_row(npp, row) == 0 as i32
                        || {
                            glp_assert_(
                                b"npp_sat_reverse_row(npp, row) == 0\0" as *const u8
                                    as *const i8,
                                b"npp/npp6.c\0" as *const u8 as *const i8,
                                1451 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    ret = _glp_npp_sat_is_pack_ineq(npp, row);
                }
                (ret == 1 as i32
                    || {
                        glp_assert_(
                            b"ret == 1\0" as *const u8 as *const i8,
                            b"npp/npp6.c\0" as *const u8 as *const i8,
                            1454 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                loop {
                    nlit = _glp_npp_row_nnz(npp, row);
                    if nlit <= desired_nlit {
                        break;
                    }
                    rrr = _glp_npp_sat_split_pack(npp, row, desired_nlit - 1 as i32);
                    _glp_npp_sat_encode_pack(npp, rrr);
                }
                _glp_npp_sat_encode_pack(npp, row);
            } else {
                ret = _glp_npp_sat_encode_row(npp, row);
                if !(ret == 0 as i32) {
                    if ret == 1 as i32 {
                        ret = 0xa as i32;
                    } else if ret == 2 as i32 {
                        ret = 0x13 as i32;
                    } else {
                        (ret != ret
                            || {
                                glp_assert_(
                                    b"ret != ret\0" as *const u8 as *const i8,
                                    b"npp/npp6.c\0" as *const u8 as *const i8,
                                    1484 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                }
                if ret != 0 as i32 {
                    current_block = 13378183133826635415;
                    break;
                }
            }
        }
        row = prev_row;
    }
    match current_block {
        2520131295878969859 => {
            ret = 0 as i32;
            if cover != 0 as i32 {
                glp_printf(
                    b"%d covering inequalities\n\0" as *const u8 as *const i8,
                    cover,
                );
            }
            if pack != 0 as i32 {
                glp_printf(
                    b"%d packing inequalities\n\0" as *const u8 as *const i8,
                    pack,
                );
            }
            if partn != 0 as i32 {
                glp_printf(
                    b"%d partitioning equalities\n\0" as *const u8 as *const i8,
                    partn,
                );
            }
        }
        _ => {}
    }
    return ret;
}