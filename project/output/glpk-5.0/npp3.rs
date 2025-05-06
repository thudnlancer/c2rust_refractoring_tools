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
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_npp_add_aij(
        npp: *mut NPP,
        row: *mut NPPROW,
        col: *mut NPPCOL,
        val: libc::c_double,
    ) -> *mut NPPAIJ;
    fn _glp_npp_col_nnz(npp: *mut NPP, col: *mut NPPCOL) -> i32;
    fn _glp_npp_push_tse(
        npp: *mut NPP,
        func: Option<unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32>,
        size: i32,
    ) -> *mut libc::c_void;
    fn _glp_npp_del_row(npp: *mut NPP, row: *mut NPPROW);
    fn _glp_npp_del_col(npp: *mut NPP, col: *mut NPPCOL);
    fn _glp_npp_del_aij(npp: *mut NPP, aij: *mut NPPAIJ);
    fn _glp_npp_free_row(npp: *mut NPP, p: *mut NPPROW);
    fn _glp_npp_fixed_col(npp: *mut NPP, q: *mut NPPCOL);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
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
pub struct empty_col {
    pub q: i32,
    pub stat: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eq_singlet {
    pub p: i32,
    pub q: i32,
    pub apq: libc::c_double,
    pub c: libc::c_double,
    pub ptr: *mut NPPLFE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ineq_singlet {
    pub p: i32,
    pub q: i32,
    pub apq: libc::c_double,
    pub c: libc::c_double,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub lb_changed: i8,
    pub ub_changed: i8,
    pub ptr: *mut NPPLFE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct implied_slack {
    pub p: i32,
    pub q: i32,
    pub apq: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
    pub ptr: *mut NPPLFE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct implied_free {
    pub p: i32,
    pub stat: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eq_doublet {
    pub p: i32,
    pub apq: libc::c_double,
    pub ptr: *mut NPPLFE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct forcing_col {
    pub j: i32,
    pub stat: i8,
    pub a: libc::c_double,
    pub c: libc::c_double,
    pub ptr: *mut NPPLFE,
    pub next: *mut forcing_col,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct forcing_row {
    pub p: i32,
    pub stat: i8,
    pub ptr: *mut forcing_col,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inactive_bound {
    pub p: i32,
    pub stat: i8,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_empty_row(
    mut npp: *mut NPP,
    mut p: *mut NPPROW,
) -> i32 {
    let mut eps: libc::c_double = 1e-3f64;
    (((*p).ptr).is_null()
        || {
            glp_assert_(
                b"p->ptr == NULL\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                78 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*p).lb > eps || (*p).ub < -eps {
        return 1 as i32;
    }
    (*p).lb = -1.7976931348623157e+308f64;
    (*p).ub = 1.7976931348623157e+308f64;
    _glp_npp_free_row(npp, p);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_empty_col(
    mut npp: *mut NPP,
    mut q: *mut NPPCOL,
) -> i32 {
    let mut current_block: u64;
    let mut info: *mut empty_col = 0 as *mut empty_col;
    let mut eps: libc::c_double = 1e-3f64;
    (((*q).ptr).is_null()
        || {
            glp_assert_(
                b"q->ptr == NULL\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                186 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*q).coef > eps && (*q).lb == -1.7976931348623157e+308f64 {
        return 1 as i32;
    }
    if (*q).coef < -eps && (*q).ub == 1.7976931348623157e+308f64 {
        return 1 as i32;
    }
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_empty_col as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<empty_col>() as u64 as i32,
    ) as *mut empty_col;
    (*info).q = (*q).j;
    if (*q).lb == -1.7976931348623157e+308f64 && (*q).ub == 1.7976931348623157e+308f64 {
        (*info).stat = 4 as i32 as i8;
        (*q).ub = 0.0f64;
        (*q).lb = (*q).ub;
    } else {
        if (*q).ub == 1.7976931348623157e+308f64 {
            current_block = 1785146851165905210;
        } else {
            if (*q).lb == -1.7976931348623157e+308f64 {
                current_block = 7476422703551009570;
            } else if (*q).lb != (*q).ub {
                if (*q).coef >= 2.2204460492503131e-16f64 {
                    current_block = 1785146851165905210;
                } else if (*q).coef <= -2.2204460492503131e-16f64 {
                    current_block = 7476422703551009570;
                } else if fabs((*q).lb) <= fabs((*q).ub) {
                    current_block = 1785146851165905210;
                } else {
                    current_block = 7476422703551009570;
                }
            } else {
                (*info).stat = 5 as i32 as i8;
                current_block = 4495394744059808450;
            }
            match current_block {
                4495394744059808450 => {}
                1785146851165905210 => {}
                _ => {
                    (*info).stat = 3 as i32 as i8;
                    (*q).lb = (*q).ub;
                    current_block = 4495394744059808450;
                }
            }
        }
        match current_block {
            4495394744059808450 => {}
            _ => {
                (*info).stat = 2 as i32 as i8;
                (*q).ub = (*q).lb;
            }
        }
    }
    _glp_npp_fixed_col(npp, q);
    return 0 as i32;
}
unsafe extern "C" fn rcv_empty_col(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut empty_col = _info as *mut empty_col;
    if (*npp).sol == 1 as i32 {
        *((*npp).c_stat).offset((*info).q as isize) = (*info).stat;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_implied_value(
    mut npp: *mut NPP,
    mut q: *mut NPPCOL,
    mut s: libc::c_double,
) -> i32 {
    let mut eps: libc::c_double = 0.;
    let mut nint: libc::c_double = 0.;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                291 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                293 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*q).is_int != 0 {
        nint = floor(s + 0.5f64);
        if fabs(s - nint) <= 1e-5f64 {
            s = nint;
        } else {
            return 2 as i32
        }
    }
    if (*q).lb != -1.7976931348623157e+308f64 {
        eps = if (*q).is_int as i32 != 0 {
            1e-5f64
        } else {
            1e-5f64 + 1e-8f64 * fabs((*q).lb)
        };
        if s < (*q).lb - eps {
            return 1 as i32;
        }
        if s < (*q).lb + 1e-3f64 * eps {
            (*q).ub = (*q).lb;
            return 0 as i32;
        }
    }
    if (*q).ub != 1.7976931348623157e+308f64 {
        eps = if (*q).is_int as i32 != 0 {
            1e-5f64
        } else {
            1e-5f64 + 1e-8f64 * fabs((*q).ub)
        };
        if s > (*q).ub + eps {
            return 1 as i32;
        }
        if s > (*q).ub - 1e-3f64 * eps {
            (*q).lb = (*q).ub;
            return 0 as i32;
        }
    }
    (*q).ub = s;
    (*q).lb = (*q).ub;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_eq_singlet(
    mut npp: *mut NPP,
    mut p: *mut NPPROW,
) -> i32 {
    let mut info: *mut eq_singlet = 0 as *mut eq_singlet;
    let mut q: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut ret: i32 = 0;
    let mut s: libc::c_double = 0.;
    ((*p).lb == (*p).ub
        || {
            glp_assert_(
                b"p->lb == p->ub\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                430 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!((*p).ptr).is_null() && ((*(*p).ptr).r_next).is_null()
        || {
            glp_assert_(
                b"p->ptr != NULL && p->ptr->r_next == NULL\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                431 as i32,
            );
            1 as i32 != 0
        }) as i32;
    aij = (*p).ptr;
    q = (*aij).col;
    s = (*p).lb / (*aij).val;
    ret = _glp_npp_implied_value(npp, q, s);
    (0 as i32 <= ret && ret <= 2 as i32
        || {
            glp_assert_(
                b"0 <= ret && ret <= 2\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                437 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if ret != 0 as i32 {
        return ret;
    }
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_eq_singlet as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<eq_singlet>() as u64 as i32,
    ) as *mut eq_singlet;
    (*info).p = (*p).i;
    (*info).q = (*q).j;
    (*info).apq = (*aij).val;
    (*info).c = (*q).coef;
    (*info).ptr = 0 as *mut NPPLFE;
    if (*npp).sol != 3 as i32 {
        aij = (*q).ptr;
        while !aij.is_null() {
            if !((*aij).row == p) {
                lfe = _glp_dmp_get_atom(
                    (*npp).stack,
                    ::core::mem::size_of::<NPPLFE>() as u64 as i32,
                ) as *mut NPPLFE;
                (*lfe).ref_0 = (*(*aij).row).i;
                (*lfe).val = (*aij).val;
                (*lfe).next = (*info).ptr;
                (*info).ptr = lfe;
            }
            aij = (*aij).c_next;
        }
    }
    _glp_npp_del_row(npp, p);
    return 0 as i32;
}
unsafe extern "C" fn rcv_eq_singlet(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut eq_singlet = _info as *mut eq_singlet;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut temp: libc::c_double = 0.;
    if (*npp).sol == 1 as i32 {
        if *((*npp).c_stat).offset((*info).q as isize) as i32 != 5 as i32 {
            return 1 as i32;
        }
        *((*npp).r_stat).offset((*info).p as isize) = 5 as i32 as i8;
        *((*npp).c_stat).offset((*info).q as isize) = 1 as i32 as i8;
    }
    if (*npp).sol != 3 as i32 {
        temp = (*info).c;
        lfe = (*info).ptr;
        while !lfe.is_null() {
            temp -= (*lfe).val * *((*npp).r_pi).offset((*lfe).ref_0 as isize);
            lfe = (*lfe).next;
        }
        *((*npp).r_pi).offset((*info).p as isize) = temp / (*info).apq;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_implied_lower(
    mut npp: *mut NPP,
    mut q: *mut NPPCOL,
    mut l: libc::c_double,
) -> i32 {
    let mut current_block: u64;
    let mut ret: i32 = 0;
    let mut eps: libc::c_double = 0.;
    let mut nint: libc::c_double = 0.;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                552 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                554 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (l != -1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"l != -DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                556 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*q).is_int != 0 {
        nint = floor(l + 0.5f64);
        if fabs(l - nint) <= 1e-5f64 {
            l = nint;
        } else {
            l = ceil(l);
        }
    }
    if (*q).lb != -1.7976931348623157e+308f64 {
        eps = if (*q).is_int as i32 != 0 {
            1e-3f64
        } else {
            1e-3f64 + 1e-6f64 * fabs((*q).lb)
        };
        if l < (*q).lb + eps {
            ret = 0 as i32;
            current_block = 9172909518926851357;
        } else {
            current_block = 17965632435239708295;
        }
    } else {
        current_block = 17965632435239708295;
    }
    match current_block {
        17965632435239708295 => {
            if (*q).ub != 1.7976931348623157e+308f64 {
                eps = if (*q).is_int as i32 != 0 {
                    1e-5f64
                } else {
                    1e-5f64 + 1e-8f64 * fabs((*q).ub)
                };
                if l > (*q).ub + eps {
                    ret = 4 as i32;
                    current_block = 9172909518926851357;
                } else if l > (*q).ub - 1e-3f64 * eps {
                    (*q).lb = (*q).ub;
                    ret = 3 as i32;
                    current_block = 9172909518926851357;
                } else {
                    current_block = 8457315219000651999;
                }
            } else {
                current_block = 8457315219000651999;
            }
            match current_block {
                9172909518926851357 => {}
                _ => {
                    if (*q).lb == -1.7976931348623157e+308f64 {
                        ret = 2 as i32;
                    } else if (*q).is_int as i32 != 0 && l > (*q).lb + 0.5f64 {
                        ret = 2 as i32;
                    } else if l > (*q).lb + 0.30f64 * (1.0f64 + fabs((*q).lb)) {
                        ret = 2 as i32;
                    } else {
                        ret = 1 as i32;
                    }
                    (*q).lb = l;
                }
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_implied_upper(
    mut npp: *mut NPP,
    mut q: *mut NPPCOL,
    mut u: libc::c_double,
) -> i32 {
    let mut current_block: u64;
    let mut ret: i32 = 0;
    let mut eps: libc::c_double = 0.;
    let mut nint: libc::c_double = 0.;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                664 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                666 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (u != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"u != +DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                668 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*q).is_int != 0 {
        nint = floor(u + 0.5f64);
        if fabs(u - nint) <= 1e-5f64 {
            u = nint;
        } else {
            u = floor(u);
        }
    }
    if (*q).ub != 1.7976931348623157e+308f64 {
        eps = if (*q).is_int as i32 != 0 {
            1e-3f64
        } else {
            1e-3f64 + 1e-6f64 * fabs((*q).ub)
        };
        if u > (*q).ub - eps {
            ret = 0 as i32;
            current_block = 17622252501650634531;
        } else {
            current_block = 17965632435239708295;
        }
    } else {
        current_block = 17965632435239708295;
    }
    match current_block {
        17965632435239708295 => {
            if (*q).lb != -1.7976931348623157e+308f64 {
                eps = if (*q).is_int as i32 != 0 {
                    1e-5f64
                } else {
                    1e-5f64 + 1e-8f64 * fabs((*q).lb)
                };
                if u < (*q).lb - eps {
                    ret = 4 as i32;
                    current_block = 17622252501650634531;
                } else if u < (*q).lb + 1e-3f64 * eps {
                    (*q).ub = (*q).lb;
                    ret = 3 as i32;
                    current_block = 17622252501650634531;
                } else {
                    current_block = 8457315219000651999;
                }
            } else {
                current_block = 8457315219000651999;
            }
            match current_block {
                17622252501650634531 => {}
                _ => {
                    if (*q).ub == 1.7976931348623157e+308f64 {
                        ret = 2 as i32;
                    } else if (*q).is_int as i32 != 0 && u < (*q).ub - 0.5f64 {
                        ret = 2 as i32;
                    } else if u < (*q).ub - 0.30f64 * (1.0f64 + fabs((*q).ub)) {
                        ret = 2 as i32;
                    } else {
                        ret = 1 as i32;
                    }
                    (*q).ub = u;
                }
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_ineq_singlet(
    mut npp: *mut NPP,
    mut p: *mut NPPROW,
) -> i32 {
    let mut info: *mut ineq_singlet = 0 as *mut ineq_singlet;
    let mut q: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut apq: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut lb_changed: i32 = 0;
    let mut ub_changed: i32 = 0;
    let mut ll: libc::c_double = 0.;
    let mut uu: libc::c_double = 0.;
    ((*p).lb != -1.7976931348623157e+308f64 || (*p).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->lb != -DBL_MAX || p->ub != +DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                891 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*p).lb < (*p).ub
        || {
            glp_assert_(
                b"p->lb < p->ub\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                892 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!((*p).ptr).is_null() && ((*(*p).ptr).r_next).is_null()
        || {
            glp_assert_(
                b"p->ptr != NULL && p->ptr->r_next == NULL\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                893 as i32,
            );
            1 as i32 != 0
        }) as i32;
    apq = (*p).ptr;
    q = (*apq).col;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                897 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*apq).val > 0.0f64 {
        ll = if (*p).lb == -1.7976931348623157e+308f64 {
            -1.7976931348623157e+308f64
        } else {
            (*p).lb / (*apq).val
        };
        uu = if (*p).ub == 1.7976931348623157e+308f64 {
            1.7976931348623157e+308f64
        } else {
            (*p).ub / (*apq).val
        };
    } else {
        ll = if (*p).ub == 1.7976931348623157e+308f64 {
            -1.7976931348623157e+308f64
        } else {
            (*p).ub / (*apq).val
        };
        uu = if (*p).lb == -1.7976931348623157e+308f64 {
            1.7976931348623157e+308f64
        } else {
            (*p).lb / (*apq).val
        };
    }
    if ll == -1.7976931348623157e+308f64 {
        lb_changed = 0 as i32;
    } else {
        lb_changed = _glp_npp_implied_lower(npp, q, ll);
        (0 as i32 <= lb_changed && lb_changed <= 4 as i32
            || {
                glp_assert_(
                    b"0 <= lb_changed && lb_changed <= 4\0" as *const u8 as *const i8,
                    b"npp/npp3.c\0" as *const u8 as *const i8,
                    911 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if lb_changed == 4 as i32 {
            return 4 as i32;
        }
    }
    if uu == 1.7976931348623157e+308f64 {
        ub_changed = 0 as i32;
    } else if lb_changed == 3 as i32 {
        ub_changed = 0 as i32;
    } else {
        ub_changed = _glp_npp_implied_upper(npp, q, uu);
        (0 as i32 <= ub_changed && ub_changed <= 4 as i32
            || {
                glp_assert_(
                    b"0 <= ub_changed && ub_changed <= 4\0" as *const u8 as *const i8,
                    b"npp/npp3.c\0" as *const u8 as *const i8,
                    924 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if ub_changed == 4 as i32 {
            return 4 as i32;
        }
    }
    if lb_changed == 0 && ub_changed == 0 {
        (*p).lb = -1.7976931348623157e+308f64;
        (*p).ub = 1.7976931348623157e+308f64;
        _glp_npp_free_row(npp, p);
        return 0 as i32;
    }
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_ineq_singlet as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32,
        ),
        ::core::mem::size_of::<ineq_singlet>() as u64 as i32,
    ) as *mut ineq_singlet;
    (*info).p = (*p).i;
    (*info).q = (*q).j;
    (*info).apq = (*apq).val;
    (*info).c = (*q).coef;
    (*info).lb = (*p).lb;
    (*info).ub = (*p).ub;
    (*info).lb_changed = lb_changed as i8;
    (*info).ub_changed = ub_changed as i8;
    (*info).ptr = 0 as *mut NPPLFE;
    if (*npp).sol != 3 as i32 {
        aij = (*q).ptr;
        while !aij.is_null() {
            if !(aij == apq) {
                lfe = _glp_dmp_get_atom(
                    (*npp).stack,
                    ::core::mem::size_of::<NPPLFE>() as u64 as i32,
                ) as *mut NPPLFE;
                (*lfe).ref_0 = (*(*aij).row).i;
                (*lfe).val = (*aij).val;
                (*lfe).next = (*info).ptr;
                (*info).ptr = lfe;
            }
            aij = (*aij).c_next;
        }
    }
    _glp_npp_del_row(npp, p);
    return if lb_changed >= ub_changed { lb_changed } else { ub_changed };
}
unsafe extern "C" fn rcv_ineq_singlet(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut ineq_singlet = _info as *mut ineq_singlet;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut lambda: libc::c_double = 0.;
    if !((*npp).sol == 3 as i32) {
        lambda = (*info).c;
        lfe = (*info).ptr;
        while !lfe.is_null() {
            lambda -= (*lfe).val * *((*npp).r_pi).offset((*lfe).ref_0 as isize);
            lfe = (*lfe).next;
        }
        if (*npp).sol == 1 as i32 {
            if *((*npp).c_stat).offset((*info).q as isize) as i32 == 1 as i32 {
                *((*npp).r_stat).offset((*info).p as isize) = 1 as i32 as i8;
                *((*npp).r_pi).offset((*info).p as isize) = 0.0f64;
            } else {
                let mut current_block_45: u64;
                if *((*npp).c_stat).offset((*info).q as isize) as i32 == 2 as i32 {
                    current_block_45 = 1916791224977129440;
                } else {
                    if *((*npp).c_stat).offset((*info).q as isize) as i32 == 3 as i32 {
                        current_block_45 = 8505709489845189822;
                    } else if *((*npp).c_stat).offset((*info).q as isize) as i32
                        == 5 as i32
                    {
                        if lambda > 1e-7f64 {
                            if (*info).apq > 0.0f64
                                && (*info).lb != -1.7976931348623157e+308f64
                                || (*info).apq < 0.0f64
                                    && (*info).ub != 1.7976931348623157e+308f64
                                || (*info).lb_changed == 0
                            {
                                *((*npp).c_stat).offset((*info).q as isize) = 2 as i32
                                    as i8;
                                current_block_45 = 1916791224977129440;
                            } else {
                                current_block_45 = 15345278821338558188;
                            }
                        } else {
                            current_block_45 = 15345278821338558188;
                        }
                        match current_block_45 {
                            1916791224977129440 => {}
                            _ => {
                                if lambda < -1e-7f64 {
                                    if (*info).apq > 0.0f64
                                        && (*info).ub != 1.7976931348623157e+308f64
                                        || (*info).apq < 0.0f64
                                            && (*info).lb != -1.7976931348623157e+308f64
                                        || (*info).ub_changed == 0
                                    {
                                        *((*npp).c_stat).offset((*info).q as isize) = 3 as i32
                                            as i8;
                                        current_block_45 = 8505709489845189822;
                                    } else {
                                        current_block_45 = 17281240262373992796;
                                    }
                                } else {
                                    current_block_45 = 17281240262373992796;
                                }
                                match current_block_45 {
                                    8505709489845189822 => {}
                                    _ => {
                                        if (*info).lb != -1.7976931348623157e+308f64
                                            && (*info).ub == 1.7976931348623157e+308f64
                                        {
                                            *((*npp).r_stat).offset((*info).p as isize) = 2 as i32
                                                as i8;
                                        } else if (*info).lb == -1.7976931348623157e+308f64
                                            && (*info).ub != 1.7976931348623157e+308f64
                                        {
                                            *((*npp).r_stat).offset((*info).p as isize) = 3 as i32
                                                as i8;
                                        } else if (*info).lb != -1.7976931348623157e+308f64
                                            && (*info).ub != 1.7976931348623157e+308f64
                                        {
                                            if (*info).apq
                                                * *((*npp).c_value).offset((*info).q as isize)
                                                <= 0.5f64 * ((*info).lb + (*info).ub)
                                            {
                                                *((*npp).r_stat).offset((*info).p as isize) = 2 as i32
                                                    as i8;
                                            } else {
                                                *((*npp).r_stat).offset((*info).p as isize) = 3 as i32
                                                    as i8;
                                            }
                                        } else {
                                            return 1 as i32
                                        }
                                        *((*npp).c_stat).offset((*info).q as isize) = 1 as i32
                                            as i8;
                                        *((*npp).r_pi).offset((*info).p as isize) = lambda
                                            / (*info).apq;
                                        current_block_45 = 3160140712158701372;
                                    }
                                }
                            }
                        }
                    } else {
                        return 1 as i32
                    }
                    match current_block_45 {
                        1916791224977129440 => {}
                        3160140712158701372 => {}
                        _ => {
                            if (*info).ub_changed != 0 {
                                *((*npp).r_stat).offset((*info).p as isize) = (if (*info)
                                    .apq > 0.0f64
                                {
                                    3 as i32
                                } else {
                                    2 as i32
                                }) as i8;
                                *((*npp).c_stat).offset((*info).q as isize) = 1 as i32
                                    as i8;
                                *((*npp).r_pi).offset((*info).p as isize) = lambda
                                    / (*info).apq;
                            } else {
                                *((*npp).r_stat).offset((*info).p as isize) = 1 as i32
                                    as i8;
                                *((*npp).r_pi).offset((*info).p as isize) = 0.0f64;
                            }
                            current_block_45 = 3160140712158701372;
                        }
                    }
                }
                match current_block_45 {
                    1916791224977129440 => {
                        if (*info).lb_changed != 0 {
                            *((*npp).r_stat).offset((*info).p as isize) = (if (*info).apq
                                > 0.0f64
                            {
                                2 as i32
                            } else {
                                3 as i32
                            }) as i8;
                            *((*npp).c_stat).offset((*info).q as isize) = 1 as i32 as i8;
                            *((*npp).r_pi).offset((*info).p as isize) = lambda
                                / (*info).apq;
                        } else {
                            *((*npp).r_stat).offset((*info).p as isize) = 1 as i32 as i8;
                            *((*npp).r_pi).offset((*info).p as isize) = 0.0f64;
                        }
                    }
                    _ => {}
                }
            }
        }
        if (*npp).sol == 2 as i32 {
            if lambda > 2.2204460492503131e-16f64 && (*info).lb_changed as i32 != 0
                || lambda < -2.2204460492503131e-16f64 && (*info).ub_changed as i32 != 0
            {
                *((*npp).r_pi).offset((*info).p as isize) = lambda / (*info).apq;
            } else {
                *((*npp).r_pi).offset((*info).p as isize) = 0.0f64;
            }
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_implied_slack(mut npp: *mut NPP, mut q: *mut NPPCOL) {
    let mut info: *mut implied_slack = 0 as *mut implied_slack;
    let mut p: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    ((*q).is_int == 0
        || {
            glp_assert_(
                b"!q->is_int\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                1244 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                1245 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!((*q).ptr).is_null() && ((*(*q).ptr).c_next).is_null()
        || {
            glp_assert_(
                b"q->ptr != NULL && q->ptr->c_next == NULL\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                1246 as i32,
            );
            1 as i32 != 0
        }) as i32;
    aij = (*q).ptr;
    p = (*aij).row;
    ((*p).lb == (*p).ub
        || {
            glp_assert_(
                b"p->lb == p->ub\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                1250 as i32,
            );
            1 as i32 != 0
        }) as i32;
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_implied_slack as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32,
        ),
        ::core::mem::size_of::<implied_slack>() as u64 as i32,
    ) as *mut implied_slack;
    (*info).p = (*p).i;
    (*info).q = (*q).j;
    (*info).apq = (*aij).val;
    (*info).b = (*p).lb;
    (*info).c = (*q).coef;
    (*info).ptr = 0 as *mut NPPLFE;
    aij = (*p).ptr;
    while !aij.is_null() {
        if !((*aij).col == q) {
            lfe = _glp_dmp_get_atom(
                (*npp).stack,
                ::core::mem::size_of::<NPPLFE>() as u64 as i32,
            ) as *mut NPPLFE;
            (*lfe).ref_0 = (*(*aij).col).j;
            (*lfe).val = (*aij).val;
            (*lfe).next = (*info).ptr;
            (*info).ptr = lfe;
            (*(*aij).col).coef -= (*info).c * ((*aij).val / (*info).apq);
        }
        aij = (*aij).r_next;
    }
    (*npp).c0 += (*info).c * ((*info).b / (*info).apq);
    if (*info).apq > 0.0f64 {
        (*p).lb = if (*q).ub == 1.7976931348623157e+308f64 {
            -1.7976931348623157e+308f64
        } else {
            (*info).b - (*info).apq * (*q).ub
        };
        (*p).ub = if (*q).lb == -1.7976931348623157e+308f64 {
            1.7976931348623157e+308f64
        } else {
            (*info).b - (*info).apq * (*q).lb
        };
    } else {
        (*p).lb = if (*q).lb == -1.7976931348623157e+308f64 {
            -1.7976931348623157e+308f64
        } else {
            (*info).b - (*info).apq * (*q).lb
        };
        (*p).ub = if (*q).ub == 1.7976931348623157e+308f64 {
            1.7976931348623157e+308f64
        } else {
            (*info).b - (*info).apq * (*q).ub
        };
    }
    _glp_npp_del_col(npp, q);
}
unsafe extern "C" fn rcv_implied_slack(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut implied_slack = _info as *mut implied_slack;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut temp: libc::c_double = 0.;
    if (*npp).sol == 1 as i32 {
        if *((*npp).r_stat).offset((*info).p as isize) as i32 == 1 as i32
            || *((*npp).r_stat).offset((*info).p as isize) as i32 == 4 as i32
        {
            *((*npp).c_stat).offset((*info).q as isize) = *((*npp).r_stat)
                .offset((*info).p as isize);
        } else if *((*npp).r_stat).offset((*info).p as isize) as i32 == 2 as i32 {
            *((*npp).c_stat).offset((*info).q as isize) = (if (*info).apq > 0.0f64 {
                3 as i32
            } else {
                2 as i32
            }) as i8;
        } else if *((*npp).r_stat).offset((*info).p as isize) as i32 == 3 as i32 {
            *((*npp).c_stat).offset((*info).q as isize) = (if (*info).apq > 0.0f64 {
                2 as i32
            } else {
                3 as i32
            }) as i8;
        } else {
            return 1 as i32
        }
        *((*npp).r_stat).offset((*info).p as isize) = 5 as i32 as i8;
    }
    if (*npp).sol != 3 as i32 {
        *((*npp).r_pi).offset((*info).p as isize) += (*info).c / (*info).apq;
    }
    temp = (*info).b;
    lfe = (*info).ptr;
    while !lfe.is_null() {
        temp -= (*lfe).val * *((*npp).c_value).offset((*lfe).ref_0 as isize);
        lfe = (*lfe).next;
    }
    *((*npp).c_value).offset((*info).q as isize) = temp / (*info).apq;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_implied_free(
    mut npp: *mut NPP,
    mut q: *mut NPPCOL,
) -> i32 {
    let mut current_block: u64;
    let mut info: *mut implied_free = 0 as *mut implied_free;
    let mut p: *mut NPPROW = 0 as *mut NPPROW;
    let mut apq: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut l: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut pi: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    ((*q).lb < (*q).ub
        || {
            glp_assert_(
                b"q->lb < q->ub\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                1494 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!((*q).ptr).is_null() && ((*(*q).ptr).c_next).is_null()
        || {
            glp_assert_(
                b"q->ptr != NULL && q->ptr->c_next == NULL\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                1495 as i32,
            );
            1 as i32 != 0
        }) as i32;
    apq = (*q).ptr;
    p = (*apq).row;
    ((*p).lb != -1.7976931348623157e+308f64 || (*p).ub != 1.7976931348623157e+308f64
        || {
            glp_assert_(
                b"p->lb != -DBL_MAX || p->ub != +DBL_MAX\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                1499 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*p).lb < (*p).ub
        || {
            glp_assert_(
                b"p->lb < p->ub\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                1500 as i32,
            );
            1 as i32 != 0
        }) as i32;
    alfa = (*p).lb;
    if alfa != -1.7976931348623157e+308f64 {
        aij = (*p).ptr;
        while !aij.is_null() {
            if !(aij == apq) {
                if (*aij).val > 0.0f64 {
                    if (*(*aij).col).ub == 1.7976931348623157e+308f64 {
                        alfa = -1.7976931348623157e+308f64;
                        break;
                    } else {
                        alfa -= (*aij).val * (*(*aij).col).ub;
                    }
                } else if (*(*aij).col).lb == -1.7976931348623157e+308f64 {
                    alfa = -1.7976931348623157e+308f64;
                    break;
                } else {
                    alfa -= (*aij).val * (*(*aij).col).lb;
                }
            }
            aij = (*aij).r_next;
        }
    }
    beta = (*p).ub;
    if beta != 1.7976931348623157e+308f64 {
        aij = (*p).ptr;
        while !aij.is_null() {
            if !(aij == apq) {
                if (*aij).val > 0.0f64 {
                    if (*(*aij).col).lb == -1.7976931348623157e+308f64 {
                        beta = 1.7976931348623157e+308f64;
                        break;
                    } else {
                        beta -= (*aij).val * (*(*aij).col).lb;
                    }
                } else if (*(*aij).col).ub == 1.7976931348623157e+308f64 {
                    beta = 1.7976931348623157e+308f64;
                    break;
                } else {
                    beta -= (*aij).val * (*(*aij).col).ub;
                }
            }
            aij = (*aij).r_next;
        }
    }
    if (*apq).val > 0.0f64 {
        l = if alfa == -1.7976931348623157e+308f64 {
            -1.7976931348623157e+308f64
        } else {
            alfa / (*apq).val
        };
    } else {
        l = if beta == 1.7976931348623157e+308f64 {
            -1.7976931348623157e+308f64
        } else {
            beta / (*apq).val
        };
    }
    if (*apq).val > 0.0f64 {
        u = if beta == 1.7976931348623157e+308f64 {
            1.7976931348623157e+308f64
        } else {
            beta / (*apq).val
        };
    } else {
        u = if alfa == -1.7976931348623157e+308f64 {
            1.7976931348623157e+308f64
        } else {
            alfa / (*apq).val
        };
    }
    if (*q).lb != -1.7976931348623157e+308f64 {
        eps = 1e-9f64 + 1e-12f64 * fabs((*q).lb);
        if l < (*q).lb - eps {
            return 1 as i32;
        }
    }
    if (*q).ub != 1.7976931348623157e+308f64 {
        eps = 1e-9f64 + 1e-12f64 * fabs((*q).ub);
        if u > (*q).ub + eps {
            return 1 as i32;
        }
    }
    (*q).lb = -1.7976931348623157e+308f64;
    (*q).ub = 1.7976931348623157e+308f64;
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_implied_free as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32,
        ),
        ::core::mem::size_of::<implied_free>() as u64 as i32,
    ) as *mut implied_free;
    (*info).p = (*p).i;
    (*info).stat = -(1 as i32) as i8;
    pi = (*q).coef / (*apq).val;
    if pi > 2.2204460492503131e-16f64 {
        if (*p).lb != -1.7976931348623157e+308f64 {
            current_block = 3205690711339328785;
        } else {
            if pi > 1e-5f64 {
                return 2 as i32;
            }
            ((*p).ub != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"p->ub != +DBL_MAX\0" as *const u8 as *const i8,
                        b"npp/npp3.c\0" as *const u8 as *const i8,
                        1582 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            current_block = 9362176315720294360;
        }
    } else if pi < -2.2204460492503131e-16f64 {
        if (*p).ub != 1.7976931348623157e+308f64 {
            current_block = 9362176315720294360;
        } else {
            if pi < -1e-5f64 {
                return 2 as i32;
            }
            ((*p).lb != -1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"p->lb != -DBL_MAX\0" as *const u8 as *const i8,
                        b"npp/npp3.c\0" as *const u8 as *const i8,
                        1595 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            current_block = 3205690711339328785;
        }
    } else if (*p).ub == 1.7976931348623157e+308f64 {
        ((*p).lb != -1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"p->lb != -DBL_MAX\0" as *const u8 as *const i8,
                    b"npp/npp3.c\0" as *const u8 as *const i8,
                    1602 as i32,
                );
                1 as i32 != 0
            }) as i32;
        current_block = 3205690711339328785;
    } else if (*p).lb == -1.7976931348623157e+308f64 {
        ((*p).ub != 1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"p->ub != +DBL_MAX\0" as *const u8 as *const i8,
                    b"npp/npp3.c\0" as *const u8 as *const i8,
                    1606 as i32,
                );
                1 as i32 != 0
            }) as i32;
        current_block = 9362176315720294360;
    } else if fabs((*p).lb) <= fabs((*p).ub) {
        current_block = 3205690711339328785;
    } else {
        current_block = 9362176315720294360;
    }
    match current_block {
        3205690711339328785 => {
            (*info).stat = 2 as i32 as i8;
            (*p).ub = (*p).lb;
        }
        _ => {
            (*info).stat = 3 as i32 as i8;
            (*p).lb = (*p).ub;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn rcv_implied_free(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut implied_free = _info as *mut implied_free;
    if (*npp).sol == 1 as i32 {
        if *((*npp).r_stat).offset((*info).p as isize) as i32 == 1 as i32 {
            *((*npp).r_stat).offset((*info).p as isize) = 1 as i32 as i8;
        } else if *((*npp).r_stat).offset((*info).p as isize) as i32 == 5 as i32 {
            ((*info).stat as i32 == 2 as i32 || (*info).stat as i32 == 3 as i32
                || {
                    glp_assert_(
                        b"info->stat == GLP_NL || info->stat == GLP_NU\0" as *const u8
                            as *const i8,
                        b"npp/npp3.c\0" as *const u8 as *const i8,
                        1621 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *((*npp).r_stat).offset((*info).p as isize) = (*info).stat;
        } else {
            return 1 as i32
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_eq_doublet(
    mut npp: *mut NPP,
    mut p: *mut NPPROW,
) -> *mut NPPCOL {
    let mut info: *mut eq_doublet = 0 as *mut eq_doublet;
    let mut i: *mut NPPROW = 0 as *mut NPPROW;
    let mut q: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut r: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut apq: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut apr: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut aiq: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut air: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut next: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut gamma: libc::c_double = 0.;
    ((*p).lb == (*p).ub
        || {
            glp_assert_(
                b"p->lb == p->ub\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                1770 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (!((*p).ptr).is_null() && !((*(*p).ptr).r_next).is_null()
        && ((*(*(*p).ptr).r_next).r_next).is_null()
        || {
            glp_assert_(
                b"p->ptr != NULL && p->ptr->r_next != NULL && p->ptr->r_next->r_next == NULL\0"
                    as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                1772 as i32,
            );
            1 as i32 != 0
        }) as i32;
    let mut a1: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut a2: *mut NPPAIJ = 0 as *mut NPPAIJ;
    a1 = (*p).ptr;
    a2 = (*a1).r_next;
    if fabs((*a2).val) < 0.001f64 * fabs((*a1).val) {
        apq = a1;
        apr = a2;
    } else if fabs((*a1).val) < 0.001f64 * fabs((*a2).val) {
        apq = a2;
        apr = a1;
    } else if _glp_npp_col_nnz(npp, (*a1).col) <= _glp_npp_col_nnz(npp, (*a2).col) {
        apq = a1;
        apr = a2;
    } else {
        apq = a2;
        apr = a1;
    }
    q = (*apq).col;
    r = (*apr).col;
    info = _glp_npp_push_tse(
        npp,
        Some(rcv_eq_doublet as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32),
        ::core::mem::size_of::<eq_doublet>() as u64 as i32,
    ) as *mut eq_doublet;
    (*info).p = (*p).i;
    (*info).apq = (*apq).val;
    (*info).ptr = 0 as *mut NPPLFE;
    aiq = (*q).ptr;
    while !aiq.is_null() {
        next = (*aiq).c_next;
        if !(aiq == apq) {
            i = (*aiq).row;
            if (*npp).sol != 3 as i32 {
                lfe = _glp_dmp_get_atom(
                    (*npp).stack,
                    ::core::mem::size_of::<NPPLFE>() as u64 as i32,
                ) as *mut NPPLFE;
                (*lfe).ref_0 = (*i).i;
                (*lfe).val = (*aiq).val;
                (*lfe).next = (*info).ptr;
                (*info).ptr = lfe;
            }
            air = (*i).ptr;
            while !air.is_null() {
                if (*air).col == r {
                    break;
                }
                air = (*air).r_next;
            }
            if air.is_null() {
                air = _glp_npp_add_aij(npp, i, r, 0.0f64);
            }
            gamma = (*aiq).val / (*apq).val;
            _glp_npp_del_aij(npp, aiq);
            (*air).val -= gamma * (*apr).val;
            if fabs((*air).val) <= 1e-10f64 {
                _glp_npp_del_aij(npp, air);
            }
            if (*i).lb == (*i).ub {
                (*i).ub = (*i).lb - gamma * (*p).lb;
                (*i).lb = (*i).ub;
            } else {
                if (*i).lb != -1.7976931348623157e+308f64 {
                    (*i).lb -= gamma * (*p).lb;
                }
                if (*i).ub != 1.7976931348623157e+308f64 {
                    (*i).ub -= gamma * (*p).lb;
                }
            }
        }
        aiq = next;
    }
    return q;
}
unsafe extern "C" fn rcv_eq_doublet(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut eq_doublet = _info as *mut eq_doublet;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut gamma: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    if (*npp).sol == 1 as i32 {
        if *((*npp).r_stat).offset((*info).p as isize) as i32 != 5 as i32 {
            return 1 as i32;
        }
    }
    if (*npp).sol != 3 as i32 {
        temp = *((*npp).r_pi).offset((*info).p as isize);
        lfe = (*info).ptr;
        while !lfe.is_null() {
            gamma = (*lfe).val / (*info).apq;
            temp -= gamma * *((*npp).r_pi).offset((*lfe).ref_0 as isize);
            lfe = (*lfe).next;
        }
        *((*npp).r_pi).offset((*info).p as isize) = temp;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_forcing_row(
    mut npp: *mut NPP,
    mut p: *mut NPPROW,
    mut at: i32,
) -> i32 {
    let mut info: *mut forcing_row = 0 as *mut forcing_row;
    let mut col: *mut forcing_col = 0 as *mut forcing_col;
    let mut j: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut apj: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut big: libc::c_double = 0.;
    (at == 0 as i32 || at == 1 as i32
        || {
            glp_assert_(
                b"at == 0 || at == 1\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                2097 as i32,
            );
            1 as i32 != 0
        }) as i32;
    big = 1.0f64;
    apj = (*p).ptr;
    while !apj.is_null() {
        if big < fabs((*apj).val) {
            big = fabs((*apj).val);
        }
        apj = (*apj).r_next;
    }
    apj = (*p).ptr;
    while !apj.is_null() {
        if fabs((*apj).val) < 1e-7f64 * big {
            return 1 as i32;
        }
        apj = (*apj).r_next;
    }
    info = _glp_npp_push_tse(
        npp,
        Some(
            rcv_forcing_row as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32,
        ),
        ::core::mem::size_of::<forcing_row>() as u64 as i32,
    ) as *mut forcing_row;
    (*info).p = (*p).i;
    if (*p).lb == (*p).ub {
        (*info).stat = 5 as i32 as i8;
    } else if at == 0 as i32 {
        (*info).stat = 2 as i32 as i8;
        ((*p).lb != -1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"p->lb != -DBL_MAX\0" as *const u8 as *const i8,
                    b"npp/npp3.c\0" as *const u8 as *const i8,
                    2117 as i32,
                );
                1 as i32 != 0
            }) as i32;
    } else {
        (*info).stat = 3 as i32 as i8;
        ((*p).ub != 1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"p->ub != +DBL_MAX\0" as *const u8 as *const i8,
                    b"npp/npp3.c\0" as *const u8 as *const i8,
                    2122 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    (*info).ptr = 0 as *mut forcing_col;
    apj = (*p).ptr;
    while !apj.is_null() {
        j = (*apj).col;
        ((*j).lb < (*j).ub
            || {
                glp_assert_(
                    b"j->lb < j->ub\0" as *const u8 as *const i8,
                    b"npp/npp3.c\0" as *const u8 as *const i8,
                    2131 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if (*npp).sol != 3 as i32 {
            col = _glp_dmp_get_atom(
                (*npp).stack,
                ::core::mem::size_of::<forcing_col>() as u64 as i32,
            ) as *mut forcing_col;
            (*col).j = (*j).j;
            (*col).stat = -(1 as i32) as i8;
            (*col).a = (*apj).val;
            (*col).c = (*j).coef;
            (*col).ptr = 0 as *mut NPPLFE;
            (*col).next = (*info).ptr;
            (*info).ptr = col;
        }
        if at == 0 as i32 && (*apj).val < 0.0f64 || at != 0 as i32 && (*apj).val > 0.0f64
        {
            if (*npp).sol != 3 as i32 {
                (*col).stat = 2 as i32 as i8;
            }
            ((*j).lb != -1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"j->lb != -DBL_MAX\0" as *const u8 as *const i8,
                        b"npp/npp3.c\0" as *const u8 as *const i8,
                        2148 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*j).ub = (*j).lb;
        } else {
            if (*npp).sol != 3 as i32 {
                (*col).stat = 3 as i32 as i8;
            }
            ((*j).ub != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"j->ub != +DBL_MAX\0" as *const u8 as *const i8,
                        b"npp/npp3.c\0" as *const u8 as *const i8,
                        2155 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*j).lb = (*j).ub;
        }
        if (*npp).sol != 3 as i32 {
            aij = (*j).ptr;
            while !aij.is_null() {
                if !(aij == apj) {
                    lfe = _glp_dmp_get_atom(
                        (*npp).stack,
                        ::core::mem::size_of::<NPPLFE>() as u64 as i32,
                    ) as *mut NPPLFE;
                    (*lfe).ref_0 = (*(*aij).row).i;
                    (*lfe).val = (*aij).val;
                    (*lfe).next = (*col).ptr;
                    (*col).ptr = lfe;
                }
                aij = (*aij).c_next;
            }
        }
        apj = (*apj).r_next;
    }
    (*p).lb = -1.7976931348623157e+308f64;
    (*p).ub = 1.7976931348623157e+308f64;
    return 0 as i32;
}
unsafe extern "C" fn rcv_forcing_row(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut forcing_row = _info as *mut forcing_row;
    let mut col: *mut forcing_col = 0 as *mut forcing_col;
    let mut piv: *mut forcing_col = 0 as *mut forcing_col;
    let mut lfe: *mut NPPLFE = 0 as *mut NPPLFE;
    let mut d: libc::c_double = 0.;
    let mut big: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    if !((*npp).sol == 3 as i32) {
        if (*npp).sol == 1 as i32 {
            if *((*npp).r_stat).offset((*info).p as isize) as i32 != 1 as i32 {
                return 1 as i32;
            }
            col = (*info).ptr;
            while !col.is_null() {
                if *((*npp).c_stat).offset((*col).j as isize) as i32 != 5 as i32 {
                    return 1 as i32;
                }
                *((*npp).c_stat).offset((*col).j as isize) = (*col).stat;
                col = (*col).next;
            }
        }
        col = (*info).ptr;
        while !col.is_null() {
            d = (*col).c;
            lfe = (*col).ptr;
            while !lfe.is_null() {
                d -= (*lfe).val * *((*npp).r_pi).offset((*lfe).ref_0 as isize);
                lfe = (*lfe).next;
            }
            (*col).c = d;
            col = (*col).next;
        }
        piv = 0 as *mut forcing_col;
        big = 0.0f64;
        col = (*info).ptr;
        while !col.is_null() {
            d = (*col).c;
            temp = fabs(d / (*col).a);
            if (*col).stat as i32 == 2 as i32 {
                if d < 0.0f64 && big < temp {
                    piv = col;
                    big = temp;
                }
            } else if (*col).stat as i32 == 3 as i32 {
                if d > 0.0f64 && big < temp {
                    piv = col;
                    big = temp;
                }
            } else {
                return 1 as i32
            }
            col = (*col).next;
        }
        if !piv.is_null() {
            if (*npp).sol == 1 as i32 {
                *((*npp).r_stat).offset((*info).p as isize) = (*info).stat;
                *((*npp).c_stat).offset((*piv).j as isize) = 1 as i32 as i8;
            }
            *((*npp).r_pi).offset((*info).p as isize) = (*piv).c / (*piv).a;
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_analyze_row(
    mut npp: *mut NPP,
    mut p: *mut NPPROW,
) -> i32 {
    let mut current_block: u64;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ret: i32 = 0 as i32;
    let mut l: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                2341 as i32,
            );
            1 as i32 != 0
        }) as i32;
    l = 0.0f64;
    aij = (*p).ptr;
    while !aij.is_null() {
        if (*aij).val > 0.0f64 {
            if (*(*aij).col).lb == -1.7976931348623157e+308f64 {
                l = -1.7976931348623157e+308f64;
                break;
            } else {
                l += (*aij).val * (*(*aij).col).lb;
            }
        } else if (*(*aij).col).ub == 1.7976931348623157e+308f64 {
            l = -1.7976931348623157e+308f64;
            break;
        } else {
            l += (*aij).val * (*(*aij).col).ub;
        }
        aij = (*aij).r_next;
    }
    u = 0.0f64;
    aij = (*p).ptr;
    while !aij.is_null() {
        if (*aij).val > 0.0f64 {
            if (*(*aij).col).ub == 1.7976931348623157e+308f64 {
                u = 1.7976931348623157e+308f64;
                break;
            } else {
                u += (*aij).val * (*(*aij).col).ub;
            }
        } else if (*(*aij).col).lb == -1.7976931348623157e+308f64 {
            u = 1.7976931348623157e+308f64;
            break;
        } else {
            u += (*aij).val * (*(*aij).col).lb;
        }
        aij = (*aij).r_next;
    }
    if (*p).lb != -1.7976931348623157e+308f64 {
        eps = 1e-3f64 + 1e-6f64 * fabs((*p).lb);
        if (*p).lb - eps > u {
            ret = 0x33 as i32;
            current_block = 896571265010555497;
        } else {
            current_block = 14818589718467733107;
        }
    } else {
        current_block = 14818589718467733107;
    }
    match current_block {
        14818589718467733107 => {
            if (*p).ub != 1.7976931348623157e+308f64 {
                eps = 1e-3f64 + 1e-6f64 * fabs((*p).ub);
                if (*p).ub + eps < l {
                    ret = 0x33 as i32;
                    current_block = 896571265010555497;
                } else {
                    current_block = 2569451025026770673;
                }
            } else {
                current_block = 2569451025026770673;
            }
            match current_block {
                896571265010555497 => {}
                _ => {
                    if (*p).lb != -1.7976931348623157e+308f64 {
                        eps = 1e-9f64 + 1e-12f64 * fabs((*p).lb);
                        if (*p).lb - eps > l {
                            if (*p).lb + eps <= u {
                                ret |= 0x1 as i32;
                            } else {
                                ret |= 0x2 as i32;
                            }
                        }
                    }
                    if (*p).ub != 1.7976931348623157e+308f64 {
                        eps = 1e-9f64 + 1e-12f64 * fabs((*p).ub);
                        if (*p).ub + eps < u {
                            if (*p).ub - eps >= l {
                                ret |= 0x10 as i32;
                            } else {
                                ret |= 0x20 as i32;
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
pub unsafe extern "C" fn _glp_npp_inactive_bound(
    mut npp: *mut NPP,
    mut p: *mut NPPROW,
    mut which: i32,
) {
    let mut info: *mut inactive_bound = 0 as *mut inactive_bound;
    if (*npp).sol == 1 as i32 {
        info = _glp_npp_push_tse(
            npp,
            Some(
                rcv_inactive_bound
                    as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32,
            ),
            ::core::mem::size_of::<inactive_bound>() as u64 as i32,
        ) as *mut inactive_bound;
        (*info).p = (*p).i;
        if (*p).ub == 1.7976931348623157e+308f64 {
            (*info).stat = 2 as i32 as i8;
        } else if (*p).lb == -1.7976931348623157e+308f64 {
            (*info).stat = 3 as i32 as i8;
        } else if (*p).lb != (*p).ub {
            (*info).stat = (if which == 0 as i32 { 3 as i32 } else { 2 as i32 }) as i8;
        } else {
            (*info).stat = 5 as i32 as i8;
        }
    }
    if which == 0 as i32 {
        ((*p).lb != -1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"p->lb != -DBL_MAX\0" as *const u8 as *const i8,
                    b"npp/npp3.c\0" as *const u8 as *const i8,
                    2500 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*p).lb = -1.7976931348623157e+308f64;
    } else if which == 1 as i32 {
        ((*p).ub != 1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"p->ub != +DBL_MAX\0" as *const u8 as *const i8,
                    b"npp/npp3.c\0" as *const u8 as *const i8,
                    2504 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*p).ub = 1.7976931348623157e+308f64;
    } else {
        (which != which
            || {
                glp_assert_(
                    b"which != which\0" as *const u8 as *const i8,
                    b"npp/npp3.c\0" as *const u8 as *const i8,
                    2508 as i32,
                );
                1 as i32 != 0
            }) as i32;
    };
}
unsafe extern "C" fn rcv_inactive_bound(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut inactive_bound = _info as *mut inactive_bound;
    if (*npp).sol != 1 as i32 {
        return 1 as i32;
    }
    if *((*npp).r_stat).offset((*info).p as isize) as i32 == 1 as i32 {
        *((*npp).r_stat).offset((*info).p as isize) = 1 as i32 as i8;
    } else {
        *((*npp).r_stat).offset((*info).p as isize) = (*info).stat;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_implied_bounds(mut npp: *mut NPP, mut p: *mut NPPROW) {
    let mut apj: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut apk: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut big: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp3.c\0" as *const u8 as *const i8,
                2749 as i32,
            );
            1 as i32 != 0
        }) as i32;
    big = 1.0f64;
    apj = (*p).ptr;
    while !apj.is_null() {
        (*(*apj).col).ll.ll = -1.7976931348623157e+308f64;
        (*(*apj).col).uu.uu = 1.7976931348623157e+308f64;
        if big < fabs((*apj).val) {
            big = fabs((*apj).val);
        }
        apj = (*apj).r_next;
    }
    eps = 1e-6f64 * big;
    if (*p).lb != -1.7976931348623157e+308f64 {
        let mut current_block_39: u64;
        apk = 0 as *mut NPPAIJ;
        apj = (*p).ptr;
        loop {
            if apj.is_null() {
                current_block_39 = 10048703153582371463;
                break;
            }
            if (*apj).val > 0.0f64 && (*(*apj).col).ub == 1.7976931348623157e+308f64
                || (*apj).val < 0.0f64 && (*(*apj).col).lb == -1.7976931348623157e+308f64
            {
                if !apk.is_null() {
                    current_block_39 = 980989089337379490;
                    break;
                }
                apk = apj;
            }
            apj = (*apj).r_next;
        }
        match current_block_39 {
            10048703153582371463 => {
                temp = (*p).lb;
                apj = (*p).ptr;
                while !apj.is_null() {
                    if !(apj == apk) {
                        if (*apj).val > 0.0f64 {
                            temp -= (*apj).val * (*(*apj).col).ub;
                        } else {
                            temp -= (*apj).val * (*(*apj).col).lb;
                        }
                    }
                    apj = (*apj).r_next;
                }
                if apk.is_null() {
                    apj = (*p).ptr;
                    while !apj.is_null() {
                        if (*apj).val >= eps {
                            (*(*apj).col).ll.ll = (*(*apj).col).ub + temp / (*apj).val;
                        } else if (*apj).val <= -eps {
                            (*(*apj).col).uu.uu = (*(*apj).col).lb + temp / (*apj).val;
                        }
                        apj = (*apj).r_next;
                    }
                } else if (*apk).val >= eps {
                    (*(*apk).col).ll.ll = temp / (*apk).val;
                } else if (*apk).val <= -eps {
                    (*(*apk).col).uu.uu = temp / (*apk).val;
                }
            }
            _ => {}
        }
    }
    if (*p).ub != 1.7976931348623157e+308f64 {
        let mut current_block_72: u64;
        apk = 0 as *mut NPPAIJ;
        apj = (*p).ptr;
        loop {
            if apj.is_null() {
                current_block_72 = 14775119014532381840;
                break;
            }
            if (*apj).val > 0.0f64 && (*(*apj).col).lb == -1.7976931348623157e+308f64
                || (*apj).val < 0.0f64 && (*(*apj).col).ub == 1.7976931348623157e+308f64
            {
                if !apk.is_null() {
                    current_block_72 = 777662472977924419;
                    break;
                }
                apk = apj;
            }
            apj = (*apj).r_next;
        }
        match current_block_72 {
            14775119014532381840 => {
                temp = (*p).ub;
                apj = (*p).ptr;
                while !apj.is_null() {
                    if !(apj == apk) {
                        if (*apj).val > 0.0f64 {
                            temp -= (*apj).val * (*(*apj).col).lb;
                        } else {
                            temp -= (*apj).val * (*(*apj).col).ub;
                        }
                    }
                    apj = (*apj).r_next;
                }
                if apk.is_null() {
                    apj = (*p).ptr;
                    while !apj.is_null() {
                        if (*apj).val >= eps {
                            (*(*apj).col).uu.uu = (*(*apj).col).lb + temp / (*apj).val;
                        } else if (*apj).val <= -eps {
                            (*(*apj).col).ll.ll = (*(*apj).col).ub + temp / (*apj).val;
                        }
                        apj = (*apj).r_next;
                    }
                } else if (*apk).val >= eps {
                    (*(*apk).col).uu.uu = temp / (*apk).val;
                } else if (*apk).val <= -eps {
                    (*(*apk).col).ll.ll = temp / (*apk).val;
                }
            }
            _ => {}
        }
    }
}