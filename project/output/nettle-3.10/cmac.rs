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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn nettle_memxor3(
        dst: *mut libc::c_void,
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac128_key {
    pub K1: nettle_block16,
    pub K2: nettle_block16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac128_ctx {
    pub X: nettle_block16,
    pub block: nettle_block16,
    pub index: size_t,
}
#[inline]
unsafe extern "C" fn block16_xor(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    (*r).u64_0[0 as i32 as usize] ^= (*x).u64_0[0 as i32 as usize];
    (*r).u64_0[1 as i32 as usize] ^= (*x).u64_0[1 as i32 as usize];
}
#[inline]
unsafe extern "C" fn block16_xor3(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
    mut y: *const nettle_block16,
) {
    (*r).u64_0[0 as i32 as usize] = (*x).u64_0[0 as i32 as usize]
        ^ (*y).u64_0[0 as i32 as usize];
    (*r).u64_0[1 as i32 as usize] = (*x).u64_0[1 as i32 as usize]
        ^ (*y).u64_0[1 as i32 as usize];
}
#[inline]
unsafe extern "C" fn block16_xor_bytes(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
    mut bytes: *const uint8_t,
) {
    nettle_memxor3(
        ((*r).b).as_mut_ptr() as *mut libc::c_void,
        ((*x).b).as_ptr() as *const libc::c_void,
        bytes as *const libc::c_void,
        16 as i32 as size_t,
    );
}
#[inline]
unsafe extern "C" fn block16_mulx_be(
    mut dst: *mut nettle_block16,
    mut src: *const nettle_block16,
) {
    let mut carry: uint64_t = ((*src).u64_0[0 as i32 as usize] & 0x80 as i32 as u64)
        >> 7 as i32;
    (*dst).u64_0[0 as i32 as usize] = ((*src).u64_0[0 as i32 as usize]
        & 0x7f7f7f7f7f7f7f7f as u64) << 1 as i32
        | ((*src).u64_0[0 as i32 as usize] & 0x8080808080808080 as u64) >> 15 as i32
        | ((*src).u64_0[1 as i32 as usize] & 0x80 as i32 as u64) << 49 as i32;
    (*dst).u64_0[1 as i32 as usize] = (((*src).u64_0[1 as i32 as usize]
        & 0x7f7f7f7f7f7f7f7f as u64) << 1 as i32
        | ((*src).u64_0[1 as i32 as usize] & 0x8080808080808080 as u64) >> 15 as i32)
        ^ (0x87 as u64) << 56 as i32 & carry.wrapping_neg();
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cmac128_set_key(
    mut key: *mut cmac128_key,
    mut cipher: *const libc::c_void,
    mut encrypt: Option<nettle_cipher_func>,
) {
    static mut zero_block: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut L: nettle_block16 = nettle_block16 { b: [0; 16] };
    encrypt
        .expect(
            "non-null function pointer",
        )(cipher, 16 as i32 as size_t, (L.b).as_mut_ptr(), (zero_block.b).as_ptr());
    block16_mulx_be(&mut (*key).K1, &mut L);
    block16_mulx_be(&mut (*key).K2, &mut (*key).K1);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cmac128_init(mut ctx: *mut cmac128_ctx) {
    memset(
        &mut (*ctx).X as *mut nettle_block16 as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<nettle_block16>() as u64,
    );
    (*ctx).index = 0 as i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cmac128_update(
    mut ctx: *mut cmac128_ctx,
    mut cipher: *const libc::c_void,
    mut encrypt: Option<nettle_cipher_func>,
    mut msg_len: size_t,
    mut msg: *const uint8_t,
) {
    let mut Y: nettle_block16 = nettle_block16 { b: [0; 16] };
    if (*ctx).index < 16 as i32 as u64 {
        let mut len: size_t = if (16 as i32 as u64).wrapping_sub((*ctx).index) < msg_len
        {
            (16 as i32 as u64).wrapping_sub((*ctx).index)
        } else {
            msg_len
        };
        memcpy(
            &mut *((*ctx).block.b).as_mut_ptr().offset((*ctx).index as isize)
                as *mut uint8_t as *mut libc::c_void,
            msg as *const libc::c_void,
            len,
        );
        msg = msg.offset(len as isize);
        msg_len = (msg_len as u64).wrapping_sub(len) as size_t as size_t;
        (*ctx).index = ((*ctx).index as u64).wrapping_add(len) as size_t as size_t;
    }
    if msg_len == 0 as i32 as u64 {
        return;
    }
    block16_xor3(&mut Y, &mut (*ctx).X, &mut (*ctx).block);
    encrypt
        .expect(
            "non-null function pointer",
        )(cipher, 16 as i32 as size_t, ((*ctx).X.b).as_mut_ptr(), (Y.b).as_mut_ptr());
    while msg_len > 16 as i32 as u64 {
        block16_xor_bytes(&mut Y, &mut (*ctx).X, msg);
        encrypt
            .expect(
                "non-null function pointer",
            )(
            cipher,
            16 as i32 as size_t,
            ((*ctx).X.b).as_mut_ptr(),
            (Y.b).as_mut_ptr(),
        );
        msg = msg.offset(16 as i32 as isize);
        msg_len = (msg_len as u64).wrapping_sub(16 as i32 as u64) as size_t as size_t;
    }
    memcpy(
        ((*ctx).block.b).as_mut_ptr() as *mut libc::c_void,
        msg as *const libc::c_void,
        msg_len,
    );
    (*ctx).index = msg_len;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cmac128_digest(
    mut ctx: *mut cmac128_ctx,
    mut key: *const cmac128_key,
    mut cipher: *const libc::c_void,
    mut encrypt: Option<nettle_cipher_func>,
    mut length: u32,
    mut dst: *mut uint8_t,
) {
    let mut Y: nettle_block16 = nettle_block16 { b: [0; 16] };
    if (*ctx).index < 16 as i32 as u64 {
        (*ctx).block.b[(*ctx).index as usize] = 0x80 as i32 as uint8_t;
        memset(
            ((*ctx).block.b)
                .as_mut_ptr()
                .offset((*ctx).index as isize)
                .offset(1 as i32 as isize) as *mut libc::c_void,
            0 as i32,
            ((16 as i32 - 1 as i32) as u64).wrapping_sub((*ctx).index),
        );
        block16_xor(&mut (*ctx).block, &(*key).K2);
    } else {
        block16_xor(&mut (*ctx).block, &(*key).K1);
    }
    block16_xor3(&mut Y, &mut (*ctx).block, &mut (*ctx).X);
    if length <= 16 as i32 as u32 {} else {
        __assert_fail(
            b"length <= 16\0" as *const u8 as *const i8,
            b"cmac.c\0" as *const u8 as *const i8,
            140 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 138],
                &[i8; 138],
            >(
                b"void nettle_cmac128_digest(struct cmac128_ctx *, const struct cmac128_key *, const void *, nettle_cipher_func *, unsigned int, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2242: {
        if length <= 16 as i32 as u32 {} else {
            __assert_fail(
                b"length <= 16\0" as *const u8 as *const i8,
                b"cmac.c\0" as *const u8 as *const i8,
                140 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 138],
                    &[i8; 138],
                >(
                    b"void nettle_cmac128_digest(struct cmac128_ctx *, const struct cmac128_key *, const void *, nettle_cipher_func *, unsigned int, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length == 16 as i32 as u32 {
        encrypt
            .expect(
                "non-null function pointer",
            )(cipher, 16 as i32 as size_t, dst, (Y.b).as_mut_ptr());
    } else {
        encrypt
            .expect(
                "non-null function pointer",
            )(
            cipher,
            16 as i32 as size_t,
            ((*ctx).block.b).as_mut_ptr(),
            (Y.b).as_mut_ptr(),
        );
        memcpy(
            dst as *mut libc::c_void,
            ((*ctx).block.b).as_mut_ptr() as *const libc::c_void,
            length as u64,
        );
    }
    nettle_cmac128_init(ctx);
}