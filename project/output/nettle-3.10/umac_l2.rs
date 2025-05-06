#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn _nettle_umac_poly64(
        kh: uint32_t,
        kl: uint32_t,
        y: uint64_t,
        m: uint64_t,
    ) -> uint64_t;
    fn _nettle_umac_poly128(
        k: *const uint32_t,
        y: *mut uint64_t,
        mh: uint64_t,
        ml: uint64_t,
    );
}
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C" fn _nettle_umac_l2_init(mut size: u32, mut k: *mut uint32_t) {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while i < size {
        let mut w: uint32_t = *k.offset(i as isize);
        w = w.swap_bytes();
        *k.offset(i as isize) = (w as u64 & 0x1ffffff as u64) as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_umac_l2(
    mut key: *const uint32_t,
    mut state: *mut uint64_t,
    mut n: u32,
    mut count: uint64_t,
    mut m: *const uint64_t,
) {
    let mut prev: *mut uint64_t = state
        .offset((2 as i32 as u32).wrapping_mul(n) as isize);
    let mut i: u32 = 0;
    if count == 0 as i32 as u64 {
        memcpy(
            prev as *mut libc::c_void,
            m as *const libc::c_void,
            (n as u64).wrapping_mul(::core::mem::size_of::<uint64_t>() as u64),
        );
    } else if count == 1 as i32 as u64 {
        i = 0 as i32 as u32;
        while i < n {
            let mut y: uint64_t = _nettle_umac_poly64(
                *key.offset(0 as i32 as isize),
                *key.offset(1 as i32 as isize),
                1 as i32 as uint64_t,
                *prev.offset(i as isize),
            );
            *state
                .offset(
                    (2 as i32 as u32).wrapping_mul(i).wrapping_add(1 as i32 as u32)
                        as isize,
                ) = _nettle_umac_poly64(
                *key.offset(0 as i32 as isize),
                *key.offset(1 as i32 as isize),
                y,
                *m.offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
            key = key.offset(6 as i32 as isize);
        }
    } else if count < 16384 as i32 as u64 {
        i = 0 as i32 as u32;
        while i < n {
            *state
                .offset(
                    (2 as i32 as u32).wrapping_mul(i).wrapping_add(1 as i32 as u32)
                        as isize,
                ) = _nettle_umac_poly64(
                *key.offset(0 as i32 as isize),
                *key.offset(1 as i32 as isize),
                *state
                    .offset(
                        (2 as i32 as u32).wrapping_mul(i).wrapping_add(1 as i32 as u32)
                            as isize,
                    ),
                *m.offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
            key = key.offset(6 as i32 as isize);
        }
    } else if count.wrapping_rem(2 as i32 as u64) == 0 as i32 as u64 {
        if count == 16384 as i32 as u64 {
            i = 0 as i32 as u32;
            key = key.offset(2 as i32 as isize);
            while i < n {
                let mut y_0: uint64_t = *state
                    .offset(
                        (2 as i32 as u32).wrapping_mul(i).wrapping_add(1 as i32 as u32)
                            as isize,
                    );
                if y_0 >= (59 as i32 as uint64_t).wrapping_neg() {
                    y_0 = (y_0 as u64)
                        .wrapping_sub((59 as i32 as uint64_t).wrapping_neg()) as uint64_t
                        as uint64_t;
                }
                *state.offset((2 as i32 as u32).wrapping_mul(i) as isize) = 0 as i32
                    as uint64_t;
                *state
                    .offset(
                        (2 as i32 as u32).wrapping_mul(i).wrapping_add(1 as i32 as u32)
                            as isize,
                    ) = 1 as i32 as uint64_t;
                _nettle_umac_poly128(
                    key,
                    state.offset((2 as i32 as u32).wrapping_mul(i) as isize),
                    0 as i32 as uint64_t,
                    y_0,
                );
                i = i.wrapping_add(1);
                i;
                key = key.offset(6 as i32 as isize);
            }
        }
        memcpy(
            prev as *mut libc::c_void,
            m as *const libc::c_void,
            (n as u64).wrapping_mul(::core::mem::size_of::<uint64_t>() as u64),
        );
    } else {
        i = 0 as i32 as u32;
        key = key.offset(2 as i32 as isize);
        while i < n {
            _nettle_umac_poly128(
                key,
                state.offset((2 as i32 as u32).wrapping_mul(i) as isize),
                *prev.offset(i as isize),
                *m.offset(i as isize),
            );
            i = i.wrapping_add(1);
            i;
            key = key.offset(6 as i32 as isize);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_umac_l2_final(
    mut key: *const uint32_t,
    mut state: *mut uint64_t,
    mut n: u32,
    mut count: uint64_t,
) {
    let mut prev: *mut uint64_t = state
        .offset((2 as i32 as u32).wrapping_mul(n) as isize);
    let mut i: u32 = 0;
    if count > 0 as i32 as u64 {} else {
        __assert_fail(
            b"count > 0\0" as *const u8 as *const i8,
            b"umac-l2.c\0" as *const u8 as *const i8,
            104 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 81],
                &[i8; 81],
            >(
                b"void _nettle_umac_l2_final(const uint32_t *, uint64_t *, unsigned int, uint64_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1516: {
        if count > 0 as i32 as u64 {} else {
            __assert_fail(
                b"count > 0\0" as *const u8 as *const i8,
                b"umac-l2.c\0" as *const u8 as *const i8,
                104 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 81],
                    &[i8; 81],
                >(
                    b"void _nettle_umac_l2_final(const uint32_t *, uint64_t *, unsigned int, uint64_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if count == 1 as i32 as u64 {
        i = 0 as i32 as u32;
        while i < n {
            let fresh0 = state;
            state = state.offset(1);
            *fresh0 = 0 as i32 as uint64_t;
            let fresh1 = prev;
            prev = prev.offset(1);
            let fresh2 = state;
            state = state.offset(1);
            *fresh2 = *fresh1;
            i = i.wrapping_add(1);
            i;
        }
    } else if count <= 16384 as i32 as u64 {
        i = 0 as i32 as u32;
        while i < n {
            let mut y: uint64_t = 0;
            let fresh3 = state;
            state = state.offset(1);
            *fresh3 = 0 as i32 as uint64_t;
            y = *state;
            if y >= (59 as i32 as uint64_t).wrapping_neg() {
                y = (y as u64).wrapping_sub((59 as i32 as uint64_t).wrapping_neg())
                    as uint64_t as uint64_t;
            }
            let fresh4 = state;
            state = state.offset(1);
            *fresh4 = y;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        let mut pad: uint64_t = (1 as i32 as uint64_t) << 63 as i32;
        if count.wrapping_rem(2 as i32 as u64) == 1 as i32 as u64 {
            i = 0 as i32 as u32;
            key = key.offset(2 as i32 as isize);
            while i < n {
                _nettle_umac_poly128(
                    key,
                    state.offset((2 as i32 as u32).wrapping_mul(i) as isize),
                    *prev.offset(i as isize),
                    pad,
                );
                i = i.wrapping_add(1);
                i;
                key = key.offset(6 as i32 as isize);
            }
        } else {
            i = 0 as i32 as u32;
            key = key.offset(2 as i32 as isize);
            while i < n {
                _nettle_umac_poly128(
                    key,
                    state.offset((2 as i32 as u32).wrapping_mul(i) as isize),
                    pad,
                    0 as i32 as uint64_t,
                );
                i = i.wrapping_add(1);
                i;
                key = key.offset(6 as i32 as isize);
            }
        }
        i = 0 as i32 as u32;
        while i < n {
            let mut yh: uint64_t = 0;
            let mut yl: uint64_t = 0;
            yh = *state.offset(0 as i32 as isize);
            yl = *state.offset(1 as i32 as isize);
            if yh == !(0 as i32 as uint64_t)
                && yl >= (159 as i32 as uint64_t).wrapping_neg()
            {
                *state.offset(0 as i32 as isize) = 0 as i32 as uint64_t;
                *state.offset(1 as i32 as isize) = yl
                    .wrapping_sub((159 as i32 as uint64_t).wrapping_neg());
            }
            i = i.wrapping_add(1);
            i;
            state = state.offset(2 as i32 as isize);
        }
    };
}