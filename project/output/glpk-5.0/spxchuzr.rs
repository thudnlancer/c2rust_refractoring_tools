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
pub struct SPXBP {
    pub i: i32,
    pub teta: libc::c_double,
    pub dc: libc::c_double,
    pub dz: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_chuzr_std(
    mut lp: *mut SPXLP,
    mut phase: i32,
    mut beta: *const libc::c_double,
    mut q: i32,
    mut s: libc::c_double,
    mut tcol: *const libc::c_double,
    mut p_flag: *mut i32,
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
    let mut i: i32 = 0;
    let mut i_flag: i32 = 0;
    let mut k: i32 = 0;
    let mut p: i32 = 0;
    let mut alfa: libc::c_double = 0.;
    let mut biga: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut lk: libc::c_double = 0.;
    let mut uk: libc::c_double = 0.;
    let mut teta: libc::c_double = 0.;
    let mut teta_min: libc::c_double = 0.;
    (phase == 1 as i32 || phase == 2 as i32
        || {
            glp_assert_(
                b"phase == 1 || phase == 2\0" as *const u8 as *const i8,
                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                107 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= q && q <= n - m
        || {
            glp_assert_(
                b"1 <= q && q <= n-m\0" as *const u8 as *const i8,
                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                108 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (s == 1.0f64 || s == -1.0f64
        || {
            glp_assert_(
                b"s == +1.0 || s == -1.0\0" as *const u8 as *const i8,
                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                109 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *head.offset((m + q) as isize);
    if *l.offset(k as isize) == -1.7976931348623157e+308f64
        || *u.offset(k as isize) == 1.7976931348623157e+308f64
    {
        p = 0 as i32;
        *p_flag = 0 as i32;
        teta_min = 1.7976931348623157e+308f64;
        biga = 0.0f64;
    } else {
        p = -(1 as i32);
        *p_flag = 0 as i32;
        teta_min = fabs(*l.offset(k as isize) - *u.offset(k as isize));
        biga = 1.0f64;
    }
    let mut current_block_35: u64;
    i = 1 as i32;
    while i <= m {
        k = *head.offset(i as isize);
        alfa = s * *tcol.offset(i as isize);
        if alfa <= -tol_piv {
            if phase == 1 as i32 && *c.offset(k as isize) < 0.0f64 {
                current_block_35 = 1917311967535052937;
            } else {
                if phase == 1 as i32 && *c.offset(k as isize) > 0.0f64 {
                    lk = *u.offset(k as isize);
                    (lk != 1.7976931348623157e+308f64
                        || {
                            glp_assert_(
                                b"lk != +DBL_MAX\0" as *const u8 as *const i8,
                                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                                135 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    i_flag = 1 as i32;
                    current_block_35 = 5601891728916014340;
                } else {
                    lk = *l.offset(k as isize);
                    if lk == -1.7976931348623157e+308f64 {
                        current_block_35 = 1917311967535052937;
                    } else {
                        i_flag = 0 as i32;
                        current_block_35 = 5601891728916014340;
                    }
                }
                match current_block_35 {
                    1917311967535052937 => {}
                    _ => {
                        delta = tol + tol1 * (if lk >= 0.0f64 { lk } else { -lk });
                        if *beta.offset(i as isize) <= lk + delta {
                            teta = 0.0f64;
                        } else {
                            teta = (lk - *beta.offset(i as isize)) / alfa;
                        }
                        current_block_35 = 5529461102203738653;
                    }
                }
            }
        } else if alfa >= tol_piv {
            if phase == 1 as i32 && *c.offset(k as isize) < 0.0f64 {
                uk = *l.offset(k as isize);
                (uk != -1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"uk != -DBL_MAX\0" as *const u8 as *const i8,
                            b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                            158 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                i_flag = 0 as i32;
                current_block_35 = 8704759739624374314;
            } else if phase == 1 as i32 && *c.offset(k as isize) > 0.0f64 {
                current_block_35 = 1917311967535052937;
            } else {
                uk = *u.offset(k as isize);
                if uk == 1.7976931348623157e+308f64 {
                    current_block_35 = 1917311967535052937;
                } else {
                    i_flag = 1 as i32;
                    current_block_35 = 8704759739624374314;
                }
            }
            match current_block_35 {
                1917311967535052937 => {}
                _ => {
                    delta = tol + tol1 * (if uk >= 0.0f64 { uk } else { -uk });
                    if *beta.offset(i as isize) >= uk - delta {
                        teta = 0.0f64;
                    } else {
                        teta = (uk - *beta.offset(i as isize)) / alfa;
                    }
                    current_block_35 = 5529461102203738653;
                }
            }
        } else {
            current_block_35 = 1917311967535052937;
        }
        match current_block_35 {
            5529461102203738653 => {
                (teta >= 0.0f64
                    || {
                        glp_assert_(
                            b"teta >= 0.0\0" as *const u8 as *const i8,
                            b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                            184 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                alfa = if alfa >= 0.0f64 { alfa } else { -alfa };
                if teta_min > teta || teta_min == teta && biga < alfa {
                    p = i;
                    *p_flag = i_flag;
                    teta_min = teta;
                    biga = alfa;
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if p > 0 as i32 {
        k = *head.offset(p as isize);
        if *l.offset(k as isize) == *u.offset(k as isize) {
            *p_flag = 0 as i32;
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_chuzr_harris(
    mut lp: *mut SPXLP,
    mut phase: i32,
    mut beta: *const libc::c_double,
    mut q: i32,
    mut s: libc::c_double,
    mut tcol: *const libc::c_double,
    mut p_flag: *mut i32,
    mut tol_piv: libc::c_double,
    mut tol: libc::c_double,
    mut tol1: libc::c_double,
) -> i32 {
    let mut current_block: u64;
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut i32 = (*lp).head;
    let mut i: i32 = 0;
    let mut i_flag: i32 = 0;
    let mut k: i32 = 0;
    let mut p: i32 = 0;
    let mut alfa: libc::c_double = 0.;
    let mut biga: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut lk: libc::c_double = 0.;
    let mut uk: libc::c_double = 0.;
    let mut teta: libc::c_double = 0.;
    let mut teta_min: libc::c_double = 0.;
    (phase == 1 as i32 || phase == 2 as i32
        || {
            glp_assert_(
                b"phase == 1 || phase == 2\0" as *const u8 as *const i8,
                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                224 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= q && q <= n - m
        || {
            glp_assert_(
                b"1 <= q && q <= n-m\0" as *const u8 as *const i8,
                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                225 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (s == 1.0f64 || s == -1.0f64
        || {
            glp_assert_(
                b"s == +1.0 || s == -1.0\0" as *const u8 as *const i8,
                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                226 as i32,
            );
            1 as i32 != 0
        }) as i32;
    teta_min = 1.7976931348623157e+308f64;
    let mut current_block_25: u64;
    i = 1 as i32;
    while i <= m {
        k = *head.offset(i as isize);
        alfa = s * *tcol.offset(i as isize);
        if alfa <= -tol_piv {
            if phase == 1 as i32 && *c.offset(k as isize) < 0.0f64 {
                current_block_25 = 15619007995458559411;
            } else {
                if phase == 1 as i32 && *c.offset(k as isize) > 0.0f64 {
                    lk = *u.offset(k as isize);
                    (lk != 1.7976931348623157e+308f64
                        || {
                            glp_assert_(
                                b"lk != +DBL_MAX\0" as *const u8 as *const i8,
                                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                                246 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    current_block_25 = 1054647088692577877;
                } else {
                    lk = *l.offset(k as isize);
                    if lk == -1.7976931348623157e+308f64 {
                        current_block_25 = 15619007995458559411;
                    } else {
                        current_block_25 = 1054647088692577877;
                    }
                }
                match current_block_25 {
                    15619007995458559411 => {}
                    _ => {
                        delta = tol + tol1 * (if lk >= 0.0f64 { lk } else { -lk });
                        if *beta.offset(i as isize) < lk {
                            teta = -delta / alfa;
                        } else {
                            teta = (lk - delta - *beta.offset(i as isize)) / alfa;
                        }
                        current_block_25 = 5494826135382683477;
                    }
                }
            }
        } else if alfa >= tol_piv {
            if phase == 1 as i32 && *c.offset(k as isize) < 0.0f64 {
                uk = *l.offset(k as isize);
                (uk != -1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"uk != -DBL_MAX\0" as *const u8 as *const i8,
                            b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                            268 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                current_block_25 = 15125582407903384992;
            } else if phase == 1 as i32 && *c.offset(k as isize) > 0.0f64 {
                current_block_25 = 15619007995458559411;
            } else {
                uk = *u.offset(k as isize);
                if uk == 1.7976931348623157e+308f64 {
                    current_block_25 = 15619007995458559411;
                } else {
                    current_block_25 = 15125582407903384992;
                }
            }
            match current_block_25 {
                15619007995458559411 => {}
                _ => {
                    delta = tol + tol1 * (if uk >= 0.0f64 { uk } else { -uk });
                    if *beta.offset(i as isize) > uk {
                        teta = delta / alfa;
                    } else {
                        teta = (uk + delta - *beta.offset(i as isize)) / alfa;
                    }
                    current_block_25 = 5494826135382683477;
                }
            }
        } else {
            current_block_25 = 15619007995458559411;
        }
        match current_block_25 {
            5494826135382683477 => {
                (teta >= 0.0f64
                    || {
                        glp_assert_(
                            b"teta >= 0.0\0" as *const u8 as *const i8,
                            b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                            292 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                if teta_min > teta {
                    teta_min = teta;
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    k = *head.offset((m + q) as isize);
    if *l.offset(k as isize) != -1.7976931348623157e+308f64
        && *u.offset(k as isize) != 1.7976931348623157e+308f64
    {
        if fabs(*l.offset(k as isize) - *u.offset(k as isize)) <= teta_min {
            p = -(1 as i32);
            *p_flag = 0 as i32;
            current_block = 4448990382659840432;
        } else {
            current_block = 14072441030219150333;
        }
    } else {
        current_block = 14072441030219150333;
    }
    match current_block {
        14072441030219150333 => {
            if teta_min == 1.7976931348623157e+308f64 {
                p = 0 as i32;
                *p_flag = 0 as i32;
            } else {
                p = 0 as i32;
                *p_flag = 0 as i32;
                biga = 0.0f64;
                let mut current_block_50: u64;
                i = 1 as i32;
                while i <= m {
                    k = *head.offset(i as isize);
                    alfa = s * *tcol.offset(i as isize);
                    if alfa <= -tol_piv {
                        if phase == 1 as i32 && *c.offset(k as isize) < 0.0f64 {
                            current_block_50 = 3222590281903869779;
                        } else {
                            if phase == 1 as i32 && *c.offset(k as isize) > 0.0f64 {
                                lk = *u.offset(k as isize);
                                (lk != 1.7976931348623157e+308f64
                                    || {
                                        glp_assert_(
                                            b"lk != +DBL_MAX\0" as *const u8 as *const i8,
                                            b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                                            330 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                                i_flag = 1 as i32;
                                current_block_50 = 8151474771948790331;
                            } else {
                                lk = *l.offset(k as isize);
                                if lk == -1.7976931348623157e+308f64 {
                                    current_block_50 = 3222590281903869779;
                                } else {
                                    i_flag = 0 as i32;
                                    current_block_50 = 8151474771948790331;
                                }
                            }
                            match current_block_50 {
                                3222590281903869779 => {}
                                _ => {
                                    teta = (lk - *beta.offset(i as isize)) / alfa;
                                    current_block_50 = 981995395831942902;
                                }
                            }
                        }
                    } else if alfa >= tol_piv {
                        if phase == 1 as i32 && *c.offset(k as isize) < 0.0f64 {
                            uk = *l.offset(k as isize);
                            (uk != -1.7976931348623157e+308f64
                                || {
                                    glp_assert_(
                                        b"uk != -DBL_MAX\0" as *const u8 as *const i8,
                                        b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                                        349 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            i_flag = 0 as i32;
                            current_block_50 = 5807581744382915773;
                        } else if phase == 1 as i32 && *c.offset(k as isize) > 0.0f64 {
                            current_block_50 = 3222590281903869779;
                        } else {
                            uk = *u.offset(k as isize);
                            if uk == 1.7976931348623157e+308f64 {
                                current_block_50 = 3222590281903869779;
                            } else {
                                i_flag = 1 as i32;
                                current_block_50 = 5807581744382915773;
                            }
                        }
                        match current_block_50 {
                            3222590281903869779 => {}
                            _ => {
                                teta = (uk - *beta.offset(i as isize)) / alfa;
                                current_block_50 = 981995395831942902;
                            }
                        }
                    } else {
                        current_block_50 = 3222590281903869779;
                    }
                    match current_block_50 {
                        981995395831942902 => {
                            alfa = if alfa >= 0.0f64 { alfa } else { -alfa };
                            if teta <= teta_min && biga < alfa {
                                p = i;
                                *p_flag = i_flag;
                                biga = alfa;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                    i;
                }
                (1 as i32 <= p && p <= m
                    || {
                        glp_assert_(
                            b"1 <= p && p <= m\0" as *const u8 as *const i8,
                            b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                            378 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                k = *head.offset(p as isize);
                if *l.offset(k as isize) == *u.offset(k as isize) {
                    *p_flag = 0 as i32;
                }
            }
        }
        _ => {}
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_ls_eval_bp(
    mut lp: *mut SPXLP,
    mut beta: *const libc::c_double,
    mut q: i32,
    mut dq: libc::c_double,
    mut tcol: *const libc::c_double,
    mut tol_piv: libc::c_double,
    mut bp: *mut SPXBP,
) -> i32 {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut i32 = (*lp).head;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut nbp: i32 = 0;
    let mut s: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    (1 as i32 <= q && q <= n - m
        || {
            glp_assert_(
                b"1 <= q && q <= n-m\0" as *const u8 as *const i8,
                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                411 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (dq != 0.0f64
        || {
            glp_assert_(
                b"dq != 0.0\0" as *const u8 as *const i8,
                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                412 as i32,
            );
            1 as i32 != 0
        }) as i32;
    s = if dq < 0.0f64 { 1.0f64 } else { -1.0f64 };
    nbp = 0 as i32;
    k = *head.offset((m + q) as isize);
    if *l.offset(k as isize) != -1.7976931348623157e+308f64
        && *u.offset(k as isize) != 1.7976931348623157e+308f64
    {
        nbp += 1;
        nbp;
        (*bp.offset(nbp as isize)).i = 0 as i32;
        (*l.offset(k as isize) < *u.offset(k as isize)
            || {
                glp_assert_(
                    b"l[k] < u[k]\0" as *const u8 as *const i8,
                    b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                    421 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*bp.offset(nbp as isize)).teta = *u.offset(k as isize) - *l.offset(k as isize);
        (*bp.offset(nbp as isize)).dc = s;
    }
    let mut current_block_61: u64;
    i = 1 as i32;
    while i <= m {
        k = *head.offset(i as isize);
        (*l.offset(k as isize) <= *u.offset(k as isize)
            || {
                glp_assert_(
                    b"l[k] <= u[k]\0" as *const u8 as *const i8,
                    b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                    429 as i32,
                );
                1 as i32 != 0
            }) as i32;
        alfa = s * *tcol.offset(i as isize);
        if alfa >= tol_piv {
            if *l.offset(k as isize) == *u.offset(k as isize) {
                if *c.offset(k as isize) <= 0.0f64 {
                    nbp += 1;
                    nbp;
                    (*bp.offset(nbp as isize)).i = i;
                    (*bp.offset(nbp as isize)).teta = (*l.offset(k as isize)
                        - *beta.offset(i as isize)) / alfa;
                    (*bp.offset(nbp as isize)).dc = 1.0f64 - *c.offset(k as isize);
                }
            } else {
                if *l.offset(k as isize) != -1.7976931348623157e+308f64
                    && *c.offset(k as isize) < 0.0f64
                {
                    nbp += 1;
                    nbp;
                    (*bp.offset(nbp as isize)).i = i;
                    (*bp.offset(nbp as isize)).teta = (*l.offset(k as isize)
                        - *beta.offset(i as isize)) / alfa;
                    (*bp.offset(nbp as isize)).dc = 1.0f64;
                }
                if *u.offset(k as isize) != 1.7976931348623157e+308f64
                    && *c.offset(k as isize) <= 0.0f64
                {
                    nbp += 1;
                    nbp;
                    (*bp.offset(nbp as isize)).i = -i;
                    (*bp.offset(nbp as isize)).teta = (*u.offset(k as isize)
                        - *beta.offset(i as isize)) / alfa;
                    (*bp.offset(nbp as isize)).dc = 1.0f64;
                }
            }
            current_block_61 = 14220266465818359136;
        } else if alfa <= -tol_piv {
            if *l.offset(k as isize) == *u.offset(k as isize) {
                if *c.offset(k as isize) >= 0.0f64 {
                    nbp += 1;
                    nbp;
                    (*bp.offset(nbp as isize)).i = i;
                    (*bp.offset(nbp as isize)).teta = (*l.offset(k as isize)
                        - *beta.offset(i as isize)) / alfa;
                    (*bp.offset(nbp as isize)).dc = -1.0f64 - *c.offset(k as isize);
                }
            } else {
                if *l.offset(k as isize) != -1.7976931348623157e+308f64
                    && *c.offset(k as isize) >= 0.0f64
                {
                    nbp += 1;
                    nbp;
                    (*bp.offset(nbp as isize)).i = i;
                    (*bp.offset(nbp as isize)).teta = (*l.offset(k as isize)
                        - *beta.offset(i as isize)) / alfa;
                    (*bp.offset(nbp as isize)).dc = -1.0f64;
                }
                if *u.offset(k as isize) != 1.7976931348623157e+308f64
                    && *c.offset(k as isize) > 0.0f64
                {
                    nbp += 1;
                    nbp;
                    (*bp.offset(nbp as isize)).i = -i;
                    (*bp.offset(nbp as isize)).teta = (*u.offset(k as isize)
                        - *beta.offset(i as isize)) / alfa;
                    (*bp.offset(nbp as isize)).dc = -1.0f64;
                }
            }
            current_block_61 = 14220266465818359136;
        } else {
            current_block_61 = 2968425633554183086;
        }
        match current_block_61 {
            14220266465818359136 => {
                if (*bp.offset(nbp as isize)).teta < 0.0f64 {
                    (*bp.offset(nbp as isize)).teta = 0.0f64;
                }
            }
            _ => {}
        }
        i += 1;
        i;
    }
    (nbp <= 2 as i32 * m + 1 as i32
        || {
            glp_assert_(
                b"nbp <= 2*m+1\0" as *const u8 as *const i8,
                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                508 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return nbp;
}
unsafe extern "C" fn fcmp(
    mut v1: *const libc::c_void,
    mut v2: *const libc::c_void,
) -> i32 {
    let mut p1: *const SPXBP = v1 as *const SPXBP;
    let mut p2: *const SPXBP = v2 as *const SPXBP;
    if (*p1).teta < (*p2).teta {
        return -(1 as i32)
    } else if (*p1).teta > (*p2).teta {
        return 1 as i32
    } else {
        return 0 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_ls_select_bp(
    mut lp: *mut SPXLP,
    mut tcol: *const libc::c_double,
    mut nbp: i32,
    mut bp: *mut SPXBP,
    mut num: i32,
    mut slope: *mut libc::c_double,
    mut teta_lim: libc::c_double,
) -> i32 {
    let mut m: i32 = (*lp).m;
    let mut i: i32 = 0;
    let mut t: i32 = 0;
    let mut num1: i32 = 0;
    let mut teta: libc::c_double = 0.;
    let mut dz: libc::c_double = 0.;
    (0 as i32 <= num && num <= nbp && nbp <= m + m + 1 as i32
        || {
            glp_assert_(
                b"0 <= num && num <= nbp && nbp <= m+m+1\0" as *const u8 as *const i8,
                b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                553 as i32,
            );
            1 as i32 != 0
        }) as i32;
    num1 = num;
    t = num + 1 as i32;
    while t <= nbp {
        if (*bp.offset(t as isize)).teta <= teta_lim {
            num1 += 1;
            num1;
            i = (*bp.offset(num1 as isize)).i;
            teta = (*bp.offset(num1 as isize)).teta;
            dz = (*bp.offset(num1 as isize)).dc;
            (*bp.offset(num1 as isize)).i = (*bp.offset(t as isize)).i;
            (*bp.offset(num1 as isize)).teta = (*bp.offset(t as isize)).teta;
            (*bp.offset(num1 as isize)).dc = (*bp.offset(t as isize)).dc;
            (*bp.offset(t as isize)).i = i;
            (*bp.offset(t as isize)).teta = teta;
            (*bp.offset(t as isize)).dc = dz;
        }
        t += 1;
        t;
    }
    if num1 - num > 1 as i32 {
        qsort(
            &mut *bp.offset((num + 1 as i32) as isize) as *mut SPXBP
                as *mut libc::c_void,
            (num1 - num) as size_t,
            ::core::mem::size_of::<SPXBP>() as u64,
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
        dz = *slope
            * ((*bp.offset(t as isize)).teta
                - (if t == 1 as i32 {
                    0.0f64
                } else {
                    (*bp.offset((t - 1 as i32) as isize)).teta
                }));
        (*bp.offset(t as isize)).dz = (if t == 1 as i32 {
            0.0f64
        } else {
            (*bp.offset((t - 1 as i32) as isize)).dz
        }) + dz;
        i = if (*bp.offset(t as isize)).i >= 0 as i32 {
            (*bp.offset(t as isize)).i
        } else {
            -(*bp.offset(t as isize)).i
        };
        (0 as i32 <= i && i <= m
            || {
                glp_assert_(
                    b"0 <= i && i <= m\0" as *const u8 as *const i8,
                    b"simplex/spxchuzr.c\0" as *const u8 as *const i8,
                    582 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if i == 0 as i32 {
            *slope += fabs(1.0f64 * (*bp.offset(t as isize)).dc);
        } else {
            *slope += fabs(*tcol.offset(i as isize) * (*bp.offset(t as isize)).dc);
        }
        t += 1;
        t;
    }
    return num1;
}