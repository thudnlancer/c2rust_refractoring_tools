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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_triang(
    mut m: i32,
    mut n: i32,
    mut mat: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            i32,
            *mut i32,
            *mut libc::c_double,
        ) -> i32,
    >,
    mut info: *mut libc::c_void,
    mut tol: libc::c_double,
    mut rn: *mut i32,
    mut cn: *mut i32,
) -> i32 {
    let mut head: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut jj: i32 = 0;
    let mut k: i32 = 0;
    let mut kk: i32 = 0;
    let mut ks: i32 = 0;
    let mut len: i32 = 0;
    let mut len2: i32 = 0;
    let mut next_j: i32 = 0;
    let mut ns: i32 = 0;
    let mut size: i32 = 0;
    let mut cind: *mut i32 = 0 as *mut i32;
    let mut rind: *mut i32 = 0 as *mut i32;
    let mut cnt: *mut i32 = 0 as *mut i32;
    let mut ptr: *mut i32 = 0 as *mut i32;
    let mut list: *mut i32 = 0 as *mut i32;
    let mut prev: *mut i32 = 0 as *mut i32;
    let mut next: *mut i32 = 0 as *mut i32;
    let mut cval: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rval: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut big: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut flag: *mut i8 = 0 as *mut i8;
    cind = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    cval = glp_alloc(
        1 as i32 + m,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    rind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    rval = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    ptr = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    cnt = ptr;
    list = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    prev = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    next = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    big = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    flag = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i8>() as u64 as i32)
        as *mut i8;
    len = 0 as i32;
    while len <= m {
        *ptr.offset(len as isize) = 0 as i32;
        len += 1;
        len;
    }
    j = 1 as i32;
    while j <= n {
        len = mat.expect("non-null function pointer")(info, -j, cind, cval);
        (0 as i32 <= len && len <= m
            || {
                glp_assert_(
                    b"0 <= len && len <= m\0" as *const u8 as *const i8,
                    b"misc/triang.c\0" as *const u8 as *const i8,
                    134 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *next.offset(j as isize) = *ptr.offset(len as isize);
        *ptr.offset(len as isize) = j;
        *big.offset(j as isize) = 0.0f64;
        k = 1 as i32;
        while k <= len {
            if *big.offset(j as isize) < fabs(*cval.offset(k as isize)) {
                *big.offset(j as isize) = fabs(*cval.offset(k as isize));
            }
            k += 1;
            k;
        }
        j += 1;
        j;
    }
    head = 0 as i32;
    len = 0 as i32;
    while len <= m {
        j = *ptr.offset(len as isize);
        while j != 0 as i32 {
            next_j = *next.offset(j as isize);
            *prev.offset(j as isize) = 0 as i32;
            *next.offset(j as isize) = head;
            if head != 0 as i32 {
                *prev.offset(head as isize) = j;
            }
            head = j;
            j = next_j;
        }
        len += 1;
        len;
    }
    j = 1 as i32;
    while j <= n {
        *flag.offset(j as isize) = 1 as i32 as i8;
        j += 1;
        j;
    }
    ns = 0 as i32;
    i = 1 as i32;
    while i <= m {
        let ref mut fresh0 = *cnt.offset(i as isize);
        *fresh0 = mat.expect("non-null function pointer")(info, i, rind, rval);
        len = *fresh0;
        (0 as i32 <= len && len <= n
            || {
                glp_assert_(
                    b"0 <= len && len <= n\0" as *const u8 as *const i8,
                    b"misc/triang.c\0" as *const u8 as *const i8,
                    192 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if len == 1 as i32 {
            j = *rind.offset(1 as i32 as isize);
            (1 as i32 <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const i8,
                        b"misc/triang.c\0" as *const u8 as *const i8,
                        196 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if *flag.offset(j as isize) as i32 != 2 as i32 {
                *flag.offset(j as isize) = 2 as i32 as i8;
                ns += 1;
                *list.offset(ns as isize) = j;
            }
        }
        i += 1;
        i;
    }
    size = 0 as i32;
    while head != 0 as i32 {
        if ns == 0 as i32 {
            j = head;
            len = mat.expect("non-null function pointer")(info, -j, cind, cval);
            (0 as i32 <= len && len <= m
                || {
                    glp_assert_(
                        b"0 <= len && len <= m\0" as *const u8 as *const i8,
                        b"misc/triang.c\0" as *const u8 as *const i8,
                        217 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        } else {
            let fresh1 = ns;
            ns = ns - 1;
            j = *list.offset(fresh1 as isize);
            (*flag.offset(j as isize) as i32 == 2 as i32
                || {
                    glp_assert_(
                        b"flag[j] == 2\0" as *const u8 as *const i8,
                        b"misc/triang.c\0" as *const u8 as *const i8,
                        222 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            len = mat.expect("non-null function pointer")(info, -j, cind, cval);
            (0 as i32 <= len && len <= m
                || {
                    glp_assert_(
                        b"0 <= len && len <= m\0" as *const u8 as *const i8,
                        b"misc/triang.c\0" as *const u8 as *const i8,
                        226 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            kk = 0 as i32;
            k = 1 as i32;
            while k <= len {
                i = *cind.offset(k as isize);
                (1 as i32 <= i && i <= m
                    || {
                        glp_assert_(
                            b"1 <= i && i <= m\0" as *const u8 as *const i8,
                            b"misc/triang.c\0" as *const u8 as *const i8,
                            230 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                if *cnt.offset(i as isize) == 1 as i32 {
                    if kk == 0 as i32
                        || fabs(*cval.offset(kk as isize))
                            < fabs(*cval.offset(k as isize))
                    {
                        kk = k;
                    }
                }
                k += 1;
                k;
            }
            (kk > 0 as i32
                || {
                    glp_assert_(
                        b"kk > 0\0" as *const u8 as *const i8,
                        b"misc/triang.c\0" as *const u8 as *const i8,
                        237 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if !(fabs(*cval.offset(kk as isize)) < tol * *big.offset(j as isize)) {
                size += 1;
                size;
                *rn.offset(size as isize) = *cind.offset(kk as isize);
                *cn.offset(size as isize) = j;
            }
        }
        (*flag.offset(j as isize) as i32 != 0
            || {
                glp_assert_(
                    b"flag[j]\0" as *const u8 as *const i8,
                    b"misc/triang.c\0" as *const u8 as *const i8,
                    250 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *flag.offset(j as isize) = 0 as i32 as i8;
        if *prev.offset(j as isize) == 0 as i32 {
            head = *next.offset(j as isize);
        } else {
            *next.offset(*prev.offset(j as isize) as isize) = *next.offset(j as isize);
        }
        if !(*next.offset(j as isize) == 0 as i32) {
            *prev.offset(*next.offset(j as isize) as isize) = *prev.offset(j as isize);
        }
        k = 1 as i32;
        while k <= len {
            i = *cind.offset(k as isize);
            (1 as i32 <= i && i <= m
                || {
                    glp_assert_(
                        b"1 <= i && i <= m\0" as *const u8 as *const i8,
                        b"misc/triang.c\0" as *const u8 as *const i8,
                        263 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*cnt.offset(i as isize) > 0 as i32
                || {
                    glp_assert_(
                        b"cnt[i] > 0\0" as *const u8 as *const i8,
                        b"misc/triang.c\0" as *const u8 as *const i8,
                        264 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            let ref mut fresh2 = *cnt.offset(i as isize);
            *fresh2 -= 1;
            *fresh2;
            if *cnt.offset(i as isize) == 1 as i32 {
                len2 = mat.expect("non-null function pointer")(info, i, rind, rval);
                (0 as i32 <= len2 && len2 <= n
                    || {
                        glp_assert_(
                            b"0 <= len2 && len2 <= n\0" as *const u8 as *const i8,
                            b"misc/triang.c\0" as *const u8 as *const i8,
                            271 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                ks = 0 as i32;
                kk = 1 as i32;
                while kk <= len2 {
                    jj = *rind.offset(kk as isize);
                    (1 as i32 <= jj && jj <= n
                        || {
                            glp_assert_(
                                b"1 <= jj && jj <= n\0" as *const u8 as *const i8,
                                b"misc/triang.c\0" as *const u8 as *const i8,
                                275 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if *flag.offset(jj as isize) != 0 {
                        (ks == 0 as i32
                            || {
                                glp_assert_(
                                    b"ks == 0\0" as *const u8 as *const i8,
                                    b"misc/triang.c\0" as *const u8 as *const i8,
                                    277 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        ks = kk;
                    }
                    kk += 1;
                    kk;
                }
                (ks > 0 as i32
                    || {
                        glp_assert_(
                            b"ks > 0\0" as *const u8 as *const i8,
                            b"misc/triang.c\0" as *const u8 as *const i8,
                            281 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                jj = *rind.offset(ks as isize);
                if *flag.offset(jj as isize) as i32 != 2 as i32 {
                    *flag.offset(jj as isize) = 2 as i32 as i8;
                    ns += 1;
                    *list.offset(ns as isize) = jj;
                }
            }
            k += 1;
            k;
        }
    }
    i = 1 as i32;
    while i <= m {
        (*cnt.offset(i as isize) == 0 as i32
            || {
                glp_assert_(
                    b"cnt[i] == 0\0" as *const u8 as *const i8,
                    b"misc/triang.c\0" as *const u8 as *const i8,
                    294 as i32,
                );
                1 as i32 != 0
            }) as i32;
        i += 1;
        i;
    }
    glp_free(cind as *mut libc::c_void);
    glp_free(cval as *mut libc::c_void);
    glp_free(rind as *mut libc::c_void);
    glp_free(rval as *mut libc::c_void);
    glp_free(ptr as *mut libc::c_void);
    glp_free(list as *mut libc::c_void);
    glp_free(prev as *mut libc::c_void);
    glp_free(next as *mut libc::c_void);
    glp_free(big as *mut libc::c_void);
    glp_free(flag as *mut libc::c_void);
    return size;
}