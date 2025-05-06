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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_bfd_factorize(
        bfd: *mut BFD,
        m: i32,
        col: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                i32,
                *mut i32,
                *mut libc::c_double,
            ) -> i32,
        >,
        info: *mut libc::c_void,
    ) -> i32;
    fn _glp_bfd_ftran(bfd: *mut BFD, x: *mut libc::c_double);
    fn _glp_bfd_btran(bfd: *mut BFD, x: *mut libc::c_double);
    fn _glp_bfd_btran_s(bfd: *mut BFD, x: *mut FVS);
    fn _glp_bfd_update(
        bfd: *mut BFD,
        j: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
    ) -> i32;
    fn _glp_fvs_clear_vec(x: *mut FVS);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FVS {
    pub n: i32,
    pub nnz: i32,
    pub ind: *mut i32,
    pub vec: *mut libc::c_double,
}
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
unsafe extern "C" fn jth_col(
    mut info: *mut libc::c_void,
    mut j: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut lp: *mut SPXLP = info as *mut SPXLP;
    let mut m: i32 = (*lp).m;
    let mut A_ptr: *mut i32 = (*lp).A_ptr;
    let mut head: *mut i32 = (*lp).head;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut len: i32 = 0;
    (1 as i32 <= j && j <= m
        || {
            glp_assert_(
                b"1 <= j && j <= m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                42 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *head.offset(j as isize);
    ptr = *A_ptr.offset(k as isize);
    len = *A_ptr.offset((k + 1 as i32) as isize) - ptr;
    memcpy(
        &mut *ind.offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        &mut *((*lp).A_ind).offset(ptr as isize) as *mut i32 as *const libc::c_void,
        (len as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    memcpy(
        &mut *val.offset(1 as i32 as isize) as *mut libc::c_double as *mut libc::c_void,
        &mut *((*lp).A_val).offset(ptr as isize) as *mut libc::c_double
            as *const libc::c_void,
        (len as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_factorize(mut lp: *mut SPXLP) -> i32 {
    let mut ret: i32 = 0;
    ret = _glp_bfd_factorize(
        (*lp).bfd,
        (*lp).m,
        Some(
            jth_col
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    i32,
                    *mut i32,
                    *mut libc::c_double,
                ) -> i32,
        ),
        lp as *mut libc::c_void,
    );
    (*lp).valid = (ret == 0 as i32) as i32;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_eval_beta(
    mut lp: *mut SPXLP,
    mut beta: *mut libc::c_double,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut A_ptr: *mut i32 = (*lp).A_ptr;
    let mut A_ind: *mut i32 = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut b: *mut libc::c_double = (*lp).b;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut i32 = (*lp).head;
    let mut flag: *mut i8 = (*lp).flag;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut fj: libc::c_double = 0.;
    let mut y: *mut libc::c_double = 0 as *mut libc::c_double;
    y = beta;
    memcpy(
        &mut *y.offset(1 as i32 as isize) as *mut libc::c_double as *mut libc::c_void,
        &mut *b.offset(1 as i32 as isize) as *mut libc::c_double as *const libc::c_void,
        (m as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    j = 1 as i32;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        fj = if *flag.offset(j as isize) as i32 != 0 {
            *u.offset(k as isize)
        } else {
            *l.offset(k as isize)
        };
        if !(fj == 0.0f64 || fj == -1.7976931348623157e+308f64) {
            ptr = *A_ptr.offset(k as isize);
            end = *A_ptr.offset((k + 1 as i32) as isize);
            while ptr < end {
                *y.offset(*A_ind.offset(ptr as isize) as isize)
                    -= *A_val.offset(ptr as isize) * fj;
                ptr += 1;
                ptr;
            }
        }
        j += 1;
        j;
    }
    ((*lp).valid != 0
        || {
            glp_assert_(
                b"lp->valid\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                122 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_bfd_ftran((*lp).bfd, beta);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_eval_obj(
    mut lp: *mut SPXLP,
    mut beta: *const libc::c_double,
) -> libc::c_double {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut i32 = (*lp).head;
    let mut flag: *mut i8 = (*lp).flag;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut fj: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    z = *c.offset(0 as i32 as isize);
    i = 1 as i32;
    while i <= m {
        k = *head.offset(i as isize);
        z += *c.offset(k as isize) * *beta.offset(i as isize);
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n - m {
        k = *head.offset((m + j) as isize);
        fj = if *flag.offset(j as isize) as i32 != 0 {
            *u.offset(k as isize)
        } else {
            *l.offset(k as isize)
        };
        if !(fj == 0.0f64 || fj == -1.7976931348623157e+308f64) {
            z += *c.offset(k as isize) * fj;
        }
        j += 1;
        j;
    }
    return z;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_eval_pi(
    mut lp: *mut SPXLP,
    mut pi: *mut libc::c_double,
) {
    let mut m: i32 = (*lp).m;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut head: *mut i32 = (*lp).head;
    let mut i: i32 = 0;
    let mut cB: *mut libc::c_double = 0 as *mut libc::c_double;
    cB = pi;
    i = 1 as i32;
    while i <= m {
        *cB.offset(i as isize) = *c.offset(*head.offset(i as isize) as isize);
        i += 1;
        i;
    }
    _glp_bfd_btran((*lp).bfd, pi);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_eval_dj(
    mut lp: *mut SPXLP,
    mut pi: *const libc::c_double,
    mut j: i32,
) -> libc::c_double {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut A_ptr: *mut i32 = (*lp).A_ptr;
    let mut A_ind: *mut i32 = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut dj: libc::c_double = 0.;
    (1 as i32 <= j && j <= n - m
        || {
            glp_assert_(
                b"1 <= j && j <= n-m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                237 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *((*lp).head).offset((m + j) as isize);
    dj = *((*lp).c).offset(k as isize);
    ptr = *A_ptr.offset(k as isize);
    end = *A_ptr.offset((k + 1 as i32) as isize);
    while ptr < end {
        dj
            -= *A_val.offset(ptr as isize)
                * *pi.offset(*A_ind.offset(ptr as isize) as isize);
        ptr += 1;
        ptr;
    }
    return dj;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_eval_tcol(
    mut lp: *mut SPXLP,
    mut j: i32,
    mut tcol: *mut libc::c_double,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut A_ptr: *mut i32 = (*lp).A_ptr;
    let mut A_ind: *mut i32 = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut head: *mut i32 = (*lp).head;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    (1 as i32 <= j && j <= n - m
        || {
            glp_assert_(
                b"1 <= j && j <= n-m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                274 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *head.offset((m + j) as isize);
    i = 1 as i32;
    while i <= m {
        *tcol.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    ptr = *A_ptr.offset(k as isize);
    end = *A_ptr.offset((k + 1 as i32) as isize);
    while ptr < end {
        *tcol.offset(*A_ind.offset(ptr as isize) as isize) = -*A_val
            .offset(ptr as isize);
        ptr += 1;
        ptr;
    }
    _glp_bfd_ftran((*lp).bfd, tcol);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_eval_rho(
    mut lp: *mut SPXLP,
    mut i: i32,
    mut rho: *mut libc::c_double,
) {
    let mut m: i32 = (*lp).m;
    let mut j: i32 = 0;
    (1 as i32 <= i && i <= m
        || {
            glp_assert_(
                b"1 <= i && i <= m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                306 as i32,
            );
            1 as i32 != 0
        }) as i32;
    j = 1 as i32;
    while j <= m {
        *rho.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    *rho.offset(i as isize) = 1.0f64;
    _glp_bfd_btran((*lp).bfd, rho);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_eval_rho_s(
    mut lp: *mut SPXLP,
    mut i: i32,
    mut rho: *mut FVS,
) {
    let mut m: i32 = (*lp).m;
    (1 as i32 <= i && i <= m
        || {
            glp_assert_(
                b"1 <= i && i <= m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                319 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*rho).n == m
        || {
            glp_assert_(
                b"rho->n == m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                321 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_fvs_clear_vec(rho);
    (*rho).nnz = 1 as i32;
    *((*rho).ind).offset(1 as i32 as isize) = i;
    *((*rho).vec).offset(i as isize) = 1.0f64;
    _glp_bfd_btran_s((*lp).bfd, rho);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_eval_tij(
    mut lp: *mut SPXLP,
    mut rho: *const libc::c_double,
    mut j: i32,
) -> libc::c_double {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut A_ptr: *mut i32 = (*lp).A_ptr;
    let mut A_ind: *mut i32 = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut k: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut tij: libc::c_double = 0.;
    (1 as i32 <= j && j <= n - m
        || {
            glp_assert_(
                b"1 <= j && j <= n-m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                355 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *((*lp).head).offset((m + j) as isize);
    tij = 0.0f64;
    ptr = *A_ptr.offset(k as isize);
    end = *A_ptr.offset((k + 1 as i32) as isize);
    while ptr < end {
        tij
            -= *A_val.offset(ptr as isize)
                * *rho.offset(*A_ind.offset(ptr as isize) as isize);
        ptr += 1;
        ptr;
    }
    return tij;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_eval_trow(
    mut lp: *mut SPXLP,
    mut rho: *const libc::c_double,
    mut trow: *mut libc::c_double,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut j: i32 = 0;
    j = 1 as i32;
    while j <= n - m {
        *trow.offset(j as isize) = _glp_spx_eval_tij(lp, rho, j);
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_update_beta(
    mut lp: *mut SPXLP,
    mut beta: *mut libc::c_double,
    mut p: i32,
    mut p_flag: i32,
    mut q: i32,
    mut tcol: *const libc::c_double,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut i32 = (*lp).head;
    let mut flag: *mut i8 = (*lp).flag;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut delta_p: libc::c_double = 0.;
    let mut delta_q: libc::c_double = 0.;
    if p < 0 as i32 {
        (1 as i32 <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    468 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k = *head.offset((m + q) as isize);
        (*l.offset(k as isize) != -1.7976931348623157e+308f64
            && *u.offset(k as isize) != 1.7976931348623157e+308f64
            && *l.offset(k as isize) != *u.offset(k as isize)
            || {
                glp_assert_(
                    b"l[k] != -DBL_MAX && u[k] != +DBL_MAX && l[k] != u[k]\0"
                        as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    471 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *flag.offset(q as isize) != 0 {
            delta_q = *l.offset(k as isize) - *u.offset(k as isize);
        } else {
            delta_q = *u.offset(k as isize) - *l.offset(k as isize);
        }
    } else {
        (1 as i32 <= p && p <= m
            || {
                glp_assert_(
                    b"1 <= p && p <= m\0" as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    484 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (1 as i32 <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    485 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k = *head.offset(p as isize);
        if p_flag != 0 {
            (*l.offset(k as isize) != *u.offset(k as isize)
                && *u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != u[k] && u[k] != +DBL_MAX\0" as *const u8 as *const i8,
                        b"simplex/spxlp.c\0" as *const u8 as *const i8,
                        490 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            delta_p = *u.offset(k as isize) - *beta.offset(p as isize);
        } else if *l.offset(k as isize) == -1.7976931348623157e+308f64 {
            (*u.offset(k as isize) == 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] == +DBL_MAX\0" as *const u8 as *const i8,
                        b"simplex/spxlp.c\0" as *const u8 as *const i8,
                        495 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            delta_p = 0.0f64 - *beta.offset(p as isize);
        } else {
            delta_p = *l.offset(k as isize) - *beta.offset(p as isize);
        }
        delta_q = delta_p / *tcol.offset(p as isize);
        k = *head.offset((m + q) as isize);
        if *flag.offset(q as isize) != 0 {
            (*l.offset(k as isize) != *u.offset(k as isize)
                && *u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != u[k] && u[k] != +DBL_MAX\0" as *const u8 as *const i8,
                        b"simplex/spxlp.c\0" as *const u8 as *const i8,
                        509 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *beta.offset(p as isize) = *u.offset(k as isize) + delta_q;
        } else if *l.offset(k as isize) == -1.7976931348623157e+308f64 {
            (*u.offset(k as isize) == 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] == +DBL_MAX\0" as *const u8 as *const i8,
                        b"simplex/spxlp.c\0" as *const u8 as *const i8,
                        514 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *beta.offset(p as isize) = 0.0f64 + delta_q;
        } else {
            *beta.offset(p as isize) = *l.offset(k as isize) + delta_q;
        }
    }
    i = 1 as i32;
    while i <= m {
        if i != p {
            *beta.offset(i as isize) += *tcol.offset(i as isize) * delta_q;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_update_beta_s(
    mut lp: *mut SPXLP,
    mut beta: *mut libc::c_double,
    mut p: i32,
    mut p_flag: i32,
    mut q: i32,
    mut tcol: *const FVS,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut i32 = (*lp).head;
    let mut flag: *mut i8 = (*lp).flag;
    let mut nnz: i32 = (*tcol).nnz;
    let mut ind: *mut i32 = (*tcol).ind;
    let mut vec: *mut libc::c_double = (*tcol).vec;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut delta_p: libc::c_double = 0.;
    let mut delta_q: libc::c_double = 0.;
    ((*tcol).n == m
        || {
            glp_assert_(
                b"tcol->n == m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                546 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if p < 0 as i32 {
        (1 as i32 <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    553 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k = *head.offset((m + q) as isize);
        (*l.offset(k as isize) != -1.7976931348623157e+308f64
            && *u.offset(k as isize) != 1.7976931348623157e+308f64
            && *l.offset(k as isize) != *u.offset(k as isize)
            || {
                glp_assert_(
                    b"l[k] != -DBL_MAX && u[k] != +DBL_MAX && l[k] != u[k]\0"
                        as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    556 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *flag.offset(q as isize) != 0 {
            delta_q = *l.offset(k as isize) - *u.offset(k as isize);
        } else {
            delta_q = *u.offset(k as isize) - *l.offset(k as isize);
        }
    } else {
        (1 as i32 <= p && p <= m
            || {
                glp_assert_(
                    b"1 <= p && p <= m\0" as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    569 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (1 as i32 <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    570 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k = *head.offset(p as isize);
        if p_flag != 0 {
            (*l.offset(k as isize) != *u.offset(k as isize)
                && *u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != u[k] && u[k] != +DBL_MAX\0" as *const u8 as *const i8,
                        b"simplex/spxlp.c\0" as *const u8 as *const i8,
                        575 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            delta_p = *u.offset(k as isize) - *beta.offset(p as isize);
        } else if *l.offset(k as isize) == -1.7976931348623157e+308f64 {
            (*u.offset(k as isize) == 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] == +DBL_MAX\0" as *const u8 as *const i8,
                        b"simplex/spxlp.c\0" as *const u8 as *const i8,
                        580 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            delta_p = 0.0f64 - *beta.offset(p as isize);
        } else {
            delta_p = *l.offset(k as isize) - *beta.offset(p as isize);
        }
        delta_q = delta_p / *vec.offset(p as isize);
        k = *head.offset((m + q) as isize);
        if *flag.offset(q as isize) != 0 {
            (*l.offset(k as isize) != *u.offset(k as isize)
                && *u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != u[k] && u[k] != +DBL_MAX\0" as *const u8 as *const i8,
                        b"simplex/spxlp.c\0" as *const u8 as *const i8,
                        594 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *beta.offset(p as isize) = *u.offset(k as isize) + delta_q;
        } else if *l.offset(k as isize) == -1.7976931348623157e+308f64 {
            (*u.offset(k as isize) == 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"u[k] == +DBL_MAX\0" as *const u8 as *const i8,
                        b"simplex/spxlp.c\0" as *const u8 as *const i8,
                        599 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *beta.offset(p as isize) = 0.0f64 + delta_q;
        } else {
            *beta.offset(p as isize) = *l.offset(k as isize) + delta_q;
        }
    }
    k = 1 as i32;
    while k <= nnz {
        i = *ind.offset(k as isize);
        if i != p {
            *beta.offset(i as isize) += *vec.offset(i as isize) * delta_q;
        }
        k += 1;
        k;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_update_d(
    mut lp: *mut SPXLP,
    mut d: *mut libc::c_double,
    mut p: i32,
    mut q: i32,
    mut trow: *const libc::c_double,
    mut tcol: *const libc::c_double,
) -> libc::c_double {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut head: *mut i32 = (*lp).head;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut dq: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    (1 as i32 <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                678 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                679 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *head.offset((m + q) as isize);
    dq = *c.offset(k as isize);
    i = 1 as i32;
    while i <= m {
        dq += *tcol.offset(i as isize) * *c.offset(*head.offset(i as isize) as isize);
        i += 1;
        i;
    }
    e = fabs(dq - *d.offset(q as isize)) / (1.0f64 + fabs(dq));
    dq /= *tcol.offset(p as isize);
    *d.offset(q as isize) = dq;
    j = 1 as i32;
    while j <= n - m {
        if j != q {
            *d.offset(j as isize) -= *trow.offset(j as isize) * dq;
        }
        j += 1;
        j;
    }
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_update_d_s(
    mut lp: *mut SPXLP,
    mut d: *mut libc::c_double,
    mut p: i32,
    mut q: i32,
    mut trow: *const FVS,
    mut tcol: *const FVS,
) -> libc::c_double {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut head: *mut i32 = (*lp).head;
    let mut trow_nnz: i32 = (*trow).nnz;
    let mut trow_ind: *mut i32 = (*trow).ind;
    let mut trow_vec: *mut libc::c_double = (*trow).vec;
    let mut tcol_nnz: i32 = (*tcol).nnz;
    let mut tcol_ind: *mut i32 = (*tcol).ind;
    let mut tcol_vec: *mut libc::c_double = (*tcol).vec;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut dq: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    (1 as i32 <= p && p <= m
        || {
            glp_assert_(
                b"1 <= p && p <= m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                714 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= q && q <= n
        || {
            glp_assert_(
                b"1 <= q && q <= n\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                715 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*trow).n == n - m
        || {
            glp_assert_(
                b"trow->n == n-m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                716 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*tcol).n == m
        || {
            glp_assert_(
                b"tcol->n == m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                717 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *head.offset((m + q) as isize);
    dq = *c.offset(k as isize);
    k = 1 as i32;
    while k <= tcol_nnz {
        i = *tcol_ind.offset(k as isize);
        dq
            += *tcol_vec.offset(i as isize)
                * *c.offset(*head.offset(i as isize) as isize);
        k += 1;
        k;
    }
    e = fabs(dq - *d.offset(q as isize)) / (1.0f64 + fabs(dq));
    dq /= *tcol_vec.offset(p as isize);
    *d.offset(q as isize) = dq;
    k = 1 as i32;
    while k <= trow_nnz {
        j = *trow_ind.offset(k as isize);
        if j != q {
            *d.offset(j as isize) -= *trow_vec.offset(j as isize) * dq;
        }
        k += 1;
        k;
    }
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_change_basis(
    mut lp: *mut SPXLP,
    mut p: i32,
    mut p_flag: i32,
    mut q: i32,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut head: *mut i32 = (*lp).head;
    let mut flag: *mut i8 = (*lp).flag;
    let mut k: i32 = 0;
    if p < 0 as i32 {
        (1 as i32 <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    759 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k = *head.offset((m + q) as isize);
        (*l.offset(k as isize) != -1.7976931348623157e+308f64
            && *u.offset(k as isize) != 1.7976931348623157e+308f64
            && *l.offset(k as isize) != *u.offset(k as isize)
            || {
                glp_assert_(
                    b"l[k] != -DBL_MAX && u[k] != +DBL_MAX && l[k] != u[k]\0"
                        as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    762 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *flag.offset(q as isize) = (1 as i32 - *flag.offset(q as isize) as i32) as i8;
    } else {
        (1 as i32 <= p && p <= m
            || {
                glp_assert_(
                    b"1 <= p && p <= m\0" as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    768 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (p_flag == 0 as i32 || p_flag == 1 as i32
            || {
                glp_assert_(
                    b"p_flag == 0 || p_flag == 1\0" as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    769 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (1 as i32 <= q && q <= n - m
            || {
                glp_assert_(
                    b"1 <= q && q <= n-m\0" as *const u8 as *const i8,
                    b"simplex/spxlp.c\0" as *const u8 as *const i8,
                    770 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k = *head.offset(p as isize);
        if p_flag != 0 {
            (*l.offset(k as isize) != *u.offset(k as isize)
                && *u.offset(k as isize) != 1.7976931348623157e+308f64
                || {
                    glp_assert_(
                        b"l[k] != u[k] && u[k] != +DBL_MAX\0" as *const u8 as *const i8,
                        b"simplex/spxlp.c\0" as *const u8 as *const i8,
                        774 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        *head.offset(p as isize) = *head.offset((m + q) as isize);
        *head.offset((m + q) as isize) = k;
        *((*lp).flag).offset(q as isize) = p_flag as i8;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_update_invb(
    mut lp: *mut SPXLP,
    mut i: i32,
    mut k: i32,
) -> i32 {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut A_ptr: *mut i32 = (*lp).A_ptr;
    let mut A_ind: *mut i32 = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut ptr: i32 = 0;
    let mut len: i32 = 0;
    let mut ret: i32 = 0;
    (1 as i32 <= i && i <= m
        || {
            glp_assert_(
                b"1 <= i && i <= m\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                808 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= k && k <= n
        || {
            glp_assert_(
                b"1 <= k && k <= n\0" as *const u8 as *const i8,
                b"simplex/spxlp.c\0" as *const u8 as *const i8,
                809 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ptr = *A_ptr.offset(k as isize);
    len = *A_ptr.offset((k + 1 as i32) as isize) - ptr;
    ret = _glp_bfd_update(
        (*lp).bfd,
        i,
        len,
        &mut *A_ind.offset((ptr - 1 as i32) as isize) as *mut i32 as *const i32,
        &mut *A_val.offset((ptr - 1 as i32) as isize) as *mut libc::c_double
            as *const libc::c_double,
    );
    (*lp).valid = (ret == 0 as i32) as i32;
    return ret;
}