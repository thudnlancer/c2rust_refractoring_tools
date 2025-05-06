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
    fn glp_difftime(t1: libc::c_double, t0: libc::c_double) -> libc::c_double;
    fn glp_time() -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n_0: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_printf(fmt: *const i8, _: ...);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub n: i32,
    pub wt: *const i32,
    pub a: *const u8,
    pub record: i32,
    pub rec_level: i32,
    pub rec: *mut i32,
    pub clique: *mut i32,
    pub set: *mut i32,
}
unsafe extern "C" fn sub(
    mut csa: *mut csa,
    mut ct: i32,
    mut table: *mut i32,
    mut level: i32,
    mut weight: i32,
    mut l_weight: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut curr_weight: i32 = 0;
    let mut left_weight: i32 = 0;
    let mut p1: *mut i32 = 0 as *mut i32;
    let mut p2: *mut i32 = 0 as *mut i32;
    let mut newtable: *mut i32 = 0 as *mut i32;
    newtable = glp_alloc((*csa).n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    if ct <= 0 as i32 {
        if ct == 0 as i32 {
            let fresh0 = level;
            level = level + 1;
            *((*csa).set).offset(fresh0 as isize) = *table.offset(0 as i32 as isize);
            weight += l_weight;
        }
        if weight > (*csa).record {
            (*csa).record = weight;
            (*csa).rec_level = level;
            i = 0 as i32;
            while i < level {
                *((*csa).rec).offset(i as isize) = *((*csa).set).offset(i as isize);
                i += 1;
                i;
            }
        }
    } else {
        i = ct;
        while i >= 0 as i32 {
            if level == 0 as i32 && i < ct {
                break;
            }
            k = *table.offset(i as isize);
            if level > 0 as i32
                && *((*csa).clique).offset(k as isize) <= (*csa).record - weight
            {
                break;
            }
            *((*csa).set).offset(level as isize) = k;
            curr_weight = weight + *((*csa).wt).offset(k as isize);
            l_weight -= *((*csa).wt).offset(k as isize);
            if l_weight <= (*csa).record - curr_weight {
                break;
            }
            p1 = newtable;
            p2 = table;
            left_weight = 0 as i32;
            while p2 < table.offset(i as isize) {
                let fresh1 = p2;
                p2 = p2.offset(1);
                j = *fresh1;
                if if j == k {
                    0 as i32
                } else if j > k {
                    *((*csa).a)
                        .offset(
                            ((j * (j - 1 as i32) / 2 as i32 + k) / 8 as i32) as isize,
                        ) as i32
                        & ((1 as i32)
                            << 8 as i32 - 1 as i32
                                - (j * (j - 1 as i32) / 2 as i32 + k) % 8 as i32) as u8
                            as i32
                } else {
                    *((*csa).a)
                        .offset(
                            ((k * (k - 1 as i32) / 2 as i32 + j) / 8 as i32) as isize,
                        ) as i32
                        & ((1 as i32)
                            << 8 as i32 - 1 as i32
                                - (k * (k - 1 as i32) / 2 as i32 + j) % 8 as i32) as u8
                            as i32
                } != 0
                {
                    let fresh2 = p1;
                    p1 = p1.offset(1);
                    *fresh2 = j;
                    left_weight += *((*csa).wt).offset(j as isize);
                }
            }
            if !(left_weight <= (*csa).record - curr_weight) {
                sub(
                    csa,
                    (p1.offset_from(newtable) as i64 - 1 as i32 as i64) as i32,
                    newtable,
                    level + 1 as i32,
                    curr_weight,
                    left_weight,
                );
            }
            i -= 1;
            i;
        }
    }
    glp_free(newtable as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_wclique(
    mut n_: i32,
    mut w: *const i32,
    mut a_: *const u8,
    mut ind: *mut i32,
) -> i32 {
    let mut csa_: csa = csa {
        n: 0,
        wt: 0 as *const i32,
        a: 0 as *const u8,
        record: 0,
        rec_level: 0,
        rec: 0 as *mut i32,
        clique: 0 as *mut i32,
        set: 0 as *mut i32,
    };
    let mut csa: *mut csa = &mut csa_;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p: i32 = 0;
    let mut max_wt: i32 = 0;
    let mut max_nwt: i32 = 0;
    let mut wth: i32 = 0;
    let mut used: *mut i32 = 0 as *mut i32;
    let mut nwt: *mut i32 = 0 as *mut i32;
    let mut pos: *mut i32 = 0 as *mut i32;
    let mut timer: libc::c_double = 0.;
    (*csa).n = n_;
    ((*csa).n > 0 as i32
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const i8,
                b"misc/wclique.c\0" as *const u8 as *const i8,
                173 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*csa).wt = &*w.offset(1 as i32 as isize) as *const i32;
    (*csa).a = a_;
    (*csa).record = 0 as i32;
    (*csa).rec_level = 0 as i32;
    (*csa).rec = &mut *ind.offset(1 as i32 as isize) as *mut i32;
    (*csa).clique = glp_alloc((*csa).n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*csa).set = glp_alloc((*csa).n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    used = glp_alloc((*csa).n, ::core::mem::size_of::<i32>() as u64 as i32) as *mut i32;
    nwt = glp_alloc((*csa).n, ::core::mem::size_of::<i32>() as u64 as i32) as *mut i32;
    pos = glp_alloc((*csa).n, ::core::mem::size_of::<i32>() as u64 as i32) as *mut i32;
    timer = glp_time();
    i = 0 as i32;
    while i < (*csa).n {
        *nwt.offset(i as isize) = 0 as i32;
        j = 0 as i32;
        while j < (*csa).n {
            if if i == j {
                0 as i32
            } else if i > j {
                *((*csa).a)
                    .offset(((i * (i - 1 as i32) / 2 as i32 + j) / 8 as i32) as isize)
                    as i32
                    & ((1 as i32)
                        << 8 as i32 - 1 as i32
                            - (i * (i - 1 as i32) / 2 as i32 + j) % 8 as i32) as u8
                        as i32
            } else {
                *((*csa).a)
                    .offset(((j * (j - 1 as i32) / 2 as i32 + i) / 8 as i32) as isize)
                    as i32
                    & ((1 as i32)
                        << 8 as i32 - 1 as i32
                            - (j * (j - 1 as i32) / 2 as i32 + i) % 8 as i32) as u8
                        as i32
            } != 0
            {
                *nwt.offset(i as isize) += *((*csa).wt).offset(j as isize);
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < (*csa).n {
        *used.offset(i as isize) = 0 as i32;
        i += 1;
        i;
    }
    i = (*csa).n - 1 as i32;
    while i >= 0 as i32 {
        max_wt = -(1 as i32);
        max_nwt = -(1 as i32);
        j = 0 as i32;
        while j < (*csa).n {
            if *used.offset(j as isize) == 0
                && (*((*csa).wt).offset(j as isize) > max_wt
                    || *((*csa).wt).offset(j as isize) == max_wt
                        && *nwt.offset(j as isize) > max_nwt)
            {
                max_wt = *((*csa).wt).offset(j as isize);
                max_nwt = *nwt.offset(j as isize);
                p = j;
            }
            j += 1;
            j;
        }
        *pos.offset(i as isize) = p;
        *used.offset(p as isize) = 1 as i32;
        j = 0 as i32;
        while j < (*csa).n {
            if *used.offset(j as isize) == 0 && j != p
                && (if p == j {
                    0 as i32
                } else {
                    (if p > j {
                        *((*csa).a)
                            .offset(
                                ((p * (p - 1 as i32) / 2 as i32 + j) / 8 as i32) as isize,
                            ) as i32
                            & ((1 as i32)
                                << 8 as i32 - 1 as i32
                                    - (p * (p - 1 as i32) / 2 as i32 + j) % 8 as i32) as u8
                                as i32
                    } else {
                        *((*csa).a)
                            .offset(
                                ((j * (j - 1 as i32) / 2 as i32 + p) / 8 as i32) as isize,
                            ) as i32
                            & ((1 as i32)
                                << 8 as i32 - 1 as i32
                                    - (j * (j - 1 as i32) / 2 as i32 + p) % 8 as i32) as u8
                                as i32
                    })
                }) != 0
            {
                *nwt.offset(j as isize) -= *((*csa).wt).offset(p as isize);
            }
            j += 1;
            j;
        }
        i -= 1;
        i;
    }
    wth = 0 as i32;
    i = 0 as i32;
    while i < (*csa).n {
        wth += *((*csa).wt).offset(*pos.offset(i as isize) as isize);
        sub(csa, i, pos, 0 as i32, 0 as i32, wth);
        *((*csa).clique).offset(*pos.offset(i as isize) as isize) = (*csa).record;
        if glp_difftime(glp_time(), timer) >= 5.0f64 - 0.001f64 {
            glp_printf(
                b"level = %d (%d); best = %d\n\0" as *const u8 as *const i8,
                i + 1 as i32,
                (*csa).n,
                (*csa).record,
            );
            timer = glp_time();
        }
        i += 1;
        i;
    }
    glp_free((*csa).clique as *mut libc::c_void);
    glp_free((*csa).set as *mut libc::c_void);
    glp_free(used as *mut libc::c_void);
    glp_free(nwt as *mut libc::c_void);
    glp_free(pos as *mut libc::c_void);
    i = 1 as i32;
    while i <= (*csa).rec_level {
        let ref mut fresh3 = *ind.offset(i as isize);
        *fresh3 += 1;
        *fresh3;
        i += 1;
        i;
    }
    return (*csa).rec_level;
}