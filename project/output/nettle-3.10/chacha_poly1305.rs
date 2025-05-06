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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn nettle_chacha_crypt32(
        ctx: *mut chacha_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_chacha_set_key(ctx: *mut chacha_ctx, key: *const uint8_t);
    fn nettle_chacha_set_nonce96(ctx: *mut chacha_ctx, nonce: *const uint8_t);
    fn _nettle_chacha_core(dst: *mut uint32_t, src: *const uint32_t, rounds: u32);
    fn _nettle_poly1305_set_key(ctx: *mut poly1305_ctx, key: *const uint8_t);
    fn _nettle_poly1305_digest(ctx: *mut poly1305_ctx, s: *mut nettle_block16);
    fn _nettle_poly1305_block(ctx: *mut poly1305_ctx, m: *const uint8_t, high: u32);
    fn _nettle_poly1305_update(
        ctx: *mut poly1305_ctx,
        buffer: *mut uint8_t,
        index: u32,
        length: size_t,
        m: *const uint8_t,
    ) -> u32;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha_ctx {
    pub state: [uint32_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poly1305_ctx {
    pub r: C2RustUnnamed_0,
    pub s32: [uint32_t; 3],
    pub hh: uint32_t,
    pub h: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub h32: [uint32_t; 4],
    pub h64: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub r32: [uint32_t; 6],
    pub r64: [uint64_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha_poly1305_ctx {
    pub chacha: chacha_ctx,
    pub poly1305: poly1305_ctx,
    pub s: nettle_block16,
    pub auth_size: uint64_t,
    pub data_size: uint64_t,
    pub block: [uint8_t; 16],
    pub index: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub x: [uint32_t; 16],
    pub subkey: [uint8_t; 32],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_poly1305_set_key(
    mut ctx: *mut chacha_poly1305_ctx,
    mut key: *const uint8_t,
) {
    nettle_chacha_set_key(&mut (*ctx).chacha, key);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_poly1305_set_nonce(
    mut ctx: *mut chacha_poly1305_ctx,
    mut nonce: *const uint8_t,
) {
    let mut u: C2RustUnnamed_1 = C2RustUnnamed_1 { x: [0; 16] };
    nettle_chacha_set_nonce96(&mut (*ctx).chacha, nonce);
    _nettle_chacha_core(
        (u.x).as_mut_ptr(),
        ((*ctx).chacha.state).as_mut_ptr(),
        20 as i32 as u32,
    );
    _nettle_poly1305_set_key(
        &mut (*ctx).poly1305,
        (u.subkey).as_mut_ptr() as *const uint8_t,
    );
    memcpy(
        ((*ctx).s.b).as_mut_ptr() as *mut libc::c_void,
        (u.subkey).as_mut_ptr().offset(16 as i32 as isize) as *const libc::c_void,
        16 as i32 as u64,
    );
    (*ctx).chacha.state[12 as i32 as usize] = 1 as i32 as uint32_t;
    (*ctx).index = 0 as i32 as u32;
    (*ctx).data_size = (*ctx).index as uint64_t;
    (*ctx).auth_size = (*ctx).data_size;
}
unsafe extern "C" fn poly1305_update(
    mut ctx: *mut chacha_poly1305_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    (*ctx).index = _nettle_poly1305_update(
        &mut (*ctx).poly1305,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        length,
        data,
    );
}
unsafe extern "C" fn poly1305_pad(mut ctx: *mut chacha_poly1305_ctx) {
    if (*ctx).index != 0 {
        memset(
            ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                as *mut libc::c_void,
            0 as i32,
            (16 as i32 as u32).wrapping_sub((*ctx).index) as u64,
        );
        _nettle_poly1305_block(
            &mut (*ctx).poly1305,
            ((*ctx).block).as_mut_ptr(),
            1 as i32 as u32,
        );
        (*ctx).index = 0 as i32 as u32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_poly1305_update(
    mut ctx: *mut chacha_poly1305_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    if (*ctx).data_size == 0 as i32 as u64 {} else {
        __assert_fail(
            b"ctx->data_size == 0\0" as *const u8 as *const i8,
            b"chacha-poly1305.c\0" as *const u8 as *const i8,
            119 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[i8; 90],
            >(
                b"void nettle_chacha_poly1305_update(struct chacha_poly1305_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_936: {
        if (*ctx).data_size == 0 as i32 as u64 {} else {
            __assert_fail(
                b"ctx->data_size == 0\0" as *const u8 as *const i8,
                b"chacha-poly1305.c\0" as *const u8 as *const i8,
                119 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 90],
                    &[i8; 90],
                >(
                    b"void nettle_chacha_poly1305_update(struct chacha_poly1305_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    poly1305_update(ctx, length, data);
    (*ctx).auth_size = ((*ctx).auth_size as u64).wrapping_add(length) as uint64_t
        as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_poly1305_encrypt(
    mut ctx: *mut chacha_poly1305_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length == 0 {
        return;
    }
    if ((*ctx).data_size).wrapping_rem(64 as i32 as u64) == 0 as i32 as u64 {} else {
        __assert_fail(
            b"ctx->data_size % CHACHA_POLY1305_BLOCK_SIZE == 0\0" as *const u8
                as *const i8,
            b"chacha-poly1305.c\0" as *const u8 as *const i8,
            132 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 102],
                &[i8; 102],
            >(
                b"void nettle_chacha_poly1305_encrypt(struct chacha_poly1305_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1096: {
        if ((*ctx).data_size).wrapping_rem(64 as i32 as u64) == 0 as i32 as u64 {} else {
            __assert_fail(
                b"ctx->data_size % CHACHA_POLY1305_BLOCK_SIZE == 0\0" as *const u8
                    as *const i8,
                b"chacha-poly1305.c\0" as *const u8 as *const i8,
                132 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 102],
                    &[i8; 102],
                >(
                    b"void nettle_chacha_poly1305_encrypt(struct chacha_poly1305_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    poly1305_pad(ctx);
    nettle_chacha_crypt32(&mut (*ctx).chacha, length, dst, src);
    poly1305_update(ctx, length, dst);
    (*ctx).data_size = ((*ctx).data_size as u64).wrapping_add(length) as uint64_t
        as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_poly1305_decrypt(
    mut ctx: *mut chacha_poly1305_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length == 0 {
        return;
    }
    if ((*ctx).data_size).wrapping_rem(64 as i32 as u64) == 0 as i32 as u64 {} else {
        __assert_fail(
            b"ctx->data_size % CHACHA_POLY1305_BLOCK_SIZE == 0\0" as *const u8
                as *const i8,
            b"chacha-poly1305.c\0" as *const u8 as *const i8,
            147 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 102],
                &[i8; 102],
            >(
                b"void nettle_chacha_poly1305_decrypt(struct chacha_poly1305_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1195: {
        if ((*ctx).data_size).wrapping_rem(64 as i32 as u64) == 0 as i32 as u64 {} else {
            __assert_fail(
                b"ctx->data_size % CHACHA_POLY1305_BLOCK_SIZE == 0\0" as *const u8
                    as *const i8,
                b"chacha-poly1305.c\0" as *const u8 as *const i8,
                147 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 102],
                    &[i8; 102],
                >(
                    b"void nettle_chacha_poly1305_decrypt(struct chacha_poly1305_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    poly1305_pad(ctx);
    poly1305_update(ctx, length, src);
    nettle_chacha_crypt32(&mut (*ctx).chacha, length, dst, src);
    (*ctx).data_size = ((*ctx).data_size as u64).wrapping_add(length) as uint64_t
        as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_poly1305_digest(
    mut ctx: *mut chacha_poly1305_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut buf: [uint8_t; 16] = [0; 16];
    poly1305_pad(ctx);
    buf[7 as i32 as usize] = ((*ctx).auth_size >> 56 as i32 & 0xff as i32 as u64)
        as uint8_t;
    buf[6 as i32 as usize] = ((*ctx).auth_size >> 48 as i32 & 0xff as i32 as u64)
        as uint8_t;
    buf[5 as i32 as usize] = ((*ctx).auth_size >> 40 as i32 & 0xff as i32 as u64)
        as uint8_t;
    buf[4 as i32 as usize] = ((*ctx).auth_size >> 32 as i32 & 0xff as i32 as u64)
        as uint8_t;
    buf[3 as i32 as usize] = ((*ctx).auth_size >> 24 as i32 & 0xff as i32 as u64)
        as uint8_t;
    buf[2 as i32 as usize] = ((*ctx).auth_size >> 16 as i32 & 0xff as i32 as u64)
        as uint8_t;
    buf[1 as i32 as usize] = ((*ctx).auth_size >> 8 as i32 & 0xff as i32 as u64)
        as uint8_t;
    buf[0 as i32 as usize] = ((*ctx).auth_size & 0xff as i32 as u64) as uint8_t;
    *buf.as_mut_ptr().offset(8 as i32 as isize).offset(7 as i32 as isize) = ((*ctx)
        .data_size >> 56 as i32 & 0xff as i32 as u64) as uint8_t;
    *buf.as_mut_ptr().offset(8 as i32 as isize).offset(6 as i32 as isize) = ((*ctx)
        .data_size >> 48 as i32 & 0xff as i32 as u64) as uint8_t;
    *buf.as_mut_ptr().offset(8 as i32 as isize).offset(5 as i32 as isize) = ((*ctx)
        .data_size >> 40 as i32 & 0xff as i32 as u64) as uint8_t;
    *buf.as_mut_ptr().offset(8 as i32 as isize).offset(4 as i32 as isize) = ((*ctx)
        .data_size >> 32 as i32 & 0xff as i32 as u64) as uint8_t;
    *buf.as_mut_ptr().offset(8 as i32 as isize).offset(3 as i32 as isize) = ((*ctx)
        .data_size >> 24 as i32 & 0xff as i32 as u64) as uint8_t;
    *buf.as_mut_ptr().offset(8 as i32 as isize).offset(2 as i32 as isize) = ((*ctx)
        .data_size >> 16 as i32 & 0xff as i32 as u64) as uint8_t;
    *buf.as_mut_ptr().offset(8 as i32 as isize).offset(1 as i32 as isize) = ((*ctx)
        .data_size >> 8 as i32 & 0xff as i32 as u64) as uint8_t;
    *buf.as_mut_ptr().offset(8 as i32 as isize).offset(0 as i32 as isize) = ((*ctx)
        .data_size & 0xff as i32 as u64) as uint8_t;
    _nettle_poly1305_block(&mut (*ctx).poly1305, buf.as_mut_ptr(), 1 as i32 as u32);
    _nettle_poly1305_digest(&mut (*ctx).poly1305, &mut (*ctx).s);
    memcpy(
        digest as *mut libc::c_void,
        &mut (*ctx).s.b as *mut [uint8_t; 16] as *const libc::c_void,
        length,
    );
}