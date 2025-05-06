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
    fn nettle_aes128_set_encrypt_key(ctx: *mut aes128_ctx, key: *const uint8_t);
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
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
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
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
pub struct poly1305_aes_ctx {
    pub pctx: poly1305_ctx,
    pub block: [uint8_t; 16],
    pub index: u32,
    pub nonce: [uint8_t; 16],
    pub aes: aes128_ctx,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_poly1305_aes_set_key(
    mut ctx: *mut poly1305_aes_ctx,
    mut key: *const uint8_t,
) {
    nettle_aes128_set_encrypt_key(&mut (*ctx).aes, key);
    _nettle_poly1305_set_key(&mut (*ctx).pctx, key.offset(16 as i32 as isize));
    (*ctx).index = 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_poly1305_aes_set_nonce(
    mut ctx: *mut poly1305_aes_ctx,
    mut nonce: *const uint8_t,
) {
    memcpy(
        ((*ctx).nonce).as_mut_ptr() as *mut libc::c_void,
        nonce as *const libc::c_void,
        16 as i32 as u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_poly1305_aes_update(
    mut ctx: *mut poly1305_aes_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    (*ctx).index = _nettle_poly1305_update(
        &mut (*ctx).pctx,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        length,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_poly1305_aes_digest(
    mut ctx: *mut poly1305_aes_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut s: nettle_block16 = nettle_block16 { b: [0; 16] };
    if (*ctx).index > 0 as i32 as u32 {
        if (*ctx).index < 16 as i32 as u32 {} else {
            __assert_fail(
                b"ctx->index < POLY1305_BLOCK_SIZE\0" as *const u8 as *const i8,
                b"poly1305-aes.c\0" as *const u8 as *const i8,
                75 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[i8; 78],
                >(
                    b"void nettle_poly1305_aes_digest(struct poly1305_aes_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_908: {
            if (*ctx).index < 16 as i32 as u32 {} else {
                __assert_fail(
                    b"ctx->index < POLY1305_BLOCK_SIZE\0" as *const u8 as *const i8,
                    b"poly1305-aes.c\0" as *const u8 as *const i8,
                    75 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 78],
                        &[i8; 78],
                    >(
                        b"void nettle_poly1305_aes_digest(struct poly1305_aes_ctx *, size_t, uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        (*ctx).block[(*ctx).index as usize] = 1 as i32 as uint8_t;
        memset(
            ((*ctx).block)
                .as_mut_ptr()
                .offset((*ctx).index as isize)
                .offset(1 as i32 as isize) as *mut libc::c_void,
            0 as i32,
            ((16 as i32 - 1 as i32) as u32).wrapping_sub((*ctx).index) as u64,
        );
        _nettle_poly1305_block(
            &mut (*ctx).pctx,
            ((*ctx).block).as_mut_ptr(),
            0 as i32 as u32,
        );
    }
    nettle_aes128_encrypt(
        &mut (*ctx).aes,
        16 as i32 as size_t,
        (s.b).as_mut_ptr(),
        ((*ctx).nonce).as_mut_ptr(),
    );
    _nettle_poly1305_digest(&mut (*ctx).pctx, &mut s);
    memcpy(
        digest as *mut libc::c_void,
        (s.b).as_mut_ptr() as *const libc::c_void,
        length,
    );
    let mut increment_i: u32 = (16 as i32 - 1 as i32) as u32;
    (*ctx).nonce[increment_i as usize] = ((*ctx).nonce[increment_i as usize])
        .wrapping_add(1);
    if (*ctx).nonce[increment_i as usize] as i32 == 0 as i32 {
        while increment_i > 0 as i32 as u32
            && {
                increment_i = increment_i.wrapping_sub(1);
                (*ctx).nonce[increment_i as usize] = ((*ctx).nonce[increment_i as usize])
                    .wrapping_add(1);
                (*ctx).nonce[increment_i as usize] as i32 == 0 as i32
            }
        {}
    }
    (*ctx).index = 0 as i32 as u32;
}