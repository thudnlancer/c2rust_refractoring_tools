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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: i32);
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
    fn _glp_npp_erase_row(npp: *mut NPP, row: *mut NPPROW);
    fn _glp_npp_lbnd_col(npp: *mut NPP, q: *mut NPPCOL);
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
pub struct binarize {
    pub q: i32,
    pub j: i32,
    pub n: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elem {
    pub aj: libc::c_double,
    pub xj: *mut NPPCOL,
    pub next: *mut elem,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_binarize_prob(mut npp: *mut NPP) -> i32 {
    let mut info: *mut binarize = 0 as *mut binarize;
    let mut row: *mut NPPROW = 0 as *mut NPPROW;
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut bin: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut u: i32 = 0;
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut temp: i32 = 0;
    let mut nfails: i32 = 0;
    let mut nvars: i32 = 0;
    let mut nbins: i32 = 0;
    let mut nrows: i32 = 0;
    nrows = 0 as i32;
    nbins = nrows;
    nvars = nbins;
    nfails = nvars;
    col = (*npp).c_tail;
    while !col.is_null() {
        if !((*col).is_int == 0) {
            if !((*col).lb == (*col).ub) {
                if !((*col).lb == 0.0f64 && (*col).ub == 1.0f64) {
                    if (*col).lb < -1e6f64 || (*col).ub > 1e6f64
                        || (*col).ub - (*col).lb > 4095.0f64
                    {
                        nfails += 1;
                        nfails;
                    } else {
                        nvars += 1;
                        nvars;
                        if (*col).lb != 0.0f64 {
                            _glp_npp_lbnd_col(npp, col);
                        }
                        ((*col).lb == 0.0f64
                            || {
                                glp_assert_(
                                    b"col->lb == 0.0\0" as *const u8 as *const i8,
                                    b"npp/npp4.c\0" as *const u8 as *const i8,
                                    159 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        u = (*col).ub as i32;
                        ((*col).ub == u as libc::c_double
                            || {
                                glp_assert_(
                                    b"col->ub == (double)u\0" as *const u8 as *const i8,
                                    b"npp/npp4.c\0" as *const u8 as *const i8,
                                    161 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        if !(u == 1 as i32) {
                            n = 2 as i32;
                            temp = 4 as i32;
                            while u >= temp {
                                n += 1;
                                n;
                                temp += temp;
                            }
                            nbins += n;
                            info = _glp_npp_push_tse(
                                npp,
                                Some(
                                    rcv_binarize_prob
                                        as unsafe extern "C" fn(*mut NPP, *mut libc::c_void) -> i32,
                                ),
                                ::core::mem::size_of::<binarize>() as u64 as i32,
                            ) as *mut binarize;
                            (*info).q = (*col).j;
                            (*info).j = 0 as i32;
                            (*info).n = n;
                            if u < temp - 1 as i32 {
                                row = _glp_npp_add_row(npp);
                                nrows += 1;
                                nrows;
                                (*row).lb = -1.7976931348623157e+308f64;
                                (*row).ub = u as libc::c_double;
                            } else {
                                row = 0 as *mut NPPROW;
                            }
                            (*col).ub = 1.0f64;
                            if !row.is_null() {
                                _glp_npp_add_aij(npp, row, col, 1.0f64);
                            }
                            k = 1 as i32;
                            temp = 2 as i32;
                            while k < n {
                                bin = _glp_npp_add_col(npp);
                                (*bin).is_int = 1 as i32 as i8;
                                (*bin).lb = 0.0f64;
                                (*bin).ub = 1.0f64;
                                (*bin).coef = temp as libc::c_double * (*col).coef;
                                if (*info).j == 0 as i32 {
                                    (*info).j = (*bin).j;
                                } else {
                                    ((*info).j + (k - 1 as i32) == (*bin).j
                                        || {
                                            glp_assert_(
                                                b"info->j + (k-1) == bin->j\0" as *const u8 as *const i8,
                                                b"npp/npp4.c\0" as *const u8 as *const i8,
                                                201 as i32,
                                            );
                                            1 as i32 != 0
                                        }) as i32;
                                }
                                aij = (*col).ptr;
                                while !aij.is_null() {
                                    _glp_npp_add_aij(
                                        npp,
                                        (*aij).row,
                                        bin,
                                        temp as libc::c_double * (*aij).val,
                                    );
                                    aij = (*aij).c_next;
                                }
                                k += 1;
                                k;
                                temp += temp;
                            }
                        }
                    }
                }
            }
        }
        col = (*col).prev;
    }
    if nvars > 0 as i32 {
        glp_printf(
            b"%d integer variable(s) were replaced by %d binary ones\n\0" as *const u8
                as *const i8,
            nvars,
            nbins,
        );
    }
    if nrows > 0 as i32 {
        glp_printf(
            b"%d row(s) were added due to binarization\n\0" as *const u8 as *const i8,
            nrows,
        );
    }
    if nfails > 0 as i32 {
        glp_printf(
            b"Binarization failed for %d integer variable(s)\n\0" as *const u8
                as *const i8,
            nfails,
        );
    }
    return nfails;
}
unsafe extern "C" fn rcv_binarize_prob(
    mut npp: *mut NPP,
    mut _info: *mut libc::c_void,
) -> i32 {
    let mut info: *mut binarize = _info as *mut binarize;
    let mut k: i32 = 0;
    let mut temp: i32 = 0;
    let mut sum: libc::c_double = 0.;
    sum = *((*npp).c_value).offset((*info).q as isize);
    k = 1 as i32;
    temp = 2 as i32;
    while k < (*info).n {
        sum
            += temp as libc::c_double
                * *((*npp).c_value).offset(((*info).j + (k - 1 as i32)) as isize);
        k += 1;
        k;
        temp += temp;
    }
    *((*npp).c_value).offset((*info).q as isize) = sum;
    return 0 as i32;
}
unsafe extern "C" fn copy_form(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut s: libc::c_double,
) -> *mut elem {
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ptr: *mut elem = 0 as *mut elem;
    let mut e: *mut elem = 0 as *mut elem;
    ptr = 0 as *mut elem;
    aij = (*row).ptr;
    while !aij.is_null() {
        e = _glp_dmp_get_atom((*npp).pool, ::core::mem::size_of::<elem>() as u64 as i32)
            as *mut elem;
        (*e).aj = s * (*aij).val;
        (*e).xj = (*aij).col;
        (*e).next = ptr;
        ptr = e;
        aij = (*aij).r_next;
    }
    return ptr;
}
unsafe extern "C" fn drop_form(mut npp: *mut NPP, mut ptr: *mut elem) {
    let mut e: *mut elem = 0 as *mut elem;
    while !ptr.is_null() {
        e = ptr;
        ptr = (*e).next;
        _glp_dmp_free_atom(
            (*npp).pool,
            e as *mut libc::c_void,
            ::core::mem::size_of::<elem>() as u64 as i32,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_is_packing(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut b: i32 = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp4.c\0" as *const u8 as *const i8,
                316 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if !((*row).lb == -1.7976931348623157e+308f64
        && (*row).ub != 1.7976931348623157e+308f64)
    {
        return 0 as i32;
    }
    b = 1 as i32;
    aij = (*row).ptr;
    while !aij.is_null() {
        col = (*aij).col;
        if !((*col).is_int as i32 != 0 && (*col).lb == 0.0f64 && (*col).ub == 1.0f64) {
            return 0 as i32;
        }
        if !((*aij).val == 1.0f64) {
            if (*aij).val == -1.0f64 {
                b -= 1;
                b;
            } else {
                return 0 as i32
            }
        }
        aij = (*aij).r_next;
    }
    if (*row).ub != b as libc::c_double {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn hidden_packing(
    mut npp: *mut NPP,
    mut ptr: *mut elem,
    mut _b: *mut libc::c_double,
) -> i32 {
    let mut e: *mut elem = 0 as *mut elem;
    let mut ej: *mut elem = 0 as *mut elem;
    let mut ek: *mut elem = 0 as *mut elem;
    let mut neg: i32 = 0;
    let mut b: libc::c_double = *_b;
    let mut eps: libc::c_double = 0.;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp4.c\0" as *const u8 as *const i8,
                435 as i32,
            );
            1 as i32 != 0
        }) as i32;
    e = ptr;
    while !e.is_null() {
        ((*e).aj != 0.0f64
            || {
                glp_assert_(
                    b"e->aj != 0.0\0" as *const u8 as *const i8,
                    b"npp/npp4.c\0" as *const u8 as *const i8,
                    438 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ((*(*e).xj).is_int as i32 != 0
            || {
                glp_assert_(
                    b"e->xj->is_int\0" as *const u8 as *const i8,
                    b"npp/npp4.c\0" as *const u8 as *const i8,
                    439 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ((*(*e).xj).lb == 0.0f64 && (*(*e).xj).ub == 1.0f64
            || {
                glp_assert_(
                    b"e->xj->lb == 0.0 && e->xj->ub == 1.0\0" as *const u8 as *const i8,
                    b"npp/npp4.c\0" as *const u8 as *const i8,
                    440 as i32,
                );
                1 as i32 != 0
            }) as i32;
        e = (*e).next;
    }
    neg = 0 as i32;
    e = ptr;
    while !e.is_null() {
        if !((*e).aj == 1.0f64) {
            if !((*e).aj == -1.0f64) {
                break;
            }
            neg += 1;
            neg;
        }
        e = (*e).next;
    }
    if e.is_null() {
        if b == (1 as i32 - neg) as libc::c_double {
            return 1 as i32;
        }
    }
    e = ptr;
    while !e.is_null() {
        if (*e).aj < 0 as i32 as libc::c_double {
            b -= (*e).aj;
        }
        e = (*e).next;
    }
    e = ptr;
    while !e.is_null() {
        if fabs((*e).aj) > b {
            return 0 as i32;
        }
        e = (*e).next;
    }
    ej = 0 as *mut elem;
    e = ptr;
    while !e.is_null() {
        if ej.is_null() || fabs((*ej).aj) > fabs((*e).aj) {
            ej = e;
        }
        e = (*e).next;
    }
    (!ej.is_null()
        || {
            glp_assert_(
                b"ej != NULL\0" as *const u8 as *const i8,
                b"npp/npp4.c\0" as *const u8 as *const i8,
                473 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ek = 0 as *mut elem;
    e = ptr;
    while !e.is_null() {
        if e != ej {
            if ek.is_null() || fabs((*ek).aj) > fabs((*e).aj) {
                ek = e;
            }
        }
        e = (*e).next;
    }
    (!ek.is_null()
        || {
            glp_assert_(
                b"ek != NULL\0" as *const u8 as *const i8,
                b"npp/npp4.c\0" as *const u8 as *const i8,
                478 as i32,
            );
            1 as i32 != 0
        }) as i32;
    eps = 1e-3f64 + 1e-6f64 * fabs(b);
    if fabs((*ej).aj) + fabs((*ek).aj) <= b + eps {
        return 0 as i32;
    }
    b = 1.0f64;
    e = ptr;
    while !e.is_null() {
        if (*e).aj > 0.0f64 {
            (*e).aj = 1.0f64;
        } else {
            (*e).aj = -1.0f64;
            b -= 1.0f64;
        }
        e = (*e).next;
    }
    *_b = b;
    return 2 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_hidden_packing(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    let mut copy: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ptr: *mut elem = 0 as *mut elem;
    let mut e: *mut elem = 0 as *mut elem;
    let mut kase: i32 = 0;
    let mut ret: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut b: libc::c_double = 0.;
    ((*row).lb < (*row).ub
        || {
            glp_assert_(
                b"row->lb < row->ub\0" as *const u8 as *const i8,
                b"npp/npp4.c\0" as *const u8 as *const i8,
                504 as i32,
            );
            1 as i32 != 0
        }) as i32;
    let mut current_block_31: u64;
    kase = 0 as i32;
    while kase <= 1 as i32 {
        if kase == 0 as i32 {
            if (*row).ub == 1.7976931348623157e+308f64 {
                current_block_31 = 12675440807659640239;
            } else {
                ptr = copy_form(npp, row, 1.0f64);
                b = (*row).ub;
                current_block_31 = 11650488183268122163;
            }
        } else if (*row).lb == -1.7976931348623157e+308f64 {
            current_block_31 = 12675440807659640239;
        } else {
            ptr = copy_form(npp, row, -1.0f64);
            b = -(*row).lb;
            current_block_31 = 11650488183268122163;
        }
        match current_block_31 {
            11650488183268122163 => {
                ret = hidden_packing(npp, ptr, &mut b);
                (0 as i32 <= ret && ret <= 2 as i32
                    || {
                        glp_assert_(
                            b"0 <= ret && ret <= 2\0" as *const u8 as *const i8,
                            b"npp/npp4.c\0" as *const u8 as *const i8,
                            520 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                if kase == 1 as i32 && ret == 1 as i32 || ret == 2 as i32 {
                    count += 1;
                    count;
                    if (*row).lb == -1.7976931348623157e+308f64
                        || (*row).ub == 1.7976931348623157e+308f64
                    {
                        copy = 0 as *mut NPPROW;
                    } else {
                        copy = _glp_npp_add_row(npp);
                        if kase == 0 as i32 {
                            (*copy).lb = (*row).lb;
                            (*copy).ub = 1.7976931348623157e+308f64;
                        } else {
                            (*copy).lb = -1.7976931348623157e+308f64;
                            (*copy).ub = (*row).ub;
                        }
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            _glp_npp_add_aij(npp, copy, (*aij).col, (*aij).val);
                            aij = (*aij).r_next;
                        }
                    }
                    _glp_npp_erase_row(npp, row);
                    (*row).lb = -1.7976931348623157e+308f64;
                    (*row).ub = b;
                    e = ptr;
                    while !e.is_null() {
                        _glp_npp_add_aij(npp, row, (*e).xj, (*e).aj);
                        e = (*e).next;
                    }
                    if !copy.is_null() {
                        row = copy;
                    }
                }
                drop_form(npp, ptr);
            }
            _ => {}
        }
        kase += 1;
        kase;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_implied_packing(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
    mut which: i32,
    mut var: *mut *mut NPPCOL,
    mut set: *mut i8,
) -> i32 {
    let mut current_block: u64;
    let mut ptr: *mut elem = 0 as *mut elem;
    let mut e: *mut elem = 0 as *mut elem;
    let mut i: *mut elem = 0 as *mut elem;
    let mut k: *mut elem = 0 as *mut elem;
    let mut len: i32 = 0 as i32;
    let mut b: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    if which == 0 as i32 {
        ptr = copy_form(npp, row, -1.0f64);
        ((*row).lb != -1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"row->lb != -DBL_MAX\0" as *const u8 as *const i8,
                    b"npp/npp4.c\0" as *const u8 as *const i8,
                    721 as i32,
                );
                1 as i32 != 0
            }) as i32;
        b = -(*row).lb;
    } else if which == 1 as i32 {
        ptr = copy_form(npp, row, 1.0f64);
        ((*row).ub != 1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"row->ub != +DBL_MAX\0" as *const u8 as *const i8,
                    b"npp/npp4.c\0" as *const u8 as *const i8,
                    726 as i32,
                );
                1 as i32 != 0
            }) as i32;
        b = (*row).ub;
    }
    e = ptr;
    loop {
        if e.is_null() {
            current_block = 5948590327928692120;
            break;
        }
        if !((*(*e).xj).is_int as i32 != 0 && (*(*e).xj).lb == 0.0f64
            && (*(*e).xj).ub == 1.0f64)
        {
            if (*e).aj > 0.0f64 {
                if (*(*e).xj).lb == -1.7976931348623157e+308f64 {
                    current_block = 7127204244045643115;
                    break;
                }
                b -= (*e).aj * (*(*e).xj).lb;
            } else {
                if (*(*e).xj).ub == 1.7976931348623157e+308f64 {
                    current_block = 7127204244045643115;
                    break;
                }
                b -= (*e).aj * (*(*e).xj).ub;
            }
            (*e).aj = 0.0f64;
        }
        e = (*e).next;
    }
    match current_block {
        5948590327928692120 => {
            e = ptr;
            while !e.is_null() {
                if (*e).aj < 0.0f64 {
                    b -= (*e).aj;
                }
                e = (*e).next;
            }
            if !(b < 1e-3f64) {
                eps = 1e-3f64 + 1e-6f64 * b;
                k = 0 as *mut elem;
                i = k;
                e = ptr;
                while !e.is_null() {
                    if fabs((*e).aj) > 0.5f64 * (b + eps) {
                        len += 1;
                        let ref mut fresh0 = *var.offset(len as isize);
                        *fresh0 = (*e).xj;
                        *set.offset(len as isize) = (if (*e).aj > 0.0f64 {
                            0 as i32
                        } else {
                            1 as i32
                        }) as i8;
                        if i.is_null() || fabs((*i).aj) > fabs((*e).aj) {
                            i = e;
                        }
                    } else if fabs((*e).aj) >= 1e-3f64 {
                        if k.is_null() || fabs((*k).aj) < fabs((*e).aj) {
                            k = e;
                        }
                    }
                    e = (*e).next;
                }
                if !i.is_null() && !k.is_null()
                    && fabs((*i).aj) + fabs((*k).aj) > b + eps
                {
                    len += 1;
                    let ref mut fresh1 = *var.offset(len as isize);
                    *fresh1 = (*k).xj;
                    *set.offset(len as isize) = (if (*k).aj > 0.0f64 {
                        0 as i32
                    } else {
                        1 as i32
                    }) as i8;
                }
                if len < 2 as i32 {
                    len = 0 as i32;
                }
            }
        }
        _ => {}
    }
    drop_form(npp, ptr);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_is_covering(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut b: i32 = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp4.c\0" as *const u8 as *const i8,
                841 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if !((*row).lb != -1.7976931348623157e+308f64
        && (*row).ub == 1.7976931348623157e+308f64)
    {
        return 0 as i32;
    }
    b = 1 as i32;
    aij = (*row).ptr;
    while !aij.is_null() {
        col = (*aij).col;
        if !((*col).is_int as i32 != 0 && (*col).lb == 0.0f64 && (*col).ub == 1.0f64) {
            return 0 as i32;
        }
        if !((*aij).val == 1.0f64) {
            if (*aij).val == -1.0f64 {
                b -= 1;
                b;
            } else {
                return 0 as i32
            }
        }
        aij = (*aij).r_next;
    }
    if (*row).lb != b as libc::c_double {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn hidden_covering(
    mut npp: *mut NPP,
    mut ptr: *mut elem,
    mut _b: *mut libc::c_double,
) -> i32 {
    let mut e: *mut elem = 0 as *mut elem;
    let mut neg: i32 = 0;
    let mut b: libc::c_double = *_b;
    let mut eps: libc::c_double = 0.;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp4.c\0" as *const u8 as *const i8,
                938 as i32,
            );
            1 as i32 != 0
        }) as i32;
    e = ptr;
    while !e.is_null() {
        ((*e).aj != 0.0f64
            || {
                glp_assert_(
                    b"e->aj != 0.0\0" as *const u8 as *const i8,
                    b"npp/npp4.c\0" as *const u8 as *const i8,
                    941 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ((*(*e).xj).is_int as i32 != 0
            || {
                glp_assert_(
                    b"e->xj->is_int\0" as *const u8 as *const i8,
                    b"npp/npp4.c\0" as *const u8 as *const i8,
                    942 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ((*(*e).xj).lb == 0.0f64 && (*(*e).xj).ub == 1.0f64
            || {
                glp_assert_(
                    b"e->xj->lb == 0.0 && e->xj->ub == 1.0\0" as *const u8 as *const i8,
                    b"npp/npp4.c\0" as *const u8 as *const i8,
                    943 as i32,
                );
                1 as i32 != 0
            }) as i32;
        e = (*e).next;
    }
    neg = 0 as i32;
    e = ptr;
    while !e.is_null() {
        if !((*e).aj == 1.0f64) {
            if !((*e).aj == -1.0f64) {
                break;
            }
            neg += 1;
            neg;
        }
        e = (*e).next;
    }
    if e.is_null() {
        if b == (1 as i32 - neg) as libc::c_double {
            return 1 as i32;
        }
    }
    e = ptr;
    while !e.is_null() {
        if (*e).aj < 0 as i32 as libc::c_double {
            b -= (*e).aj;
        }
        e = (*e).next;
    }
    if b < 1e-3f64 {
        return 0 as i32;
    }
    eps = 1e-9f64 + 1e-12f64 * fabs(b);
    e = ptr;
    while !e.is_null() {
        if fabs((*e).aj) < b - eps {
            return 0 as i32;
        }
        e = (*e).next;
    }
    b = 1.0f64;
    e = ptr;
    while !e.is_null() {
        if (*e).aj > 0.0f64 {
            (*e).aj = 1.0f64;
        } else {
            (*e).aj = -1.0f64;
            b -= 1.0f64;
        }
        e = (*e).next;
    }
    *_b = b;
    return 2 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_hidden_covering(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    let mut copy: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ptr: *mut elem = 0 as *mut elem;
    let mut e: *mut elem = 0 as *mut elem;
    let mut kase: i32 = 0;
    let mut ret: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut b: libc::c_double = 0.;
    ((*row).lb < (*row).ub
        || {
            glp_assert_(
                b"row->lb < row->ub\0" as *const u8 as *const i8,
                b"npp/npp4.c\0" as *const u8 as *const i8,
                997 as i32,
            );
            1 as i32 != 0
        }) as i32;
    let mut current_block_31: u64;
    kase = 0 as i32;
    while kase <= 1 as i32 {
        if kase == 0 as i32 {
            if (*row).lb == -1.7976931348623157e+308f64 {
                current_block_31 = 12675440807659640239;
            } else {
                ptr = copy_form(npp, row, 1.0f64);
                b = (*row).lb;
                current_block_31 = 11650488183268122163;
            }
        } else if (*row).ub == 1.7976931348623157e+308f64 {
            current_block_31 = 12675440807659640239;
        } else {
            ptr = copy_form(npp, row, -1.0f64);
            b = -(*row).ub;
            current_block_31 = 11650488183268122163;
        }
        match current_block_31 {
            11650488183268122163 => {
                ret = hidden_covering(npp, ptr, &mut b);
                (0 as i32 <= ret && ret <= 2 as i32
                    || {
                        glp_assert_(
                            b"0 <= ret && ret <= 2\0" as *const u8 as *const i8,
                            b"npp/npp4.c\0" as *const u8 as *const i8,
                            1013 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                if kase == 1 as i32 && ret == 1 as i32 || ret == 2 as i32 {
                    count += 1;
                    count;
                    if (*row).lb == -1.7976931348623157e+308f64
                        || (*row).ub == 1.7976931348623157e+308f64
                    {
                        copy = 0 as *mut NPPROW;
                    } else {
                        copy = _glp_npp_add_row(npp);
                        if kase == 0 as i32 {
                            (*copy).lb = -1.7976931348623157e+308f64;
                            (*copy).ub = (*row).ub;
                        } else {
                            (*copy).lb = (*row).lb;
                            (*copy).ub = 1.7976931348623157e+308f64;
                        }
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            _glp_npp_add_aij(npp, copy, (*aij).col, (*aij).val);
                            aij = (*aij).r_next;
                        }
                    }
                    _glp_npp_erase_row(npp, row);
                    (*row).lb = b;
                    (*row).ub = 1.7976931348623157e+308f64;
                    e = ptr;
                    while !e.is_null() {
                        _glp_npp_add_aij(npp, row, (*e).xj, (*e).aj);
                        e = (*e).next;
                    }
                    if !copy.is_null() {
                        row = copy;
                    }
                }
                drop_form(npp, ptr);
            }
            _ => {}
        }
        kase += 1;
        kase;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_is_partitioning(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    let mut col: *mut NPPCOL = 0 as *mut NPPCOL;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut b: i32 = 0;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp4.c\0" as *const u8 as *const i8,
                1113 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*row).lb != (*row).ub {
        return 0 as i32;
    }
    b = 1 as i32;
    aij = (*row).ptr;
    while !aij.is_null() {
        col = (*aij).col;
        if !((*col).is_int as i32 != 0 && (*col).lb == 0.0f64 && (*col).ub == 1.0f64) {
            return 0 as i32;
        }
        if !((*aij).val == 1.0f64) {
            if (*aij).val == -1.0f64 {
                b -= 1;
                b;
            } else {
                return 0 as i32
            }
        }
        aij = (*aij).r_next;
    }
    if (*row).lb != b as libc::c_double {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn reduce_ineq_coef(
    mut npp: *mut NPP,
    mut ptr: *mut elem,
    mut _b: *mut libc::c_double,
) -> i32 {
    let mut current_block: u64;
    let mut e: *mut elem = 0 as *mut elem;
    let mut count: i32 = 0 as i32;
    let mut h: libc::c_double = 0.;
    let mut inf_t: libc::c_double = 0.;
    let mut new_a: libc::c_double = 0.;
    let mut b: libc::c_double = *_b;
    (npp == npp
        || {
            glp_assert_(
                b"npp == npp\0" as *const u8 as *const i8,
                b"npp/npp4.c\0" as *const u8 as *const i8,
                1279 as i32,
            );
            1 as i32 != 0
        }) as i32;
    h = 0.0f64;
    e = ptr;
    loop {
        if e.is_null() {
            current_block = 1917311967535052937;
            break;
        }
        if (*e).aj > 0.0f64 {
            if (*(*e).xj).lb == -1.7976931348623157e+308f64 {
                current_block = 2321369402472612274;
                break;
            }
            h += (*e).aj * (*(*e).xj).lb;
        } else {
            if (*(*e).xj).ub == 1.7976931348623157e+308f64 {
                current_block = 2321369402472612274;
                break;
            }
            h += (*e).aj * (*(*e).xj).ub;
        }
        e = (*e).next;
    }
    match current_block {
        1917311967535052937 => {
            e = ptr;
            while !e.is_null() {
                if (*(*e).xj).is_int as i32 != 0 && (*(*e).xj).lb == 0.0f64
                    && (*(*e).xj).ub == 1.0f64
                {
                    if (*e).aj > 0.0f64 {
                        inf_t = h;
                        if b - (*e).aj < inf_t && inf_t < b {
                            new_a = b - inf_t;
                            if new_a >= 1e-3f64
                                && (*e).aj - new_a >= 0.01f64 * (1.0f64 + (*e).aj)
                            {
                                (*e).aj = new_a;
                                count += 1;
                                count;
                            }
                        }
                    } else {
                        inf_t = h - (*e).aj;
                        if b < inf_t && inf_t < b - (*e).aj {
                            new_a = (*e).aj + (inf_t - b);
                            if new_a <= -1e-3f64
                                && new_a - (*e).aj >= 0.01f64 * (1.0f64 - (*e).aj)
                            {
                                (*e).aj = new_a;
                                h += inf_t - b;
                                b = inf_t;
                                count += 1;
                                count;
                            }
                        }
                    }
                }
                e = (*e).next;
            }
            *_b = b;
        }
        _ => {}
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_npp_reduce_ineq_coef(
    mut npp: *mut NPP,
    mut row: *mut NPPROW,
) -> i32 {
    let mut copy: *mut NPPROW = 0 as *mut NPPROW;
    let mut aij: *mut NPPAIJ = 0 as *mut NPPAIJ;
    let mut ptr: *mut elem = 0 as *mut elem;
    let mut e: *mut elem = 0 as *mut elem;
    let mut kase: i32 = 0;
    let mut count: [i32; 2] = [0; 2];
    let mut b: libc::c_double = 0.;
    ((*row).lb < (*row).ub
        || {
            glp_assert_(
                b"row->lb < row->ub\0" as *const u8 as *const i8,
                b"npp/npp4.c\0" as *const u8 as *const i8,
                1348 as i32,
            );
            1 as i32 != 0
        }) as i32;
    count[1 as i32 as usize] = 0 as i32;
    count[0 as i32 as usize] = count[1 as i32 as usize];
    let mut current_block_30: u64;
    kase = 0 as i32;
    while kase <= 1 as i32 {
        if kase == 0 as i32 {
            if (*row).lb == -1.7976931348623157e+308f64 {
                current_block_30 = 4644295000439058019;
            } else {
                ptr = copy_form(npp, row, 1.0f64);
                b = (*row).lb;
                current_block_30 = 7746791466490516765;
            }
        } else if (*row).ub == 1.7976931348623157e+308f64 {
            current_block_30 = 4644295000439058019;
        } else {
            ptr = copy_form(npp, row, -1.0f64);
            b = -(*row).ub;
            current_block_30 = 7746791466490516765;
        }
        match current_block_30 {
            7746791466490516765 => {
                count[kase as usize] = reduce_ineq_coef(npp, ptr, &mut b);
                if count[kase as usize] > 0 as i32 {
                    if (*row).lb == -1.7976931348623157e+308f64
                        || (*row).ub == 1.7976931348623157e+308f64
                    {
                        copy = 0 as *mut NPPROW;
                    } else {
                        copy = _glp_npp_add_row(npp);
                        if kase == 0 as i32 {
                            (*copy).lb = -1.7976931348623157e+308f64;
                            (*copy).ub = (*row).ub;
                        } else {
                            (*copy).lb = (*row).lb;
                            (*copy).ub = 1.7976931348623157e+308f64;
                        }
                        aij = (*row).ptr;
                        while !aij.is_null() {
                            _glp_npp_add_aij(npp, copy, (*aij).col, (*aij).val);
                            aij = (*aij).r_next;
                        }
                    }
                    _glp_npp_erase_row(npp, row);
                    (*row).lb = b;
                    (*row).ub = 1.7976931348623157e+308f64;
                    e = ptr;
                    while !e.is_null() {
                        _glp_npp_add_aij(npp, row, (*e).xj, (*e).aj);
                        e = (*e).next;
                    }
                    if !copy.is_null() {
                        row = copy;
                    }
                }
                drop_form(npp, ptr);
            }
            _ => {}
        }
        kase += 1;
        kase;
    }
    return count[0 as i32 as usize] + count[1 as i32 as usize];
}