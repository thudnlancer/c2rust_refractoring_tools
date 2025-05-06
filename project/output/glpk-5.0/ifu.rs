#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IFU {
    pub n_max: i32,
    pub n: i32,
    pub f: *mut libc::c_double,
    pub u: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ifu_expand(
    mut ifu: *mut IFU,
    mut c: *mut libc::c_double,
    mut r: *mut libc::c_double,
    mut d: libc::c_double,
) {
    let mut n_max: i32 = (*ifu).n_max;
    let mut n: i32 = (*ifu).n;
    let mut f_: *mut libc::c_double = (*ifu).f;
    let mut u_: *mut libc::c_double = (*ifu).u;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut t: libc::c_double = 0.;
    (0 as i32 <= n && n < n_max
        || {
            glp_assert_(
                b"0 <= n && n < n_max\0" as *const u8 as *const i8,
                b"bflib/ifu.c\0" as *const u8 as *const i8,
                64 as i32,
            );
            1 as i32 != 0
        }) as i32;
    c = c.offset(1);
    c;
    r = r.offset(1);
    r;
    i = 0 as i32;
    while i < n {
        *f_.offset((i * n_max + n) as isize) = 0.0f64;
        i += 1;
        i;
    }
    j = 0 as i32;
    while j < n {
        *f_.offset((n * n_max + j) as isize) = 0.0f64;
        j += 1;
        j;
    }
    *f_.offset((n * n_max + n) as isize) = 1.0f64;
    i = 0 as i32;
    while i < n {
        t = 0.0f64;
        j = 0 as i32;
        while j < n {
            t += *f_.offset((i * n_max + j) as isize) * *c.offset(j as isize);
            j += 1;
            j;
        }
        *u_.offset((i * n_max + n) as isize) = t;
        i += 1;
        i;
    }
    j = 0 as i32;
    while j < n {
        *u_.offset((n * n_max + j) as isize) = *r.offset(j as isize);
        j += 1;
        j;
    }
    *u_.offset((n * n_max + n) as isize) = d;
    (*ifu).n += 1;
    (*ifu).n;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ifu_bg_update(
    mut ifu: *mut IFU,
    mut c: *mut libc::c_double,
    mut r: *mut libc::c_double,
    mut d: libc::c_double,
) -> i32 {
    let mut n_max: i32 = (*ifu).n_max;
    let mut n: i32 = (*ifu).n;
    let mut f_: *mut libc::c_double = (*ifu).f;
    let mut u_: *mut libc::c_double = (*ifu).u;
    let mut tol: libc::c_double = 1e-5f64;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut t: libc::c_double = 0.;
    _glp_ifu_expand(ifu, c, r, d);
    k = 0 as i32;
    while k < n {
        if fabs(*u_.offset((k * n_max + k) as isize))
            < fabs(*u_.offset((n * n_max + k) as isize))
        {
            j = k;
            while j <= n {
                t = *u_.offset((k * n_max + j) as isize);
                *u_.offset((k * n_max + j) as isize) = *u_
                    .offset((n * n_max + j) as isize);
                *u_.offset((n * n_max + j) as isize) = t;
                j += 1;
                j;
            }
            j = 0 as i32;
            while j <= n {
                t = *f_.offset((k * n_max + j) as isize);
                *f_.offset((k * n_max + j) as isize) = *f_
                    .offset((n * n_max + j) as isize);
                *f_.offset((n * n_max + j) as isize) = t;
                j += 1;
                j;
            }
        }
        if fabs(*u_.offset((k * n_max + k) as isize)) < tol {
            return 1 as i32;
        }
        if !(*u_.offset((n * n_max + k) as isize) == 0.0f64) {
            t = *u_.offset((n * n_max + k) as isize)
                / *u_.offset((k * n_max + k) as isize);
            j = k + 1 as i32;
            while j <= n {
                *u_.offset((n * n_max + j) as isize)
                    -= t * *u_.offset((k * n_max + j) as isize);
                j += 1;
                j;
            }
            j = 0 as i32;
            while j <= n {
                *f_.offset((n * n_max + j) as isize)
                    -= t * *f_.offset((k * n_max + j) as isize);
                j += 1;
                j;
            }
        }
        k += 1;
        k;
    }
    if fabs(*u_.offset((n * n_max + n) as isize)) < tol {
        return 2 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn givens(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut c: *mut libc::c_double,
    mut s: *mut libc::c_double,
) {
    let mut t: libc::c_double = 0.;
    if b == 0.0f64 {
        *c = 1.0f64;
        *s = 0.0f64;
    } else if fabs(a) <= fabs(b) {
        t = -a / b;
        *s = 1.0f64 / sqrt(1.0f64 + t * t);
        *c = *s * t;
    } else {
        t = -b / a;
        *c = 1.0f64 / sqrt(1.0f64 + t * t);
        *s = *c * t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ifu_gr_update(
    mut ifu: *mut IFU,
    mut c: *mut libc::c_double,
    mut r: *mut libc::c_double,
    mut d: libc::c_double,
) -> i32 {
    let mut n_max: i32 = (*ifu).n_max;
    let mut n: i32 = (*ifu).n;
    let mut f_: *mut libc::c_double = (*ifu).f;
    let mut u_: *mut libc::c_double = (*ifu).u;
    let mut tol: libc::c_double = 1e-5f64;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut cs: libc::c_double = 0.;
    let mut sn: libc::c_double = 0.;
    _glp_ifu_expand(ifu, c, r, d);
    k = 0 as i32;
    while k < n {
        if fabs(*u_.offset((k * n_max + k) as isize)) < tol
            && fabs(*u_.offset((n * n_max + k) as isize)) < tol
        {
            return 1 as i32;
        }
        if !(*u_.offset((n * n_max + k) as isize) == 0.0f64) {
            givens(
                *u_.offset((k * n_max + k) as isize),
                *u_.offset((n * n_max + k) as isize),
                &mut cs,
                &mut sn,
            );
            j = k;
            while j <= n {
                let mut ukj: libc::c_double = *u_.offset((k * n_max + j) as isize);
                let mut unj: libc::c_double = *u_.offset((n * n_max + j) as isize);
                *u_.offset((k * n_max + j) as isize) = cs * ukj - sn * unj;
                *u_.offset((n * n_max + j) as isize) = sn * ukj + cs * unj;
                j += 1;
                j;
            }
            j = 0 as i32;
            while j <= n {
                let mut fkj: libc::c_double = *f_.offset((k * n_max + j) as isize);
                let mut fnj: libc::c_double = *f_.offset((n * n_max + j) as isize);
                *f_.offset((k * n_max + j) as isize) = cs * fkj - sn * fnj;
                *f_.offset((n * n_max + j) as isize) = sn * fkj + cs * fnj;
                j += 1;
                j;
            }
        }
        k += 1;
        k;
    }
    if fabs(*u_.offset((n * n_max + n) as isize)) < tol {
        return 2 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ifu_a_solve(
    mut ifu: *mut IFU,
    mut x: *mut libc::c_double,
    mut w: *mut libc::c_double,
) {
    let mut n_max: i32 = (*ifu).n_max;
    let mut n: i32 = (*ifu).n;
    let mut f_: *mut libc::c_double = (*ifu).f;
    let mut u_: *mut libc::c_double = (*ifu).u;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut t: libc::c_double = 0.;
    (0 as i32 <= n && n <= n_max
        || {
            glp_assert_(
                b"0 <= n && n <= n_max\0" as *const u8 as *const i8,
                b"bflib/ifu.c\0" as *const u8 as *const i8,
                310 as i32,
            );
            1 as i32 != 0
        }) as i32;
    x = x.offset(1);
    x;
    w = w.offset(1);
    w;
    memcpy(
        w as *mut libc::c_void,
        x as *const libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
    i = 0 as i32;
    while i < n {
        t = 0.0f64;
        j = 0 as i32;
        while j < n {
            t += *f_.offset((i * n_max + j) as isize) * *w.offset(j as isize);
            j += 1;
            j;
        }
        *x.offset(i as isize) = t;
        i += 1;
        i;
    }
    i = n - 1 as i32;
    while i >= 0 as i32 {
        t = *x.offset(i as isize);
        j = i + 1 as i32;
        while j < n {
            t -= *u_.offset((i * n_max + j) as isize) * *x.offset(j as isize);
            j += 1;
            j;
        }
        *x.offset(i as isize) = t / *u_.offset((i * n_max + i) as isize);
        i -= 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ifu_at_solve(
    mut ifu: *mut IFU,
    mut x: *mut libc::c_double,
    mut w: *mut libc::c_double,
) {
    let mut n_max: i32 = (*ifu).n_max;
    let mut n: i32 = (*ifu).n;
    let mut f_: *mut libc::c_double = (*ifu).f;
    let mut u_: *mut libc::c_double = (*ifu).u;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut t: libc::c_double = 0.;
    (0 as i32 <= n && n <= n_max
        || {
            glp_assert_(
                b"0 <= n && n <= n_max\0" as *const u8 as *const i8,
                b"bflib/ifu.c\0" as *const u8 as *const i8,
                367 as i32,
            );
            1 as i32 != 0
        }) as i32;
    x = x.offset(1);
    x;
    w = w.offset(1);
    w;
    i = 0 as i32;
    while i < n {
        let ref mut fresh0 = *x.offset(i as isize);
        *fresh0 /= *u_.offset((i * n_max + i) as isize);
        t = *fresh0;
        j = i + 1 as i32;
        while j < n {
            *x.offset(j as isize) -= *u_.offset((i * n_max + j) as isize) * t;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    j = 0 as i32;
    while j < n {
        t = 0.0f64;
        i = 0 as i32;
        while i < n {
            t += *f_.offset((i * n_max + j) as isize) * *x.offset(i as isize);
            i += 1;
            i;
        }
        *w.offset(j as isize) = t;
        j += 1;
        j;
    }
    memcpy(
        x as *mut libc::c_void,
        w as *const libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    );
}