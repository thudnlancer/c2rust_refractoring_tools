#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn zfree(_: *mut zahl);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn zadd(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsub(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_temp_allocation: *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_error: libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut libzahl_const_1: z_t;
    fn lrand48() -> libc::c_long;
    fn srand48(__seedval: libc::c_long);
    fn random() -> libc::c_long;
    fn srandom(__seed: libc::c_uint);
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
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
pub type z_t = [zahl; 1];
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zranddev {
    FAST_RANDOM = 0,
    SECURE_RANDOM,
    DEFAULT_RANDOM,
    FASTEST_RANDOM,
    LIBC_RAND_RANDOM,
    LIBC_RANDOM_RANDOM,
    LIBC_RAND48_RANDOM,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zranddist {
    QUASIUNIFORM = 0,
    UNIFORM,
    MODUNIFORM,
}  // end of enum

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zerror {
    ZERROR_ERRNO_SET = 0,
    ZERROR_0_POW_0,
    ZERROR_0_DIV_0,
    ZERROR_DIV_0,
    ZERROR_NEGATIVE,
    ZERROR_INVALID_RADIX,
}  // end of enum

pub type time_t = __time_t;
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zcmpmag(mut a: *mut zahl, mut b: *mut zahl) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a) != 0) as libc::c_int as libc::c_long != 0 {
        return -((zzero(b) == 0) as libc::c_int);
    }
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        return 1 as libc::c_int;
    }
    i = ((*a).used).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    j = ((*b).used).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i > j {
        if *((*a).chars).offset(i as isize) != 0 {
            return 1 as libc::c_int;
        }
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
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
    while i != 0 && *((*a).chars).offset(i as isize) == *((*b).chars).offset(i as isize)
    {
        i = i.wrapping_sub(1);
        i;
    }
    return if *((*a).chars).offset(i as isize) < *((*b).chars).offset(i as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*a).chars).offset(i as isize) > *((*b).chars).offset(i as isize))
            as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zmul(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut b_sign: libc::c_int = 0;
    let mut c_sign: libc::c_int = 0;
    b_sign = (*b).sign;
    (*b).sign *= b_sign;
    c_sign = (*c).sign;
    (*c).sign *= c_sign;
    zmul_ll(a, b, c);
    (*c).sign = c_sign;
    (*b).sign = b_sign;
    (*a).sign = zsignum(b) * zsignum(c);
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
unsafe extern "C" fn libzahl_failure(mut error: libc::c_int) {
    libzahl_error = error;
    if !libzahl_temp_stack.is_null() {
        while libzahl_temp_stack_head != libzahl_temp_stack {
            libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
            zfree(*libzahl_temp_stack_head);
        }
    }
    free(libzahl_temp_allocation);
    libzahl_temp_allocation = 0 as *mut libc::c_void;
    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
}
unsafe extern "C" fn zrand_libc_rand(
    mut out: *mut libc::c_void,
    mut n: size_t,
    mut statep: *mut libc::c_void,
) {
    static mut inited: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut ri: libc::c_uint = 0;
    let mut rd: libc::c_double = 0.;
    let mut buf: *mut libc::c_uchar = out as *mut libc::c_uchar;
    if inited == 0 {
        inited = 1 as libc::c_int as libc::c_char;
        srand((out as intptr_t | time(0 as *mut time_t)) as libc::c_uint);
    }
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        ri = rand() as libc::c_uint;
        rd = ri as libc::c_double
            / (2147483647 as libc::c_int as libc::c_double
                + 1 as libc::c_int as libc::c_double);
        rd *= (256 as libc::c_int * 256 as libc::c_int) as libc::c_double;
        ri = rd as libc::c_uint;
        *buf
            .offset(
                n as isize,
            ) = (ri >> 0 as libc::c_int & 255 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        let fresh1 = n;
        n = n.wrapping_sub(1);
        if fresh1 == 0 {
            break;
        }
        *buf
            .offset(
                n as isize,
            ) = (ri >> 8 as libc::c_int & 255 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
    };
}
unsafe extern "C" fn zrand_libc_rand48(
    mut out: *mut libc::c_void,
    mut n: size_t,
    mut statep: *mut libc::c_void,
) {
    static mut inited: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut r0: libc::c_long = 0;
    let mut r1: libc::c_long = 0;
    let mut buf: *mut libc::c_uchar = out as *mut libc::c_uchar;
    if inited == 0 {
        inited = 1 as libc::c_int as libc::c_char;
        srand48(out as intptr_t | time(0 as *mut time_t));
    }
    loop {
        let fresh2 = n;
        n = n.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        r0 = lrand48() & 15 as libc::c_int as libc::c_long;
        r1 = lrand48() & 15 as libc::c_int as libc::c_long;
        *buf.offset(n as isize) = (r0 << 4 as libc::c_int | r1) as libc::c_uchar;
    };
}
unsafe extern "C" fn zrand_libc_random(
    mut out: *mut libc::c_void,
    mut n: size_t,
    mut statep: *mut libc::c_void,
) {
    static mut inited: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut ri: libc::c_long = 0;
    let mut buf: *mut libc::c_uchar = out as *mut libc::c_uchar;
    if inited == 0 {
        inited = 1 as libc::c_int as libc::c_char;
        srandom((out as intptr_t | time(0 as *mut time_t)) as libc::c_uint);
    }
    loop {
        let fresh3 = n;
        n = n.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        ri = random();
        *buf
            .offset(
                n as isize,
            ) = (ri >> 0 as libc::c_int & 255 as libc::c_int as libc::c_long)
            as libc::c_uchar;
        let fresh4 = n;
        n = n.wrapping_sub(1);
        if fresh4 == 0 {
            break;
        }
        *buf
            .offset(
                n as isize,
            ) = (ri >> 8 as libc::c_int & 255 as libc::c_int as libc::c_long)
            as libc::c_uchar;
        let fresh5 = n;
        n = n.wrapping_sub(1);
        if fresh5 == 0 {
            break;
        }
        *buf
            .offset(
                n as isize,
            ) = (ri >> 16 as libc::c_int & 255 as libc::c_int as libc::c_long)
            as libc::c_uchar;
    };
}
unsafe extern "C" fn zrand_fd(
    mut out: *mut libc::c_void,
    mut n: size_t,
    mut statep: *mut libc::c_void,
) {
    let mut fd: libc::c_int = *(statep as *mut libc::c_int);
    let mut read_just: ssize_t = 0;
    let mut read_total: size_t = 0 as libc::c_int as size_t;
    let mut buf: *mut libc::c_char = out as *mut libc::c_char;
    while n != 0 {
        read_just = read(fd, buf.offset(read_total as isize) as *mut libc::c_void, n);
        if (read_just < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long
            != 0
        {
            libzahl_failure(*__errno_location());
        }
        read_total = (read_total as libc::c_ulong).wrapping_add(read_just as size_t)
            as size_t as size_t;
        n = (n as libc::c_ulong).wrapping_sub(read_just as size_t) as size_t as size_t;
    }
}
unsafe extern "C" fn zrand_get_random_bits(
    mut r: *mut zahl,
    mut bits: size_t,
    mut fun: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
    >,
    mut statep: *mut libc::c_void,
) {
    let mut n: size_t = 0;
    let mut chars: size_t = bits
        .wrapping_add((64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
        >> 6 as libc::c_int;
    let mut mask: zahl_char_t = 1 as libc::c_int as zahl_char_t;
    if (*r).alloced < chars {
        libzahl_realloc(r, chars);
    }
    fun
        .expect(
            "non-null function pointer",
        )(
        (*r).chars as *mut libc::c_void,
        chars.wrapping_mul(::core::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        statep,
    );
    bits = bits & (64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    mask <<= bits;
    mask = (mask as libc::c_ulong).wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as zahl_char_t as zahl_char_t;
    let ref mut fresh6 = *((*r).chars)
        .offset(chars.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    *fresh6 &= mask;
    n = chars;
    loop {
        let fresh7 = n;
        n = n.wrapping_sub(1);
        if !(fresh7 != 0) {
            break;
        }
        if (*((*r).chars).offset(n as isize) != 0) as libc::c_int as libc::c_long != 0 {
            (*r).used = n.wrapping_add(1 as libc::c_int as libc::c_ulong);
            (*r).sign = 1 as libc::c_int;
            return;
        }
    }
    (*r).sign = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zrand(
    mut r: *mut zahl,
    mut dev: zranddev,
    mut dist: zranddist,
    mut n: *mut zahl,
) {
    let mut pathname: *const libc::c_char = 0 as *const libc::c_char;
    let mut bits: size_t = 0;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut statep: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut random_fun: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
    > = Some(
        zrand_fd
            as unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
    );
    match dev as libc::c_uint {
        0 => {
            pathname = b"/dev/urandom\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            pathname = b"/dev/random\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            random_fun = Some(
                zrand_libc_rand
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut libc::c_void,
                    ) -> (),
            );
        }
        2 | 3 | 5 => {
            random_fun = Some(
                zrand_libc_random
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut libc::c_void,
                    ) -> (),
            );
        }
        6 => {
            random_fun = Some(
                zrand_libc_rand48
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                        *mut libc::c_void,
                    ) -> (),
            );
        }
        _ => {
            libzahl_failure(22 as libc::c_int);
        }
    }
    if (zzero(n) != 0) as libc::c_int as libc::c_long != 0 {
        (*r).sign = 0 as libc::c_int;
        return;
    }
    if !pathname.is_null() {
        fd = open(pathname, 0 as libc::c_int);
        if (fd < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
            libzahl_failure(*__errno_location());
        }
        statep = &mut fd as *mut libc::c_int as *mut libc::c_void;
    }
    match dist as libc::c_uint {
        0 => {
            if (zsignum(n) < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
                libzahl_failure(-(ZERROR_NEGATIVE as libc::c_int));
            }
            bits = zbits(n);
            loop {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(0 as libc::c_int != 0
                    && (zcmpmag(r, n) > 0 as libc::c_int) as libc::c_int as libc::c_long
                        != 0)
                {
                    break;
                }
            }
            zadd(r, r, libzahl_const_1.as_mut_ptr());
            zmul(r, r, n);
            zrsh(r, r, bits);
        }
        1 => {
            if (zsignum(n) < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
                libzahl_failure(-(ZERROR_NEGATIVE as libc::c_int));
            }
            bits = zbits(n);
            loop {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(1 as libc::c_int != 0
                    && (zcmpmag(r, n) > 0 as libc::c_int) as libc::c_int as libc::c_long
                        != 0)
                {
                    break;
                }
            }
        }
        2 => {
            if (zsignum(n) < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
                libzahl_failure(-(ZERROR_NEGATIVE as libc::c_int));
            }
            bits = zbits(n);
            loop {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(0 as libc::c_int != 0
                    && (zcmpmag(r, n) > 0 as libc::c_int) as libc::c_int as libc::c_long
                        != 0)
                {
                    break;
                }
            }
            if (zcmpmag(r, n) > 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
                zsub(r, r, n);
            }
        }
        _ => {
            libzahl_failure(22 as libc::c_int);
        }
    }
    if fd >= 0 as libc::c_int {
        close(fd);
    }
}
