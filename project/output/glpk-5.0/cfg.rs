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
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_cfg {
    pub n: i32,
    pub pos: *mut i32,
    pub neg: *mut i32,
    pub pool: *mut DMP,
    pub nv_max: i32,
    pub nv: i32,
    pub ref_0: *mut i32,
    pub vptr: *mut *mut CFGVLE,
    pub cptr: *mut *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGCLE {
    pub vptr: *mut CFGVLE,
    pub next: *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGVLE {
    pub v: i32,
    pub next: *mut CFGVLE,
}
pub type CFG = glp_cfg;
#[no_mangle]
pub unsafe extern "C" fn _glp_cfg_create_graph(mut n: i32, mut nv_max: i32) -> *mut CFG {
    let mut G: *mut CFG = 0 as *mut CFG;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"intopt/cfg.c\0" as *const u8 as *const i8,
                40 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (0 as i32 <= nv_max && nv_max <= n + n
        || {
            glp_assert_(
                b"0 <= nv_max && nv_max <= n + n\0" as *const u8 as *const i8,
                b"intopt/cfg.c\0" as *const u8 as *const i8,
                41 as i32,
            );
            1 as i32 != 0
        }) as i32;
    G = glp_alloc(1 as i32, ::core::mem::size_of::<CFG>() as u64 as i32) as *mut CFG;
    (*G).n = n;
    (*G).pos = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    memset(
        &mut *((*G).pos).offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        0 as i32,
        (n as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    (*G).neg = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    memset(
        &mut *((*G).neg).offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        0 as i32,
        (n as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    (*G).pool = _glp_dmp_create_pool();
    (*G).nv_max = nv_max;
    (*G).nv = 0 as i32;
    (*G).ref_0 = glp_alloc(
        1 as i32 + nv_max,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*G).vptr = glp_alloc(
        1 as i32 + nv_max,
        ::core::mem::size_of::<*mut CFGVLE>() as u64 as i32,
    ) as *mut *mut CFGVLE;
    (*G).cptr = glp_alloc(
        1 as i32 + nv_max,
        ::core::mem::size_of::<*mut CFGCLE>() as u64 as i32,
    ) as *mut *mut CFGCLE;
    return G;
}
unsafe extern "C" fn add_edge(mut G: *mut CFG, mut v: i32, mut w: i32) {
    let mut pool: *mut DMP = (*G).pool;
    let mut nv: i32 = (*G).nv;
    let mut vptr: *mut *mut CFGVLE = (*G).vptr;
    let mut vle: *mut CFGVLE = 0 as *mut CFGVLE;
    (1 as i32 <= v && v <= nv
        || {
            glp_assert_(
                b"1 <= v && v <= nv\0" as *const u8 as *const i8,
                b"intopt/cfg.c\0" as *const u8 as *const i8,
                84 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= w && w <= nv
        || {
            glp_assert_(
                b"1 <= w && w <= nv\0" as *const u8 as *const i8,
                b"intopt/cfg.c\0" as *const u8 as *const i8,
                85 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (v != w
        || {
            glp_assert_(
                b"v != w\0" as *const u8 as *const i8,
                b"intopt/cfg.c\0" as *const u8 as *const i8,
                86 as i32,
            );
            1 as i32 != 0
        }) as i32;
    vle = _glp_dmp_get_atom(pool, ::core::mem::size_of::<CFGVLE>() as u64 as i32)
        as *mut CFGVLE;
    (*vle).v = w;
    (*vle).next = *vptr.offset(v as isize);
    let ref mut fresh0 = *vptr.offset(v as isize);
    *fresh0 = vle;
    vle = _glp_dmp_get_atom(pool, ::core::mem::size_of::<CFGVLE>() as u64 as i32)
        as *mut CFGVLE;
    (*vle).v = v;
    (*vle).next = *vptr.offset(w as isize);
    let ref mut fresh1 = *vptr.offset(w as isize);
    *fresh1 = vle;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_cfg_add_clique(
    mut G: *mut CFG,
    mut size: i32,
    mut ind: *const i32,
) {
    let mut n: i32 = (*G).n;
    let mut pos: *mut i32 = (*G).pos;
    let mut neg: *mut i32 = (*G).neg;
    let mut pool: *mut DMP = (*G).pool;
    let mut nv_max: i32 = (*G).nv_max;
    let mut ref_0: *mut i32 = (*G).ref_0;
    let mut vptr: *mut *mut CFGVLE = (*G).vptr;
    let mut cptr: *mut *mut CFGCLE = (*G).cptr;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut v: i32 = 0;
    (2 as i32 <= size && size <= nv_max
        || {
            glp_assert_(
                b"2 <= size && size <= nv_max\0" as *const u8 as *const i8,
                b"intopt/cfg.c\0" as *const u8 as *const i8,
                108 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = 1 as i32;
    while k <= size {
        j = *ind.offset(k as isize);
        if j > 0 as i32 {
            (1 as i32 <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const i8,
                        b"intopt/cfg.c\0" as *const u8 as *const i8,
                        114 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if *pos.offset(j as isize) == 0 as i32 {
                (*G).nv += 1;
                let ref mut fresh2 = *pos.offset(j as isize);
                *fresh2 = (*G).nv;
                v = *fresh2;
                (v <= nv_max
                    || {
                        glp_assert_(
                            b"v <= nv_max\0" as *const u8 as *const i8,
                            b"intopt/cfg.c\0" as *const u8 as *const i8,
                            118 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                *ref_0.offset(v as isize) = j;
                let ref mut fresh3 = *vptr.offset(v as isize);
                *fresh3 = 0 as *mut CFGVLE;
                let ref mut fresh4 = *cptr.offset(v as isize);
                *fresh4 = 0 as *mut CFGCLE;
                if *neg.offset(j as isize) != 0 as i32 {
                    add_edge(G, v, *neg.offset(j as isize));
                }
            }
        } else {
            j = -j;
            (1 as i32 <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const i8,
                        b"intopt/cfg.c\0" as *const u8 as *const i8,
                        131 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if *neg.offset(j as isize) == 0 as i32 {
                (*G).nv += 1;
                let ref mut fresh5 = *neg.offset(j as isize);
                *fresh5 = (*G).nv;
                v = *fresh5;
                (v <= nv_max
                    || {
                        glp_assert_(
                            b"v <= nv_max\0" as *const u8 as *const i8,
                            b"intopt/cfg.c\0" as *const u8 as *const i8,
                            135 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                *ref_0.offset(v as isize) = j;
                let ref mut fresh6 = *vptr.offset(v as isize);
                *fresh6 = 0 as *mut CFGVLE;
                let ref mut fresh7 = *cptr.offset(v as isize);
                *fresh7 = 0 as *mut CFGCLE;
                if *pos.offset(j as isize) != 0 as i32 {
                    add_edge(G, v, *pos.offset(j as isize));
                }
            }
        }
        k += 1;
        k;
    }
    if size == 2 as i32 {
        add_edge(
            G,
            if *ind.offset(1 as i32 as isize) > 0 as i32 {
                *pos.offset(*ind.offset(1 as i32 as isize) as isize)
            } else {
                *neg.offset(-*ind.offset(1 as i32 as isize) as isize)
            },
            if *ind.offset(2 as i32 as isize) > 0 as i32 {
                *pos.offset(*ind.offset(2 as i32 as isize) as isize)
            } else {
                *neg.offset(-*ind.offset(2 as i32 as isize) as isize)
            },
        );
    } else {
        let mut vp: *mut CFGVLE = 0 as *mut CFGVLE;
        let mut vle: *mut CFGVLE = 0 as *mut CFGVLE;
        let mut cle: *mut CFGCLE = 0 as *mut CFGCLE;
        vp = 0 as *mut CFGVLE;
        k = 1 as i32;
        while k <= size {
            vle = _glp_dmp_get_atom(pool, ::core::mem::size_of::<CFGVLE>() as u64 as i32)
                as *mut CFGVLE;
            (*vle).v = if *ind.offset(k as isize) > 0 as i32 {
                *pos.offset(*ind.offset(k as isize) as isize)
            } else {
                *neg.offset(-*ind.offset(k as isize) as isize)
            };
            (*vle).next = vp;
            vp = vle;
            k += 1;
            k;
        }
        k = 1 as i32;
        while k <= size {
            cle = _glp_dmp_get_atom(pool, ::core::mem::size_of::<CFGCLE>() as u64 as i32)
                as *mut CFGCLE;
            (*cle).vptr = vp;
            v = if *ind.offset(k as isize) > 0 as i32 {
                *pos.offset(*ind.offset(k as isize) as isize)
            } else {
                *neg.offset(-*ind.offset(k as isize) as isize)
            };
            (*cle).next = *cptr.offset(v as isize);
            let ref mut fresh8 = *cptr.offset(v as isize);
            *fresh8 = cle;
            k += 1;
            k;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_cfg_get_adjacent(
    mut G: *mut CFG,
    mut v: i32,
    mut ind: *mut i32,
) -> i32 {
    let mut nv: i32 = (*G).nv;
    let mut ref_0: *mut i32 = (*G).ref_0;
    let mut vptr: *mut *mut CFGVLE = (*G).vptr;
    let mut cptr: *mut *mut CFGCLE = (*G).cptr;
    let mut vle: *mut CFGVLE = 0 as *mut CFGVLE;
    let mut cle: *mut CFGCLE = 0 as *mut CFGCLE;
    let mut k: i32 = 0;
    let mut w: i32 = 0;
    let mut len: i32 = 0;
    (1 as i32 <= v && v <= nv
        || {
            glp_assert_(
                b"1 <= v && v <= nv\0" as *const u8 as *const i8,
                b"intopt/cfg.c\0" as *const u8 as *const i8,
                193 as i32,
            );
            1 as i32 != 0
        }) as i32;
    len = 0 as i32;
    vle = *vptr.offset(v as isize);
    while !vle.is_null() {
        w = (*vle).v;
        (1 as i32 <= w && w <= nv
            || {
                glp_assert_(
                    b"1 <= w && w <= nv\0" as *const u8 as *const i8,
                    b"intopt/cfg.c\0" as *const u8 as *const i8,
                    198 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (w != v
            || {
                glp_assert_(
                    b"w != v\0" as *const u8 as *const i8,
                    b"intopt/cfg.c\0" as *const u8 as *const i8,
                    199 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *ref_0.offset(w as isize) > 0 as i32 {
            len += 1;
            *ind.offset(len as isize) = w;
            *ref_0.offset(w as isize) = -*ref_0.offset(w as isize);
        }
        vle = (*vle).next;
    }
    cle = *cptr.offset(v as isize);
    while !cle.is_null() {
        vle = (*cle).vptr;
        while !vle.is_null() {
            w = (*vle).v;
            (1 as i32 <= w && w <= nv
                || {
                    glp_assert_(
                        b"1 <= w && w <= nv\0" as *const u8 as *const i8,
                        b"intopt/cfg.c\0" as *const u8 as *const i8,
                        210 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if w != v && *ref_0.offset(w as isize) > 0 as i32 {
                len += 1;
                *ind.offset(len as isize) = w;
                *ref_0.offset(w as isize) = -*ref_0.offset(w as isize);
            }
            vle = (*vle).next;
        }
        cle = (*cle).next;
    }
    (1 as i32 <= len && len < nv
        || {
            glp_assert_(
                b"1 <= len && len < nv\0" as *const u8 as *const i8,
                b"intopt/cfg.c\0" as *const u8 as *const i8,
                217 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = 1 as i32;
    while k <= len {
        w = *ind.offset(k as isize);
        *ref_0.offset(w as isize) = -*ref_0.offset(w as isize);
        k += 1;
        k;
    }
    return len;
}
unsafe extern "C" fn intersection(
    mut d_len: i32,
    mut d_ind: *mut i32,
    mut d_pos: *mut i32,
    mut len: i32,
    mut ind: *const i32,
) -> i32 {
    let mut k: i32 = 0;
    let mut t: i32 = 0;
    let mut v: i32 = 0;
    let mut new_len: i32 = 0;
    t = 1 as i32;
    while t <= len {
        v = *ind.offset(t as isize);
        k = *d_pos.offset(v as isize);
        if k != 0 as i32 {
            (*d_ind.offset(k as isize) == v
                || {
                    glp_assert_(
                        b"d_ind[k] == v\0" as *const u8 as *const i8,
                        b"intopt/cfg.c\0" as *const u8 as *const i8,
                        269 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *d_ind.offset(k as isize) = -v;
        }
        t += 1;
        t;
    }
    new_len = 0 as i32;
    k = 1 as i32;
    while k <= d_len {
        v = *d_ind.offset(k as isize);
        if v < 0 as i32 {
            v = -v;
            new_len += 1;
            new_len;
            *d_ind.offset(new_len as isize) = v;
            *d_pos.offset(v as isize) = new_len;
        } else {
            *d_pos.offset(v as isize) = 0 as i32;
        }
        k += 1;
        k;
    }
    return new_len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_cfg_expand_clique(
    mut G: *mut CFG,
    mut c_len: i32,
    mut c_ind: *mut i32,
) -> i32 {
    let mut nv: i32 = (*G).nv;
    let mut d_len: i32 = 0;
    let mut d_ind: *mut i32 = 0 as *mut i32;
    let mut d_pos: *mut i32 = 0 as *mut i32;
    let mut len: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut k: i32 = 0;
    let mut v: i32 = 0;
    (0 as i32 <= c_len && c_len <= nv
        || {
            glp_assert_(
                b"0 <= c_len && c_len <= nv\0" as *const u8 as *const i8,
                b"intopt/cfg.c\0" as *const u8 as *const i8,
                298 as i32,
            );
            1 as i32 != 0
        }) as i32;
    d_ind = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    d_pos = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    ind = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    d_len = nv;
    k = 1 as i32;
    while k <= nv {
        let ref mut fresh9 = *d_pos.offset(k as isize);
        *fresh9 = k;
        *d_ind.offset(k as isize) = *fresh9;
        k += 1;
        k;
    }
    k = 1 as i32;
    while k <= c_len {
        v = *c_ind.offset(k as isize);
        (1 as i32 <= v && v <= nv
            || {
                glp_assert_(
                    b"1 <= v && v <= nv\0" as *const u8 as *const i8,
                    b"intopt/cfg.c\0" as *const u8 as *const i8,
                    311 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*d_pos.offset(v as isize) != 0 as i32
            || {
                glp_assert_(
                    b"d_pos[v] != 0\0" as *const u8 as *const i8,
                    b"intopt/cfg.c\0" as *const u8 as *const i8,
                    313 as i32,
                );
                1 as i32 != 0
            }) as i32;
        len = _glp_cfg_get_adjacent(G, v, ind);
        d_len = intersection(d_len, d_ind, d_pos, len, ind as *const i32);
        (*d_pos.offset(v as isize) == 0 as i32
            || {
                glp_assert_(
                    b"d_pos[v] == 0\0" as *const u8 as *const i8,
                    b"intopt/cfg.c\0" as *const u8 as *const i8,
                    319 as i32,
                );
                1 as i32 != 0
            }) as i32;
        k += 1;
        k;
    }
    while d_len > 0 as i32 {
        v = *d_ind.offset(1 as i32 as isize);
        (1 as i32 <= v && v <= nv
            || {
                glp_assert_(
                    b"1 <= v && v <= nv\0" as *const u8 as *const i8,
                    b"intopt/cfg.c\0" as *const u8 as *const i8,
                    325 as i32,
                );
                1 as i32 != 0
            }) as i32;
        c_len += 1;
        *c_ind.offset(c_len as isize) = v;
        len = _glp_cfg_get_adjacent(G, v, ind);
        d_len = intersection(d_len, d_ind, d_pos, len, ind as *const i32);
        (*d_pos.offset(v as isize) == 0 as i32
            || {
                glp_assert_(
                    b"d_pos[v] == 0\0" as *const u8 as *const i8,
                    b"intopt/cfg.c\0" as *const u8 as *const i8,
                    334 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    glp_free(d_ind as *mut libc::c_void);
    glp_free(d_pos as *mut libc::c_void);
    glp_free(ind as *mut libc::c_void);
    return c_len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_cfg_check_clique(
    mut G: *mut CFG,
    mut c_len: i32,
    mut c_ind: *const i32,
) {
    let mut nv: i32 = (*G).nv;
    let mut k: i32 = 0;
    let mut kk: i32 = 0;
    let mut v: i32 = 0;
    let mut w: i32 = 0;
    let mut len: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut flag: *mut i8 = 0 as *mut i8;
    ind = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    flag = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i8>() as u64 as i32)
        as *mut i8;
    memset(
        &mut *flag.offset(1 as i32 as isize) as *mut i8 as *mut libc::c_void,
        0 as i32,
        nv as u64,
    );
    (c_len >= 0 as i32
        || {
            glp_assert_(
                b"c_len >= 0\0" as *const u8 as *const i8,
                b"intopt/cfg.c\0" as *const u8 as *const i8,
                360 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = 1 as i32;
    while k <= c_len {
        v = *c_ind.offset(k as isize);
        (1 as i32 <= v && v <= nv
            || {
                glp_assert_(
                    b"1 <= v && v <= nv\0" as *const u8 as *const i8,
                    b"intopt/cfg.c\0" as *const u8 as *const i8,
                    364 as i32,
                );
                1 as i32 != 0
            }) as i32;
        len = _glp_cfg_get_adjacent(G, v, ind);
        kk = 1 as i32;
        while kk <= len {
            w = *ind.offset(kk as isize);
            (1 as i32 <= w && w <= nv
                || {
                    glp_assert_(
                        b"1 <= w && w <= nv\0" as *const u8 as *const i8,
                        b"intopt/cfg.c\0" as *const u8 as *const i8,
                        369 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (w != v
                || {
                    glp_assert_(
                        b"w != v\0" as *const u8 as *const i8,
                        b"intopt/cfg.c\0" as *const u8 as *const i8,
                        370 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *flag.offset(w as isize) = 1 as i32 as i8;
            kk += 1;
            kk;
        }
        kk = 1 as i32;
        while kk <= c_len {
            w = *c_ind.offset(kk as isize);
            (1 as i32 <= w && w <= nv
                || {
                    glp_assert_(
                        b"1 <= w && w <= nv\0" as *const u8 as *const i8,
                        b"intopt/cfg.c\0" as *const u8 as *const i8,
                        377 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if w != v {
                (*flag.offset(w as isize) as i32 != 0
                    || {
                        glp_assert_(
                            b"flag[w]\0" as *const u8 as *const i8,
                            b"intopt/cfg.c\0" as *const u8 as *const i8,
                            379 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            kk += 1;
            kk;
        }
        kk = 1 as i32;
        while kk <= len {
            *flag.offset(*ind.offset(kk as isize) as isize) = 0 as i32 as i8;
            kk += 1;
            kk;
        }
        k += 1;
        k;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(flag as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_cfg_delete_graph(mut G: *mut CFG) {
    glp_free((*G).pos as *mut libc::c_void);
    glp_free((*G).neg as *mut libc::c_void);
    _glp_dmp_delete_pool((*G).pool);
    glp_free((*G).ref_0 as *mut libc::c_void);
    glp_free((*G).vptr as *mut libc::c_void);
    glp_free((*G).cptr as *mut libc::c_void);
    glp_free(G as *mut libc::c_void);
}