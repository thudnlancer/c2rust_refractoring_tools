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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
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
pub struct vertex {
    pub i: i32,
    pub cw: libc::c_double,
}
unsafe extern "C" fn fcmp(
    mut xx: *const libc::c_void,
    mut yy: *const libc::c_void,
) -> i32 {
    let mut x: *const vertex = xx as *const vertex;
    let mut y: *const vertex = yy as *const vertex;
    if (*x).cw > (*y).cw {
        return -(1 as i32);
    }
    if (*x).cw < (*y).cw {
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_wclique1(
    mut n: i32,
    mut w: *const libc::c_double,
    mut func: Option<unsafe extern "C" fn(*mut libc::c_void, i32, *mut i32) -> i32>,
    mut info: *mut libc::c_void,
    mut c: *mut i32,
) -> i32 {
    let mut current_block: u64;
    let mut v_list: *mut vertex = 0 as *mut vertex;
    let mut deg: i32 = 0;
    let mut c_size: i32 = 0;
    let mut d_size: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut kk: i32 = 0;
    let mut l: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut c_list: *mut i32 = 0 as *mut i32;
    let mut d_list: *mut i32 = 0 as *mut i32;
    let mut size: i32 = 0 as i32;
    let mut c_wght: libc::c_double = 0.;
    let mut d_wght: libc::c_double = 0.;
    let mut sw: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut best: libc::c_double = 0.0f64;
    let mut d_flag: *mut i8 = 0 as *mut i8;
    let mut skip: *mut i8 = 0 as *mut i8;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"misc/wclique1.c\0" as *const u8 as *const i8,
                80 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = 1 as i32;
    while i <= n {
        (*w.offset(i as isize) >= 0.0f64
            || {
                glp_assert_(
                    b"w[i] >= 0.0\0" as *const u8 as *const i8,
                    b"misc/wclique1.c\0" as *const u8 as *const i8,
                    82 as i32,
                );
                1 as i32 != 0
            }) as i32;
        i += 1;
        i;
    }
    if !(n == 0 as i32) {
        ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        v_list = glp_alloc(1 as i32 + n, ::core::mem::size_of::<vertex>() as u64 as i32)
            as *mut vertex;
        c_list = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        d_list = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        d_flag = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i8>() as u64 as i32)
            as *mut i8;
        skip = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i8>() as u64 as i32)
            as *mut i8;
        sw = glp_alloc(
            1 as i32 + n,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        i = 1 as i32;
        while i <= n {
            (*v_list.offset(i as isize)).i = i;
            (*v_list.offset(i as isize)).cw = *w.offset(i as isize);
            deg = func.expect("non-null function pointer")(info, i, ind);
            (0 as i32 <= deg && deg < n
                || {
                    glp_assert_(
                        b"0 <= deg && deg < n\0" as *const u8 as *const i8,
                        b"misc/wclique1.c\0" as *const u8 as *const i8,
                        100 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            k = 1 as i32;
            while k <= deg {
                j = *ind.offset(k as isize);
                (1 as i32 <= j && j <= n && j != i
                    || {
                        glp_assert_(
                            b"1 <= j && j <= n && j != i\0" as *const u8 as *const i8,
                            b"misc/wclique1.c\0" as *const u8 as *const i8,
                            103 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*v_list.offset(i as isize)).cw += *w.offset(j as isize);
                k += 1;
                k;
            }
            i += 1;
            i;
        }
        qsort(
            &mut *v_list.offset(1 as i32 as isize) as *mut vertex as *mut libc::c_void,
            n as size_t,
            ::core::mem::size_of::<vertex>() as u64,
            Some(
                fcmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
        );
        memset(
            &mut *skip.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
            0 as i32,
            (::core::mem::size_of::<i8>() as u64).wrapping_mul(n as u64),
        );
        memset(
            &mut *d_flag.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
            0 as i32,
            (::core::mem::size_of::<i8>() as u64).wrapping_mul(n as u64),
        );
        l = 1 as i32;
        while l <= n {
            i = (*v_list.offset(l as isize)).i;
            if !(*skip.offset(i as isize) != 0) {
                c_size = 1 as i32;
                *c_list.offset(1 as i32 as isize) = i;
                c_wght = *w.offset(i as isize);
                d_size = func.expect("non-null function pointer")(info, i, d_list);
                (0 as i32 <= d_size && d_size < n
                    || {
                        glp_assert_(
                            b"0 <= d_size && d_size < n\0" as *const u8 as *const i8,
                            b"misc/wclique1.c\0" as *const u8 as *const i8,
                            127 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                d_wght = 0.0f64;
                k = 1 as i32;
                while k <= d_size {
                    j = *d_list.offset(k as isize);
                    (1 as i32 <= j && j <= n && j != i
                        || {
                            glp_assert_(
                                b"1 <= j && j <= n && j != i\0" as *const u8 as *const i8,
                                b"misc/wclique1.c\0" as *const u8 as *const i8,
                                131 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    (*d_flag.offset(j as isize) == 0
                        || {
                            glp_assert_(
                                b"!d_flag[j]\0" as *const u8 as *const i8,
                                b"misc/wclique1.c\0" as *const u8 as *const i8,
                                132 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    *d_flag.offset(j as isize) = 1 as i32 as i8;
                    d_wght += *w.offset(j as isize);
                    k += 1;
                    k;
                }
                if !(c_wght + d_wght < best + 1e-5f64 * (1.0f64 + fabs(best))) {
                    k = 1 as i32;
                    while k <= d_size {
                        i = *d_list.offset(k as isize);
                        *sw.offset(i as isize) = *w.offset(i as isize);
                        deg = func.expect("non-null function pointer")(info, i, ind);
                        (0 as i32 <= deg && deg < n
                            || {
                                glp_assert_(
                                    b"0 <= deg && deg < n\0" as *const u8 as *const i8,
                                    b"misc/wclique1.c\0" as *const u8 as *const i8,
                                    148 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        kk = 1 as i32;
                        while kk <= deg {
                            j = *ind.offset(kk as isize);
                            (1 as i32 <= j && j <= n && j != i
                                || {
                                    glp_assert_(
                                        b"1 <= j && j <= n && j != i\0" as *const u8 as *const i8,
                                        b"misc/wclique1.c\0" as *const u8 as *const i8,
                                        151 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            if *d_flag.offset(j as isize) != 0 {
                                *sw.offset(i as isize) += *w.offset(j as isize);
                            }
                            kk += 1;
                            kk;
                        }
                        k += 1;
                        k;
                    }
                    loop {
                        if !(d_size > 0 as i32) {
                            current_block = 13723035087248630346;
                            break;
                        }
                        if c_wght + d_wght < best + 1e-5f64 * (1.0f64 + fabs(best)) {
                            current_block = 17973717047374398875;
                            break;
                        }
                        i = *d_list.offset(1 as i32 as isize);
                        k = 2 as i32;
                        while k <= d_size {
                            j = *d_list.offset(k as isize);
                            if *sw.offset(i as isize) < *sw.offset(j as isize) {
                                i = j;
                            }
                            k += 1;
                            k;
                        }
                        c_size += 1;
                        c_size;
                        *c_list.offset(c_size as isize) = i;
                        c_wght += *w.offset(i as isize);
                        deg = func.expect("non-null function pointer")(info, i, ind);
                        (0 as i32 <= deg && deg < n
                            || {
                                glp_assert_(
                                    b"0 <= deg && deg < n\0" as *const u8 as *const i8,
                                    b"misc/wclique1.c\0" as *const u8 as *const i8,
                                    175 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        k = 1 as i32;
                        while k <= deg {
                            j = *ind.offset(k as isize);
                            (1 as i32 <= j && j <= n && j != i
                                || {
                                    glp_assert_(
                                        b"1 <= j && j <= n && j != i\0" as *const u8 as *const i8,
                                        b"misc/wclique1.c\0" as *const u8 as *const i8,
                                        178 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            if *d_flag.offset(j as isize) != 0 {
                                (*d_flag.offset(j as isize) as i32 == 1 as i32
                                    || {
                                        glp_assert_(
                                            b"d_flag[j] == 1\0" as *const u8 as *const i8,
                                            b"misc/wclique1.c\0" as *const u8 as *const i8,
                                            181 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                                *d_flag.offset(j as isize) = 2 as i32 as i8;
                            }
                            k += 1;
                            k;
                        }
                        kk = d_size;
                        d_size = 0 as i32;
                        k = 1 as i32;
                        while k <= kk {
                            j = *d_list.offset(k as isize);
                            if *d_flag.offset(j as isize) as i32 == 1 as i32 {
                                *d_flag.offset(j as isize) = 0 as i32 as i8;
                                d_wght -= *w.offset(j as isize);
                            } else if *d_flag.offset(j as isize) as i32 == 2 as i32 {
                                d_size += 1;
                                *d_list.offset(d_size as isize) = j;
                                *d_flag.offset(j as isize) = 1 as i32 as i8;
                            } else {
                                (d_flag != d_flag
                                    || {
                                        glp_assert_(
                                            b"d_flag != d_flag\0" as *const u8 as *const i8,
                                            b"misc/wclique1.c\0" as *const u8 as *const i8,
                                            200 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                            }
                            k += 1;
                            k;
                        }
                    }
                    match current_block {
                        17973717047374398875 => {}
                        _ => {
                            if best < c_wght {
                                best = c_wght;
                                size = c_size;
                                (1 as i32 <= size && size <= n
                                    || {
                                        glp_assert_(
                                            b"1 <= size && size <= n\0" as *const u8 as *const i8,
                                            b"misc/wclique1.c\0" as *const u8 as *const i8,
                                            207 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                                memcpy(
                                    &mut *c.offset(1 as i32 as isize) as *mut i32
                                        as *mut libc::c_void,
                                    &mut *c_list.offset(1 as i32 as isize) as *mut i32
                                        as *const libc::c_void,
                                    (size as u64)
                                        .wrapping_mul(::core::mem::size_of::<i32>() as u64),
                                );
                            }
                        }
                    }
                }
                k = 1 as i32;
                while k <= c_size {
                    *skip.offset(*c_list.offset(k as isize) as isize) = 1 as i32 as i8;
                    k += 1;
                    k;
                }
                k = 1 as i32;
                while k <= d_size {
                    *d_flag.offset(*d_list.offset(k as isize) as isize) = 0 as i32 as i8;
                    k += 1;
                    k;
                }
            }
            l += 1;
            l;
        }
        glp_free(ind as *mut libc::c_void);
        glp_free(v_list as *mut libc::c_void);
        glp_free(c_list as *mut libc::c_void);
        glp_free(d_list as *mut libc::c_void);
        glp_free(d_flag as *mut libc::c_void);
        glp_free(skip as *mut libc::c_void);
        glp_free(sw as *mut libc::c_void);
    }
    return size;
}