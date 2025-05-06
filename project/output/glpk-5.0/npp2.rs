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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn _glp_npp_add_row(npp: *mut NPP) -> *mut NPPROW;
    fn _glp_npp_add_col(npp: *mut NPP) -> *mut NPPCOL;
    fn _glp_npp_add_aij(
        npp: *mut NPP,
        row: *mut NPPROW,
        col: *mut NPPCOL,
        val: libc::c_double,
    ) -> *mut NPPAIJ;
    fn _glp_npp_push_tse(
        npp: *mut NPP,
        func: Option<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32>,
        size: i32,
    ) -> *mut libc::c_void;
    fn _glp_npp_del_row(npp: *mut NPP, row: *mut NPPROW);
    fn _glp_npp_del_col(npp: *mut NPP, col: *mut NPPCOL);
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
pub struct NPPLFE {
    pub ref_0: i32,
    pub val: libc::c_double,
    pub next: *mut NPPLFE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct free_row {
    pub p: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ineq_row {
    pub p: i32,
    pub s: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct free_col {
    pub q: i32,
    pub s: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bnd_col {
    pub q: i32,
    pub bnd: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dbnd_col {
    pub q: i32,
    pub s: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fixed_col {
    pub q: i32,
    pub s: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct make_equality {
    pub p: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct make_fixed {
    pub q: i32,
    pub c: libc::c_double,
    pub ptr: *mut NPPLFE,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_free_row(mut npp: *mut NPP, mut p: *mut NPPROW) {
    let mut info: *mut free_row = 0 as *mut free_row;
    ((*p).lb == -1.7976931348623157e+308f64 && (*p).ub == 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->lb == -DBL_MAX && p->ub == +DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                79 as i32,
            );
            1 as i32 != 0
        }) as i32;
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_free_row as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<free_row>() as u64 as i32,
    ) as *mut free_row;
    (*info).p = (*p).i;
    _glp_npp_del_row(npp, p);
}
unsafe extern "C" fn rcv_free_row(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut free_row = _info as *mut free_row;
    if (*npp).sol == 1 as i32 {
        *((*npp).r_stat).offset((*info).p as isize) = 1 as i32 as i8;
    }
    if (*npp).sol != 3 as i32 {
        *((*npp).r_pi).offset((*info).p as isize) = 0.0f64;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_geq_row(mut npp: *mut NPP, mut p: *mut NPPROW) {
    let mut info: *mut ineq_row = 0 as *mut ineq_row;
    let mut s: *mut NPPCOL = 0 as *mut NPPCOL;
    ((*p).lb != -1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->lb != -DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                196 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*p).lb < (*p).ub
        || {
            glp_assert_(
                b"p->lb < p->ub\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                197 as i32,
            );
            1 as i32 != 0
        }) as i32;
    s = _glp_npp_add_col(npp);
    (*s).lb = 0.0f64;
    (*s).ub = if (*p).ub == 1.7976931348623157e+308f64 {
        1.7976931348623157e+308f64
    } else {
        (*p).ub - (*p).lb
    };
    _glp_npp_add_aij(npp, p, s, -1.0f64);
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_geq_row as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<ineq_row>() as u64 as i32,
    ) as *mut ineq_row;
    (*info).p = (*p).i;
    (*info).s = (*s).j;
    (*p).ub = (*p).lb;
}
unsafe extern "C" fn rcv_geq_row(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut ineq_row = _info as *mut ineq_row;
    if (*npp).sol == 1 as i32 {
        if *((*npp).r_stat).offset((*info).p as isize) as i32 == 1 as i32 {
            if *((*npp).c_stat).offset((*info).s as isize) as i32 == 1 as i32 {
                return 1 as i32
            } else if *((*npp).c_stat).offset((*info).s as isize) as i32 == 2 as i32
                || *((*npp).c_stat).offset((*info).s as isize) as i32 == 3 as i32
            {
                *((*npp).r_stat).offset((*info).p as isize) = 1 as i32 as i8;
            } else {
                return 1 as i32
            }
        } else if *((*npp).r_stat).offset((*info).p as isize) as i32 == 5 as i32 {
            if *((*npp).c_stat).offset((*info).s as isize) as i32 == 1 as i32 {
                *((*npp).r_stat).offset((*info).p as isize) = 1 as i32 as i8;
            } else if *((*npp).c_stat).offset((*info).s as isize) as i32 == 2 as i32 {
                *((*npp).r_stat).offset((*info).p as isize) = 2 as i32 as i8;
            } else if *((*npp).c_stat).offset((*info).s as isize) as i32 == 3 as i32 {
                *((*npp).r_stat).offset((*info).p as isize) = 3 as i32 as i8;
            } else {
                return 1 as i32
            }
        } else {
            return 1 as i32
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_leq_row(mut npp: *mut NPP, mut p: *mut NPPROW) {
    let mut info: *mut ineq_row = 0 as *mut ineq_row;
    let mut s: *mut NPPCOL = 0 as *mut NPPCOL;
    ((*p).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->ub != +DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                340 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*p).lb < (*p).ub
        || {
            glp_assert_(
                b"p->lb < p->ub\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                341 as i32,
            );
            1 as i32 != 0
        }) as i32;
    s = _glp_npp_add_col(npp);
    (*s).lb = 0.0f64;
    (*s).ub = if (*p).lb == -1.7976931348623157e+308f64 {
        1.7976931348623157e+308f64
    } else {
        (*p).ub - (*p).lb
    };
    _glp_npp_add_aij(npp, p, s, 1.0f64);
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_leq_row as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<ineq_row>() as u64 as i32,
    ) as *mut ineq_row;
    (*info).p = (*p).i;
    (*info).s = (*s).j;
    (*p).lb = (*p).ub;
}
unsafe extern "C" fn rcv_leq_row(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut ineq_row = _info as *mut ineq_row;
    if (*npp).sol == 1 as i32 {
        if *((*npp).r_stat).offset((*info).p as isize) as i32 == 1 as i32 {
            if *((*npp).c_stat).offset((*info).s as isize) as i32 == 1 as i32 {
                return 1 as i32
            } else if *((*npp).c_stat).offset((*info).s as isize) as i32 == 2 as i32
                || *((*npp).c_stat).offset((*info).s as isize) as i32 == 3 as i32
            {
                *((*npp).r_stat).offset((*info).p as isize) = 1 as i32 as i8;
            } else {
                return 1 as i32
            }
        } else if *((*npp).r_stat).offset((*info).p as isize) as i32 == 5 as i32 {
            if *((*npp).c_stat).offset((*info).s as isize) as i32 == 1 as i32 {
                *((*npp).r_stat).offset((*info).p as isize) = 1 as i32 as i8;
            } else if *((*npp).c_stat).offset((*info).s as isize) as i32 == 2 as i32 {
                *((*npp).r_stat).offset((*info).p as isize) = 3 as i32 as i8;
            } else if *((*npp).c_stat).offset((*info).s as isize) as i32 == 3 as i32 {
                *((*npp).r_stat).offset((*info).p as isize) = 2 as i32 as i8;
            } else {
                return 1 as i32
            }
        } else {
            return 1 as i32
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_free_col(mut npp: *mut NPP, mut q: *mut NPPCOL) {
    let mut info: *mut free_col = 0 as *mut free_col;
    let mut s: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    ((*q).lb == -1.7976931348623157e+308f64 && (*q).ub == 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->lb == -DBL_MAX && q->ub == +DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                492 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*q).lb = 0.0f64;
    (*q).ub = 1.7976931348623157e+308f64;
    s = _glp_npp_add_col(npp);
    (*s).is_int = (*q).is_int;
    (*s).lb = 0.0f64;
    (*s).ub = 1.7976931348623157e+308f64;
    (*s).coef = -(*q).coef;
    aij = (*q).ptr;
    while !aij.is_null() {
        _glp_npp_add_aij(npp, (*aij).row, s, -(*aij).val);
        aij = (*aij).c_next;
    }
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_free_col as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<free_col>() as u64 as i32,
    ) as *mut free_col;
    (*info).q = (*q).j;
    (*info).s = (*s).j;
}
unsafe extern "C" fn rcv_free_col(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut free_col = _info as *mut free_col;
    if (*npp).sol == 1 as i32 {
        if *((*npp).c_stat).offset((*info).q as isize) as i32 == 1 as i32 {
            if *((*npp).c_stat).offset((*info).s as isize) as i32 == 1 as i32 {
                return 1 as i32
            } else if *((*npp).c_stat).offset((*info).s as isize) as i32 == 2 as i32 {
                *((*npp).c_stat).offset((*info).q as isize) = 1 as i32 as i8;
            } else {
                return -(1 as i32)
            }
        } else if *((*npp).c_stat).offset((*info).q as isize) as i32 == 2 as i32 {
            if *((*npp).c_stat).offset((*info).s as isize) as i32 == 1 as i32 {
                *((*npp).c_stat).offset((*info).q as isize) = 1 as i32 as i8;
            } else if *((*npp).c_stat).offset((*info).s as isize) as i32 == 2 as i32 {
                *((*npp).c_stat).offset((*info).q as isize) = 4 as i32 as i8;
            } else {
                return -(1 as i32)
            }
        } else {
            return -(1 as i32)
        }
    }
    *((*npp).c_value).offset((*info).q as isize)
        -= *((*npp).c_value).offset((*info).s as isize);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_lbnd_col(mut npp: *mut NPP, mut q: *mut NPPCOL) {
    let mut info: *mut bnd_col = 0 as *mut bnd_col;
    let mut i: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    ((*q).lb != 0.0f64
        || {
            glp_assert_(
                b"q->lb != 0.0\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                650 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*q).lb != -1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->lb != -DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                651 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                652 as i32,
            );
            1 as i32 != 0
        }) as i32;
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_lbnd_col as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<bnd_col>() as u64 as i32,
    ) as *mut bnd_col;
    (*info).q = (*q).j;
    (*info).bnd = (*q).lb;
    (*npp).c0 += (*q).coef * (*q).lb;
    aij = (*q).ptr;
    while !aij.is_null() {
        i = (*aij).row;
        if (*i).lb == (*i).ub {
            (*i).lb -= (*aij).val * (*q).lb;
            (*i).ub = (*i).lb;
        } else {
            if (*i).lb != -1.7976931348623157e+308f64 {
                (*i).lb -= (*aij).val * (*q).lb;
            }
            if (*i).ub != 1.7976931348623157e+308f64 {
                (*i).ub -= (*aij).val * (*q).lb;
            }
        }
        aij = (*aij).c_next;
    }
    if (*q).ub != 1.7976931348623157e+308f64 {
        (*q).ub -= (*q).lb;
    }
    (*q).lb = 0.0f64;
}
unsafe extern "C" fn rcv_lbnd_col(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut bnd_col = _info as *mut bnd_col;
    if (*npp).sol == 1 as i32 {
        if *((*npp).c_stat).offset((*info).q as isize) as i32 == 1 as i32
            || *((*npp).c_stat).offset((*info).q as isize) as i32 == 2 as i32
            || *((*npp).c_stat).offset((*info).q as isize) as i32 == 3 as i32
        {
            *((*npp).c_stat).offset((*info).q as isize) = *((*npp).c_stat)
                .offset((*info).q as isize);
        } else {
            return 1 as i32
        }
    }
    *((*npp).c_value).offset((*info).q as isize) = (*info).bnd
        + *((*npp).c_value).offset((*info).q as isize);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_ubnd_col(mut npp: *mut NPP, mut q: *mut NPPCOL) {
    let mut info: *mut bnd_col = 0 as *mut bnd_col;
    let mut i: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    ((*q).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->ub != +DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                818 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                819 as i32,
            );
            1 as i32 != 0
        }) as i32;
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_ubnd_col as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<bnd_col>() as u64 as i32,
    ) as *mut bnd_col;
    (*info).q = (*q).j;
    (*info).bnd = (*q).ub;
    (*npp).c0 += (*q).coef * (*q).ub;
    (*q).coef = -(*q).coef;
    aij = (*q).ptr;
    while !aij.is_null() {
        i = (*aij).row;
        if (*i).lb == (*i).ub {
            (*i).lb -= (*aij).val * (*q).ub;
            (*i).ub = (*i).lb;
        } else {
            if (*i).lb != -1.7976931348623157e+308f64 {
                (*i).lb -= (*aij).val * (*q).ub;
            }
            if (*i).ub != 1.7976931348623157e+308f64 {
                (*i).ub -= (*aij).val * (*q).ub;
            }
        }
        (*aij).val = -(*aij).val;
        aij = (*aij).c_next;
    }
    if (*q).lb != -1.7976931348623157e+308f64 {
        (*q).ub -= (*q).lb;
    } else {
        (*q).ub = 1.7976931348623157e+308f64;
    }
    (*q).lb = 0.0f64;
}
unsafe extern "C" fn rcv_ubnd_col(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut bnd_col = _info as *mut bnd_col;
    if (*npp).sol == 1 as i32 {
        if *((*npp).c_stat).offset((*info).q as isize) as i32 == 1 as i32 {
            *((*npp).c_stat).offset((*info).q as isize) = 1 as i32 as i8;
        } else if *((*npp).c_stat).offset((*info).q as isize) as i32 == 2 as i32 {
            *((*npp).c_stat).offset((*info).q as isize) = 3 as i32 as i8;
        } else if *((*npp).c_stat).offset((*info).q as isize) as i32 == 3 as i32 {
            *((*npp).c_stat).offset((*info).q as isize) = 2 as i32 as i8;
        } else {
            return 1 as i32
        }
    }
    *((*npp).c_value).offset((*info).q as isize) = (*info).bnd
        - *((*npp).c_value).offset((*info).q as isize);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_dbnd_col(mut npp: *mut NPP, mut q: *mut NPPCOL) {
    let mut info: *mut dbnd_col = 0 as *mut dbnd_col;
    let mut p: *mut NPPROW = 0 as *mut NPPROW;
    let mut s: *mut NPPCOL = 0 as *mut NPPCOL;
    ((*q).lb == 0.0f64
        || {
            glp_assert_(
                b"q->lb == 0.0\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                966 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*q).ub > 0.0f64
        || {
            glp_assert_(
                b"q->ub > 0.0\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                967 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*q).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->ub != +DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                968 as i32,
            );
            1 as i32 != 0
        }) as i32;
    s = _glp_npp_add_col(npp);
    (*s).is_int = (*q).is_int;
    (*s).lb = 0.0f64;
    (*s).ub = 1.7976931348623157e+308f64;
    p = _glp_npp_add_row(npp);
    (*p).ub = (*q).ub;
    (*p).lb = (*p).ub;
    _glp_npp_add_aij(npp, p, q, 1.0f64);
    _glp_npp_add_aij(npp, p, s, 1.0f64);
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_dbnd_col as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<dbnd_col>() as u64 as i32,
    ) as *mut dbnd_col;
    (*info).q = (*q).j;
    (*info).s = (*s).j;
    (*q).ub = 1.7976931348623157e+308f64;
}
unsafe extern "C" fn rcv_dbnd_col(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut dbnd_col = _info as *mut dbnd_col;
    if (*npp).sol == 1 as i32 {
        if *((*npp).c_stat).offset((*info).q as isize) as i32 == 1 as i32 {
            if *((*npp).c_stat).offset((*info).s as isize) as i32 == 1 as i32 {
                *((*npp).c_stat).offset((*info).q as isize) = 1 as i32 as i8;
            } else if *((*npp).c_stat).offset((*info).s as isize) as i32 == 2 as i32 {
                *((*npp).c_stat).offset((*info).q as isize) = 3 as i32 as i8;
            } else {
                return 1 as i32
            }
        } else if *((*npp).c_stat).offset((*info).q as isize) as i32 == 2 as i32 {
            if *((*npp).c_stat).offset((*info).s as isize) as i32 == 1 as i32
                || *((*npp).c_stat).offset((*info).s as isize) as i32 == 2 as i32
            {
                *((*npp).c_stat).offset((*info).q as isize) = 2 as i32 as i8;
            } else {
                return 1 as i32
            }
        } else {
            return 1 as i32
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_fixed_col(mut npp: *mut NPP, mut q: *mut NPPCOL) {
    let mut info: *mut fixed_col = 0 as *mut fixed_col;
    let mut i: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    ((*q).lb == (*q).ub
        || {
            glp_assert_(
                b"q->lb == q->ub\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                1110 as i32,
            );
            1 as i32 != 0
        }) as i32;
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_fixed_col as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<fixed_col>() as u64 as i32,
    ) as *mut fixed_col;
    (*info).q = (*q).j;
    (*info).s = (*q).lb;
    (*npp).c0 += (*q).coef * (*q).lb;
    aij = (*q).ptr;
    while !aij.is_null() {
        i = (*aij).row;
        if (*i).lb == (*i).ub {
            (*i).lb -= (*aij).val * (*q).lb;
            (*i).ub = (*i).lb;
        } else {
            if (*i).lb != -1.7976931348623157e+308f64 {
                (*i).lb -= (*aij).val * (*q).lb;
            }
            if (*i).ub != 1.7976931348623157e+308f64 {
                (*i).ub -= (*aij).val * (*q).lb;
            }
        }
        aij = (*aij).c_next;
    }
    _glp_npp_del_col(npp, q);
}
unsafe extern "C" fn rcv_fixed_col(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut fixed_col = _info as *mut fixed_col;
    if (*npp).sol == 1 as i32 {
        *((*npp).c_stat).offset((*info).q as isize) = 5 as i32 as i8;
    }
    *((*npp).c_value).offset((*info).q as isize) = (*info).s;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_make_equality(
    mut npp: *mut NPP,
    mut p: *mut NPPROW,
) -> i32 {
    let mut info: *mut make_equality = 0 as *mut make_equality;
    let mut b: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut nint: libc::c_double = 0.;
    ((*p).lb != -1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->lb != -DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                1229 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*p).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->ub != +DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                1230 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*p).lb < (*p).ub
        || {
            glp_assert_(
                b"p->lb < p->ub\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                1231 as i32,
            );
            1 as i32 != 0
        }) as i32;
    eps = 1e-9f64 + 1e-12f64 * fabs((*p).lb);
    if (*p).ub - (*p).lb > eps {
        return 0 as i32;
    }
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_make_equality as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32,
        ),
        ::core::mem::size_of::<make_equality>() as u64 as i32,
    ) as *mut make_equality;
    (*info).p = (*p).i;
    b = 0.5f64 * ((*p).ub + (*p).lb);
    nint = floor(b + 0.5f64);
    if fabs(b - nint) <= eps {
        b = nint;
    }
    (*p).ub = b;
    (*p).lb = (*p).ub;
    return 1 as i32;
}
unsafe extern "C" fn rcv_make_equality(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut make_equality = _info as *mut make_equality;
    if (*npp).sol == 1 as i32 {
        if *((*npp).r_stat).offset((*info).p as isize) as i32 == 1 as i32 {
            *((*npp).r_stat).offset((*info).p as isize) = 1 as i32 as i8;
        } else if *((*npp).r_stat).offset((*info).p as isize) as i32 == 5 as i32 {
            if *((*npp).r_pi).offset((*info).p as isize) >= 0.0f64 {
                *((*npp).r_stat).offset((*info).p as isize) = 2 as i32 as i8;
            } else {
                *((*npp).r_stat).offset((*info).p as isize) = 3 as i32 as i8;
            }
        } else {
            return 1 as i32
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_make_fixed(
    mut npp: *mut NPP,
    mut q: *mut NPPCOL,
) -> i32 {
    let mut info: *mut make_fixed = 0 as *mut make_fixed;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut s: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut nint: libc::c_double = 0.;
    ((*q).lb != -1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->lb != -DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                1371 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*q).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"q->ub != +DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                1372 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const i8,
                b"npp/npp2.c\0" as *const u8 as *const i8,
                1373 as i32,
            );
            1 as i32 != 0
        }) as i32;
    eps = 1e-9f64 + 1e-12f64 * fabs((*q).lb);
    if (*q).ub - (*q).lb > eps {
        return 0 as i32;
    }
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_make_fixed as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<make_fixed>() as u64 as i32,
    ) as *mut make_fixed;
    (*info).q = (*q).j;
    (*info).c = (*q).coef;
    (*info).ptr = 0 as *mut NPPLFE;
    if (*npp).sol == 1 as i32 {
        aij = (*q).ptr;
        while !aij.is_null() {
            lfe = _glp_dmp_get_atom(
                (*npp).stack,
                ::core::mem::size_of::<NPPLFE>() as u64 as i32,
            ) as *mut NPPLFE;
            (*lfe).ref_0 = (*(*aij).row).i;
            (*lfe).val = (*aij).val;
            (*lfe).next = (*info).ptr;
            (*info).ptr = lfe;
            aij = (*aij).c_next;
        }
    }
    s = 0.5f64 * ((*q).ub + (*q).lb);
    nint = floor(s + 0.5f64);
    if fabs(s - nint) <= eps {
        s = nint;
    }
    (*q).ub = s;
    (*q).lb = (*q).ub;
    return 1 as i32;
}
unsafe extern "C" fn rcv_make_fixed(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut make_fixed = _info as *mut make_fixed;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut lambda: libc::c_double = 0.;
    if (*npp).sol == 1 as i32 {
        if *((*npp).c_stat).offset((*info).q as isize) as i32 == 1 as i32 {
            *((*npp).c_stat).offset((*info).q as isize) = 1 as i32 as i8;
        } else if *((*npp).c_stat).offset((*info).q as isize) as i32 == 5 as i32 {
            lambda = (*info).c;
            lfe = (*info).ptr;
            while !lfe.is_null() {
                lambda -= (*lfe).val * *((*npp).r_pi).offset((*lfe).ref_0 as isize);
                lfe = (*lfe).next;
            }
            if lambda >= 0.0f64 {
                *((*npp).c_stat).offset((*info).q as isize) = 2 as i32 as i8;
            } else {
                *((*npp).c_stat).offset((*info).q as isize) = 3 as i32 as i8;
            }
        } else {
            return 1 as i32
        }
    }
    return 0 as i32;
}