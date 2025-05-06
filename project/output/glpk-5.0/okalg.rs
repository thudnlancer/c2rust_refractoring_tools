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
}
unsafe extern "C" fn overflow(mut u: i32, mut v: i32) -> i32 {
    if u > 0 as i32 && v > 0 as i32 && u + v < 0 as i32 {
        return 1 as i32;
    }
    if u < 0 as i32 && v < 0 as i32 && u + v > 0 as i32 {
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_okalg(
    mut nv: i32,
    mut na: i32,
    mut tail: *const i32,
    mut head: *const i32,
    mut low: *const i32,
    mut cap: *const i32,
    mut cost: *const i32,
    mut x: *mut i32,
    mut pi: *mut i32,
) -> i32 {
    let mut current_block: u64;
    let mut a: i32 = 0;
    let mut aok: i32 = 0;
    let mut delta: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut lambda: i32 = 0;
    let mut pos1: i32 = 0;
    let mut pos2: i32 = 0;
    let mut s: i32 = 0;
    let mut t: i32 = 0;
    let mut temp: i32 = 0;
    let mut ret: i32 = 0;
    let mut ptr: *mut i32 = 0 as *mut i32;
    let mut arc: *mut i32 = 0 as *mut i32;
    let mut link: *mut i32 = 0 as *mut i32;
    let mut list: *mut i32 = 0 as *mut i32;
    (nv >= 0 as i32
        || {
            glp_assert_(
                b"nv >= 0\0" as *const u8 as *const i8,
                b"misc/okalg.c\0" as *const u8 as *const i8,
                103 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (na >= 0 as i32
        || {
            glp_assert_(
                b"na >= 0\0" as *const u8 as *const i8,
                b"misc/okalg.c\0" as *const u8 as *const i8,
                104 as i32,
            );
            1 as i32 != 0
        }) as i32;
    a = 1 as i32;
    while a <= na {
        i = *tail.offset(a as isize);
        j = *head.offset(a as isize);
        (1 as i32 <= i && i <= nv
            || {
                glp_assert_(
                    b"1 <= i && i <= nv\0" as *const u8 as *const i8,
                    b"misc/okalg.c\0" as *const u8 as *const i8,
                    107 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (1 as i32 <= j && j <= nv
            || {
                glp_assert_(
                    b"1 <= j && j <= nv\0" as *const u8 as *const i8,
                    b"misc/okalg.c\0" as *const u8 as *const i8,
                    108 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (i != j
            || {
                glp_assert_(
                    b"i != j\0" as *const u8 as *const i8,
                    b"misc/okalg.c\0" as *const u8 as *const i8,
                    109 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (0 as i32 <= *low.offset(a as isize)
            && *low.offset(a as isize) <= *cap.offset(a as isize)
            || {
                glp_assert_(
                    b"0 <= low[a] && low[a] <= cap[a]\0" as *const u8 as *const i8,
                    b"misc/okalg.c\0" as *const u8 as *const i8,
                    110 as i32,
                );
                1 as i32 != 0
            }) as i32;
        a += 1;
        a;
    }
    ptr = glp_alloc(
        1 as i32 + nv + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    arc = glp_alloc(1 as i32 + na + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    link = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    list = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    i = 1 as i32;
    while i <= nv {
        *ptr.offset(i as isize) = 0 as i32;
        i += 1;
        i;
    }
    a = 1 as i32;
    while a <= na {
        let ref mut fresh0 = *ptr.offset(*tail.offset(a as isize) as isize);
        *fresh0 += 1;
        *fresh0;
        let ref mut fresh1 = *ptr.offset(*head.offset(a as isize) as isize);
        *fresh1 += 1;
        *fresh1;
        a += 1;
        a;
    }
    let ref mut fresh2 = *ptr.offset(1 as i32 as isize);
    *fresh2 += 1;
    *fresh2;
    i = 1 as i32;
    while i < nv {
        *ptr.offset((i + 1 as i32) as isize) += *ptr.offset(i as isize);
        i += 1;
        i;
    }
    *ptr.offset((nv + 1 as i32) as isize) = *ptr.offset(nv as isize);
    a = 1 as i32;
    while a <= na {
        let ref mut fresh3 = *ptr.offset(*tail.offset(a as isize) as isize);
        *fresh3 -= 1;
        *arc.offset(*fresh3 as isize) = a;
        let ref mut fresh4 = *ptr.offset(*head.offset(a as isize) as isize);
        *fresh4 -= 1;
        *arc.offset(*fresh4 as isize) = a;
        a += 1;
        a;
    }
    (*ptr.offset(1 as i32 as isize) == 1 as i32
        || {
            glp_assert_(
                b"ptr[1] == 1\0" as *const u8 as *const i8,
                b"misc/okalg.c\0" as *const u8 as *const i8,
                134 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*ptr.offset((nv + 1 as i32) as isize) == na + na + 1 as i32
        || {
            glp_assert_(
                b"ptr[nv+1] == na+na+1\0" as *const u8 as *const i8,
                b"misc/okalg.c\0" as *const u8 as *const i8,
                135 as i32,
            );
            1 as i32 != 0
        }) as i32;
    a = 1 as i32;
    while a <= na {
        *x.offset(a as isize) = 0 as i32;
        a += 1;
        a;
    }
    i = 1 as i32;
    while i <= nv {
        *pi.offset(i as isize) = 0 as i32;
        i += 1;
        i;
    }
    '_loop: loop {
        aok = 0 as i32;
        a = 1 as i32;
        while a <= na {
            i = *tail.offset(a as isize);
            j = *head.offset(a as isize);
            if overflow(
                *cost.offset(a as isize),
                *pi.offset(i as isize) - *pi.offset(j as isize),
            ) != 0
            {
                ret = 2 as i32;
                current_block = 2603414505842432426;
                break '_loop;
            } else {
                lambda = *cost.offset(a as isize)
                    + (*pi.offset(i as isize) - *pi.offset(j as isize));
                if *x.offset(a as isize) < *low.offset(a as isize)
                    || lambda < 0 as i32
                        && *x.offset(a as isize) < *cap.offset(a as isize)
                {
                    aok = a;
                    s = j;
                    t = i;
                    break;
                } else if *x.offset(a as isize) > *cap.offset(a as isize)
                    || lambda > 0 as i32
                        && *x.offset(a as isize) > *low.offset(a as isize)
                {
                    aok = a;
                    s = i;
                    t = j;
                    break;
                } else {
                    a += 1;
                    a;
                }
            }
        }
        if aok == 0 as i32 {
            a = 1 as i32;
            current_block = 790185930182612747;
            break;
        } else {
            i = 1 as i32;
            while i <= nv {
                *link.offset(i as isize) = 0 as i32;
                i += 1;
                i;
            }
            *link.offset(s as isize) = aok;
            *list.offset(1 as i32 as isize) = s;
            pos2 = 1 as i32;
            pos1 = pos2;
            's_367: loop {
                if !(pos1 <= pos2) {
                    current_block = 7416055328783156979;
                    break;
                }
                let fresh5 = pos1;
                pos1 = pos1 + 1;
                i = *list.offset(fresh5 as isize);
                k = *ptr.offset(i as isize);
                while k < *ptr.offset((i + 1 as i32) as isize) {
                    a = *arc.offset(k as isize);
                    if *tail.offset(a as isize) == i {
                        j = *head.offset(a as isize);
                        if *link.offset(j as isize) != 0 as i32 {
                            current_block = 2168227384378665163;
                        } else if *x.offset(a as isize) >= *cap.offset(a as isize) {
                            current_block = 2168227384378665163;
                        } else if overflow(
                            *cost.offset(a as isize),
                            *pi.offset(i as isize) - *pi.offset(j as isize),
                        ) != 0
                        {
                            ret = 2 as i32;
                            current_block = 2603414505842432426;
                            break '_loop;
                        } else {
                            lambda = *cost.offset(a as isize)
                                + (*pi.offset(i as isize) - *pi.offset(j as isize));
                            if lambda > 0 as i32
                                && *x.offset(a as isize) >= *low.offset(a as isize)
                            {
                                current_block = 2168227384378665163;
                            } else {
                                current_block = 11071260907632769126;
                            }
                        }
                    } else if *head.offset(a as isize) == i {
                        j = *tail.offset(a as isize);
                        if *link.offset(j as isize) != 0 as i32 {
                            current_block = 2168227384378665163;
                        } else if *x.offset(a as isize) <= *low.offset(a as isize) {
                            current_block = 2168227384378665163;
                        } else if overflow(
                            *cost.offset(a as isize),
                            *pi.offset(j as isize) - *pi.offset(i as isize),
                        ) != 0
                        {
                            ret = 2 as i32;
                            current_block = 2603414505842432426;
                            break '_loop;
                        } else {
                            lambda = *cost.offset(a as isize)
                                + (*pi.offset(j as isize) - *pi.offset(i as isize));
                            if lambda < 0 as i32
                                && *x.offset(a as isize) <= *cap.offset(a as isize)
                            {
                                current_block = 2168227384378665163;
                            } else {
                                current_block = 11071260907632769126;
                            }
                        }
                    } else {
                        (a != a
                            || {
                                glp_assert_(
                                    b"a != a\0" as *const u8 as *const i8,
                                    b"misc/okalg.c\0" as *const u8 as *const i8,
                                    256 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        current_block = 11071260907632769126;
                    }
                    match current_block {
                        11071260907632769126 => {
                            *link.offset(j as isize) = a;
                            pos2 += 1;
                            *list.offset(pos2 as isize) = j;
                            if j == t {
                                current_block = 14850352103120540814;
                                break 's_367;
                            }
                        }
                        _ => {}
                    }
                    k += 1;
                    k;
                }
            }
            match current_block {
                14850352103120540814 => {
                    delta = 0 as i32;
                    j = t;
                    loop {
                        a = *link.offset(j as isize);
                        if *head.offset(a as isize) == j {
                            i = *tail.offset(a as isize);
                            lambda = *cost.offset(a as isize)
                                + (*pi.offset(i as isize) - *pi.offset(j as isize));
                            if lambda > 0 as i32
                                && *x.offset(a as isize) < *low.offset(a as isize)
                            {
                                temp = *low.offset(a as isize) - *x.offset(a as isize);
                            } else if lambda <= 0 as i32
                                && *x.offset(a as isize) < *cap.offset(a as isize)
                            {
                                temp = *cap.offset(a as isize) - *x.offset(a as isize);
                            } else {
                                (a != a
                                    || {
                                        glp_assert_(
                                            b"a != a\0" as *const u8 as *const i8,
                                            b"misc/okalg.c\0" as *const u8 as *const i8,
                                            326 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                            }
                        } else if *tail.offset(a as isize) == j {
                            i = *head.offset(a as isize);
                            lambda = *cost.offset(a as isize)
                                + (*pi.offset(j as isize) - *pi.offset(i as isize));
                            if lambda < 0 as i32
                                && *x.offset(a as isize) > *cap.offset(a as isize)
                            {
                                temp = *x.offset(a as isize) - *cap.offset(a as isize);
                            } else if lambda >= 0 as i32
                                && *x.offset(a as isize) > *low.offset(a as isize)
                            {
                                temp = *x.offset(a as isize) - *low.offset(a as isize);
                            } else {
                                (a != a
                                    || {
                                        glp_assert_(
                                            b"a != a\0" as *const u8 as *const i8,
                                            b"misc/okalg.c\0" as *const u8 as *const i8,
                                            341 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                            }
                        } else {
                            (a != a
                                || {
                                    glp_assert_(
                                        b"a != a\0" as *const u8 as *const i8,
                                        b"misc/okalg.c\0" as *const u8 as *const i8,
                                        344 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                        }
                        if delta == 0 as i32 || delta > temp {
                            delta = temp;
                        }
                        if i == t {
                            break;
                        }
                        j = i;
                    }
                    (delta > 0 as i32
                        || {
                            glp_assert_(
                                b"delta > 0\0" as *const u8 as *const i8,
                                b"misc/okalg.c\0" as *const u8 as *const i8,
                                349 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    j = t;
                    loop {
                        a = *link.offset(j as isize);
                        if *head.offset(a as isize) == j {
                            i = *tail.offset(a as isize);
                            *x.offset(a as isize) += delta;
                        } else if *tail.offset(a as isize) == j {
                            i = *head.offset(a as isize);
                            *x.offset(a as isize) -= delta;
                        } else {
                            (a != a
                                || {
                                    glp_assert_(
                                        b"a != a\0" as *const u8 as *const i8,
                                        b"misc/okalg.c\0" as *const u8 as *const i8,
                                        367 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                        }
                        if i == t {
                            break;
                        }
                        j = i;
                    }
                }
                _ => {
                    delta = 0 as i32;
                    a = 1 as i32;
                    while a <= na {
                        i = *tail.offset(a as isize);
                        j = *head.offset(a as isize);
                        if *link.offset(i as isize) != 0 as i32
                            && *link.offset(j as isize) == 0 as i32
                        {
                            if overflow(
                                *cost.offset(a as isize),
                                *pi.offset(i as isize) - *pi.offset(j as isize),
                            ) != 0
                            {
                                ret = 2 as i32;
                                current_block = 2603414505842432426;
                                break '_loop;
                            } else {
                                lambda = *cost.offset(a as isize)
                                    + (*pi.offset(i as isize) - *pi.offset(j as isize));
                                if *x.offset(a as isize) <= *cap.offset(a as isize)
                                    && lambda > 0 as i32
                                {
                                    if delta == 0 as i32 || delta > lambda {
                                        delta = lambda;
                                    }
                                }
                            }
                        } else if *link.offset(i as isize) == 0 as i32
                            && *link.offset(j as isize) != 0 as i32
                        {
                            if overflow(
                                *cost.offset(a as isize),
                                *pi.offset(i as isize) - *pi.offset(j as isize),
                            ) != 0
                            {
                                ret = 2 as i32;
                                current_block = 2603414505842432426;
                                break '_loop;
                            } else {
                                lambda = *cost.offset(a as isize)
                                    + (*pi.offset(i as isize) - *pi.offset(j as isize));
                                if *x.offset(a as isize) >= *low.offset(a as isize)
                                    && lambda < 0 as i32
                                {
                                    if delta == 0 as i32 || delta > -lambda {
                                        delta = -lambda;
                                    }
                                }
                            }
                        }
                        a += 1;
                        a;
                    }
                    if delta == 0 as i32 {
                        ret = 1 as i32;
                        current_block = 2603414505842432426;
                        break;
                    } else {
                        i = 1 as i32;
                        while i <= nv {
                            if *link.offset(i as isize) == 0 as i32 {
                                if overflow(*pi.offset(i as isize), delta) != 0 {
                                    ret = 2 as i32;
                                    current_block = 2603414505842432426;
                                    break '_loop;
                                } else {
                                    *pi.offset(i as isize) += delta;
                                }
                            }
                            i += 1;
                            i;
                        }
                    }
                }
            }
        }
    }
    's_219: loop {
        match current_block {
            2603414505842432426 => {
                glp_free(ptr as *mut libc::c_void);
                break;
            }
            _ => {
                if a <= na {
                    if !(*low.offset(a as isize) <= *x.offset(a as isize)
                        && *x.offset(a as isize) <= *cap.offset(a as isize))
                    {
                        ret = 3 as i32;
                        current_block = 2603414505842432426;
                    } else {
                        a += 1;
                        a;
                        current_block = 790185930182612747;
                    }
                } else {
                    i = 1 as i32;
                    while i <= nv {
                        temp = 0 as i32;
                        k = *ptr.offset(i as isize);
                        while k < *ptr.offset((i + 1 as i32) as isize) {
                            a = *arc.offset(k as isize);
                            if *tail.offset(a as isize) == i {
                                temp += *x.offset(a as isize);
                            } else if *head.offset(a as isize) == i {
                                temp -= *x.offset(a as isize);
                            } else {
                                (a != a
                                    || {
                                        glp_assert_(
                                            b"a != a\0" as *const u8 as *const i8,
                                            b"misc/okalg.c\0" as *const u8 as *const i8,
                                            188 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                            }
                            k += 1;
                            k;
                        }
                        if temp != 0 as i32 {
                            ret = 3 as i32;
                            current_block = 2603414505842432426;
                            continue 's_219;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    a = 1 as i32;
                    while a <= na {
                        i = *tail.offset(a as isize);
                        j = *head.offset(a as isize);
                        lambda = *cost.offset(a as isize)
                            + (*pi.offset(i as isize) - *pi.offset(j as isize));
                        if lambda > 0 as i32
                            && *x.offset(a as isize) != *low.offset(a as isize)
                            || lambda < 0 as i32
                                && *x.offset(a as isize) != *cap.offset(a as isize)
                        {
                            ret = 3 as i32;
                            current_block = 2603414505842432426;
                            continue 's_219;
                        } else {
                            a += 1;
                            a;
                        }
                    }
                    ret = 0 as i32;
                    current_block = 2603414505842432426;
                }
            }
        }
    }
    glp_free(arc as *mut libc::c_void);
    glp_free(link as *mut libc::c_void);
    glp_free(list as *mut libc::c_void);
    return ret;
}