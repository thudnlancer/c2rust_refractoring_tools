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
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn _glp_mt1(
        n: i32,
        p: *mut i32,
        w: *mut i32,
        c: i32,
        x: *mut i32,
        jck: i32,
        xx: *mut i32,
        min: *mut i32,
        psign: *mut i32,
        wsign: *mut i32,
        zsign: *mut i32,
    ) -> i32;
}
pub type size_t = u64;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ks {
    pub orig_n: i32,
    pub n: i32,
    pub a: *mut i32,
    pub b: i32,
    pub c: *mut i32,
    pub c0: i32,
    pub x: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mt {
    pub j: i32,
    pub r: libc::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ks_enum(
    mut n: i32,
    mut a: *const i32,
    mut b: i32,
    mut c: *const i32,
    mut x: *mut i8,
) -> i32 {
    let mut j: i32 = 0;
    let mut s: i32 = 0;
    let mut z: i32 = 0;
    let mut z_best: i32 = 0;
    let mut x_best: [i8; 41] = [0; 41];
    (0 as i32 <= n && n <= 40 as i32
        || {
            glp_assert_(
                b"0 <= n && n <= N_MAX\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                62 as i32,
            );
            1 as i32 != 0
        }) as i32;
    memset(
        &mut *x.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
        0 as i32,
        (n as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    z_best = -(2147483647 as i32) - 1 as i32;
    '_loop: loop {
        z = 0 as i32;
        s = z;
        j = 1 as i32;
        while j <= n {
            if *x.offset(j as isize) != 0 {
                s += *a.offset(j as isize);
                z += *c.offset(j as isize);
            }
            j += 1;
            j;
        }
        if !(s > b) {
            if z_best < z {
                memcpy(
                    &mut *x_best.as_mut_ptr().offset(1 as i32 as isize) as *mut i8
                        as *mut libc::c_void,
                    &mut *x.offset(1 as i32 as isize) as *mut i8 as *const libc::c_void,
                    (n as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
                );
                z_best = z;
            }
        }
        j = 1 as i32;
        loop {
            if !(j <= n) {
                break '_loop;
            }
            if *x.offset(j as isize) == 0 {
                *x.offset(j as isize) = 1 as i32 as i8;
                break;
            } else {
                *x.offset(j as isize) = 0 as i32 as i8;
                j += 1;
                j;
            }
        }
    }
    memcpy(
        &mut *x.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
        &mut *x_best.as_mut_ptr().offset(1 as i32 as isize) as *mut i8
            as *const libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    return z_best;
}
unsafe extern "C" fn reduce(
    n: i32,
    mut a: *const i32,
    mut b: i32,
    mut c: *const i32,
) -> *mut ks {
    let mut ks: *mut ks = 0 as *mut ks;
    let mut j: i32 = 0;
    let mut s: i32 = 0;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                146 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ks = glp_alloc(1 as i32, ::core::mem::size_of::<ks>() as u64 as i32) as *mut ks;
    (*ks).orig_n = n;
    (*ks).n = 0 as i32;
    (*ks).a = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    memcpy(
        &mut *((*ks).a).offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        &*a.offset(1 as i32 as isize) as *const i32 as *const libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    (*ks).b = b;
    (*ks).c = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    memcpy(
        &mut *((*ks).c).offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        &*c.offset(1 as i32 as isize) as *const i32 as *const libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    (*ks).c0 = 0 as i32;
    (*ks).x = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i8>() as u64 as i32)
        as *mut i8;
    j = 1 as i32;
    while j <= n {
        if *a.offset(j as isize) >= 0 as i32 {
            *((*ks).x).offset(j as isize) = 0x10 as i32 as i8;
        } else {
            *((*ks).x).offset(j as isize) = 0x11 as i32 as i8;
            *((*ks).a).offset(j as isize) = -*((*ks).a).offset(j as isize);
            (*ks).b += *((*ks).a).offset(j as isize);
            (*ks).c0 += *((*ks).c).offset(j as isize);
            *((*ks).c).offset(j as isize) = -*((*ks).c).offset(j as isize);
        }
        j += 1;
        j;
    }
    if (*ks).b < 0 as i32 {
        free_ks(ks);
        return 0 as *mut ks;
    }
    j = 1 as i32;
    while j <= n {
        if *((*ks).a).offset(j as isize) == 0 as i32 {
            if *((*ks).c).offset(j as isize) <= 0 as i32 {
                let ref mut fresh0 = *((*ks).x).offset(j as isize);
                *fresh0 = (*fresh0 as i32 ^ 0x10 as i32) as i8;
            } else {
                let ref mut fresh1 = *((*ks).x).offset(j as isize);
                *fresh1 = (*fresh1 as i32 ^ 0x11 as i32) as i8;
                (*ks).c0 += *((*ks).c).offset(j as isize);
            }
        } else if *((*ks).a).offset(j as isize) > (*ks).b
            || *((*ks).c).offset(j as isize) <= 0 as i32
        {
            let ref mut fresh2 = *((*ks).x).offset(j as isize);
            *fresh2 = (*fresh2 as i32 ^ 0x10 as i32) as i8;
        } else {
            (*ks).n += 1;
            (*ks).n;
            *((*ks).a).offset((*ks).n as isize) = *((*ks).a).offset(j as isize);
            *((*ks).c).offset((*ks).n as isize) = *((*ks).c).offset(j as isize);
        }
        j += 1;
        j;
    }
    s = 0 as i32;
    j = 1 as i32;
    while j <= (*ks).n {
        (1 as i32 <= *((*ks).a).offset(j as isize)
            && *((*ks).a).offset(j as isize) <= (*ks).b
            || {
                glp_assert_(
                    b"1 <= ks->a[j] && ks->a[j] <= ks->b\0" as *const u8 as *const i8,
                    b"misc/ks.c\0" as *const u8 as *const i8,
                    213 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*((*ks).c).offset(j as isize) >= 1 as i32
            || {
                glp_assert_(
                    b"ks->c[j] >= 1\0" as *const u8 as *const i8,
                    b"misc/ks.c\0" as *const u8 as *const i8,
                    214 as i32,
                );
                1 as i32 != 0
            }) as i32;
        s += *((*ks).a).offset(j as isize);
        j += 1;
        j;
    }
    if s <= (*ks).b {
        j = 1 as i32;
        while j <= n {
            if *((*ks).x).offset(j as isize) as i32 & 0x10 as i32 != 0 {
                let ref mut fresh3 = *((*ks).x).offset(j as isize);
                *fresh3 = (*fresh3 as i32 ^ 0x11 as i32) as i8;
            }
            j += 1;
            j;
        }
        j = 1 as i32;
        while j <= (*ks).n {
            (*ks).c0 += *((*ks).c).offset(j as isize);
            j += 1;
            j;
        }
        (*ks).n = 0 as i32;
    }
    ((*ks).n == 0 as i32 || (*ks).n >= 2 as i32
        || {
            glp_assert_(
                b"ks->n == 0 || ks->n >= 2\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                230 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return ks;
}
unsafe extern "C" fn restore(mut ks: *mut ks, mut x: *mut i8) -> i32 {
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut z: i32 = 0;
    z = (*ks).c0;
    j = 1 as i32;
    k = 0 as i32;
    while j <= (*ks).orig_n {
        if *((*ks).x).offset(j as isize) as i32 & 0x10 as i32 != 0 {
            k += 1;
            k;
            (k <= (*ks).n
                || {
                    glp_assert_(
                        b"k <= ks->n\0" as *const u8 as *const i8,
                        b"misc/ks.c\0" as *const u8 as *const i8,
                        253 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*x.offset(k as isize) as i32 == 0 as i32
                || *x.offset(k as isize) as i32 == 1 as i32
                || {
                    glp_assert_(
                        b"x[k] == 0 || x[k] == 1\0" as *const u8 as *const i8,
                        b"misc/ks.c\0" as *const u8 as *const i8,
                        254 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if *((*ks).x).offset(j as isize) as i32 & 1 as i32 != 0 {
                *((*ks).x).offset(j as isize) = (1 as i32 - *x.offset(k as isize) as i32)
                    as i8;
            } else {
                *((*ks).x).offset(j as isize) = *x.offset(k as isize);
            }
            if *x.offset(k as isize) != 0 {
                z += *((*ks).c).offset(k as isize);
            }
        }
        j += 1;
        j;
    }
    (k == (*ks).n
        || {
            glp_assert_(
                b"k == ks->n\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                263 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return z;
}
unsafe extern "C" fn free_ks(mut ks: *mut ks) {
    (!ks.is_null()
        || {
            glp_assert_(
                b"ks != NULL\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                274 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_free((*ks).a as *mut libc::c_void);
    glp_free((*ks).c as *mut libc::c_void);
    glp_free((*ks).x as *mut libc::c_void);
    glp_free(ks as *mut libc::c_void);
}
unsafe extern "C" fn fcmp(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    if (*(p1 as *mut mt)).r > (*(p2 as *mut mt)).r {
        return -(1 as i32)
    } else if (*(p1 as *mut mt)).r < (*(p2 as *mut mt)).r {
        return 1 as i32
    } else {
        return 0 as i32
    };
}
unsafe extern "C" fn mt1a(
    mut n: i32,
    mut a: *const i32,
    mut b: i32,
    mut c: *const i32,
    mut x: *mut i8,
) -> i32 {
    let mut mt: *mut mt = 0 as *mut mt;
    let mut j: i32 = 0;
    let mut z: i32 = 0;
    let mut p: *mut i32 = 0 as *mut i32;
    let mut w: *mut i32 = 0 as *mut i32;
    let mut x1: *mut i32 = 0 as *mut i32;
    let mut xx: *mut i32 = 0 as *mut i32;
    let mut min: *mut i32 = 0 as *mut i32;
    let mut psign: *mut i32 = 0 as *mut i32;
    let mut wsign: *mut i32 = 0 as *mut i32;
    let mut zsign: *mut i32 = 0 as *mut i32;
    (n >= 2 as i32
        || {
            glp_assert_(
                b"n >= 2\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                318 as i32,
            );
            1 as i32 != 0
        }) as i32;
    mt = glp_alloc(1 as i32 + n, ::core::mem::size_of::<mt>() as u64 as i32) as *mut mt;
    p = glp_alloc(1 as i32 + n + 1 as i32, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    w = glp_alloc(1 as i32 + n + 1 as i32, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    x1 = glp_alloc(1 as i32 + n + 1 as i32, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    xx = glp_alloc(1 as i32 + n + 1 as i32, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    min = glp_alloc(1 as i32 + n + 1 as i32, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    psign = glp_alloc(
        1 as i32 + n + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    wsign = glp_alloc(
        1 as i32 + n + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    zsign = glp_alloc(
        1 as i32 + n + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    j = 1 as i32;
    while j <= n {
        (*mt.offset(j as isize)).j = j;
        (*mt.offset(j as isize)).r = *c.offset(j as isize) as libc::c_float
            / *a.offset(j as isize) as libc::c_float;
        j += 1;
        j;
    }
    qsort(
        &mut *mt.offset(1 as i32 as isize) as *mut mt as *mut libc::c_void,
        n as size_t,
        ::core::mem::size_of::<mt>() as u64,
        Some(
            fcmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    j = 1 as i32;
    while j <= n {
        *p.offset(j as isize) = *c.offset((*mt.offset(j as isize)).j as isize);
        *w.offset(j as isize) = *a.offset((*mt.offset(j as isize)).j as isize);
        j += 1;
        j;
    }
    z = _glp_mt1(n, p, w, b, x1, 1 as i32, xx, min, psign, wsign, zsign);
    (z >= 0 as i32
        || {
            glp_assert_(
                b"z >= 0\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                342 as i32,
            );
            1 as i32 != 0
        }) as i32;
    j = 1 as i32;
    while j <= n {
        (*x1.offset(j as isize) == 0 as i32 || *x1.offset(j as isize) == 1 as i32
            || {
                glp_assert_(
                    b"x1[j] == 0 || x1[j] == 1\0" as *const u8 as *const i8,
                    b"misc/ks.c\0" as *const u8 as *const i8,
                    345 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *x.offset((*mt.offset(j as isize)).j as isize) = *x1.offset(j as isize) as i8;
        j += 1;
        j;
    }
    glp_free(mt as *mut libc::c_void);
    glp_free(p as *mut libc::c_void);
    glp_free(w as *mut libc::c_void);
    glp_free(x1 as *mut libc::c_void);
    glp_free(xx as *mut libc::c_void);
    glp_free(min as *mut libc::c_void);
    glp_free(psign as *mut libc::c_void);
    glp_free(wsign as *mut libc::c_void);
    glp_free(zsign as *mut libc::c_void);
    return z;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ks_mt1(
    mut n: i32,
    mut a: *const i32,
    mut b: i32,
    mut c: *const i32,
    mut x: *mut i8,
) -> i32 {
    let mut ks: *mut ks = 0 as *mut ks;
    let mut j: i32 = 0;
    let mut s1: i32 = 0;
    let mut s2: i32 = 0;
    let mut z: i32 = 0;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                365 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ks = reduce(n, a, b, c);
    if ks.is_null() {
        return -(2147483647 as i32) - 1 as i32;
    }
    if (*ks).n > 0 as i32 {
        mt1a((*ks).n, (*ks).a as *const i32, (*ks).b, (*ks).c as *const i32, x);
    }
    z = restore(ks, x);
    memcpy(
        &mut *x.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
        &mut *((*ks).x).offset(1 as i32 as isize) as *mut i8 as *const libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    free_ks(ks);
    s2 = 0 as i32;
    s1 = s2;
    j = 1 as i32;
    while j <= n {
        (*x.offset(j as isize) as i32 == 0 as i32
            || *x.offset(j as isize) as i32 == 1 as i32
            || {
                glp_assert_(
                    b"x[j] == 0 || x[j] == 1\0" as *const u8 as *const i8,
                    b"misc/ks.c\0" as *const u8 as *const i8,
                    382 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *x.offset(j as isize) != 0 {
            s1 += *a.offset(j as isize);
            s2 += *c.offset(j as isize);
        }
        j += 1;
        j;
    }
    (s1 <= b
        || {
            glp_assert_(
                b"s1 <= b\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                386 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (s2 == z
        || {
            glp_assert_(
                b"s2 == z\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                387 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return z;
}
unsafe extern "C" fn greedy(
    mut n: i32,
    mut a: *const i32,
    mut b: i32,
    mut c: *const i32,
    mut x: *mut i8,
) -> i32 {
    let mut mt: *mut mt = 0 as *mut mt;
    let mut j: i32 = 0;
    let mut s: i32 = 0;
    let mut z: i32 = 0;
    (n >= 2 as i32
        || {
            glp_assert_(
                b"n >= 2\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                409 as i32,
            );
            1 as i32 != 0
        }) as i32;
    mt = glp_alloc(1 as i32 + n, ::core::mem::size_of::<mt>() as u64 as i32) as *mut mt;
    j = 1 as i32;
    while j <= n {
        (*mt.offset(j as isize)).j = j;
        (*mt.offset(j as isize)).r = *c.offset(j as isize) as libc::c_float
            / *a.offset(j as isize) as libc::c_float;
        j += 1;
        j;
    }
    qsort(
        &mut *mt.offset(1 as i32 as isize) as *mut mt as *mut libc::c_void,
        n as size_t,
        ::core::mem::size_of::<mt>() as u64,
        Some(
            fcmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    z = 0 as i32;
    s = z;
    j = 1 as i32;
    while j <= n {
        if s + *a.offset((*mt.offset(j as isize)).j as isize) > b {
            break;
        }
        *x.offset((*mt.offset(j as isize)).j as isize) = 1 as i32 as i8;
        s += *a.offset((*mt.offset(j as isize)).j as isize);
        z += *c.offset((*mt.offset(j as isize)).j as isize);
        j += 1;
        j;
    }
    j = j;
    while j <= n {
        *x.offset((*mt.offset(j as isize)).j as isize) = 0 as i32 as i8;
        j += 1;
        j;
    }
    glp_free(mt as *mut libc::c_void);
    return z;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ks_greedy(
    mut n: i32,
    mut a: *const i32,
    mut b: i32,
    mut c: *const i32,
    mut x: *mut i8,
) -> i32 {
    let mut ks: *mut ks = 0 as *mut ks;
    let mut j: i32 = 0;
    let mut s1: i32 = 0;
    let mut s2: i32 = 0;
    let mut z: i32 = 0;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                438 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ks = reduce(n, a, b, c);
    if ks.is_null() {
        return -(2147483647 as i32) - 1 as i32;
    }
    if (*ks).n > 0 as i32 {
        greedy((*ks).n, (*ks).a as *const i32, (*ks).b, (*ks).c as *const i32, x);
    }
    z = restore(ks, x);
    memcpy(
        &mut *x.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
        &mut *((*ks).x).offset(1 as i32 as isize) as *mut i8 as *const libc::c_void,
        (n as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    free_ks(ks);
    s2 = 0 as i32;
    s1 = s2;
    j = 1 as i32;
    while j <= n {
        (*x.offset(j as isize) as i32 == 0 as i32
            || *x.offset(j as isize) as i32 == 1 as i32
            || {
                glp_assert_(
                    b"x[j] == 0 || x[j] == 1\0" as *const u8 as *const i8,
                    b"misc/ks.c\0" as *const u8 as *const i8,
                    455 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *x.offset(j as isize) != 0 {
            s1 += *a.offset(j as isize);
            s2 += *c.offset(j as isize);
        }
        j += 1;
        j;
    }
    (s1 <= b
        || {
            glp_assert_(
                b"s1 <= b\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                459 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (s2 == z
        || {
            glp_assert_(
                b"s2 == z\0" as *const u8 as *const i8,
                b"misc/ks.c\0" as *const u8 as *const i8,
                460 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return z;
}