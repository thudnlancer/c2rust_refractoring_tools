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
    static _nettle_aes_encrypt_table: aes_table;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes_table {
    pub sbox: [uint8_t; 256],
    pub table: [[uint32_t; 256]; 4],
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_aes_set_key(
    mut nr: u32,
    mut nk: u32,
    mut subkeys: *mut uint32_t,
    mut key: *const uint8_t,
) {
    static mut rcon: [uint8_t; 10] = [
        0x1 as i32 as uint8_t,
        0x2 as i32 as uint8_t,
        0x4 as i32 as uint8_t,
        0x8 as i32 as uint8_t,
        0x10 as i32 as uint8_t,
        0x20 as i32 as uint8_t,
        0x40 as i32 as uint8_t,
        0x80 as i32 as uint8_t,
        0x1b as i32 as uint8_t,
        0x36 as i32 as uint8_t,
    ];
    let mut rp: *const uint8_t = 0 as *const uint8_t;
    let mut lastkey: u32 = 0;
    let mut i: u32 = 0;
    let mut t: uint32_t = 0;
    if nk != 0 as i32 as u32 {} else {
        __assert_fail(
            b"nk != 0\0" as *const u8 as *const i8,
            b"aes-set-key-internal.c\0" as *const u8 as *const i8,
            56 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[i8; 82],
            >(
                b"void _nettle_aes_set_key(unsigned int, unsigned int, uint32_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_812: {
        if nk != 0 as i32 as u32 {} else {
            __assert_fail(
                b"nk != 0\0" as *const u8 as *const i8,
                b"aes-set-key-internal.c\0" as *const u8 as *const i8,
                56 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 82],
                    &[i8; 82],
                >(
                    b"void _nettle_aes_set_key(unsigned int, unsigned int, uint32_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    lastkey = ((16 as i32 / 4 as i32) as u32)
        .wrapping_mul(nr.wrapping_add(1 as i32 as u32));
    i = 0 as i32 as u32;
    rp = rcon.as_ptr();
    while i < nk {
        *subkeys.offset(i as isize) = (*key
            .offset(i.wrapping_mul(4 as i32 as u32) as isize)
            .offset(3 as i32 as isize) as uint32_t) << 24 as i32
            | (*key
                .offset(i.wrapping_mul(4 as i32 as u32) as isize)
                .offset(2 as i32 as isize) as uint32_t) << 16 as i32
            | (*key
                .offset(i.wrapping_mul(4 as i32 as u32) as isize)
                .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *key
                .offset(i.wrapping_mul(4 as i32 as u32) as isize)
                .offset(0 as i32 as isize) as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
    i = nk;
    while i < lastkey {
        t = *subkeys.offset(i.wrapping_sub(1 as i32 as u32) as isize);
        if i.wrapping_rem(nk) == 0 as i32 as u32 {
            let fresh0 = rp;
            rp = rp.offset(1);
            t = (_nettle_aes_encrypt_table
                .sbox[((t << 24 as i32 | t >> (-(24 as i32) & 31 as i32))
                & 0xff as i32 as u32) as usize] as uint32_t
                | (_nettle_aes_encrypt_table
                    .sbox[((t << 24 as i32 | t >> (-(24 as i32) & 31 as i32)) >> 8 as i32
                    & 0xff as i32 as u32) as usize] as uint32_t) << 8 as i32
                | (_nettle_aes_encrypt_table
                    .sbox[((t << 24 as i32 | t >> (-(24 as i32) & 31 as i32))
                    >> 16 as i32 & 0xff as i32 as u32) as usize] as uint32_t)
                    << 16 as i32
                | (_nettle_aes_encrypt_table
                    .sbox[((t << 24 as i32 | t >> (-(24 as i32) & 31 as i32))
                    >> 24 as i32 & 0xff as i32 as u32) as usize] as uint32_t)
                    << 24 as i32) ^ *fresh0 as u32;
        } else if nk > 6 as i32 as u32 && i.wrapping_rem(nk) == 4 as i32 as u32 {
            t = _nettle_aes_encrypt_table.sbox[(t & 0xff as i32 as u32) as usize]
                as uint32_t
                | (_nettle_aes_encrypt_table
                    .sbox[(t >> 8 as i32 & 0xff as i32 as u32) as usize] as uint32_t)
                    << 8 as i32
                | (_nettle_aes_encrypt_table
                    .sbox[(t >> 16 as i32 & 0xff as i32 as u32) as usize] as uint32_t)
                    << 16 as i32
                | (_nettle_aes_encrypt_table
                    .sbox[(t >> 24 as i32 & 0xff as i32 as u32) as usize] as uint32_t)
                    << 24 as i32;
        }
        *subkeys.offset(i as isize) = *subkeys.offset(i.wrapping_sub(nk) as isize) ^ t;
        i = i.wrapping_add(1);
        i;
    }
}