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
    fn zfree(_: *mut zahl);
    fn zadd_unsigned_assign(_: *mut zahl, _: *mut zahl);
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_temp_stack_end: *mut *mut zahl;
    fn __errno_location() -> *mut i32;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_temp_allocation: *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut libzahl_error: i32;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
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
pub type uint64_t = __uint64_t;
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
#[inline]
unsafe extern "C" fn zinit(mut a: *mut zahl) {
    (*a).alloced = 0 as i32 as size_t;
    (*a).chars = 0 as *mut zahl_char_t;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> i32 {
    return ((*a).sign == 0) as i32;
}
#[inline]
unsafe extern "C" fn zfree_temp(mut a: *mut zahl) {
    zfree(a);
    libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
    libzahl_temp_stack_head;
}
#[inline]
unsafe extern "C" fn zinit_temp(mut a: *mut zahl) {
    zinit(a);
    if (libzahl_temp_stack_head == libzahl_temp_stack_end) as i32 as i64 != 0 {
        let mut n: size_t = libzahl_temp_stack_end.offset_from(libzahl_temp_stack) as i64
            as size_t;
        let mut old: *mut libc::c_void = libzahl_temp_stack as *mut libc::c_void;
        libzahl_temp_stack = realloc(
            old,
            (2 as i32 as u64)
                .wrapping_mul(n)
                .wrapping_mul(::core::mem::size_of::<*mut zahl>() as u64),
        ) as *mut *mut zahl;
        if libzahl_temp_stack.is_null() as i32 as i64 != 0 {
            libzahl_temp_stack = old as *mut *mut zahl;
            libzahl_memfailure();
        }
        libzahl_temp_stack_head = libzahl_temp_stack.offset(n as isize);
        libzahl_temp_stack_end = libzahl_temp_stack_head.offset(n as isize);
    }
    let fresh0 = libzahl_temp_stack_head;
    libzahl_temp_stack_head = libzahl_temp_stack_head.offset(1);
    *fresh0 = a;
}
#[inline]
unsafe extern "C" fn libzahl_memfailure() {
    if *__errno_location() == 0 {
        *__errno_location() = 2 as i32;
    }
    libzahl_failure(*__errno_location());
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
#[inline]
unsafe extern "C" fn zsplit_unsigned_fast_large_taint(
    mut high: *mut zahl,
    mut low: *mut zahl,
    mut a: *mut zahl,
    mut n: size_t,
) {
    n >>= 6 as i32;
    (*high).sign = 1 as i32;
    (*high).used = ((*a).used).wrapping_sub(n);
    (*high).chars = ((*a).chars).offset(n as isize);
    (*low).sign = 1 as i32;
    (*low).used = n;
    (*low).chars = (*a).chars;
    while (*low).used != 0
        && *((*low).chars).offset(((*low).used).wrapping_sub(1 as i32 as u64) as isize)
            == 0
    {
        (*low).used = ((*low).used).wrapping_sub(1);
        (*low).used;
    }
    if (*low).used == 0 {
        (*low).sign = 0 as i32;
    }
}
#[inline]
unsafe extern "C" fn zsplit_unsigned_fast_small_auto(
    mut high: *mut zahl,
    mut low: *mut zahl,
    mut a: *mut zahl,
    mut n: size_t,
) {
    let mut mask: zahl_char_t = 1 as i32 as zahl_char_t;
    mask = (mask << n).wrapping_sub(1 as i32 as u64);
    (*high).sign = 1 as i32;
    (*high).used = 1 as i32 as size_t;
    *((*high).chars).offset(0 as i32 as isize) = *((*a).chars).offset(0 as i32 as isize)
        >> n;
    if (*a).used == 2 as i32 as u64 {
        *((*high).chars).offset(1 as i32 as isize) = *((*a).chars)
            .offset(1 as i32 as isize) >> n;
        (*high).used = ((*high).used as u64)
            .wrapping_add(
                (*((*high).chars).offset(1 as i32 as isize) != 0) as i32 as u64,
            ) as size_t as size_t;
        n = (64 as i32 as u64).wrapping_sub(n);
        let ref mut fresh1 = *((*high).chars).offset(0 as i32 as isize);
        *fresh1 |= (*((*a).chars).offset(1 as i32 as isize) & mask) << n;
    }
    (*low).sign = 1 as i32;
    (*low).used = 1 as i32 as size_t;
    *((*low).chars).offset(0 as i32 as isize) = *((*a).chars).offset(0 as i32 as isize)
        & mask;
    if (*((*low).chars).offset(0 as i32 as isize) == 0) as i32 as i64 != 0 {
        (*low).sign = 0 as i32;
    }
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
#[inline]
unsafe extern "C" fn zsqr_ll_single_char(mut a: *mut zahl, mut b: *mut zahl) {
    if (*a).alloced < 1 as i32 as u64 {
        libzahl_realloc(a, 1 as i32 as size_t);
    }
    (*a).used = 1 as i32 as size_t;
    *((*a).chars).offset(0 as i32 as isize) = (*((*b).chars).offset(0 as i32 as isize))
        .wrapping_mul(*((*b).chars).offset(0 as i32 as isize));
    (*a).sign = 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn zsqr_ll(mut a: *mut zahl, mut b: *mut zahl) {
    let mut z0: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut z1: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut high: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut low: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *mut zahl_char_t,
    }; 1];
    let mut auxchars: [zahl_char_t; 12] = [0; 12];
    let mut bits: size_t = 0;
    bits = zbits(b);
    if bits <= (64 as i32 / 2 as i32) as u64 {
        zsqr_ll_single_char(a, b);
        return;
    }
    bits >>= 1 as i32;
    if bits < 64 as i32 as u64 {
        let ref mut fresh2 = (*low.as_mut_ptr()).chars;
        *fresh2 = auxchars.as_mut_ptr();
        let ref mut fresh3 = (*high.as_mut_ptr()).chars;
        *fresh3 = auxchars.as_mut_ptr().offset(4 as i32 as isize);
        zsplit_unsigned_fast_small_auto(high.as_mut_ptr(), low.as_mut_ptr(), b, bits);
    } else {
        bits = bits & !((64 as i32 - 1 as i32) as size_t);
        zsplit_unsigned_fast_large_taint(high.as_mut_ptr(), low.as_mut_ptr(), b, bits);
    }
    if (zzero(low.as_mut_ptr()) != 0) as i32 as i64 != 0 {
        zsqr_ll(a, high.as_mut_ptr());
        zlsh(a, a, bits << 1 as i32);
    } else {
        zinit_temp(z0.as_mut_ptr());
        zinit_temp(z1.as_mut_ptr());
        zsqr_ll(z0.as_mut_ptr(), low.as_mut_ptr());
        zmul_ll(z1.as_mut_ptr(), low.as_mut_ptr(), high.as_mut_ptr());
        zlsh(z1.as_mut_ptr(), z1.as_mut_ptr(), bits.wrapping_add(1 as i32 as u64));
        zsqr_ll(a, high.as_mut_ptr());
        zlsh(a, a, bits << 1 as i32);
        zadd_unsigned_assign(a, z1.as_mut_ptr());
        zadd_unsigned_assign(a, z0.as_mut_ptr());
        zfree_temp(z1.as_mut_ptr());
        zfree_temp(z0.as_mut_ptr());
    };
}