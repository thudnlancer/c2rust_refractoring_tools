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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = u64;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPXLP {
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub A_ptr: *mut i32,
    pub A_ind: *mut i32,
    pub A_val: *mut libc::c_double,
    pub b: *mut libc::c_double,
    pub c: *mut libc::c_double,
    pub l: *mut libc::c_double,
    pub u: *mut libc::c_double,
    pub head: *mut i32,
    pub flag: *mut i8,
    pub valid: i32,
    pub bfd: *mut BFD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPYBP {
    pub j: i32,
    pub teta: libc::c_double,
    pub dz: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spy_chuzc_std(
    mut lp: *mut SPXLP,
    mut d: *const libc::c_double,
    mut r: libc::c_double,
    mut trow: *const libc::c_double,
    mut tol_piv: libc::c_double,
    mut tol: libc::c_double,
    mut tol1: libc::c_double,
) -> i32 {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut i32 = (*lp).head;
    let mut flag: *mut i8 = (*lp).flag;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut q: i32 = 0;
    let mut alfa: libc::c_double = 0.;
    let mut biga: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut teta: libc::c_double = 0.;
    let mut teta_min: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    (r != 0.0f64
        || {
            glp_assert_(
                b"r != 0.0\0" as *const u8 as *const i8,
                b"simplex/spychuzc.c\0" as *const u8 as *const i8,
                97 as i32,
            );
            1 as i32 != 0
        }) as i32;
    s = if r > 0.0f64 { 1.0f64 } else { -1.0f64 };
    q = 0 as i32;
    teta_min = 1.7976931348623157e+308f64;
    biga = 0.0f64;
    let mut current_block_15: u64;
    j = 1 as i32;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        if !(*l.offset(k as isize) == *u.offset(k as isize)) {
            alfa = s * *trow.offset(j as isize);
            if alfa >= tol_piv && *flag.offset(j as isize) == 0 {
                delta = tol
                    + tol1
                        * (if *c.offset(k as isize) >= 0.0f64 {
                            *c.offset(k as isize)
                        } else {
                            -*c.offset(k as isize)
                        });
                teta = if *d.offset(j as isize) < delta {
                    0.0f64
                } else {
                    *d.offset(j as isize) / alfa
                };
                current_block_15 = 8831408221741692167;
            } else if alfa <= -tol_piv
                && (*l.offset(k as isize) == -1.7976931348623157e+308f64
                    || *flag.offset(j as isize) as i32 != 0)
            {
                delta = tol
                    + tol1
                        * (if *c.offset(k as isize) >= 0.0f64 {
                            *c.offset(k as isize)
                        } else {
                            -*c.offset(k as isize)
                        });
                teta = if *d.offset(j as isize) > -delta {
                    0.0f64
                } else {
                    *d.offset(j as isize) / alfa
                };
                current_block_15 = 8831408221741692167;
            } else {
                current_block_15 = 6483416627284290920;
            }
            match current_block_15 {
                6483416627284290920 => {}
                _ => {
                    (teta >= 0.0f64
                        || {
                            glp_assert_(
                                b"teta >= 0.0\0" as *const u8 as *const i8,
                                b"simplex/spychuzc.c\0" as *const u8 as *const i8,
                                129 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    alfa = if alfa >= 0.0f64 { alfa } else { -alfa };
                    if teta_min > teta || teta_min == teta && biga < alfa {
                        q = j;
                        teta_min = teta;
                        biga = alfa;
                    }
                }
            }
        }
        j += 1;
        j;
    }
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spy_chuzc_harris(
    mut lp: *mut SPXLP,
    mut d: *const libc::c_double,
    mut r: libc::c_double,
    mut trow: *const libc::c_double,
    mut tol_piv: libc::c_double,
    mut tol: libc::c_double,
    mut tol1: libc::c_double,
) -> i32 {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut i32 = (*lp).head;
    let mut flag: *mut i8 = (*lp).flag;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut q: i32 = 0;
    let mut alfa: libc::c_double = 0.;
    let mut biga: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut teta: libc::c_double = 0.;
    let mut teta_min: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    (r != 0.0f64
        || {
            glp_assert_(
                b"r != 0.0\0" as *const u8 as *const i8,
                b"simplex/spychuzc.c\0" as *const u8 as *const i8,
                172 as i32,
            );
            1 as i32 != 0
        }) as i32;
    s = if r > 0.0f64 { 1.0f64 } else { -1.0f64 };
    teta_min = 1.7976931348623157e+308f64;
    let mut current_block_14: u64;
    j = 1 as i32;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        if !(*l.offset(k as isize) == *u.offset(k as isize)) {
            alfa = s * *trow.offset(j as isize);
            if alfa >= tol_piv && *flag.offset(j as isize) == 0 {
                delta = tol
                    + tol1
                        * (if *c.offset(k as isize) >= 0.0f64 {
                            *c.offset(k as isize)
                        } else {
                            -*c.offset(k as isize)
                        });
                teta = ((if *d.offset(j as isize) < 0.0f64 {
                    0.0f64
                } else {
                    *d.offset(j as isize)
                }) + delta) / alfa;
                current_block_14 = 8831408221741692167;
            } else if alfa <= -tol_piv
                && (*l.offset(k as isize) == -1.7976931348623157e+308f64
                    || *flag.offset(j as isize) as i32 != 0)
            {
                delta = tol
                    + tol1
                        * (if *c.offset(k as isize) >= 0.0f64 {
                            *c.offset(k as isize)
                        } else {
                            -*c.offset(k as isize)
                        });
                teta = ((if *d.offset(j as isize) > 0.0f64 {
                    0.0f64
                } else {
                    *d.offset(j as isize)
                }) - delta) / alfa;
                current_block_14 = 8831408221741692167;
            } else {
                current_block_14 = 6483416627284290920;
            }
            match current_block_14 {
                6483416627284290920 => {}
                _ => {
                    (teta >= 0.0f64
                        || {
                            glp_assert_(
                                b"teta >= 0.0\0" as *const u8 as *const i8,
                                b"simplex/spychuzc.c\0" as *const u8 as *const i8,
                                204 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if teta_min > teta {
                        teta_min = teta;
                    }
                }
            }
        }
        j += 1;
        j;
    }
    if teta_min == 1.7976931348623157e+308f64 {
        q = 0 as i32;
    } else {
        q = 0 as i32;
        biga = 0.0f64;
        let mut current_block_27: u64;
        j = 1 as i32;
        while j <= n - m {
            k = *head.offset((m + j) as isize);
            if !(*l.offset(k as isize) == *u.offset(k as isize)) {
                alfa = s * *trow.offset(j as isize);
                if alfa >= tol_piv && *flag.offset(j as isize) == 0 {
                    teta = *d.offset(j as isize) / alfa;
                    current_block_27 = 17281240262373992796;
                } else if alfa <= -tol_piv
                    && (*l.offset(k as isize) == -1.7976931348623157e+308f64
                        || *flag.offset(j as isize) as i32 != 0)
                {
                    teta = *d.offset(j as isize) / alfa;
                    current_block_27 = 17281240262373992796;
                } else {
                    current_block_27 = 1109700713171191020;
                }
                match current_block_27 {
                    1109700713171191020 => {}
                    _ => {
                        alfa = if alfa >= 0.0f64 { alfa } else { -alfa };
                        if teta <= teta_min && biga < alfa {
                            q = j;
                            biga = alfa;
                        }
                    }
                }
            }
            j += 1;
            j;
        }
        (1 as i32 <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const i8,
                    b"simplex/spychuzc.c\0" as *const u8 as *const i8,
                    249 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spy_ls_eval_bp(
    mut lp: *mut SPXLP,
    mut d: *const libc::c_double,
    mut r: libc::c_double,
    mut trow: *const libc::c_double,
    mut tol_piv: libc::c_double,
    mut bp: *mut SPYBP,
) -> i32 {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut i32 = (*lp).head;
    let mut flag: *mut i8 = (*lp).flag;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut t: i32 = 0;
    let mut nnn: i32 = 0;
    let mut nbp: i32 = 0;
    let mut s: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut teta: libc::c_double = 0.;
    let mut teta_max: libc::c_double = 0.;
    (r != 0.0f64
        || {
            glp_assert_(
                b"r != 0.0\0" as *const u8 as *const i8,
                b"simplex/spychuzc.c\0" as *const u8 as *const i8,
                421 as i32,
            );
            1 as i32 != 0
        }) as i32;
    s = if r > 0.0f64 { 1.0f64 } else { -1.0f64 };
    nnn = 0 as i32;
    teta_max = 1.7976931348623157e+308f64;
    let mut current_block_16: u64;
    j = 1 as i32;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        if !(*l.offset(k as isize) == *u.offset(k as isize)) {
            alfa = s * *trow.offset(j as isize);
            if alfa >= tol_piv && *flag.offset(j as isize) == 0 {
                teta = if *d.offset(j as isize) < 0.0f64 {
                    0.0f64
                } else {
                    *d.offset(j as isize) / alfa
                };
                if *u.offset(k as isize) == 1.7976931348623157e+308f64 && teta_max > teta
                {
                    teta_max = teta;
                }
                current_block_16 = 15652330335145281839;
            } else if alfa <= -tol_piv
                && (*l.offset(k as isize) == -1.7976931348623157e+308f64
                    || *flag.offset(j as isize) as i32 != 0)
            {
                teta = if *d.offset(j as isize) > 0.0f64 {
                    0.0f64
                } else {
                    *d.offset(j as isize) / alfa
                };
                if *l.offset(k as isize) == -1.7976931348623157e+308f64
                    && teta_max > teta
                {
                    teta_max = teta;
                }
                current_block_16 = 15652330335145281839;
            } else {
                current_block_16 = 15427931788582360902;
            }
            match current_block_16 {
                15427931788582360902 => {}
                _ => {
                    nnn += 1;
                    nnn;
                    (*bp.offset(nnn as isize)).j = j;
                    (*bp.offset(nnn as isize)).teta = teta;
                }
            }
        }
        j += 1;
        j;
    }
    nbp = 0 as i32;
    t = 1 as i32;
    while t <= nnn {
        if (*bp.offset(t as isize)).teta <= teta_max + 1e-6f64 {
            nbp += 1;
            nbp;
            (*bp.offset(nbp as isize)).j = (*bp.offset(t as isize)).j;
            (*bp.offset(nbp as isize)).teta = (*bp.offset(t as isize)).teta;
        }
        t += 1;
        t;
    }
    return nbp;
}
unsafe extern "C" fn fcmp(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> i32 {
    let mut p1: *const SPYBP = v1 as *const SPYBP;
    let mut p2: *const SPYBP = v2 as *const SPYBP;
    if (*p1).teta < (*p2).teta {
        return -(1 as i32)
    } else if (*p1).teta > (*p2).teta {
        return 1 as i32
    } else {
        return 0 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spy_ls_select_bp(
    mut lp: *mut SPXLP,
    mut trow: *const libc::c_double,
    mut nbp: i32,
    mut bp: *mut SPYBP,
    mut num: i32,
    mut slope: *mut libc::c_double,
    mut teta_lim: libc::c_double,
) -> i32 {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut i32 = (*lp).head;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut t: i32 = 0;
    let mut num1: i32 = 0;
    let mut teta: libc::c_double = 0.;
    let mut dz: libc::c_double = 0.;
    (0 as i32 <= num && num <= nbp && nbp <= n - m
        || {
            glp_assert_(
                b"0 <= num && num <= nbp && nbp <= n-m\0" as *const u8 as *const i8,
                b"simplex/spychuzc.c\0" as *const u8 as *const i8,
                518 as i32,
            );
            1 as i32 != 0
        }) as i32;
    num1 = num;
    t = num + 1 as i32;
    while t <= nbp {
        if (*bp.offset(t as isize)).teta <= teta_lim {
            num1 += 1;
            num1;
            j = (*bp.offset(num1 as isize)).j;
            teta = (*bp.offset(num1 as isize)).teta;
            (*bp.offset(num1 as isize)).j = (*bp.offset(t as isize)).j;
            (*bp.offset(num1 as isize)).teta = (*bp.offset(t as isize)).teta;
            (*bp.offset(t as isize)).j = j;
            (*bp.offset(t as isize)).teta = teta;
        }
        t += 1;
        t;
    }
    if num1 - num > 1 as i32 {
        qsort(
            &mut *bp.offset((num + 1 as i32) as isize) as *mut SPYBP
                as *mut libc::c_void,
            (num1 - num) as size_t,
            ::core::mem::size_of::<SPYBP>() as u64,
            Some(
                fcmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
        );
    }
    t = num + 1 as i32;
    while t <= num1 {
        if *slope == -1.7976931348623157e+308f64 {
            dz = -1.7976931348623157e+308f64;
        } else {
            dz = *slope
                * ((*bp.offset(t as isize)).teta
                    - (if t == 1 as i32 {
                        0.0f64
                    } else {
                        (*bp.offset((t - 1 as i32) as isize)).teta
                    }));
        }
        if dz == -1.7976931348623157e+308f64 {
            (*bp.offset(t as isize)).dz = -1.7976931348623157e+308f64;
        } else {
            (*bp.offset(t as isize)).dz = (if t == 1 as i32 {
                0.0f64
            } else {
                (*bp.offset((t - 1 as i32) as isize)).dz
            }) + dz;
        }
        if *slope != -1.7976931348623157e+308f64 {
            j = (*bp.offset(t as isize)).j;
            k = *head.offset((m + j) as isize);
            if *l.offset(k as isize) == -1.7976931348623157e+308f64
                || *u.offset(k as isize) == 1.7976931348623157e+308f64
            {
                *slope = -1.7976931348623157e+308f64;
            } else {
                (*l.offset(k as isize) < *u.offset(k as isize)
                    || {
                        glp_assert_(
                            b"l[k] < u[k]\0" as *const u8 as *const i8,
                            b"simplex/spychuzc.c\0" as *const u8 as *const i8,
                            557 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                *slope
                    -= fabs(*trow.offset(j as isize))
                        * (*u.offset(k as isize) - *l.offset(k as isize));
            }
        }
        t += 1;
        t;
    }
    return num1;
}