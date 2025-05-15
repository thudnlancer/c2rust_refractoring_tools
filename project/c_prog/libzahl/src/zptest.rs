use ::libc;
use core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zmodsqr(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zmodpow(_: *mut zahl, _: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zadd_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsub_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    static mut libzahl_tmp_ptest_a: z_t;
    static mut libzahl_tmp_ptest_n1: z_t;
    static mut libzahl_tmp_ptest_x: z_t;
    static mut libzahl_const_1: z_t;
    static mut libzahl_tmp_ptest_d: z_t;
    static mut libzahl_const_2: z_t;
    static mut libzahl_tmp_ptest_n4: z_t;
    fn zrand(_: *mut zahl, _: zranddev, _: zranddist, _: *mut zahl);
    static mut libzahl_const_4: z_t;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
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
pub type z_t = [zahl; 1];
pub type zprimality = libc::c_uint;
pub const PRIME: zprimality = 2;
pub const PROBABLY_PRIME: zprimality = 1;
pub const NONPRIME: zprimality = 0;
pub type zranddev = libc::c_uint;
pub const LIBC_RAND48_RANDOM: zranddev = 6;
pub const LIBC_RANDOM_RANDOM: zranddev = 5;
pub const LIBC_RAND_RANDOM: zranddev = 4;
pub const FASTEST_RANDOM: zranddev = 3;
pub const DEFAULT_RANDOM: zranddev = 2;
pub const SECURE_RANDOM: zranddev = 1;
pub const FAST_RANDOM: zranddev = 0;
pub type zranddist = libc::c_uint;
pub const MODUNIFORM: zranddist = 2;
pub const UNIFORM: zranddist = 1;
pub const QUASIUNIFORM: zranddist = 0;
#[inline]
unsafe extern "C" fn libzahl_memcpy(
    mut d_0: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d_0
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
                lateout(reg) t, inlateout(reg) d_0, inlateout(reg) s, inlateout(reg) n,
                options(preserves_flags, att_syntax)
            );
            current_block_42 = 1836292691772056875;
        }
    }
    match current_block_42 {
        17254579315488500390 => {
            *d_0
                .offset(
                    (19 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((19 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 16864574447450575806;
        }
        _ => {}
    }
    match current_block_42 {
        16864574447450575806 => {
            *d_0
                .offset(
                    (18 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((18 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 4745558310823793980;
        }
        _ => {}
    }
    match current_block_42 {
        4745558310823793980 => {
            *d_0
                .offset(
                    (17 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((17 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 3609958504996442327;
        }
        _ => {}
    }
    match current_block_42 {
        3609958504996442327 => {
            *d_0
                .offset(
                    (16 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((16 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7591989803176764182;
        }
        _ => {}
    }
    match current_block_42 {
        7591989803176764182 => {
            *d_0
                .offset(
                    (15 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((15 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 18018901114339951972;
        }
        _ => {}
    }
    match current_block_42 {
        18018901114339951972 => {
            *d_0
                .offset(
                    (14 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((14 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 12689878998560279573;
        }
        _ => {}
    }
    match current_block_42 {
        12689878998560279573 => {
            *d_0
                .offset(
                    (13 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((13 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 5286769970458876423;
        }
        _ => {}
    }
    match current_block_42 {
        5286769970458876423 => {
            *d_0
                .offset(
                    (12 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((12 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7754037464575168929;
        }
        _ => {}
    }
    match current_block_42 {
        7754037464575168929 => {
            *d_0
                .offset(
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 3450148331727835525;
        }
        _ => {}
    }
    match current_block_42 {
        3450148331727835525 => {
            *d_0
                .offset(
                    (10 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((10 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17513148302838498461;
        }
        _ => {}
    }
    match current_block_42 {
        17513148302838498461 => {
            *d_0
                .offset(
                    (9 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((9 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 5033545425852514390;
        }
        _ => {}
    }
    match current_block_42 {
        5033545425852514390 => {
            *d_0
                .offset(
                    (8 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((8 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7822129706622160761;
        }
        _ => {}
    }
    match current_block_42 {
        7822129706622160761 => {
            *d_0
                .offset(
                    (7 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((7 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 14314526103220412958;
        }
        _ => {}
    }
    match current_block_42 {
        14314526103220412958 => {
            *d_0
                .offset(
                    (6 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((6 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11474465627670557597;
        }
        _ => {}
    }
    match current_block_42 {
        11474465627670557597 => {
            *d_0
                .offset(
                    (5 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 13771482776398185125;
        }
        _ => {}
    }
    match current_block_42 {
        13771482776398185125 => {
            *d_0
                .offset(
                    (4 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((4 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 519160363748337264;
        }
        _ => {}
    }
    match current_block_42 {
        519160363748337264 => {
            *d_0
                .offset(
                    (3 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((3 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17126217794288990979;
        }
        _ => {}
    }
    match current_block_42 {
        17126217794288990979 => {
            *d_0
                .offset(
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11647970138234258413;
        }
        _ => {}
    }
    match current_block_42 {
        11647970138234258413 => {
            *d_0
                .offset(
                    (1 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((1 as libc::c_int - 1 as libc::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zswap(mut a_: *mut zahl, mut b_: *mut zahl) {
    let mut t: libc::c_long = 0;
    let mut a_0: *mut libc::c_long = a_ as *mut libc::c_long;
    let mut b: *mut libc::c_long = b_ as *mut libc::c_long;
    t = *a_0.offset(0 as libc::c_int as isize);
    *a_0.offset(0 as libc::c_int as isize) = *b.offset(0 as libc::c_int as isize);
    *b.offset(0 as libc::c_int as isize) = t;
    t = *b.offset(1 as libc::c_int as isize);
    *b.offset(1 as libc::c_int as isize) = *a_0.offset(1 as libc::c_int as isize);
    *a_0.offset(1 as libc::c_int as isize) = t;
    t = *a_0.offset(2 as libc::c_int as isize);
    *a_0.offset(2 as libc::c_int as isize) = *b.offset(2 as libc::c_int as isize);
    *b.offset(2 as libc::c_int as isize) = t;
    t = *b.offset(3 as libc::c_int as isize);
    *b.offset(3 as libc::c_int as isize) = *a_0.offset(3 as libc::c_int as isize);
    *a_0.offset(3 as libc::c_int as isize) = t;
}
#[inline]
unsafe extern "C" fn zzero(mut a_0: *mut zahl) -> libc::c_int {
    return ((*a_0).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zset(mut a_0: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        (*a_0).sign = 0 as libc::c_int;
    } else {
        (*a_0).sign = (*b).sign;
        (*a_0).used = (*b).used;
        if (*a_0).alloced < (*b).used {
            libzahl_realloc(a_0, (*b).used);
        }
        libzahl_memcpy((*a_0).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zsetu(mut a_0: *mut zahl, mut b: uint64_t) {
    if (b == 0) as libc::c_int as libc::c_long != 0 {
        (*a_0).sign = 0 as libc::c_int;
        return;
    }
    if (*a_0).alloced < 1 as libc::c_int as libc::c_ulong {
        libzahl_realloc(a_0, 1 as libc::c_int as size_t);
    }
    (*a_0).sign = 1 as libc::c_int;
    *((*a_0).chars).offset(0 as libc::c_int as isize) = b;
    (*a_0).used = 1 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn zcmp(mut a_0: *mut zahl, mut b: *mut zahl) -> libc::c_int {
    if zsignum(a_0) != zsignum(b) {
        return if zsignum(a_0) < zsignum(b) {
            -(1 as libc::c_int)
        } else {
            (zsignum(a_0) > zsignum(b)) as libc::c_int
        };
    }
    return zsignum(a_0) * zcmpmag(a_0, b);
}
#[inline]
unsafe extern "C" fn zcmpmag(mut a_0: *mut zahl, mut b: *mut zahl) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a_0) != 0) as libc::c_int as libc::c_long != 0 {
        return -((zzero(b) == 0) as libc::c_int);
    }
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        return 1 as libc::c_int;
    }
    i = ((*a_0).used).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    j = ((*b).used).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i > j {
        if *((*a_0).chars).offset(i as isize) != 0 {
            return 1 as libc::c_int;
        }
        (*a_0).used = ((*a_0).used).wrapping_sub(1);
        (*a_0).used;
        i = i.wrapping_sub(1);
        i;
    }
    while j > i {
        if *((*b).chars).offset(j as isize) != 0 {
            return -(1 as libc::c_int);
        }
        (*b).used = ((*b).used).wrapping_sub(1);
        (*b).used;
        j = j.wrapping_sub(1);
        j;
    }
    while i != 0
        && *((*a_0).chars).offset(i as isize) == *((*b).chars).offset(i as isize)
    {
        i = i.wrapping_sub(1);
        i;
    }
    return if *((*a_0).chars).offset(i as isize) < *((*b).chars).offset(i as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*a_0).chars).offset(i as isize) > *((*b).chars).offset(i as isize))
            as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn zsignum(mut a_0: *mut zahl) -> libc::c_int {
    return (*a_0).sign;
}
#[inline]
unsafe extern "C" fn zcmpu(mut a_0: *mut zahl, mut b: uint64_t) -> libc::c_int {
    if (b == 0) as libc::c_int as libc::c_long != 0 {
        return zsignum(a_0);
    }
    if (zsignum(a_0) <= 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int);
    }
    while *((*a_0).chars)
        .offset(((*a_0).used).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        == 0
    {
        (*a_0).used = ((*a_0).used).wrapping_sub(1);
        (*a_0).used;
    }
    if (*a_0).used > 1 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    return if *((*a_0).chars).offset(0 as libc::c_int as isize) < b {
        -(1 as libc::c_int)
    } else {
        (*((*a_0).chars).offset(0 as libc::c_int as isize) > b) as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn zlsb(mut a_0: *mut zahl) -> size_t {
    let mut i: size_t = 0 as libc::c_int as size_t;
    if (zzero(a_0) != 0) as libc::c_int as libc::c_long != 0 {
        return 18446744073709551615 as libc::c_ulong;
    }
    while *((*a_0).chars).offset(i as isize) == 0 {
        i = i.wrapping_add(1);
        i;
    }
    i = (i as libc::c_ulong)
        .wrapping_mul(
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        ) as size_t as size_t;
    i = (i as libc::c_ulong)
        .wrapping_add(
            (*((*a_0).chars).offset(i as isize) as libc::c_ulonglong).trailing_zeros()
                as i32 as size_t,
        ) as size_t as size_t;
    return i;
}
#[inline]
unsafe extern "C" fn zeven(mut a_0: *mut zahl) -> libc::c_int {
    return ((*a_0).sign == 0
        || !*((*a_0).chars).offset(0 as libc::c_int as isize)
            & 1 as libc::c_int as libc::c_ulong != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zptest(
    mut witness: *mut zahl,
    mut n: *mut zahl,
    mut t: libc::c_int,
) -> zprimality {
    let mut i: size_t = 0;
    let mut r: size_t = 0;
    if (zcmpu(n, 3 as libc::c_int as uint64_t) <= 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        if zcmpu(n, 1 as libc::c_int as uint64_t) <= 0 as libc::c_int {
            if !witness.is_null() {
                if witness != n {
                    zset(witness, n);
                }
            }
            return NONPRIME;
        } else {
            return PRIME
        }
    }
    if (zeven(n) != 0) as libc::c_int as libc::c_long != 0 {
        if !witness.is_null() {
            zsetu(witness, 2 as libc::c_int as uint64_t);
        }
        return NONPRIME;
    }
    zsub_unsigned(libzahl_tmp_ptest_n1.as_mut_ptr(), n, libzahl_const_1.as_mut_ptr());
    zsub_unsigned(libzahl_tmp_ptest_n4.as_mut_ptr(), n, libzahl_const_4.as_mut_ptr());
    r = zlsb(libzahl_tmp_ptest_n1.as_mut_ptr());
    zrsh(libzahl_tmp_ptest_d.as_mut_ptr(), libzahl_tmp_ptest_n1.as_mut_ptr(), r);
    loop {
        let fresh0 = t;
        t = t - 1;
        if !(fresh0 != 0) {
            break;
        }
        zrand(
            libzahl_tmp_ptest_a.as_mut_ptr(),
            DEFAULT_RANDOM,
            UNIFORM,
            libzahl_tmp_ptest_n4.as_mut_ptr(),
        );
        zadd_unsigned(
            libzahl_tmp_ptest_a.as_mut_ptr(),
            libzahl_tmp_ptest_a.as_mut_ptr(),
            libzahl_const_2.as_mut_ptr(),
        );
        zmodpow(
            libzahl_tmp_ptest_x.as_mut_ptr(),
            libzahl_tmp_ptest_a.as_mut_ptr(),
            libzahl_tmp_ptest_d.as_mut_ptr(),
            n,
        );
        if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_const_1.as_mut_ptr()) == 0
            || zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_tmp_ptest_n1.as_mut_ptr())
                == 0
        {
            continue;
        }
        i = 1 as libc::c_int as size_t;
        while i < r {
            zmodsqr(
                libzahl_tmp_ptest_x.as_mut_ptr(),
                libzahl_tmp_ptest_x.as_mut_ptr(),
                n,
            );
            if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_const_1.as_mut_ptr()) == 0
            {
                if !witness.is_null() {
                    zswap(witness, libzahl_tmp_ptest_a.as_mut_ptr());
                }
                return NONPRIME;
            }
            if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_tmp_ptest_n1.as_mut_ptr())
                == 0
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        if i == r {
            if !witness.is_null() {
                zswap(witness, libzahl_tmp_ptest_a.as_mut_ptr());
            }
            return NONPRIME;
        }
    }
    return PROBABLY_PRIME;
}
