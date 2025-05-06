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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bigmul(
    mut n: i32,
    mut m: i32,
    mut x: *mut libc::c_ushort,
    mut y: *mut libc::c_ushort,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut t: u32 = 0;
    (n >= 1 as i32
        || {
            glp_assert_(
                b"n >= 1\0" as *const u8 as *const i8,
                b"misc/bignum.c\0" as *const u8 as *const i8,
                75 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (m >= 1 as i32
        || {
            glp_assert_(
                b"m >= 1\0" as *const u8 as *const i8,
                b"misc/bignum.c\0" as *const u8 as *const i8,
                76 as i32,
            );
            1 as i32 != 0
        }) as i32;
    j = 0 as i32;
    while j < m {
        *x.offset(j as isize) = 0 as i32 as libc::c_ushort;
        j += 1;
        j;
    }
    i = 0 as i32;
    while i < n {
        if *x.offset((i + m) as isize) != 0 {
            t = 0 as i32 as u32;
            j = 0 as i32;
            while j < m {
                t = t
                    .wrapping_add(
                        (*x.offset((i + m) as isize) as u32)
                            .wrapping_mul(*y.offset(j as isize) as u32)
                            .wrapping_add(*x.offset((i + j) as isize) as u32),
                    );
                *x.offset((i + j) as isize) = t as libc::c_ushort;
                t >>= 16 as i32;
                j += 1;
                j;
            }
            *x.offset((i + m) as isize) = t as libc::c_ushort;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_bigdiv(
    mut n: i32,
    mut m: i32,
    mut x: *mut libc::c_ushort,
    mut y: *mut libc::c_ushort,
) {
    let mut current_block: u64;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut t: u32 = 0;
    let mut d: libc::c_ushort = 0;
    let mut q: libc::c_ushort = 0;
    let mut r: libc::c_ushort = 0;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"misc/bignum.c\0" as *const u8 as *const i8,
                133 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (m >= 1 as i32
        || {
            glp_assert_(
                b"m >= 1\0" as *const u8 as *const i8,
                b"misc/bignum.c\0" as *const u8 as *const i8,
                134 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*y.offset((m - 1 as i32) as isize) as i32 != 0 as i32
        || {
            glp_assert_(
                b"y[m-1] != 0\0" as *const u8 as *const i8,
                b"misc/bignum.c\0" as *const u8 as *const i8,
                135 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if m == 1 as i32 {
        d = 0 as i32 as libc::c_ushort;
        i = n;
        while i >= 0 as i32 {
            t = ((d as u32) << 16 as i32).wrapping_add(*x.offset(i as isize) as u32);
            *x.offset((i + 1 as i32) as isize) = t
                .wrapping_div(*y.offset(0 as i32 as isize) as u32) as libc::c_ushort;
            d = t.wrapping_rem(*y.offset(0 as i32 as isize) as u32) as libc::c_ushort;
            i -= 1;
            i;
        }
        *x.offset(0 as i32 as isize) = d;
    } else {
        d = (0x10000 as i32 as u32)
            .wrapping_div(
                (*y.offset((m - 1 as i32) as isize) as u32).wrapping_add(1 as i32 as u32),
            ) as libc::c_ushort;
        if d as i32 == 1 as i32 {
            *x.offset((n + m) as isize) = 0 as i32 as libc::c_ushort;
        } else {
            t = 0 as i32 as u32;
            i = 0 as i32;
            while i < n + m {
                t = t
                    .wrapping_add((*x.offset(i as isize) as u32).wrapping_mul(d as u32));
                *x.offset(i as isize) = t as libc::c_ushort;
                t >>= 16 as i32;
                i += 1;
                i;
            }
            *x.offset((n + m) as isize) = t as libc::c_ushort;
            t = 0 as i32 as u32;
            j = 0 as i32;
            while j < m {
                t = t
                    .wrapping_add((*y.offset(j as isize) as u32).wrapping_mul(d as u32));
                *y.offset(j as isize) = t as libc::c_ushort;
                t >>= 16 as i32;
                j += 1;
                j;
            }
        }
        i = n;
        while i >= 0 as i32 {
            if (*x.offset((i + m) as isize) as i32)
                < *y.offset((m - 1 as i32) as isize) as i32
            {
                t = ((*x.offset((i + m) as isize) as u32) << 16 as i32)
                    .wrapping_add(*x.offset((i + m - 1 as i32) as isize) as u32);
                q = t.wrapping_div(*y.offset((m - 1 as i32) as isize) as u32)
                    as libc::c_ushort;
                r = t.wrapping_rem(*y.offset((m - 1 as i32) as isize) as u32)
                    as libc::c_ushort;
                if q as i32 == 0 as i32 {
                    current_block = 6472328528319226318;
                } else {
                    current_block = 14122167067469077234;
                }
            } else {
                q = 0 as i32 as libc::c_ushort;
                r = *x.offset((i + m - 1 as i32) as isize);
                current_block = 16758441474344832020;
            }
            loop {
                match current_block {
                    6472328528319226318 => {
                        *x.offset((i + m) as isize) = q;
                        break;
                    }
                    16758441474344832020 => {
                        q = q.wrapping_sub(1);
                        q;
                        t = (r as u32)
                            .wrapping_add(*y.offset((m - 1 as i32) as isize) as u32);
                        r = t as libc::c_ushort;
                        if !(t > 0xffff as i32 as u32) {
                            current_block = 14122167067469077234;
                            continue;
                        }
                    }
                    _ => {
                        t = (*y.offset((m - 2 as i32) as isize) as u32)
                            .wrapping_mul(q as u32);
                        if (t >> 16 as i32) as libc::c_ushort as i32 > r as i32 {
                            current_block = 16758441474344832020;
                            continue;
                        }
                        if !(((t >> 16 as i32) as libc::c_ushort as i32) < r as i32) {
                            if t as libc::c_ushort as i32
                                > *x.offset((i + m - 2 as i32) as isize) as i32
                            {
                                current_block = 16758441474344832020;
                                continue;
                            }
                        }
                    }
                }
                if q as i32 == 0 as i32 {
                    current_block = 6472328528319226318;
                    continue;
                }
                t = 0 as i32 as u32;
                j = 0 as i32;
                while j < m {
                    t = t
                        .wrapping_add(
                            (*y.offset(j as isize) as u32).wrapping_mul(q as u32),
                        );
                    if (*x.offset((i + j) as isize) as i32) < t as libc::c_ushort as i32
                    {
                        t = t.wrapping_add(0x10000 as i32 as u32);
                    }
                    let ref mut fresh0 = *x.offset((i + j) as isize);
                    *fresh0 = (*fresh0 as i32 - t as libc::c_ushort as i32)
                        as libc::c_ushort;
                    t >>= 16 as i32;
                    j += 1;
                    j;
                }
                if *x.offset((i + m) as isize) as i32 >= t as libc::c_ushort as i32 {
                    current_block = 6472328528319226318;
                    continue;
                }
                q = q.wrapping_sub(1);
                q;
                t = 0 as i32 as u32;
                j = 0 as i32;
                while j < m {
                    t = t
                        .wrapping_add(
                            (*x.offset((i + j) as isize) as u32)
                                .wrapping_add(*y.offset(j as isize) as u32),
                        );
                    *x.offset((i + j) as isize) = t as libc::c_ushort;
                    t >>= 16 as i32;
                    j += 1;
                    j;
                }
                current_block = 6472328528319226318;
            }
            i -= 1;
            i;
        }
        if d as i32 > 1 as i32 {
            t = 0 as i32 as u32;
            i = m - 1 as i32;
            while i >= 0 as i32 {
                t = (t << 16 as i32).wrapping_add(*x.offset(i as isize) as u32);
                *x.offset(i as isize) = t.wrapping_div(d as u32) as libc::c_ushort;
                t = t.wrapping_rem(d as u32);
                i -= 1;
                i;
            }
            t = 0 as i32 as u32;
            j = m - 1 as i32;
            while j >= 0 as i32 {
                t = (t << 16 as i32).wrapping_add(*y.offset(j as isize) as u32);
                *y.offset(j as isize) = t.wrapping_div(d as u32) as libc::c_ushort;
                t = t.wrapping_rem(d as u32);
                j -= 1;
                j;
            }
        }
    };
}