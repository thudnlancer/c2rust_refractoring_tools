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
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arctwo_ctx {
    pub S: [uint16_t; 64],
}
static mut arctwo_sbox: [uint8_t; 256] = [
    0xd9 as i32 as uint8_t,
    0x78 as i32 as uint8_t,
    0xf9 as i32 as uint8_t,
    0xc4 as i32 as uint8_t,
    0x19 as i32 as uint8_t,
    0xdd as i32 as uint8_t,
    0xb5 as i32 as uint8_t,
    0xed as i32 as uint8_t,
    0x28 as i32 as uint8_t,
    0xe9 as i32 as uint8_t,
    0xfd as i32 as uint8_t,
    0x79 as i32 as uint8_t,
    0x4a as i32 as uint8_t,
    0xa0 as i32 as uint8_t,
    0xd8 as i32 as uint8_t,
    0x9d as i32 as uint8_t,
    0xc6 as i32 as uint8_t,
    0x7e as i32 as uint8_t,
    0x37 as i32 as uint8_t,
    0x83 as i32 as uint8_t,
    0x2b as i32 as uint8_t,
    0x76 as i32 as uint8_t,
    0x53 as i32 as uint8_t,
    0x8e as i32 as uint8_t,
    0x62 as i32 as uint8_t,
    0x4c as i32 as uint8_t,
    0x64 as i32 as uint8_t,
    0x88 as i32 as uint8_t,
    0x44 as i32 as uint8_t,
    0x8b as i32 as uint8_t,
    0xfb as i32 as uint8_t,
    0xa2 as i32 as uint8_t,
    0x17 as i32 as uint8_t,
    0x9a as i32 as uint8_t,
    0x59 as i32 as uint8_t,
    0xf5 as i32 as uint8_t,
    0x87 as i32 as uint8_t,
    0xb3 as i32 as uint8_t,
    0x4f as i32 as uint8_t,
    0x13 as i32 as uint8_t,
    0x61 as i32 as uint8_t,
    0x45 as i32 as uint8_t,
    0x6d as i32 as uint8_t,
    0x8d as i32 as uint8_t,
    0x9 as i32 as uint8_t,
    0x81 as i32 as uint8_t,
    0x7d as i32 as uint8_t,
    0x32 as i32 as uint8_t,
    0xbd as i32 as uint8_t,
    0x8f as i32 as uint8_t,
    0x40 as i32 as uint8_t,
    0xeb as i32 as uint8_t,
    0x86 as i32 as uint8_t,
    0xb7 as i32 as uint8_t,
    0x7b as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0xf0 as i32 as uint8_t,
    0x95 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x22 as i32 as uint8_t,
    0x5c as i32 as uint8_t,
    0x6b as i32 as uint8_t,
    0x4e as i32 as uint8_t,
    0x82 as i32 as uint8_t,
    0x54 as i32 as uint8_t,
    0xd6 as i32 as uint8_t,
    0x65 as i32 as uint8_t,
    0x93 as i32 as uint8_t,
    0xce as i32 as uint8_t,
    0x60 as i32 as uint8_t,
    0xb2 as i32 as uint8_t,
    0x1c as i32 as uint8_t,
    0x73 as i32 as uint8_t,
    0x56 as i32 as uint8_t,
    0xc0 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0xa7 as i32 as uint8_t,
    0x8c as i32 as uint8_t,
    0xf1 as i32 as uint8_t,
    0xdc as i32 as uint8_t,
    0x12 as i32 as uint8_t,
    0x75 as i32 as uint8_t,
    0xca as i32 as uint8_t,
    0x1f as i32 as uint8_t,
    0x3b as i32 as uint8_t,
    0xbe as i32 as uint8_t,
    0xe4 as i32 as uint8_t,
    0xd1 as i32 as uint8_t,
    0x42 as i32 as uint8_t,
    0x3d as i32 as uint8_t,
    0xd4 as i32 as uint8_t,
    0x30 as i32 as uint8_t,
    0xa3 as i32 as uint8_t,
    0x3c as i32 as uint8_t,
    0xb6 as i32 as uint8_t,
    0x26 as i32 as uint8_t,
    0x6f as i32 as uint8_t,
    0xbf as i32 as uint8_t,
    0xe as i32 as uint8_t,
    0xda as i32 as uint8_t,
    0x46 as i32 as uint8_t,
    0x69 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0x57 as i32 as uint8_t,
    0x27 as i32 as uint8_t,
    0xf2 as i32 as uint8_t,
    0x1d as i32 as uint8_t,
    0x9b as i32 as uint8_t,
    0xbc as i32 as uint8_t,
    0x94 as i32 as uint8_t,
    0x43 as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0xf8 as i32 as uint8_t,
    0x11 as i32 as uint8_t,
    0xc7 as i32 as uint8_t,
    0xf6 as i32 as uint8_t,
    0x90 as i32 as uint8_t,
    0xef as i32 as uint8_t,
    0x3e as i32 as uint8_t,
    0xe7 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0xc3 as i32 as uint8_t,
    0xd5 as i32 as uint8_t,
    0x2f as i32 as uint8_t,
    0xc8 as i32 as uint8_t,
    0x66 as i32 as uint8_t,
    0x1e as i32 as uint8_t,
    0xd7 as i32 as uint8_t,
    0x8 as i32 as uint8_t,
    0xe8 as i32 as uint8_t,
    0xea as i32 as uint8_t,
    0xde as i32 as uint8_t,
    0x80 as i32 as uint8_t,
    0x52 as i32 as uint8_t,
    0xee as i32 as uint8_t,
    0xf7 as i32 as uint8_t,
    0x84 as i32 as uint8_t,
    0xaa as i32 as uint8_t,
    0x72 as i32 as uint8_t,
    0xac as i32 as uint8_t,
    0x35 as i32 as uint8_t,
    0x4d as i32 as uint8_t,
    0x6a as i32 as uint8_t,
    0x2a as i32 as uint8_t,
    0x96 as i32 as uint8_t,
    0x1a as i32 as uint8_t,
    0xd2 as i32 as uint8_t,
    0x71 as i32 as uint8_t,
    0x5a as i32 as uint8_t,
    0x15 as i32 as uint8_t,
    0x49 as i32 as uint8_t,
    0x74 as i32 as uint8_t,
    0x4b as i32 as uint8_t,
    0x9f as i32 as uint8_t,
    0xd0 as i32 as uint8_t,
    0x5e as i32 as uint8_t,
    0x4 as i32 as uint8_t,
    0x18 as i32 as uint8_t,
    0xa4 as i32 as uint8_t,
    0xec as i32 as uint8_t,
    0xc2 as i32 as uint8_t,
    0xe0 as i32 as uint8_t,
    0x41 as i32 as uint8_t,
    0x6e as i32 as uint8_t,
    0xf as i32 as uint8_t,
    0x51 as i32 as uint8_t,
    0xcb as i32 as uint8_t,
    0xcc as i32 as uint8_t,
    0x24 as i32 as uint8_t,
    0x91 as i32 as uint8_t,
    0xaf as i32 as uint8_t,
    0x50 as i32 as uint8_t,
    0xa1 as i32 as uint8_t,
    0xf4 as i32 as uint8_t,
    0x70 as i32 as uint8_t,
    0x39 as i32 as uint8_t,
    0x99 as i32 as uint8_t,
    0x7c as i32 as uint8_t,
    0x3a as i32 as uint8_t,
    0x85 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0xb8 as i32 as uint8_t,
    0xb4 as i32 as uint8_t,
    0x7a as i32 as uint8_t,
    0xfc as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0x36 as i32 as uint8_t,
    0x5b as i32 as uint8_t,
    0x25 as i32 as uint8_t,
    0x55 as i32 as uint8_t,
    0x97 as i32 as uint8_t,
    0x31 as i32 as uint8_t,
    0x2d as i32 as uint8_t,
    0x5d as i32 as uint8_t,
    0xfa as i32 as uint8_t,
    0x98 as i32 as uint8_t,
    0xe3 as i32 as uint8_t,
    0x8a as i32 as uint8_t,
    0x92 as i32 as uint8_t,
    0xae as i32 as uint8_t,
    0x5 as i32 as uint8_t,
    0xdf as i32 as uint8_t,
    0x29 as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x67 as i32 as uint8_t,
    0x6c as i32 as uint8_t,
    0xba as i32 as uint8_t,
    0xc9 as i32 as uint8_t,
    0xd3 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0xe6 as i32 as uint8_t,
    0xcf as i32 as uint8_t,
    0xe1 as i32 as uint8_t,
    0x9e as i32 as uint8_t,
    0xa8 as i32 as uint8_t,
    0x2c as i32 as uint8_t,
    0x63 as i32 as uint8_t,
    0x16 as i32 as uint8_t,
    0x1 as i32 as uint8_t,
    0x3f as i32 as uint8_t,
    0x58 as i32 as uint8_t,
    0xe2 as i32 as uint8_t,
    0x89 as i32 as uint8_t,
    0xa9 as i32 as uint8_t,
    0xd as i32 as uint8_t,
    0x38 as i32 as uint8_t,
    0x34 as i32 as uint8_t,
    0x1b as i32 as uint8_t,
    0xab as i32 as uint8_t,
    0x33 as i32 as uint8_t,
    0xff as i32 as uint8_t,
    0xb0 as i32 as uint8_t,
    0xbb as i32 as uint8_t,
    0x48 as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0x5f as i32 as uint8_t,
    0xb9 as i32 as uint8_t,
    0xb1 as i32 as uint8_t,
    0xcd as i32 as uint8_t,
    0x2e as i32 as uint8_t,
    0xc5 as i32 as uint8_t,
    0xf3 as i32 as uint8_t,
    0xdb as i32 as uint8_t,
    0x47 as i32 as uint8_t,
    0xe5 as i32 as uint8_t,
    0xa5 as i32 as uint8_t,
    0x9c as i32 as uint8_t,
    0x77 as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0xa6 as i32 as uint8_t,
    0x20 as i32 as uint8_t,
    0x68 as i32 as uint8_t,
    0xfe as i32 as uint8_t,
    0x7f as i32 as uint8_t,
    0xc1 as i32 as uint8_t,
    0xad as i32 as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn nettle_arctwo_encrypt(
    mut ctx: *mut arctwo_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!((length) % (8))\0" as *const u8 as *const i8,
            b"arctwo.c\0" as *const u8 as *const i8,
            98 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 84],
                &[i8; 84],
            >(
                b"void nettle_arctwo_encrypt(struct arctwo_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1903: {
        if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!((length) % (8))\0" as *const u8 as *const i8,
                b"arctwo.c\0" as *const u8 as *const i8,
                98 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 84],
                    &[i8; 84],
                >(
                    b"void nettle_arctwo_encrypt(struct arctwo_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut i: u32 = 0;
        let mut w0: uint16_t = 0;
        let mut w1: uint16_t = 0;
        let mut w2: uint16_t = 0;
        let mut w3: uint16_t = 0;
        w0 = ((*(&*src.offset(0 as i32 as isize) as *const uint8_t)
            .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *(&*src.offset(0 as i32 as isize) as *const uint8_t)
                .offset(0 as i32 as isize) as uint32_t) as uint16_t;
        w1 = ((*(&*src.offset(2 as i32 as isize) as *const uint8_t)
            .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *(&*src.offset(2 as i32 as isize) as *const uint8_t)
                .offset(0 as i32 as isize) as uint32_t) as uint16_t;
        w2 = ((*(&*src.offset(4 as i32 as isize) as *const uint8_t)
            .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *(&*src.offset(4 as i32 as isize) as *const uint8_t)
                .offset(0 as i32 as isize) as uint32_t) as uint16_t;
        w3 = ((*(&*src.offset(6 as i32 as isize) as *const uint8_t)
            .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *(&*src.offset(6 as i32 as isize) as *const uint8_t)
                .offset(0 as i32 as isize) as uint32_t) as uint16_t;
        i = 0 as i32 as u32;
        while i < 16 as i32 as u32 {
            let mut j: u32 = i.wrapping_mul(4 as i32 as u32);
            w0 = (w0 as i32
                + ((w1 as i32 & !(w3 as i32)) + (w2 as i32 & w3 as i32)
                    + (*ctx).S[j as usize] as i32)) as uint16_t;
            w0 = ((w0 as i32) << 1 as i32 as uint16_t as i32
                | w0 as i32 >> 16 as i32 - 1 as i32 as uint16_t as i32) as uint16_t;
            w1 = (w1 as i32
                + ((w2 as i32 & !(w0 as i32)) + (w3 as i32 & w0 as i32)
                    + (*ctx).S[j.wrapping_add(1 as i32 as u32) as usize] as i32))
                as uint16_t;
            w1 = ((w1 as i32) << 2 as i32 as uint16_t as i32
                | w1 as i32 >> 16 as i32 - 2 as i32 as uint16_t as i32) as uint16_t;
            w2 = (w2 as i32
                + ((w3 as i32 & !(w1 as i32)) + (w0 as i32 & w1 as i32)
                    + (*ctx).S[j.wrapping_add(2 as i32 as u32) as usize] as i32))
                as uint16_t;
            w2 = ((w2 as i32) << 3 as i32 as uint16_t as i32
                | w2 as i32 >> 16 as i32 - 3 as i32 as uint16_t as i32) as uint16_t;
            w3 = (w3 as i32
                + ((w0 as i32 & !(w2 as i32)) + (w1 as i32 & w2 as i32)
                    + (*ctx).S[j.wrapping_add(3 as i32 as u32) as usize] as i32))
                as uint16_t;
            w3 = ((w3 as i32) << 5 as i32 as uint16_t as i32
                | w3 as i32 >> 16 as i32 - 5 as i32 as uint16_t as i32) as uint16_t;
            if i == 4 as i32 as u32 || i == 10 as i32 as u32 {
                w0 = (w0 as i32 + (*ctx).S[(w3 as i32 & 63 as i32) as usize] as i32)
                    as uint16_t;
                w1 = (w1 as i32 + (*ctx).S[(w0 as i32 & 63 as i32) as usize] as i32)
                    as uint16_t;
                w2 = (w2 as i32 + (*ctx).S[(w1 as i32 & 63 as i32) as usize] as i32)
                    as uint16_t;
                w3 = (w3 as i32 + (*ctx).S[(w2 as i32 & 63 as i32) as usize] as i32)
                    as uint16_t;
            }
            i = i.wrapping_add(1);
            i;
        }
        *(&mut *dst.offset(0 as i32 as isize) as *mut uint8_t)
            .offset(1 as i32 as isize) = (w0 as i32 >> 8 as i32 & 0xff as i32)
            as uint8_t;
        *(&mut *dst.offset(0 as i32 as isize) as *mut uint8_t)
            .offset(0 as i32 as isize) = (w0 as i32 & 0xff as i32) as uint8_t;
        *(&mut *dst.offset(2 as i32 as isize) as *mut uint8_t)
            .offset(1 as i32 as isize) = (w1 as i32 >> 8 as i32 & 0xff as i32)
            as uint8_t;
        *(&mut *dst.offset(2 as i32 as isize) as *mut uint8_t)
            .offset(0 as i32 as isize) = (w1 as i32 & 0xff as i32) as uint8_t;
        *(&mut *dst.offset(4 as i32 as isize) as *mut uint8_t)
            .offset(1 as i32 as isize) = (w2 as i32 >> 8 as i32 & 0xff as i32)
            as uint8_t;
        *(&mut *dst.offset(4 as i32 as isize) as *mut uint8_t)
            .offset(0 as i32 as isize) = (w2 as i32 & 0xff as i32) as uint8_t;
        *(&mut *dst.offset(6 as i32 as isize) as *mut uint8_t)
            .offset(1 as i32 as isize) = (w3 as i32 >> 8 as i32 & 0xff as i32)
            as uint8_t;
        *(&mut *dst.offset(6 as i32 as isize) as *mut uint8_t)
            .offset(0 as i32 as isize) = (w3 as i32 & 0xff as i32) as uint8_t;
        length = (length as u64).wrapping_sub(8 as i32 as u64) as size_t as size_t;
        dst = dst.offset(8 as i32 as isize);
        src = src.offset(8 as i32 as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_arctwo_decrypt(
    mut ctx: *mut arctwo_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!((length) % (8))\0" as *const u8 as *const i8,
            b"arctwo.c\0" as *const u8 as *const i8,
            143 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 84],
                &[i8; 84],
            >(
                b"void nettle_arctwo_decrypt(struct arctwo_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2639: {
        if length.wrapping_rem(8 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!((length) % (8))\0" as *const u8 as *const i8,
                b"arctwo.c\0" as *const u8 as *const i8,
                143 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 84],
                    &[i8; 84],
                >(
                    b"void nettle_arctwo_decrypt(struct arctwo_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut i: u32 = 0;
        let mut w0: uint16_t = 0;
        let mut w1: uint16_t = 0;
        let mut w2: uint16_t = 0;
        let mut w3: uint16_t = 0;
        w0 = ((*(&*src.offset(0 as i32 as isize) as *const uint8_t)
            .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *(&*src.offset(0 as i32 as isize) as *const uint8_t)
                .offset(0 as i32 as isize) as uint32_t) as uint16_t;
        w1 = ((*(&*src.offset(2 as i32 as isize) as *const uint8_t)
            .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *(&*src.offset(2 as i32 as isize) as *const uint8_t)
                .offset(0 as i32 as isize) as uint32_t) as uint16_t;
        w2 = ((*(&*src.offset(4 as i32 as isize) as *const uint8_t)
            .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *(&*src.offset(4 as i32 as isize) as *const uint8_t)
                .offset(0 as i32 as isize) as uint32_t) as uint16_t;
        w3 = ((*(&*src.offset(6 as i32 as isize) as *const uint8_t)
            .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *(&*src.offset(6 as i32 as isize) as *const uint8_t)
                .offset(0 as i32 as isize) as uint32_t) as uint16_t;
        i = 16 as i32 as u32;
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 > 0 as i32 as u32) {
                break;
            }
            let mut j: u32 = i.wrapping_mul(4 as i32 as u32);
            w3 = (w3 as i32 >> 5 as i32 as uint16_t as i32
                | (w3 as i32) << 16 as i32 - 5 as i32 as uint16_t as i32) as uint16_t;
            w3 = (w3 as i32
                - ((w0 as i32 & !(w2 as i32)) + (w1 as i32 & w2 as i32)
                    + (*ctx).S[j.wrapping_add(3 as i32 as u32) as usize] as i32))
                as uint16_t;
            w2 = (w2 as i32 >> 3 as i32 as uint16_t as i32
                | (w2 as i32) << 16 as i32 - 3 as i32 as uint16_t as i32) as uint16_t;
            w2 = (w2 as i32
                - ((w3 as i32 & !(w1 as i32)) + (w0 as i32 & w1 as i32)
                    + (*ctx).S[j.wrapping_add(2 as i32 as u32) as usize] as i32))
                as uint16_t;
            w1 = (w1 as i32 >> 2 as i32 as uint16_t as i32
                | (w1 as i32) << 16 as i32 - 2 as i32 as uint16_t as i32) as uint16_t;
            w1 = (w1 as i32
                - ((w2 as i32 & !(w0 as i32)) + (w3 as i32 & w0 as i32)
                    + (*ctx).S[j.wrapping_add(1 as i32 as u32) as usize] as i32))
                as uint16_t;
            w0 = (w0 as i32 >> 1 as i32 as uint16_t as i32
                | (w0 as i32) << 16 as i32 - 1 as i32 as uint16_t as i32) as uint16_t;
            w0 = (w0 as i32
                - ((w1 as i32 & !(w3 as i32)) + (w2 as i32 & w3 as i32)
                    + (*ctx).S[j as usize] as i32)) as uint16_t;
            if i == 5 as i32 as u32 || i == 11 as i32 as u32 {
                w3 = (w3 as i32 - (*ctx).S[(w2 as i32 & 63 as i32) as usize] as i32)
                    as uint16_t;
                w2 = (w2 as i32 - (*ctx).S[(w1 as i32 & 63 as i32) as usize] as i32)
                    as uint16_t;
                w1 = (w1 as i32 - (*ctx).S[(w0 as i32 & 63 as i32) as usize] as i32)
                    as uint16_t;
                w0 = (w0 as i32 - (*ctx).S[(w3 as i32 & 63 as i32) as usize] as i32)
                    as uint16_t;
            }
        }
        *(&mut *dst.offset(0 as i32 as isize) as *mut uint8_t)
            .offset(1 as i32 as isize) = (w0 as i32 >> 8 as i32 & 0xff as i32)
            as uint8_t;
        *(&mut *dst.offset(0 as i32 as isize) as *mut uint8_t)
            .offset(0 as i32 as isize) = (w0 as i32 & 0xff as i32) as uint8_t;
        *(&mut *dst.offset(2 as i32 as isize) as *mut uint8_t)
            .offset(1 as i32 as isize) = (w1 as i32 >> 8 as i32 & 0xff as i32)
            as uint8_t;
        *(&mut *dst.offset(2 as i32 as isize) as *mut uint8_t)
            .offset(0 as i32 as isize) = (w1 as i32 & 0xff as i32) as uint8_t;
        *(&mut *dst.offset(4 as i32 as isize) as *mut uint8_t)
            .offset(1 as i32 as isize) = (w2 as i32 >> 8 as i32 & 0xff as i32)
            as uint8_t;
        *(&mut *dst.offset(4 as i32 as isize) as *mut uint8_t)
            .offset(0 as i32 as isize) = (w2 as i32 & 0xff as i32) as uint8_t;
        *(&mut *dst.offset(6 as i32 as isize) as *mut uint8_t)
            .offset(1 as i32 as isize) = (w3 as i32 >> 8 as i32 & 0xff as i32)
            as uint8_t;
        *(&mut *dst.offset(6 as i32 as isize) as *mut uint8_t)
            .offset(0 as i32 as isize) = (w3 as i32 & 0xff as i32) as uint8_t;
        length = (length as u64).wrapping_sub(8 as i32 as u64) as size_t as size_t;
        dst = dst.offset(8 as i32 as isize);
        src = src.offset(8 as i32 as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_arctwo_set_key_ekb(
    mut ctx: *mut arctwo_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
    mut ekb: u32,
) {
    let mut i: size_t = 0;
    let mut S: [uint8_t; 128] = [0; 128];
    let mut x: uint8_t = 0;
    if length >= 1 as i32 as u64 {} else {
        __assert_fail(
            b"length >= ARCTWO_MIN_KEY_SIZE\0" as *const u8 as *const i8,
            b"arctwo.c\0" as *const u8 as *const i8,
            194 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1088: {
        if length >= 1 as i32 as u64 {} else {
            __assert_fail(
                b"length >= ARCTWO_MIN_KEY_SIZE\0" as *const u8 as *const i8,
                b"arctwo.c\0" as *const u8 as *const i8,
                194 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length <= 128 as i32 as u64 {} else {
        __assert_fail(
            b"length <= ARCTWO_MAX_KEY_SIZE\0" as *const u8 as *const i8,
            b"arctwo.c\0" as *const u8 as *const i8,
            195 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1049: {
        if length <= 128 as i32 as u64 {} else {
            __assert_fail(
                b"length <= ARCTWO_MAX_KEY_SIZE\0" as *const u8 as *const i8,
                b"arctwo.c\0" as *const u8 as *const i8,
                195 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ekb <= 1024 as i32 as u32 {} else {
        __assert_fail(
            b"ekb <= 1024\0" as *const u8 as *const i8,
            b"arctwo.c\0" as *const u8 as *const i8,
            196 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1006: {
        if ekb <= 1024 as i32 as u32 {} else {
            __assert_fail(
                b"ekb <= 1024\0" as *const u8 as *const i8,
                b"arctwo.c\0" as *const u8 as *const i8,
                196 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"void nettle_arctwo_set_key_ekb(struct arctwo_ctx *, size_t, const uint8_t *, unsigned int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = 0 as i32 as size_t;
    while i < length {
        S[i as usize] = *key.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = length;
    while i < 128 as i32 as u64 {
        S[i as usize] = arctwo_sbox[(S[i.wrapping_sub(length) as usize] as i32
            + S[i.wrapping_sub(1 as i32 as u64) as usize] as i32 & 255 as i32) as usize];
        i = i.wrapping_add(1);
        i;
    }
    S[0 as i32 as usize] = arctwo_sbox[S[0 as i32 as usize] as usize];
    if ekb > 0 as i32 as u32 && ekb < 1024 as i32 as u32 {
        let mut len: i32 = (ekb.wrapping_add(7 as i32 as u32) >> 3 as i32) as i32;
        i = (128 as i32 - len) as size_t;
        x = arctwo_sbox[(S[i as usize] as i32
            & 255 as i32 >> (7 as i32 as u32 & ekb.wrapping_neg())) as usize];
        S[i as usize] = x;
        loop {
            let fresh1 = i;
            i = i.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            x = arctwo_sbox[(x as i32 ^ S[i.wrapping_add(len as u64) as usize] as i32)
                as usize];
            S[i as usize] = x;
        }
    }
    i = 0 as i32 as size_t;
    while i < 64 as i32 as u64 {
        (*ctx).S[i as usize] = ((*S
            .as_mut_ptr()
            .offset(i.wrapping_mul(2 as i32 as u64) as isize)
            .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *S
                .as_mut_ptr()
                .offset(i.wrapping_mul(2 as i32 as u64) as isize)
                .offset(0 as i32 as isize) as uint32_t) as uint16_t;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_arctwo_set_key(
    mut ctx: *mut arctwo_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(
        ctx,
        length,
        key,
        (8 as i32 as u64).wrapping_mul(length) as u32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_arctwo_set_key_gutmann(
    mut ctx: *mut arctwo_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(ctx, length, key, 0 as i32 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_arctwo40_set_key(
    mut ctx: *mut arctwo_ctx,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(ctx, 5 as i32 as size_t, key, 40 as i32 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_arctwo64_set_key(
    mut ctx: *mut arctwo_ctx,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(ctx, 8 as i32 as size_t, key, 64 as i32 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_arctwo128_set_key(
    mut ctx: *mut arctwo_ctx,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(ctx, 16 as i32 as size_t, key, 128 as i32 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_arctwo128_set_key_gutmann(
    mut ctx: *mut arctwo_ctx,
    mut key: *const uint8_t,
) {
    nettle_arctwo_set_key_ekb(ctx, 16 as i32 as size_t, key, 1024 as i32 as u32);
}