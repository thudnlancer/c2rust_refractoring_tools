use ::libc;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
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
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> libc::c_int {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zand(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    } else if (zzero(c) != 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    (*a).used = if (*b).used < (*c).used { (*b).used } else { (*c).used };
    if a == b {
        let mut a__: *mut zahl_char_t = (*a).chars;
        let mut b__: *const zahl_char_t = (*a).chars;
        let mut c__: *const zahl_char_t = (*c).chars;
        let mut i__: size_t = 0;
        let mut n__: size_t = (*a).used;
        i__ = 0 as libc::c_int as size_t;
        while i__ < n__ {
            *a__
                .offset(
                    i__.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *b__
                .offset(i__.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize)
                & *c__
                    .offset(
                        i__.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                    );
            *a__
                .offset(
                    i__.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *b__
                .offset(i__.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                & *c__
                    .offset(
                        i__.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            *a__
                .offset(
                    i__.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *b__
                .offset(i__.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                & *c__
                    .offset(
                        i__.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                    );
            *a__
                .offset(
                    i__.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                ) = *b__
                .offset(i__.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize)
                & *c__
                    .offset(
                        i__.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                    );
            i__ = (i__ as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        }
    } else if (a == c) as libc::c_int as libc::c_long != 0 {
        let mut a___0: *mut zahl_char_t = (*a).chars;
        let mut b___0: *const zahl_char_t = (*a).chars;
        let mut c___0: *const zahl_char_t = (*b).chars;
        let mut i___0: size_t = 0;
        let mut n___0: size_t = (*a).used;
        i___0 = 0 as libc::c_int as size_t;
        while i___0 < n___0 {
            *a___0
                .offset(
                    i___0.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *b___0
                .offset(i___0.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize)
                & *c___0
                    .offset(
                        i___0.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                    );
            *a___0
                .offset(
                    i___0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *b___0
                .offset(i___0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                & *c___0
                    .offset(
                        i___0.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            *a___0
                .offset(
                    i___0.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *b___0
                .offset(i___0.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                & *c___0
                    .offset(
                        i___0.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                    );
            *a___0
                .offset(
                    i___0.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                ) = *b___0
                .offset(i___0.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize)
                & *c___0
                    .offset(
                        i___0.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                    );
            i___0 = (i___0 as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
    } else {
        if (*a).alloced < (*a).used {
            libzahl_realloc(a, (*a).used);
        }
        let mut a___1: *mut zahl_char_t = (*a).chars;
        let mut b___1: *const zahl_char_t = (*b).chars;
        let mut c___1: *const zahl_char_t = (*c).chars;
        let mut i___1: size_t = 0;
        let mut n___1: size_t = (*a).used;
        i___1 = 0 as libc::c_int as size_t;
        while i___1 < n___1 {
            *a___1
                .offset(
                    i___1.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                ) = *b___1
                .offset(i___1.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize)
                & *c___1
                    .offset(
                        i___1.wrapping_add(0 as libc::c_int as libc::c_ulong) as isize,
                    );
            *a___1
                .offset(
                    i___1.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *b___1
                .offset(i___1.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                & *c___1
                    .offset(
                        i___1.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
            *a___1
                .offset(
                    i___1.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                ) = *b___1
                .offset(i___1.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize)
                & *c___1
                    .offset(
                        i___1.wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
                    );
            *a___1
                .offset(
                    i___1.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                ) = *b___1
                .offset(i___1.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize)
                & *c___1
                    .offset(
                        i___1.wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                    );
            i___1 = (i___1 as libc::c_ulong)
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
    }
    while (*a).used != 0
        && *((*a).chars)
            .offset(((*a).used).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            == 0
    {
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
    }
    (*a)
        .sign = if (*a).used != 0 {
        ((zsignum(b) > 0 as libc::c_int) as libc::c_int
            + (zsignum(c) > 0 as libc::c_int) as libc::c_int > 0 as libc::c_int)
            as libc::c_int * 2 as libc::c_int - 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
