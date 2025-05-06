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
#[no_mangle]
pub unsafe extern "C" fn _glp_ffalg(
    mut nv: i32,
    mut na: i32,
    mut tail: *const i32,
    mut head: *const i32,
    mut s: i32,
    mut t: i32,
    mut cap: *const i32,
    mut x: *mut i32,
    mut cut: *mut i8,
) {
    let mut current_block: u64;
    let mut a: i32 = 0;
    let mut delta: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut pos1: i32 = 0;
    let mut pos2: i32 = 0;
    let mut temp: i32 = 0;
    let mut ptr: *mut i32 = 0 as *mut i32;
    let mut arc: *mut i32 = 0 as *mut i32;
    let mut link: *mut i32 = 0 as *mut i32;
    let mut list: *mut i32 = 0 as *mut i32;
    (nv >= 2 as i32
        || {
            glp_assert_(
                b"nv >= 2\0" as *const u8 as *const i8,
                b"misc/ffalg.c\0" as *const u8 as *const i8,
                79 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (na >= 0 as i32
        || {
            glp_assert_(
                b"na >= 0\0" as *const u8 as *const i8,
                b"misc/ffalg.c\0" as *const u8 as *const i8,
                80 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= s && s <= nv
        || {
            glp_assert_(
                b"1 <= s && s <= nv\0" as *const u8 as *const i8,
                b"misc/ffalg.c\0" as *const u8 as *const i8,
                81 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (1 as i32 <= t && t <= nv
        || {
            glp_assert_(
                b"1 <= t && t <= nv\0" as *const u8 as *const i8,
                b"misc/ffalg.c\0" as *const u8 as *const i8,
                82 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (s != t
        || {
            glp_assert_(
                b"s != t\0" as *const u8 as *const i8,
                b"misc/ffalg.c\0" as *const u8 as *const i8,
                83 as i32,
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
                    b"misc/ffalg.c\0" as *const u8 as *const i8,
                    86 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (1 as i32 <= j && j <= nv
            || {
                glp_assert_(
                    b"1 <= j && j <= nv\0" as *const u8 as *const i8,
                    b"misc/ffalg.c\0" as *const u8 as *const i8,
                    87 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (i != j
            || {
                glp_assert_(
                    b"i != j\0" as *const u8 as *const i8,
                    b"misc/ffalg.c\0" as *const u8 as *const i8,
                    88 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*cap.offset(a as isize) >= 0 as i32
            || {
                glp_assert_(
                    b"cap[a] >= 0\0" as *const u8 as *const i8,
                    b"misc/ffalg.c\0" as *const u8 as *const i8,
                    89 as i32,
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
                b"misc/ffalg.c\0" as *const u8 as *const i8,
                113 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*ptr.offset((nv + 1 as i32) as isize) == na + na + 1 as i32
        || {
            glp_assert_(
                b"ptr[nv+1] == na+na+1\0" as *const u8 as *const i8,
                b"misc/ffalg.c\0" as *const u8 as *const i8,
                114 as i32,
            );
            1 as i32 != 0
        }) as i32;
    a = 1 as i32;
    while a <= na {
        *x.offset(a as isize) = 0 as i32;
        a += 1;
        a;
    }
    '_loop: loop {
        i = 1 as i32;
        while i <= nv {
            *link.offset(i as isize) = 0 as i32;
            i += 1;
            i;
        }
        *link.offset(s as isize) = -(1 as i32);
        *list.offset(1 as i32 as isize) = s;
        pos2 = 1 as i32;
        pos1 = pos2;
        's_175: loop {
            if !(pos1 <= pos2) {
                break '_loop;
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
                        current_block = 11048769245176032998;
                    } else if *x.offset(a as isize) == *cap.offset(a as isize) {
                        current_block = 11048769245176032998;
                    } else {
                        current_block = 14775119014532381840;
                    }
                } else if *head.offset(a as isize) == i {
                    j = *tail.offset(a as isize);
                    if *link.offset(j as isize) != 0 as i32 {
                        current_block = 11048769245176032998;
                    } else if *x.offset(a as isize) == 0 as i32 {
                        current_block = 11048769245176032998;
                    } else {
                        current_block = 14775119014532381840;
                    }
                } else {
                    (a != a
                        || {
                            glp_assert_(
                                b"a != a\0" as *const u8 as *const i8,
                                b"misc/ffalg.c\0" as *const u8 as *const i8,
                                154 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    current_block = 14775119014532381840;
                }
                match current_block {
                    14775119014532381840 => {
                        *link.offset(j as isize) = a;
                        pos2 += 1;
                        *list.offset(pos2 as isize) = j;
                        if j == t {
                            break 's_175;
                        }
                    }
                    _ => {}
                }
                k += 1;
                k;
            }
        }
        delta = 0 as i32;
        j = t;
        while j != s {
            a = *link.offset(j as isize);
            if *head.offset(a as isize) == j {
                i = *tail.offset(a as isize);
                temp = *cap.offset(a as isize) - *x.offset(a as isize);
            } else if *tail.offset(a as isize) == j {
                i = *head.offset(a as isize);
                temp = *x.offset(a as isize);
            } else {
                (a != a
                    || {
                        glp_assert_(
                            b"a != a\0" as *const u8 as *const i8,
                            b"misc/ffalg.c\0" as *const u8 as *const i8,
                            189 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if delta == 0 as i32 || delta > temp {
                delta = temp;
            }
            j = i;
        }
        (delta > 0 as i32
            || {
                glp_assert_(
                    b"delta > 0\0" as *const u8 as *const i8,
                    b"misc/ffalg.c\0" as *const u8 as *const i8,
                    192 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j = t;
        while j != s {
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
                            b"misc/ffalg.c\0" as *const u8 as *const i8,
                            208 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            j = i;
        }
    }
    if !cut.is_null() {
        i = 1 as i32;
        while i <= nv {
            *cut.offset(i as isize) = (*link.offset(i as isize) != 0 as i32) as i32
                as i8;
            i += 1;
            i;
        }
    }
    glp_free(ptr as *mut libc::c_void);
    glp_free(arc as *mut libc::c_void);
    glp_free(link as *mut libc::c_void);
    glp_free(list as *mut libc::c_void);
}