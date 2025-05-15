use ::libc;
use core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type ssize_t = __ssize_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: libc::c_int,
    pub padding__: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
#[inline]
unsafe extern "C" fn libzahl_memcpy(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d
                .offset(
                    (20 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((20 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17254579315488500390;
        }
        19 => {
            current_block_42 = 17254579315488500390;
        }
        18 => {
            current_block_42 = 16864574447450575806;
        }
        17 => {
            current_block_42 = 4745558310823793980;
        }
        16 => {
            current_block_42 = 3609958504996442327;
        }
        15 => {
            current_block_42 = 7591989803176764182;
        }
        14 => {
            current_block_42 = 18018901114339951972;
        }
        13 => {
            current_block_42 = 12689878998560279573;
        }
        12 => {
            current_block_42 = 5286769970458876423;
        }
        11 => {
            current_block_42 = 7754037464575168929;
        }
        10 => {
            current_block_42 = 3450148331727835525;
        }
        9 => {
            current_block_42 = 17513148302838498461;
        }
        8 => {
            current_block_42 = 5033545425852514390;
        }
        7 => {
            current_block_42 = 7822129706622160761;
        }
        6 => {
            current_block_42 = 14314526103220412958;
        }
        5 => {
            current_block_42 = 11474465627670557597;
        }
        4 => {
            current_block_42 = 13771482776398185125;
        }
        3 => {
            current_block_42 = 519160363748337264;
        }
        2 => {
            current_block_42 = 17126217794288990979;
        }
        1 => {
            current_block_42 = 11647970138234258413;
        }
        0 => {
            current_block_42 = 1836292691772056875;
        }
        _ => {
            let mut t: zahl_char_t = 0;
            asm!(
                "\n    shlq $3, {3}\n    addq {1}, {3}\n 1:\n    movq 0({2}), {0}\n    movq {0}, 0({1})\n    movq 8({2}), {0}\n    movq {0}, 8({1})\n    movq 16({2}), {0}\n    movq {0}, 16({1})\n    movq 24({2}), {0}\n    movq {0}, 24({1})\n    addq $32, {2}\n    addq $32, {1}\n    cmpq {3}, {1}\n    jl 1b",
                lateout(reg) t, inlateout(reg) d, inlateout(reg) s, inlateout(reg) n,
                options(preserves_flags, att_syntax)
            );
            current_block_42 = 1836292691772056875;
        }
    }
    match current_block_42 {
        17254579315488500390 => {
            *d
                .offset(
                    (19 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((19 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 16864574447450575806;
        }
        _ => {}
    }
    match current_block_42 {
        16864574447450575806 => {
            *d
                .offset(
                    (18 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((18 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 4745558310823793980;
        }
        _ => {}
    }
    match current_block_42 {
        4745558310823793980 => {
            *d
                .offset(
                    (17 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((17 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 3609958504996442327;
        }
        _ => {}
    }
    match current_block_42 {
        3609958504996442327 => {
            *d
                .offset(
                    (16 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((16 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7591989803176764182;
        }
        _ => {}
    }
    match current_block_42 {
        7591989803176764182 => {
            *d
                .offset(
                    (15 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((15 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 18018901114339951972;
        }
        _ => {}
    }
    match current_block_42 {
        18018901114339951972 => {
            *d
                .offset(
                    (14 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((14 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 12689878998560279573;
        }
        _ => {}
    }
    match current_block_42 {
        12689878998560279573 => {
            *d
                .offset(
                    (13 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((13 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 5286769970458876423;
        }
        _ => {}
    }
    match current_block_42 {
        5286769970458876423 => {
            *d
                .offset(
                    (12 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((12 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7754037464575168929;
        }
        _ => {}
    }
    match current_block_42 {
        7754037464575168929 => {
            *d
                .offset(
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 3450148331727835525;
        }
        _ => {}
    }
    match current_block_42 {
        3450148331727835525 => {
            *d
                .offset(
                    (10 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((10 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17513148302838498461;
        }
        _ => {}
    }
    match current_block_42 {
        17513148302838498461 => {
            *d
                .offset(
                    (9 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((9 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 5033545425852514390;
        }
        _ => {}
    }
    match current_block_42 {
        5033545425852514390 => {
            *d
                .offset(
                    (8 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((8 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7822129706622160761;
        }
        _ => {}
    }
    match current_block_42 {
        7822129706622160761 => {
            *d
                .offset(
                    (7 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((7 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 14314526103220412958;
        }
        _ => {}
    }
    match current_block_42 {
        14314526103220412958 => {
            *d
                .offset(
                    (6 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((6 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11474465627670557597;
        }
        _ => {}
    }
    match current_block_42 {
        11474465627670557597 => {
            *d
                .offset(
                    (5 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 13771482776398185125;
        }
        _ => {}
    }
    match current_block_42 {
        13771482776398185125 => {
            *d
                .offset(
                    (4 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((4 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 519160363748337264;
        }
        _ => {}
    }
    match current_block_42 {
        519160363748337264 => {
            *d
                .offset(
                    (3 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((3 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17126217794288990979;
        }
        _ => {}
    }
    match current_block_42 {
        17126217794288990979 => {
            *d
                .offset(
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11647970138234258413;
        }
        _ => {}
    }
    match current_block_42 {
        11647970138234258413 => {
            *d
                .offset(
                    (1 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((1 as libc::c_int - 1 as libc::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn libzahl_memmovef(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    if n != 0 && n < 4 as libc::c_int as libc::c_ulong {
        *d.offset(0 as libc::c_int as isize) = *s.offset(0 as libc::c_int as isize);
        *d.offset(1 as libc::c_int as isize) = *s.offset(1 as libc::c_int as isize);
        *d.offset(2 as libc::c_int as isize) = *s.offset(2 as libc::c_int as isize);
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < n {
            *d
                .offset(
                    i.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *s
                .offset(i.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize);
            *d
                .offset(
                    i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *s
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            *d
                .offset(
                    i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *s
                .offset(i.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize);
            *d
                .offset(
                    i.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                ) = *s
                .offset(i.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize);
            i = (i as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
    };
}
#[inline]
unsafe extern "C" fn libzahl_memmoveb(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut i: ssize_t = 0;
    let mut current_block_47: u64;
    match n {
        20 => {
            *d
                .offset(
                    (20 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((20 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 3708020343659147998;
        }
        19 => {
            current_block_47 = 3708020343659147998;
        }
        18 => {
            current_block_47 = 10597256872703347551;
        }
        17 => {
            current_block_47 = 16368336330394982322;
        }
        16 => {
            current_block_47 = 9630013451124115526;
        }
        15 => {
            current_block_47 = 14283634218784605551;
        }
        14 => {
            current_block_47 = 1513176906760517186;
        }
        13 => {
            current_block_47 = 8451468998463349627;
        }
        12 => {
            current_block_47 = 9944861384781218757;
        }
        11 => {
            current_block_47 = 11763560597460547600;
        }
        10 => {
            current_block_47 = 15461180760796017574;
        }
        9 => {
            current_block_47 = 1083807695818635084;
        }
        8 => {
            current_block_47 = 10234853314349311949;
        }
        7 => {
            current_block_47 = 4192008369500975849;
        }
        6 => {
            current_block_47 = 302541485824227200;
        }
        5 => {
            current_block_47 = 17931195545865348945;
        }
        4 => {
            current_block_47 = 12533541166356859325;
        }
        3 => {
            current_block_47 = 14597459921621826422;
        }
        2 => {
            current_block_47 = 2597896670383610064;
        }
        1 => {
            current_block_47 = 14613590046668411832;
        }
        0 => {
            current_block_47 = 7226443171521532240;
        }
        _ => {
            i = n as ssize_t + 3 as libc::c_int as libc::c_long
                & !(3 as libc::c_int) as libc::c_long;
            loop {
                i -= 4 as libc::c_int as libc::c_long;
                if !(i >= 0 as libc::c_int as libc::c_long) {
                    break;
                }
                *d
                    .offset(
                        (i + 3 as libc::c_int as libc::c_long) as isize,
                    ) = *s.offset((i + 3 as libc::c_int as libc::c_long) as isize);
                *d
                    .offset(
                        (i + 2 as libc::c_int as libc::c_long) as isize,
                    ) = *s.offset((i + 2 as libc::c_int as libc::c_long) as isize);
                *d
                    .offset(
                        (i + 1 as libc::c_int as libc::c_long) as isize,
                    ) = *s.offset((i + 1 as libc::c_int as libc::c_long) as isize);
                *d
                    .offset(
                        (i + 0 as libc::c_int as libc::c_long) as isize,
                    ) = *s.offset((i + 0 as libc::c_int as libc::c_long) as isize);
            }
            current_block_47 = 7226443171521532240;
        }
    }
    match current_block_47 {
        3708020343659147998 => {
            *d
                .offset(
                    (19 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((19 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 10597256872703347551;
        }
        _ => {}
    }
    match current_block_47 {
        10597256872703347551 => {
            *d
                .offset(
                    (18 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((18 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 16368336330394982322;
        }
        _ => {}
    }
    match current_block_47 {
        16368336330394982322 => {
            *d
                .offset(
                    (17 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((17 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 9630013451124115526;
        }
        _ => {}
    }
    match current_block_47 {
        9630013451124115526 => {
            *d
                .offset(
                    (16 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((16 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 14283634218784605551;
        }
        _ => {}
    }
    match current_block_47 {
        14283634218784605551 => {
            *d
                .offset(
                    (15 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((15 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 1513176906760517186;
        }
        _ => {}
    }
    match current_block_47 {
        1513176906760517186 => {
            *d
                .offset(
                    (14 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((14 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 8451468998463349627;
        }
        _ => {}
    }
    match current_block_47 {
        8451468998463349627 => {
            *d
                .offset(
                    (13 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((13 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 9944861384781218757;
        }
        _ => {}
    }
    match current_block_47 {
        9944861384781218757 => {
            *d
                .offset(
                    (12 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((12 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 11763560597460547600;
        }
        _ => {}
    }
    match current_block_47 {
        11763560597460547600 => {
            *d
                .offset(
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 15461180760796017574;
        }
        _ => {}
    }
    match current_block_47 {
        15461180760796017574 => {
            *d
                .offset(
                    (10 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((10 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 1083807695818635084;
        }
        _ => {}
    }
    match current_block_47 {
        1083807695818635084 => {
            *d
                .offset(
                    (9 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((9 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 10234853314349311949;
        }
        _ => {}
    }
    match current_block_47 {
        10234853314349311949 => {
            *d
                .offset(
                    (8 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((8 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 4192008369500975849;
        }
        _ => {}
    }
    match current_block_47 {
        4192008369500975849 => {
            *d
                .offset(
                    (7 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((7 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 302541485824227200;
        }
        _ => {}
    }
    match current_block_47 {
        302541485824227200 => {
            *d
                .offset(
                    (6 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((6 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 17931195545865348945;
        }
        _ => {}
    }
    match current_block_47 {
        17931195545865348945 => {
            *d
                .offset(
                    (5 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 12533541166356859325;
        }
        _ => {}
    }
    match current_block_47 {
        12533541166356859325 => {
            *d
                .offset(
                    (4 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((4 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 14597459921621826422;
        }
        _ => {}
    }
    match current_block_47 {
        14597459921621826422 => {
            *d
                .offset(
                    (3 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((3 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 2597896670383610064;
        }
        _ => {}
    }
    match current_block_47 {
        2597896670383610064 => {
            *d
                .offset(
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_47 = 14613590046668411832;
        }
        _ => {}
    }
    match current_block_47 {
        14613590046668411832 => {
            *d
                .offset(
                    (1 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((1 as libc::c_int - 1 as libc::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn libzahl_memmove(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    if d < s as *mut zahl_char_t {
        libzahl_memmovef(d, s, n);
    } else {
        libzahl_memmoveb(d, s, n);
    };
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zset(mut a: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
    } else {
        (*a).sign = (*b).sign;
        (*a).used = (*b).used;
        if (*a).alloced < (*b).used {
            libzahl_realloc(a, (*b).used);
        }
        libzahl_memcpy((*a).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zbits(mut a: *mut zahl) -> size_t {
    let mut rc: size_t = 0;
    if (zzero(a) != 0) as libc::c_int as libc::c_long != 0 {
        return 1 as libc::c_int as size_t;
    }
    while *((*a).chars)
        .offset(((*a).used).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        == 0
    {
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
    }
    rc = ((*a).used)
        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<zahl_char_t>() as libc::c_ulong);
    rc = (rc as libc::c_ulong)
        .wrapping_sub(
            (*((*a).chars)
                .offset(
                    ((*a).used).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_ulonglong)
                .leading_zeros() as i32 as size_t,
        ) as size_t as size_t;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn zrsh(mut a: *mut zahl, mut b: *mut zahl, mut bits: size_t) {
    let mut i: size_t = 0;
    let mut chars: size_t = 0;
    let mut cbits: size_t = 0;
    if (bits == 0) as libc::c_int as libc::c_long != 0 {
        if a != b {
            zset(a, b);
        }
        return;
    }
    chars = bits >> 6 as libc::c_int;
    if (zzero(b) != 0 || chars >= (*b).used || zbits(b) <= bits) as libc::c_int
        as libc::c_long != 0
    {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    bits = bits & (64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    cbits = (64 as libc::c_int as libc::c_ulong).wrapping_sub(bits);
    if (chars != 0) as libc::c_int as libc::c_long != 0
        && (a == b) as libc::c_int as libc::c_long != 0
    {
        (*a).used = ((*a).used as libc::c_ulong).wrapping_sub(chars) as size_t as size_t;
        libzahl_memmove((*a).chars, ((*a).chars).offset(chars as isize), (*a).used);
    } else if (a != b) as libc::c_int as libc::c_long != 0 {
        (*a).used = ((*b).used).wrapping_sub(chars);
        if (*a).alloced < (*a).used {
            libzahl_realloc(a, (*a).used);
        }
        libzahl_memcpy((*a).chars, ((*b).chars).offset(chars as isize), (*a).used);
    }
    if (bits != 0) as libc::c_int as libc::c_long != 0 {
        *((*a).chars).offset(0 as libc::c_int as isize) >>= bits;
        i = 1 as libc::c_int as size_t;
        while i < (*a).used {
            let ref mut fresh0 = *((*a).chars)
                .offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            *fresh0 |= *((*a).chars).offset(i as isize) << cbits;
            *((*a).chars).offset(i as isize) >>= bits;
            i = i.wrapping_add(1);
            i;
        }
        while *((*a).chars)
            .offset(((*a).used).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0
        {
            (*a).used = ((*a).used).wrapping_sub(1);
            (*a).used;
        }
    }
    (*a).sign = zsignum(b);
}
