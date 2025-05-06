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
    pub type BFX;
    fn glp_difftime(t1: libc::c_double, t0: libc::c_double) -> libc::c_double;
    fn glp_time() -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_mpq_add(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_sub(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_mul(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_neg(z: mpq_t, x: mpq_t);
    fn _glp_mpq_cmp(x: mpq_t, y: mpq_t) -> i32;
    fn _glp_mpq_sgn(x: mpq_t) -> i32;
    fn _glp_mpq_get_d(x: mpq_t) -> libc::c_double;
    fn _glp_mpq_set_si(x: mpq_t, p: i32, q: u32);
    fn _glp_mpq_set(z: mpq_t, x: mpq_t);
    fn _glp_mpq_clear(x: mpq_t);
    fn _glp_mpq_init() -> mpq_t;
    fn _glp_ssx_factorize(ssx: *mut SSX) -> i32;
    fn _glp_ssx_eval_bbar(ssx: *mut SSX);
    fn _glp_ssx_eval_pi(ssx: *mut SSX);
    fn _glp_ssx_eval_cbar(ssx: *mut SSX);
    fn _glp_ssx_eval_rho(ssx: *mut SSX);
    fn _glp_ssx_eval_row(ssx: *mut SSX);
    fn _glp_ssx_eval_col(ssx: *mut SSX);
    fn _glp_ssx_chuzc(ssx: *mut SSX);
    fn _glp_ssx_chuzr(ssx: *mut SSX);
    fn _glp_ssx_update_bbar(ssx: *mut SSX);
    fn _glp_ssx_update_pi(ssx: *mut SSX);
    fn _glp_ssx_update_cbar(ssx: *mut SSX);
    fn _glp_ssx_change_basis(ssx: *mut SSX);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz {
    pub val: i32,
    pub ptr: *mut mpz_seg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpz_seg {
    pub d: [libc::c_ushort; 6],
    pub next: *mut mpz_seg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpq {
    pub p: mpz,
    pub q: mpz,
}
pub type mpq_t = *mut mpq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSX {
    pub m: i32,
    pub n: i32,
    pub type_0: *mut i32,
    pub lb: *mut mpq_t,
    pub ub: *mut mpq_t,
    pub dir: i32,
    pub coef: *mut mpq_t,
    pub A_ptr: *mut i32,
    pub A_ind: *mut i32,
    pub A_val: *mut mpq_t,
    pub stat: *mut i32,
    pub Q_row: *mut i32,
    pub Q_col: *mut i32,
    pub binv: *mut BFX,
    pub bbar: *mut mpq_t,
    pub pi: *mut mpq_t,
    pub cbar: *mut mpq_t,
    pub p: i32,
    pub rho: *mut mpq_t,
    pub ap: *mut mpq_t,
    pub q: i32,
    pub aq: *mut mpq_t,
    pub q_dir: i32,
    pub p_stat: i32,
    pub delta: mpq_t,
    pub msg_lev: i32,
    pub it_lim: i32,
    pub it_cnt: i32,
    pub tm_lim: libc::c_double,
    pub out_frq: libc::c_double,
    pub tm_beg: libc::c_double,
    pub tm_lag: libc::c_double,
}
unsafe extern "C" fn show_progress(mut ssx: *mut SSX, mut phase: i32) {
    let mut i: i32 = 0;
    let mut def: i32 = 0 as i32;
    i = 1 as i32;
    while i <= (*ssx).m {
        if *((*ssx).type_0).offset(*((*ssx).Q_col).offset(i as isize) as isize)
            == 4 as i32
        {
            def += 1;
            def;
        }
        i += 1;
        i;
    }
    glp_printf(
        b"%s%6d:   %s = %22.15g   (%d)\n\0" as *const u8 as *const i8,
        if phase == 1 as i32 {
            b" \0" as *const u8 as *const i8
        } else {
            b"*\0" as *const u8 as *const i8
        },
        (*ssx).it_cnt,
        if phase == 1 as i32 {
            b"infsum\0" as *const u8 as *const i8
        } else {
            b"objval\0" as *const u8 as *const i8
        },
        _glp_mpq_get_d(*((*ssx).bbar).offset(0 as i32 as isize)),
        def,
    );
    (*ssx).tm_lag = glp_time();
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_phase_I(mut ssx: *mut SSX) -> i32 {
    let mut m: i32 = (*ssx).m;
    let mut n: i32 = (*ssx).n;
    let mut type_0: *mut i32 = (*ssx).type_0;
    let mut lb: *mut mpq_t = (*ssx).lb;
    let mut ub: *mut mpq_t = (*ssx).ub;
    let mut coef: *mut mpq_t = (*ssx).coef;
    let mut A_ptr: *mut i32 = (*ssx).A_ptr;
    let mut A_ind: *mut i32 = (*ssx).A_ind;
    let mut A_val: *mut mpq_t = (*ssx).A_val;
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut bbar: *mut mpq_t = (*ssx).bbar;
    let mut pi: *mut mpq_t = (*ssx).pi;
    let mut cbar: *mut mpq_t = (*ssx).cbar;
    let mut orig_type: *mut i32 = 0 as *mut i32;
    let mut orig_dir: i32 = 0;
    let mut orig_lb: *mut mpq_t = 0 as *mut mpq_t;
    let mut orig_ub: *mut mpq_t = 0 as *mut mpq_t;
    let mut orig_coef: *mut mpq_t = 0 as *mut mpq_t;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut ret: i32 = 0;
    orig_type = glp_alloc(1 as i32 + m + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    orig_lb = glp_alloc(1 as i32 + m + n, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    orig_ub = glp_alloc(1 as i32 + m + n, ::core::mem::size_of::<mpq_t>() as u64 as i32)
        as *mut mpq_t;
    orig_coef = glp_alloc(
        1 as i32 + m + n,
        ::core::mem::size_of::<mpq_t>() as u64 as i32,
    ) as *mut mpq_t;
    k = 1 as i32;
    while k <= m + n {
        *orig_type.offset(k as isize) = *type_0.offset(k as isize);
        let ref mut fresh0 = *orig_lb.offset(k as isize);
        *fresh0 = _glp_mpq_init();
        _glp_mpq_set(*orig_lb.offset(k as isize), *lb.offset(k as isize));
        let ref mut fresh1 = *orig_ub.offset(k as isize);
        *fresh1 = _glp_mpq_init();
        _glp_mpq_set(*orig_ub.offset(k as isize), *ub.offset(k as isize));
        k += 1;
        k;
    }
    orig_dir = (*ssx).dir;
    k = 0 as i32;
    while k <= m + n {
        let ref mut fresh2 = *orig_coef.offset(k as isize);
        *fresh2 = _glp_mpq_init();
        _glp_mpq_set(*orig_coef.offset(k as isize), *coef.offset(k as isize));
        k += 1;
        k;
    }
    (*ssx).dir = 0 as i32;
    k = 0 as i32;
    while k <= m + n {
        _glp_mpq_set_si(*coef.offset(k as isize), 0 as i32, 1 as i32 as u32);
        k += 1;
        k;
    }
    _glp_mpq_set_si(*bbar.offset(0 as i32 as isize), 0 as i32, 1 as i32 as u32);
    i = 1 as i32;
    while i <= m {
        let mut t: i32 = 0;
        k = *Q_col.offset(i as isize);
        t = *type_0.offset(k as isize);
        if t == 1 as i32 || t == 3 as i32 || t == 4 as i32 {
            if _glp_mpq_cmp(*bbar.offset(i as isize), *lb.offset(k as isize)) < 0 as i32
            {
                *type_0.offset(k as isize) = 2 as i32;
                _glp_mpq_set(*ub.offset(k as isize), *lb.offset(k as isize));
                _glp_mpq_set_si(*lb.offset(k as isize), 0 as i32, 1 as i32 as u32);
                _glp_mpq_set_si(*coef.offset(k as isize), -(1 as i32), 1 as i32 as u32);
                _glp_mpq_add(
                    *bbar.offset(0 as i32 as isize),
                    *bbar.offset(0 as i32 as isize),
                    *ub.offset(k as isize),
                );
                _glp_mpq_sub(
                    *bbar.offset(0 as i32 as isize),
                    *bbar.offset(0 as i32 as isize),
                    *bbar.offset(i as isize),
                );
            }
        }
        if t == 2 as i32 || t == 3 as i32 || t == 4 as i32 {
            if _glp_mpq_cmp(*bbar.offset(i as isize), *ub.offset(k as isize)) > 0 as i32
            {
                *type_0.offset(k as isize) = 1 as i32;
                _glp_mpq_set(*lb.offset(k as isize), *ub.offset(k as isize));
                _glp_mpq_set_si(*ub.offset(k as isize), 0 as i32, 1 as i32 as u32);
                _glp_mpq_set_si(*coef.offset(k as isize), 1 as i32, 1 as i32 as u32);
                _glp_mpq_add(
                    *bbar.offset(0 as i32 as isize),
                    *bbar.offset(0 as i32 as isize),
                    *bbar.offset(i as isize),
                );
                _glp_mpq_sub(
                    *bbar.offset(0 as i32 as isize),
                    *bbar.offset(0 as i32 as isize),
                    *lb.offset(k as isize),
                );
            }
        }
        i += 1;
        i;
    }
    _glp_ssx_eval_pi(ssx);
    _glp_ssx_eval_cbar(ssx);
    if (*ssx).msg_lev >= 2 as i32 {
        show_progress(ssx, 1 as i32);
    }
    loop {
        if (*ssx).msg_lev >= 2 as i32 {
            if glp_difftime(glp_time(), (*ssx).tm_lag) >= (*ssx).out_frq - 0.001f64 {
                show_progress(ssx, 1 as i32);
            }
        }
        if _glp_mpq_sgn(*bbar.offset(0 as i32 as isize)) == 0 as i32 {
            ret = 0 as i32;
            break;
        } else if (*ssx).it_lim == 0 as i32 {
            ret = 2 as i32;
            break;
        } else if (*ssx).tm_lim >= 0.0f64
            && (*ssx).tm_lim <= glp_difftime(glp_time(), (*ssx).tm_beg)
        {
            ret = 3 as i32;
            break;
        } else {
            _glp_ssx_chuzc(ssx);
            if (*ssx).q == 0 as i32 {
                ret = 1 as i32;
                break;
            } else {
                _glp_ssx_eval_col(ssx);
                _glp_ssx_chuzr(ssx);
                ((*ssx).p != 0 as i32
                    || {
                        glp_assert_(
                            b"ssx->p != 0\0" as *const u8 as *const i8,
                            b"draft/glpssx02.c\0" as *const u8 as *const i8,
                            186 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                _glp_ssx_update_bbar(ssx);
                if (*ssx).p > 0 as i32 {
                    _glp_ssx_eval_rho(ssx);
                    _glp_ssx_eval_row(ssx);
                    (_glp_mpq_cmp(
                        *((*ssx).aq).offset((*ssx).p as isize),
                        *((*ssx).ap).offset((*ssx).q as isize),
                    ) == 0 as i32
                        || {
                            glp_assert_(
                                b"mpq_cmp(ssx->aq[ssx->p], ssx->ap[ssx->q]) == 0\0"
                                    as *const u8 as *const i8,
                                b"draft/glpssx02.c\0" as *const u8 as *const i8,
                                194 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    _glp_ssx_update_pi(ssx);
                    _glp_ssx_update_cbar(ssx);
                }
                if (*ssx).p > 0 as i32 {
                    k = *Q_col.offset((*ssx).p as isize);
                    if *type_0.offset(k as isize) != *orig_type.offset(k as isize) {
                        *type_0.offset(k as isize) = *orig_type.offset(k as isize);
                        _glp_mpq_set(
                            *lb.offset(k as isize),
                            *orig_lb.offset(k as isize),
                        );
                        _glp_mpq_set(
                            *ub.offset(k as isize),
                            *orig_ub.offset(k as isize),
                        );
                        ((*ssx).p_stat == 1 as i32 || (*ssx).p_stat == 2 as i32
                            || {
                                glp_assert_(
                                    b"ssx->p_stat == SSX_NL || ssx->p_stat == SSX_NU\0"
                                        as *const u8 as *const i8,
                                    b"draft/glpssx02.c\0" as *const u8 as *const i8,
                                    211 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        (*ssx).p_stat = if (*ssx).p_stat == 1 as i32 {
                            2 as i32
                        } else {
                            1 as i32
                        };
                        if *type_0.offset(k as isize) == 4 as i32 {
                            (*ssx).p_stat = 4 as i32;
                        }
                        _glp_mpq_set_si(
                            *coef.offset(k as isize),
                            0 as i32,
                            1 as i32 as u32,
                        );
                        if k <= m {
                            _glp_mpq_neg(
                                *cbar.offset((*ssx).q as isize),
                                *pi.offset(k as isize),
                            );
                        } else {
                            let mut ptr: i32 = 0;
                            let mut temp: mpq_t = 0 as *mut mpq;
                            temp = _glp_mpq_init();
                            _glp_mpq_set_si(
                                *cbar.offset((*ssx).q as isize),
                                0 as i32,
                                1 as i32 as u32,
                            );
                            ptr = *A_ptr.offset((k - m) as isize);
                            while ptr < *A_ptr.offset((k - m + 1 as i32) as isize) {
                                _glp_mpq_mul(
                                    temp,
                                    *pi.offset(*A_ind.offset(ptr as isize) as isize),
                                    *A_val.offset(ptr as isize),
                                );
                                _glp_mpq_add(
                                    *cbar.offset((*ssx).q as isize),
                                    *cbar.offset((*ssx).q as isize),
                                    temp,
                                );
                                ptr += 1;
                                ptr;
                            }
                            _glp_mpq_clear(temp);
                        }
                    }
                }
                _glp_ssx_change_basis(ssx);
                if (*ssx).it_lim > 0 as i32 {
                    (*ssx).it_lim -= 1;
                    (*ssx).it_lim;
                }
                (*ssx).it_cnt += 1;
                (*ssx).it_cnt;
            }
        }
    }
    if (*ssx).msg_lev >= 2 as i32 {
        show_progress(ssx, 1 as i32);
    }
    k = 1 as i32;
    while k <= m + n {
        *type_0.offset(k as isize) = *orig_type.offset(k as isize);
        _glp_mpq_set(*lb.offset(k as isize), *orig_lb.offset(k as isize));
        _glp_mpq_clear(*orig_lb.offset(k as isize));
        _glp_mpq_set(*ub.offset(k as isize), *orig_ub.offset(k as isize));
        _glp_mpq_clear(*orig_ub.offset(k as isize));
        k += 1;
        k;
    }
    (*ssx).dir = orig_dir;
    k = 0 as i32;
    while k <= m + n {
        _glp_mpq_set(*coef.offset(k as isize), *orig_coef.offset(k as isize));
        _glp_mpq_clear(*orig_coef.offset(k as isize));
        k += 1;
        k;
    }
    glp_free(orig_type as *mut libc::c_void);
    glp_free(orig_lb as *mut libc::c_void);
    glp_free(orig_ub as *mut libc::c_void);
    glp_free(orig_coef as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_phase_II(mut ssx: *mut SSX) -> i32 {
    let mut ret: i32 = 0;
    if (*ssx).msg_lev >= 2 as i32 {
        show_progress(ssx, 2 as i32);
    }
    loop {
        if (*ssx).msg_lev >= 2 as i32 {
            if glp_difftime(glp_time(), (*ssx).tm_lag) >= (*ssx).out_frq - 0.001f64 {
                show_progress(ssx, 2 as i32);
            }
        }
        if (*ssx).it_lim == 0 as i32 {
            ret = 2 as i32;
            break;
        } else if (*ssx).tm_lim >= 0.0f64
            && (*ssx).tm_lim <= glp_difftime(glp_time(), (*ssx).tm_beg)
        {
            ret = 3 as i32;
            break;
        } else {
            _glp_ssx_chuzc(ssx);
            if (*ssx).q == 0 as i32 {
                ret = 0 as i32;
                break;
            } else {
                _glp_ssx_eval_col(ssx);
                _glp_ssx_chuzr(ssx);
                if (*ssx).p == 0 as i32 {
                    ret = 1 as i32;
                    break;
                } else {
                    _glp_ssx_update_bbar(ssx);
                    if (*ssx).p > 0 as i32 {
                        _glp_ssx_eval_rho(ssx);
                        _glp_ssx_eval_row(ssx);
                        (_glp_mpq_cmp(
                            *((*ssx).aq).offset((*ssx).p as isize),
                            *((*ssx).ap).offset((*ssx).q as isize),
                        ) == 0 as i32
                            || {
                                glp_assert_(
                                    b"mpq_cmp(ssx->aq[ssx->p], ssx->ap[ssx->q]) == 0\0"
                                        as *const u8 as *const i8,
                                    b"draft/glpssx02.c\0" as *const u8 as *const i8,
                                    347 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        _glp_ssx_update_cbar(ssx);
                    }
                    _glp_ssx_change_basis(ssx);
                    if (*ssx).it_lim > 0 as i32 {
                        (*ssx).it_lim -= 1;
                        (*ssx).it_lim;
                    }
                    (*ssx).it_cnt += 1;
                    (*ssx).it_cnt;
                }
            }
        }
    }
    if (*ssx).msg_lev >= 2 as i32 {
        show_progress(ssx, 2 as i32);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ssx_driver(mut ssx: *mut SSX) -> i32 {
    let mut m: i32 = (*ssx).m;
    let mut type_0: *mut i32 = (*ssx).type_0;
    let mut lb: *mut mpq_t = (*ssx).lb;
    let mut ub: *mut mpq_t = (*ssx).ub;
    let mut Q_col: *mut i32 = (*ssx).Q_col;
    let mut bbar: *mut mpq_t = (*ssx).bbar;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut ret: i32 = 0;
    (*ssx).tm_beg = glp_time();
    if _glp_ssx_factorize(ssx) != 0 {
        if (*ssx).msg_lev >= 1 as i32 {
            glp_printf(
                b"Initial basis matrix is singular\n\0" as *const u8 as *const i8,
            );
        }
        ret = 7 as i32;
    } else {
        _glp_ssx_eval_bbar(ssx);
        i = 1 as i32;
        while i <= m {
            let mut t: i32 = 0;
            k = *Q_col.offset(i as isize);
            t = *type_0.offset(k as isize);
            if t == 1 as i32 || t == 3 as i32 || t == 4 as i32 {
                if _glp_mpq_cmp(*bbar.offset(i as isize), *lb.offset(k as isize))
                    < 0 as i32
                {
                    break;
                }
            }
            if t == 2 as i32 || t == 3 as i32 || t == 4 as i32 {
                if _glp_mpq_cmp(*bbar.offset(i as isize), *ub.offset(k as isize))
                    > 0 as i32
                {
                    break;
                }
            }
            i += 1;
            i;
        }
        if i > m {
            ret = 0 as i32;
        } else {
            ret = _glp_ssx_phase_I(ssx);
            match ret {
                0 => {
                    ret = 0 as i32;
                }
                1 => {
                    if (*ssx).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"PROBLEM HAS NO FEASIBLE SOLUTION\n\0" as *const u8
                                as *const i8,
                        );
                    }
                    ret = 1 as i32;
                }
                2 => {
                    if (*ssx).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"ITERATIONS LIMIT EXCEEDED; SEARCH TERMINATED\n\0"
                                as *const u8 as *const i8,
                        );
                    }
                    ret = 3 as i32;
                }
                3 => {
                    if (*ssx).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"TIME LIMIT EXCEEDED; SEARCH TERMINATED\n\0" as *const u8
                                as *const i8,
                        );
                    }
                    ret = 5 as i32;
                }
                _ => {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const i8,
                                b"draft/glpssx02.c\0" as *const u8 as *const i8,
                                463 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
            _glp_ssx_eval_bbar(ssx);
        }
        _glp_ssx_eval_pi(ssx);
        _glp_ssx_eval_cbar(ssx);
        if !(ret != 0 as i32) {
            ret = _glp_ssx_phase_II(ssx);
            match ret {
                0 => {
                    if (*ssx).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"OPTIMAL SOLUTION FOUND\n\0" as *const u8 as *const i8,
                        );
                    }
                    ret = 0 as i32;
                }
                1 => {
                    if (*ssx).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"PROBLEM HAS UNBOUNDED SOLUTION\n\0" as *const u8
                                as *const i8,
                        );
                    }
                    ret = 2 as i32;
                }
                2 => {
                    if (*ssx).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"ITERATIONS LIMIT EXCEEDED; SEARCH TERMINATED\n\0"
                                as *const u8 as *const i8,
                        );
                    }
                    ret = 4 as i32;
                }
                3 => {
                    if (*ssx).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"TIME LIMIT EXCEEDED; SEARCH TERMINATED\n\0" as *const u8
                                as *const i8,
                        );
                    }
                    ret = 6 as i32;
                }
                _ => {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const i8,
                                b"draft/glpssx02.c\0" as *const u8 as *const i8,
                                506 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
        }
    }
    if (*ssx).tm_lim >= 0.0f64 {
        (*ssx).tm_lim -= glp_difftime(glp_time(), (*ssx).tm_beg);
        if (*ssx).tm_lim < 0.0f64 {
            (*ssx).tm_lim = 0.0f64;
        }
    }
    return ret;
}