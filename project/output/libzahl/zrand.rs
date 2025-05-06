#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn zfree(_: *mut zahl);
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn zadd(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsub(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_temp_allocation: *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_error: i32;
    fn __errno_location() -> *mut i32;
    static mut libzahl_const_1: z_t;
    fn lrand48() -> i64;
    fn srand48(__seedval: i64);
    fn random() -> i64;
    fn srandom(__seed: u32);
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type size_t = u64;
pub type __uint64_t = u64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type uint64_t = __uint64_t;
pub type intptr_t = i64;
pub type ssize_t = __ssize_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: i32,
    pub padding__: i32,
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
}
impl zranddev {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            zranddev::FAST_RANDOM => 0,
            zranddev::SECURE_RANDOM => 1,
            zranddev::DEFAULT_RANDOM => 2,
            zranddev::FASTEST_RANDOM => 3,
            zranddev::LIBC_RAND_RANDOM => 4,
            zranddev::LIBC_RANDOM_RANDOM => 5,
            zranddev::LIBC_RAND48_RANDOM => 6,
        }
    }
    fn from_libc_c_uint(value: u32) -> zranddev {
        match value {
            0 => zranddev::FAST_RANDOM,
            1 => zranddev::SECURE_RANDOM,
            2 => zranddev::DEFAULT_RANDOM,
            3 => zranddev::FASTEST_RANDOM,
            4 => zranddev::LIBC_RAND_RANDOM,
            5 => zranddev::LIBC_RANDOM_RANDOM,
            6 => zranddev::LIBC_RAND48_RANDOM,
            _ => panic!("Invalid value for zranddev: {}", value),
        }
    }
}
impl AddAssign<u32> for zranddev {
    fn add_assign(&mut self, rhs: u32) {
        *self = zranddev::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for zranddev {
    fn sub_assign(&mut self, rhs: u32) {
        *self = zranddev::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for zranddev {
    fn mul_assign(&mut self, rhs: u32) {
        *self = zranddev::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for zranddev {
    fn div_assign(&mut self, rhs: u32) {
        *self = zranddev::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for zranddev {
    fn rem_assign(&mut self, rhs: u32) {
        *self = zranddev::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for zranddev {
    type Output = zranddev;
    fn add(self, rhs: u32) -> zranddev {
        zranddev::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for zranddev {
    type Output = zranddev;
    fn sub(self, rhs: u32) -> zranddev {
        zranddev::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for zranddev {
    type Output = zranddev;
    fn mul(self, rhs: u32) -> zranddev {
        zranddev::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for zranddev {
    type Output = zranddev;
    fn div(self, rhs: u32) -> zranddev {
        zranddev::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for zranddev {
    type Output = zranddev;
    fn rem(self, rhs: u32) -> zranddev {
        zranddev::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zranddist {
    QUASIUNIFORM = 0,
    UNIFORM,
    MODUNIFORM,
}
impl zranddist {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            zranddist::QUASIUNIFORM => 0,
            zranddist::UNIFORM => 1,
            zranddist::MODUNIFORM => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> zranddist {
        match value {
            0 => zranddist::QUASIUNIFORM,
            1 => zranddist::UNIFORM,
            2 => zranddist::MODUNIFORM,
            _ => panic!("Invalid value for zranddist: {}", value),
        }
    }
}
impl AddAssign<u32> for zranddist {
    fn add_assign(&mut self, rhs: u32) {
        *self = zranddist::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for zranddist {
    fn sub_assign(&mut self, rhs: u32) {
        *self = zranddist::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for zranddist {
    fn mul_assign(&mut self, rhs: u32) {
        *self = zranddist::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for zranddist {
    fn div_assign(&mut self, rhs: u32) {
        *self = zranddist::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for zranddist {
    fn rem_assign(&mut self, rhs: u32) {
        *self = zranddist::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for zranddist {
    type Output = zranddist;
    fn add(self, rhs: u32) -> zranddist {
        zranddist::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for zranddist {
    type Output = zranddist;
    fn sub(self, rhs: u32) -> zranddist {
        zranddist::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for zranddist {
    type Output = zranddist;
    fn mul(self, rhs: u32) -> zranddist {
        zranddist::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for zranddist {
    type Output = zranddist;
    fn div(self, rhs: u32) -> zranddist {
        zranddist::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for zranddist {
    type Output = zranddist;
    fn rem(self, rhs: u32) -> zranddist {
        zranddist::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zerror {
    ZERROR_ERRNO_SET = 0,
    ZERROR_0_POW_0,
    ZERROR_0_DIV_0,
    ZERROR_DIV_0,
    ZERROR_NEGATIVE,
    ZERROR_INVALID_RADIX,
}
impl zerror {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            zerror::ZERROR_ERRNO_SET => 0,
            zerror::ZERROR_0_POW_0 => 1,
            zerror::ZERROR_0_DIV_0 => 2,
            zerror::ZERROR_DIV_0 => 3,
            zerror::ZERROR_NEGATIVE => 4,
            zerror::ZERROR_INVALID_RADIX => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> zerror {
        match value {
            0 => zerror::ZERROR_ERRNO_SET,
            1 => zerror::ZERROR_0_POW_0,
            2 => zerror::ZERROR_0_DIV_0,
            3 => zerror::ZERROR_DIV_0,
            4 => zerror::ZERROR_NEGATIVE,
            5 => zerror::ZERROR_INVALID_RADIX,
            _ => panic!("Invalid value for zerror: {}", value),
        }
    }
}
impl AddAssign<u32> for zerror {
    fn add_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for zerror {
    fn sub_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for zerror {
    fn mul_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for zerror {
    fn div_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for zerror {
    fn rem_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for zerror {
    type Output = zerror;
    fn add(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for zerror {
    type Output = zerror;
    fn sub(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for zerror {
    type Output = zerror;
    fn mul(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for zerror {
    type Output = zerror;
    fn div(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for zerror {
    type Output = zerror;
    fn rem(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type time_t = __time_t;
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> i32 {
    return ((*a).sign == 0) as i32;
}
#[inline]
unsafe extern "C" fn zcmpmag(mut a: *mut zahl, mut b: *mut zahl) -> i32 {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a) != 0) as i32 as i64 != 0 {
        return -((zzero(b) == 0) as i32);
    }
    if (zzero(b) != 0) as i32 as i64 != 0 {
        return 1 as i32;
    }
    i = ((*a).used).wrapping_sub(1 as i32 as u64);
    j = ((*b).used).wrapping_sub(1 as i32 as u64);
    while i > j {
        if *((*a).chars).offset(i as isize) != 0 {
            return 1 as i32;
        }
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
        i = i.wrapping_sub(1);
        i;
    }
    while j > i {
        if *((*b).chars).offset(j as isize) != 0 {
            return -(1 as i32);
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
        -(1 as i32)
    } else {
        (*((*a).chars).offset(i as isize) > *((*b).chars).offset(i as isize)) as i32
    };
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> i32 {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zmul(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut b_sign: i32 = 0;
    let mut c_sign: i32 = 0;
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
    if (zzero(a) != 0) as i32 as i64 != 0 {
        return 1 as i32 as size_t;
    }
    while *((*a).chars).offset(((*a).used).wrapping_sub(1 as i32 as u64) as isize) == 0 {
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
    }
    rc = ((*a).used)
        .wrapping_mul(8 as i32 as u64)
        .wrapping_mul(::core::mem::size_of::<zahl_char_t>() as u64);
    rc = (rc as u64)
        .wrapping_sub(
            (*((*a).chars).offset(((*a).used).wrapping_sub(1 as i32 as u64) as isize)
                as libc::c_ulonglong)
                .leading_zeros() as i32 as size_t,
        ) as size_t as size_t;
    return rc;
}
unsafe extern "C" fn libzahl_failure(mut error: i32) {
    libzahl_error = error;
    if !libzahl_temp_stack.is_null() {
        while libzahl_temp_stack_head != libzahl_temp_stack {
            libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
            zfree(*libzahl_temp_stack_head);
        }
    }
    free(libzahl_temp_allocation);
    libzahl_temp_allocation = 0 as *mut libc::c_void;
    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as i32);
}
unsafe extern "C" fn zrand_libc_rand(
    mut out: *mut libc::c_void,
    mut n: size_t,
    mut statep: *mut libc::c_void,
) {
    static mut inited: i8 = 0 as i32 as i8;
    let mut ri: u32 = 0;
    let mut rd: libc::c_double = 0.;
    let mut buf: *mut u8 = out as *mut u8;
    if inited == 0 {
        inited = 1 as i32 as i8;
        srand((out as intptr_t | time(0 as *mut time_t)) as u32);
    }
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        ri = rand() as u32;
        rd = ri as libc::c_double
            / (2147483647 as i32 as libc::c_double + 1 as i32 as libc::c_double);
        rd *= (256 as i32 * 256 as i32) as libc::c_double;
        ri = rd as u32;
        *buf.offset(n as isize) = (ri >> 0 as i32 & 255 as i32 as u32) as u8;
        let fresh1 = n;
        n = n.wrapping_sub(1);
        if fresh1 == 0 {
            break;
        }
        *buf.offset(n as isize) = (ri >> 8 as i32 & 255 as i32 as u32) as u8;
    };
}
unsafe extern "C" fn zrand_libc_rand48(
    mut out: *mut libc::c_void,
    mut n: size_t,
    mut statep: *mut libc::c_void,
) {
    static mut inited: i8 = 0 as i32 as i8;
    let mut r0: i64 = 0;
    let mut r1: i64 = 0;
    let mut buf: *mut u8 = out as *mut u8;
    if inited == 0 {
        inited = 1 as i32 as i8;
        srand48(out as intptr_t | time(0 as *mut time_t));
    }
    loop {
        let fresh2 = n;
        n = n.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        r0 = lrand48() & 15 as i32 as i64;
        r1 = lrand48() & 15 as i32 as i64;
        *buf.offset(n as isize) = (r0 << 4 as i32 | r1) as u8;
    };
}
unsafe extern "C" fn zrand_libc_random(
    mut out: *mut libc::c_void,
    mut n: size_t,
    mut statep: *mut libc::c_void,
) {
    static mut inited: i8 = 0 as i32 as i8;
    let mut ri: i64 = 0;
    let mut buf: *mut u8 = out as *mut u8;
    if inited == 0 {
        inited = 1 as i32 as i8;
        srandom((out as intptr_t | time(0 as *mut time_t)) as u32);
    }
    loop {
        let fresh3 = n;
        n = n.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        ri = random();
        *buf.offset(n as isize) = (ri >> 0 as i32 & 255 as i32 as i64) as u8;
        let fresh4 = n;
        n = n.wrapping_sub(1);
        if fresh4 == 0 {
            break;
        }
        *buf.offset(n as isize) = (ri >> 8 as i32 & 255 as i32 as i64) as u8;
        let fresh5 = n;
        n = n.wrapping_sub(1);
        if fresh5 == 0 {
            break;
        }
        *buf.offset(n as isize) = (ri >> 16 as i32 & 255 as i32 as i64) as u8;
    };
}
unsafe extern "C" fn zrand_fd(
    mut out: *mut libc::c_void,
    mut n: size_t,
    mut statep: *mut libc::c_void,
) {
    let mut fd: i32 = *(statep as *mut i32);
    let mut read_just: ssize_t = 0;
    let mut read_total: size_t = 0 as i32 as size_t;
    let mut buf: *mut i8 = out as *mut i8;
    while n != 0 {
        read_just = read(fd, buf.offset(read_total as isize) as *mut libc::c_void, n);
        if (read_just < 0 as i32 as i64) as i32 as i64 != 0 {
            libzahl_failure(*__errno_location());
        }
        read_total = (read_total as u64).wrapping_add(read_just as size_t) as size_t
            as size_t;
        n = (n as u64).wrapping_sub(read_just as size_t) as size_t as size_t;
    }
}
unsafe extern "C" fn zrand_get_random_bits(
    mut r: *mut zahl,
    mut bits: size_t,
    mut fun: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
    >,
    mut statep: *mut libc::c_void,
) {
    let mut n: size_t = 0;
    let mut chars: size_t = bits.wrapping_add((64 as i32 - 1 as i32) as u64) >> 6 as i32;
    let mut mask: zahl_char_t = 1 as i32 as zahl_char_t;
    if (*r).alloced < chars {
        libzahl_realloc(r, chars);
    }
    fun
        .expect(
            "non-null function pointer",
        )(
        (*r).chars as *mut libc::c_void,
        chars.wrapping_mul(::core::mem::size_of::<zahl_char_t>() as u64),
        statep,
    );
    bits = bits & (64 as i32 - 1 as i32) as u64;
    mask <<= bits;
    mask = (mask as u64).wrapping_sub(1 as i32 as u64) as zahl_char_t as zahl_char_t;
    let ref mut fresh6 = *((*r).chars)
        .offset(chars.wrapping_sub(1 as i32 as u64) as isize);
    *fresh6 &= mask;
    n = chars;
    loop {
        let fresh7 = n;
        n = n.wrapping_sub(1);
        if !(fresh7 != 0) {
            break;
        }
        if (*((*r).chars).offset(n as isize) != 0) as i32 as i64 != 0 {
            (*r).used = n.wrapping_add(1 as i32 as u64);
            (*r).sign = 1 as i32;
            return;
        }
    }
    (*r).sign = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn zrand(
    mut r: *mut zahl,
    mut dev: zranddev,
    mut dist: zranddist,
    mut n: *mut zahl,
) {
    let mut pathname: *const i8 = 0 as *const i8;
    let mut bits: size_t = 0;
    let mut fd: i32 = -(1 as i32);
    let mut statep: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut random_fun: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
    > = Some(
        zrand_fd
            as unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void) -> (),
    );
    match dev as u32 {
        0 => {
            pathname = b"/dev/urandom\0" as *const u8 as *const i8;
        }
        1 => {
            pathname = b"/dev/random\0" as *const u8 as *const i8;
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
            libzahl_failure(22 as i32);
        }
    }
    if (zzero(n) != 0) as i32 as i64 != 0 {
        (*r).sign = 0 as i32;
        return;
    }
    if !pathname.is_null() {
        fd = open(pathname, 0 as i32);
        if (fd < 0 as i32) as i32 as i64 != 0 {
            libzahl_failure(*__errno_location());
        }
        statep = &mut fd as *mut i32 as *mut libc::c_void;
    }
    match dist as u32 {
        0 => {
            if (zsignum(n) < 0 as i32) as i32 as i64 != 0 {
                libzahl_failure(-(zerror::ZERROR_NEGATIVE as i32));
            }
            bits = zbits(n);
            loop {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(0 as i32 != 0 && (zcmpmag(r, n) > 0 as i32) as i32 as i64 != 0) {
                    break;
                }
            }
            zadd(r, r, libzahl_const_1.as_mut_ptr());
            zmul(r, r, n);
            zrsh(r, r, bits);
        }
        1 => {
            if (zsignum(n) < 0 as i32) as i32 as i64 != 0 {
                libzahl_failure(-(zerror::ZERROR_NEGATIVE as i32));
            }
            bits = zbits(n);
            loop {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(1 as i32 != 0 && (zcmpmag(r, n) > 0 as i32) as i32 as i64 != 0) {
                    break;
                }
            }
        }
        2 => {
            if (zsignum(n) < 0 as i32) as i32 as i64 != 0 {
                libzahl_failure(-(zerror::ZERROR_NEGATIVE as i32));
            }
            bits = zbits(n);
            loop {
                zrand_get_random_bits(r, bits, random_fun, statep);
                if !(0 as i32 != 0 && (zcmpmag(r, n) > 0 as i32) as i32 as i64 != 0) {
                    break;
                }
            }
            if (zcmpmag(r, n) > 0 as i32) as i32 as i64 != 0 {
                zsub(r, r, n);
            }
        }
        _ => {
            libzahl_failure(22 as i32);
        }
    }
    if fd >= 0 as i32 {
        close(fd);
    }
}