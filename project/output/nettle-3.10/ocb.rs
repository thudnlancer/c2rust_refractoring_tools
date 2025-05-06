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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn nettle_memxor3(
        dst: *mut libc::c_void,
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn nettle_memeql_sec(
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> i32;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = u64;
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
pub struct ocb_key {
    pub L: [nettle_block16; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ocb_ctx {
    pub initial: nettle_block16,
    pub offset: nettle_block16,
    pub sum: nettle_block16,
    pub checksum: nettle_block16,
    pub data_count: size_t,
    pub message_count: size_t,
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
#[inline]
unsafe extern "C" fn block16_xor(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    (*r).u64_0[0 as i32 as usize] ^= (*x).u64_0[0 as i32 as usize];
    (*r).u64_0[1 as i32 as usize] ^= (*x).u64_0[1 as i32 as usize];
}
#[inline]
unsafe extern "C" fn block16_set(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    (*r).u64_0[0 as i32 as usize] = (*x).u64_0[0 as i32 as usize];
    (*r).u64_0[1 as i32 as usize] = (*x).u64_0[1 as i32 as usize];
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
unsafe extern "C" fn extract(
    mut u0: uint64_t,
    mut u1: uint64_t,
    mut offset: u32,
) -> uint64_t {
    if offset == 0 as i32 as u32 {
        return u0;
    }
    u0 = (u0 as libc::c_ulonglong).swap_bytes() as uint64_t;
    u1 = (u1 as libc::c_ulonglong).swap_bytes() as uint64_t;
    return ((u0 << offset | u1 >> (64 as i32 as u32).wrapping_sub(offset))
        as libc::c_ulonglong)
        .swap_bytes() as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ocb_set_key(
    mut key: *mut ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option<nettle_cipher_func>,
) {
    static mut zero_block: nettle_block16 = nettle_block16 { b: [0; 16] };
    f
        .expect(
            "non-null function pointer",
        )(
        cipher,
        16 as i32 as size_t,
        ((*key).L[0 as i32 as usize].b).as_mut_ptr(),
        (zero_block.b).as_ptr(),
    );
    block16_mulx_be(
        &mut *((*key).L).as_mut_ptr().offset(1 as i32 as isize),
        &mut *((*key).L).as_mut_ptr().offset(0 as i32 as isize),
    );
    block16_mulx_be(
        &mut *((*key).L).as_mut_ptr().offset(2 as i32 as isize),
        &mut *((*key).L).as_mut_ptr().offset(1 as i32 as isize),
    );
}
unsafe extern "C" fn update_offset(
    mut key: *const ocb_key,
    mut offset: *mut nettle_block16,
    mut i: size_t,
) {
    if i & 1 as i32 as u64 != 0 {
        block16_xor(offset, &*((*key).L).as_ptr().offset(2 as i32 as isize));
    } else {
        if i > 0 as i32 as u64 {} else {
            __assert_fail(
                b"i > 0\0" as *const u8 as *const i8,
                b"ocb.c\0" as *const u8 as *const i8,
                76 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[i8; 75],
                >(
                    b"void update_offset(const struct ocb_key *, union nettle_block16 *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_1697: {
            if i > 0 as i32 as u64 {} else {
                __assert_fail(
                    b"i > 0\0" as *const u8 as *const i8,
                    b"ocb.c\0" as *const u8 as *const i8,
                    76 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 75],
                        &[i8; 75],
                    >(
                        b"void update_offset(const struct ocb_key *, union nettle_block16 *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut diff: nettle_block16 = nettle_block16 { b: [0; 16] };
        block16_mulx_be(&mut diff, &*((*key).L).as_ptr().offset(2 as i32 as isize));
        i >>= 1 as i32;
        while i & 1 as i32 as u64 == 0 {
            block16_mulx_be(&mut diff, &mut diff);
            i >>= 1 as i32;
        }
        block16_xor(offset, &mut diff);
    };
}
unsafe extern "C" fn pad_block(
    mut block: *mut nettle_block16,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    memcpy(
        ((*block).b).as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        length,
    );
    (*block).b[length as usize] = 0x80 as i32 as uint8_t;
    memset(
        ((*block).b).as_mut_ptr().offset(length as isize).offset(1 as i32 as isize)
            as *mut libc::c_void,
        0 as i32,
        ((16 as i32 - 1 as i32) as u64).wrapping_sub(length),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ocb_set_nonce(
    mut ctx: *mut ocb_ctx,
    mut cipher: *const libc::c_void,
    mut f: Option<nettle_cipher_func>,
    mut tag_length: size_t,
    mut nonce_length: size_t,
    mut nonce: *const uint8_t,
) {
    let mut top: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut stretch: uint64_t = 0;
    let mut bottom: u32 = 0;
    if nonce_length < 16 as i32 as u64 {} else {
        __assert_fail(
            b"nonce_length < 16\0" as *const u8 as *const i8,
            b"ocb.c\0" as *const u8 as *const i8,
            104 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 113],
                &[i8; 113],
            >(
                b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1196: {
        if nonce_length < 16 as i32 as u64 {} else {
            __assert_fail(
                b"nonce_length < 16\0" as *const u8 as *const i8,
                b"ocb.c\0" as *const u8 as *const i8,
                104 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 113],
                    &[i8; 113],
                >(
                    b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if tag_length > 0 as i32 as u64 {} else {
        __assert_fail(
            b"tag_length > 0\0" as *const u8 as *const i8,
            b"ocb.c\0" as *const u8 as *const i8,
            105 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 113],
                &[i8; 113],
            >(
                b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1157: {
        if tag_length > 0 as i32 as u64 {} else {
            __assert_fail(
                b"tag_length > 0\0" as *const u8 as *const i8,
                b"ocb.c\0" as *const u8 as *const i8,
                105 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 113],
                    &[i8; 113],
                >(
                    b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if tag_length <= 16 as i32 as u64 {} else {
        __assert_fail(
            b"tag_length <= 16\0" as *const u8 as *const i8,
            b"ocb.c\0" as *const u8 as *const i8,
            106 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 113],
                &[i8; 113],
            >(
                b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1108: {
        if tag_length <= 16 as i32 as u64 {} else {
            __assert_fail(
                b"tag_length <= 16\0" as *const u8 as *const i8,
                b"ocb.c\0" as *const u8 as *const i8,
                106 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 113],
                    &[i8; 113],
                >(
                    b"void nettle_ocb_set_nonce(struct ocb_ctx *, const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    top.b[0 as i32 as usize] = ((tag_length & 15 as i32 as u64) << 4 as i32) as uint8_t;
    memset(
        (top.b).as_mut_ptr().offset(1 as i32 as isize) as *mut libc::c_void,
        0 as i32,
        (15 as i32 as u64).wrapping_sub(nonce_length),
    );
    top.b[(15 as i32 as u64).wrapping_sub(nonce_length) as usize] = (top
        .b[(15 as i32 as u64).wrapping_sub(nonce_length) as usize] as i32 | 1 as i32)
        as uint8_t;
    memcpy(
        (top.b).as_mut_ptr().offset(16 as i32 as isize).offset(-(nonce_length as isize))
            as *mut libc::c_void,
        nonce as *const libc::c_void,
        nonce_length,
    );
    bottom = (top.b[15 as i32 as usize] as i32 & 0x3f as i32) as u32;
    top.b[15 as i32 as usize] = (top.b[15 as i32 as usize] as i32 & 0xc0 as i32)
        as uint8_t;
    f
        .expect(
            "non-null function pointer",
        )(cipher, 16 as i32 as size_t, (top.b).as_mut_ptr(), (top.b).as_mut_ptr());
    stretch = top.u64_0[0 as i32 as usize];
    stretch
        ^= top.u64_0[0 as i32 as usize] >> 8 as i32
            | top.u64_0[1 as i32 as usize] << 56 as i32;
    (*ctx).initial.u64_0[0 as i32 as usize] = extract(
        top.u64_0[0 as i32 as usize],
        top.u64_0[1 as i32 as usize],
        bottom,
    );
    (*ctx).initial.u64_0[1 as i32 as usize] = extract(
        top.u64_0[1 as i32 as usize],
        stretch,
        bottom,
    );
    (*ctx).sum.u64_0[1 as i32 as usize] = 0 as i32 as uint64_t;
    (*ctx).sum.u64_0[0 as i32 as usize] = (*ctx).sum.u64_0[1 as i32 as usize];
    (*ctx).checksum.u64_0[1 as i32 as usize] = 0 as i32 as uint64_t;
    (*ctx).checksum.u64_0[0 as i32 as usize] = (*ctx).checksum.u64_0[1 as i32 as usize];
    (*ctx).message_count = 0 as i32 as size_t;
    (*ctx).data_count = (*ctx).message_count;
}
unsafe extern "C" fn ocb_fill_n(
    mut key: *const ocb_key,
    mut offset: *mut nettle_block16,
    mut count: size_t,
    mut n: size_t,
    mut o: *mut nettle_block16,
) {
    if n > 0 as i32 as u64 {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const i8,
            b"ocb.c\0" as *const u8 as *const i8,
            138 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[i8; 104],
            >(
                b"void ocb_fill_n(const struct ocb_key *, union nettle_block16 *, size_t, size_t, union nettle_block16 *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1980: {
        if n > 0 as i32 as u64 {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const i8,
                b"ocb.c\0" as *const u8 as *const i8,
                138 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 104],
                    &[i8; 104],
                >(
                    b"void ocb_fill_n(const struct ocb_key *, union nettle_block16 *, size_t, size_t, union nettle_block16 *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut prev: *mut nettle_block16 = 0 as *mut nettle_block16;
    if count & 1 as i32 as u64 != 0 {
        prev = offset;
    } else {
        count = count.wrapping_add(1);
        count;
        block16_xor(offset, &*((*key).L).as_ptr().offset(2 as i32 as isize));
        block16_set(&mut *o.offset(0 as i32 as isize), offset);
        prev = o;
        n = n.wrapping_sub(1);
        n;
        o = o.offset(1);
        o;
    }
    while n >= 2 as i32 as u64 {
        let mut i: size_t = 0;
        count = (count as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
        block16_mulx_be(
            &mut *o.offset(0 as i32 as isize),
            &*((*key).L).as_ptr().offset(2 as i32 as isize),
        );
        i = count >> 1 as i32;
        while i & 1 as i32 as u64 == 0 {
            block16_mulx_be(
                &mut *o.offset(0 as i32 as isize),
                &mut *o.offset(0 as i32 as isize),
            );
            i >>= 1 as i32;
        }
        block16_xor(&mut *o.offset(0 as i32 as isize), prev);
        block16_xor3(
            &mut *o.offset(1 as i32 as isize),
            &mut *o.offset(0 as i32 as isize),
            &*((*key).L).as_ptr().offset(2 as i32 as isize),
        );
        prev = &mut *o.offset(1 as i32 as isize) as *mut nettle_block16;
        n = (n as u64).wrapping_sub(2 as i32 as u64) as size_t as size_t;
        o = o.offset(2 as i32 as isize);
    }
    block16_set(offset, prev);
    if n > 0 as i32 as u64 {
        count = count.wrapping_add(1);
        update_offset(key, offset, count);
        block16_set(o, offset);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ocb_update(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option<nettle_cipher_func>,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut block: [nettle_block16; 16] = [nettle_block16 { b: [0; 16] }; 16];
    let mut n: size_t = length.wrapping_div(16 as i32 as u64);
    if (*ctx).message_count == 0 as i32 as u64 {} else {
        __assert_fail(
            b"ctx->message_count == 0\0" as *const u8 as *const i8,
            b"ocb.c\0" as *const u8 as *const i8,
            183 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 126],
                &[i8; 126],
            >(
                b"void nettle_ocb_update(struct ocb_ctx *, const struct ocb_key *, const void *, nettle_cipher_func *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2049: {
        if (*ctx).message_count == 0 as i32 as u64 {} else {
            __assert_fail(
                b"ctx->message_count == 0\0" as *const u8 as *const i8,
                b"ocb.c\0" as *const u8 as *const i8,
                183 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 126],
                    &[i8; 126],
                >(
                    b"void nettle_ocb_update(struct ocb_ctx *, const struct ocb_key *, const void *, nettle_cipher_func *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ctx).data_count == 0 as i32 as u64 {
        (*ctx).offset.u64_0[1 as i32 as usize] = 0 as i32 as uint64_t;
        (*ctx).offset.u64_0[0 as i32 as usize] = (*ctx).offset.u64_0[1 as i32 as usize];
    }
    while n > 0 as i32 as u64 {
        let mut size: size_t = 0;
        let mut i: size_t = 0;
        let mut blocks: size_t = if n <= 16 as i32 as u64 {
            n
        } else {
            ((16 as i32 - 1 as i32) as u64)
                .wrapping_add((*ctx).data_count & 1 as i32 as u64)
        };
        ocb_fill_n(
            key,
            &mut (*ctx).offset,
            (*ctx).data_count,
            blocks,
            block.as_mut_ptr(),
        );
        (*ctx).data_count = ((*ctx).data_count as u64).wrapping_add(blocks) as size_t
            as size_t;
        size = blocks.wrapping_mul(16 as i32 as u64);
        nettle_memxor(
            (block[0 as i32 as usize].b).as_mut_ptr() as *mut libc::c_void,
            data as *const libc::c_void,
            size,
        );
        f
            .expect(
                "non-null function pointer",
            )(
            cipher,
            size,
            (block[0 as i32 as usize].b).as_mut_ptr(),
            (block[0 as i32 as usize].b).as_mut_ptr(),
        );
        i = 0 as i32 as size_t;
        while i < blocks {
            block16_xor(&mut (*ctx).sum, &mut *block.as_mut_ptr().offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        n = (n as u64).wrapping_sub(blocks) as size_t as size_t;
        data = data.offset(size as isize);
    }
    length &= 15 as i32 as u64;
    if length > 0 as i32 as u64 {
        let mut block_0: nettle_block16 = nettle_block16 { b: [0; 16] };
        pad_block(&mut block_0, length, data);
        block16_xor(&mut (*ctx).offset, &*((*key).L).as_ptr().offset(0 as i32 as isize));
        block16_xor(&mut block_0, &mut (*ctx).offset);
        f
            .expect(
                "non-null function pointer",
            )(
            cipher,
            16 as i32 as size_t,
            (block_0.b).as_mut_ptr(),
            (block_0.b).as_mut_ptr(),
        );
        block16_xor(&mut (*ctx).sum, &mut block_0);
    }
}
unsafe extern "C" fn ocb_crypt_n(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option<nettle_cipher_func>,
    mut n: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut o: [nettle_block16; 16] = [nettle_block16 { b: [0; 16] }; 16];
    let mut block: [nettle_block16; 16] = [nettle_block16 { b: [0; 16] }; 16];
    let mut size: size_t = 0;
    while n > 0 as i32 as u64 {
        let mut blocks: size_t = if n <= 16 as i32 as u64 {
            n
        } else {
            ((16 as i32 - 1 as i32) as u64)
                .wrapping_add((*ctx).message_count & 1 as i32 as u64)
        };
        ocb_fill_n(
            key,
            &mut (*ctx).offset,
            (*ctx).message_count,
            blocks,
            o.as_mut_ptr(),
        );
        (*ctx).message_count = ((*ctx).message_count as u64).wrapping_add(blocks)
            as size_t as size_t;
        size = blocks.wrapping_mul(16 as i32 as u64);
        nettle_memxor3(
            (block[0 as i32 as usize].b).as_mut_ptr() as *mut libc::c_void,
            (o[0 as i32 as usize].b).as_mut_ptr() as *const libc::c_void,
            src as *const libc::c_void,
            size,
        );
        f
            .expect(
                "non-null function pointer",
            )(
            cipher,
            size,
            (block[0 as i32 as usize].b).as_mut_ptr(),
            (block[0 as i32 as usize].b).as_mut_ptr(),
        );
        nettle_memxor3(
            dst as *mut libc::c_void,
            (block[0 as i32 as usize].b).as_mut_ptr() as *const libc::c_void,
            (o[0 as i32 as usize].b).as_mut_ptr() as *const libc::c_void,
            size,
        );
        n = (n as u64).wrapping_sub(blocks) as size_t as size_t;
        src = src.offset(size as isize);
        dst = dst.offset(size as isize);
    }
}
unsafe extern "C" fn ocb_checksum_n(
    mut checksum: *mut nettle_block16,
    mut n: size_t,
    mut src: *const uint8_t,
) {
    let mut initial: u32 = 0;
    let mut edge_word: uint64_t = 0 as i32 as uint64_t;
    let mut s0: uint64_t = 0;
    let mut s1: uint64_t = 0;
    if n == 1 as i32 as u64 {
        nettle_memxor(
            ((*checksum).b).as_mut_ptr() as *mut libc::c_void,
            src as *const libc::c_void,
            16 as i32 as size_t,
        );
        return;
    }
    initial = ((src as uintptr_t).wrapping_neg() & 7 as i32 as u64) as u32;
    if initial > 0 as i32 as u32 {
        let mut i: u32 = 0;
        i = initial;
        while i > 0 as i32 as u32 {
            let fresh0 = src;
            src = src.offset(1);
            edge_word = (edge_word << 8 as i32).wrapping_add(*fresh0 as u64);
            i = i.wrapping_sub(1);
            i;
        }
        n = n.wrapping_sub(1);
        n;
    }
    s1 = 0 as i32 as uint64_t;
    s0 = s1;
    while n > 0 as i32 as u64 {
        s0 ^= *(src as *const uint64_t).offset(0 as i32 as isize);
        s1 ^= *(src as *const uint64_t).offset(1 as i32 as isize);
        n = n.wrapping_sub(1);
        n;
        src = src.offset(16 as i32 as isize);
    }
    if initial > 0 as i32 as u32 {
        let mut i_0: u32 = 0;
        let mut mask: uint64_t = 0;
        s0 ^= *(src as *const uint64_t).offset(0 as i32 as isize);
        i_0 = (8 as i32 as u32).wrapping_sub(initial);
        src = src.offset(8 as i32 as isize);
        while i_0 > 0 as i32 as u32 {
            let fresh1 = src;
            src = src.offset(1);
            edge_word = (edge_word << 8 as i32).wrapping_add(*fresh1 as u64);
            i_0 = i_0.wrapping_sub(1);
            i_0;
        }
        let mut __rotate_t: uint64_t = s0 << (8 as i32 as u32).wrapping_mul(initial)
            | s1
                >> (64 as i32 as u32)
                    .wrapping_sub((8 as i32 as u32).wrapping_mul(initial));
        s1 = s1 << (8 as i32 as u32).wrapping_mul(initial)
            | s0
                >> (64 as i32 as u32)
                    .wrapping_sub((8 as i32 as u32).wrapping_mul(initial));
        s0 = __rotate_t;
        mask = ((1 as i32 as uint64_t) << (8 as i32 as u32).wrapping_mul(initial))
            .wrapping_sub(1 as i32 as u64);
        edge_word = (edge_word as libc::c_ulonglong).swap_bytes() as uint64_t;
        s0 ^= edge_word & mask;
        s1 ^= edge_word & !mask;
    }
    (*checksum).u64_0[0 as i32 as usize] ^= s0;
    (*checksum).u64_0[1 as i32 as usize] ^= s1;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ocb_encrypt(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option<nettle_cipher_func>,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut n: size_t = length.wrapping_div(16 as i32 as u64);
    if (*ctx).message_count == 0 as i32 as u64 {
        (*ctx).offset = (*ctx).initial;
    }
    if n > 0 as i32 as u64 {
        ocb_checksum_n(&mut (*ctx).checksum, n, src);
        ocb_crypt_n(ctx, key, cipher, f, n, dst, src);
        length &= 15 as i32 as u64;
    }
    if length > 0 as i32 as u64 {
        let mut block: nettle_block16 = nettle_block16 { b: [0; 16] };
        src = src.offset(n.wrapping_mul(16 as i32 as u64) as isize);
        dst = dst.offset(n.wrapping_mul(16 as i32 as u64) as isize);
        pad_block(&mut block, length, src);
        block16_xor(&mut (*ctx).checksum, &mut block);
        block16_xor(&mut (*ctx).offset, &*((*key).L).as_ptr().offset(0 as i32 as isize));
        f
            .expect(
                "non-null function pointer",
            )(
            cipher,
            16 as i32 as size_t,
            (block.b).as_mut_ptr(),
            ((*ctx).offset.b).as_mut_ptr(),
        );
        nettle_memxor3(
            dst as *mut libc::c_void,
            (block.b).as_mut_ptr() as *const libc::c_void,
            src as *const libc::c_void,
            length,
        );
        (*ctx).message_count = ((*ctx).message_count).wrapping_add(1);
        (*ctx).message_count;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ocb_decrypt(
    mut ctx: *mut ocb_ctx,
    mut key: *const ocb_key,
    mut encrypt_ctx: *const libc::c_void,
    mut encrypt: Option<nettle_cipher_func>,
    mut decrypt_ctx: *const libc::c_void,
    mut decrypt: Option<nettle_cipher_func>,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut n: size_t = length.wrapping_div(16 as i32 as u64);
    if (*ctx).message_count == 0 as i32 as u64 {
        (*ctx).offset = (*ctx).initial;
    }
    if n > 0 as i32 as u64 {
        ocb_crypt_n(ctx, key, decrypt_ctx, decrypt, n, dst, src);
        ocb_checksum_n(&mut (*ctx).checksum, n, dst);
        length &= 15 as i32 as u64;
    }
    if length > 0 as i32 as u64 {
        let mut block: nettle_block16 = nettle_block16 { b: [0; 16] };
        src = src.offset(n.wrapping_mul(16 as i32 as u64) as isize);
        dst = dst.offset(n.wrapping_mul(16 as i32 as u64) as isize);
        block16_xor(&mut (*ctx).offset, &*((*key).L).as_ptr().offset(0 as i32 as isize));
        encrypt
            .expect(
                "non-null function pointer",
            )(
            encrypt_ctx,
            16 as i32 as size_t,
            (block.b).as_mut_ptr(),
            ((*ctx).offset.b).as_mut_ptr(),
        );
        nettle_memxor3(
            dst as *mut libc::c_void,
            (block.b).as_mut_ptr() as *const libc::c_void,
            src as *const libc::c_void,
            length,
        );
        pad_block(&mut block, length, dst);
        block16_xor(&mut (*ctx).checksum, &mut block);
        (*ctx).message_count = ((*ctx).message_count).wrapping_add(1);
        (*ctx).message_count;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ocb_digest(
    mut ctx: *const ocb_ctx,
    mut key: *const ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option<nettle_cipher_func>,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut block: nettle_block16 = nettle_block16 { b: [0; 16] };
    if length <= 16 as i32 as u64 {} else {
        __assert_fail(
            b"length <= OCB_DIGEST_SIZE\0" as *const u8 as *const i8,
            b"ocb.c\0" as *const u8 as *const i8,
            391 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 126],
                &[i8; 126],
            >(
                b"void nettle_ocb_digest(const struct ocb_ctx *, const struct ocb_key *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3024: {
        if length <= 16 as i32 as u64 {} else {
            __assert_fail(
                b"length <= OCB_DIGEST_SIZE\0" as *const u8 as *const i8,
                b"ocb.c\0" as *const u8 as *const i8,
                391 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 126],
                    &[i8; 126],
                >(
                    b"void nettle_ocb_digest(const struct ocb_ctx *, const struct ocb_key *, const void *, nettle_cipher_func *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    block16_xor3(
        &mut block,
        &*((*key).L).as_ptr().offset(1 as i32 as isize),
        if (*ctx).message_count > 0 as i32 as u64 {
            &(*ctx).offset
        } else {
            &(*ctx).initial
        },
    );
    block16_xor(&mut block, &(*ctx).checksum);
    f
        .expect(
            "non-null function pointer",
        )(cipher, 16 as i32 as size_t, (block.b).as_mut_ptr(), (block.b).as_mut_ptr());
    nettle_memxor3(
        digest as *mut libc::c_void,
        (block.b).as_mut_ptr() as *const libc::c_void,
        ((*ctx).sum.b).as_ptr() as *const libc::c_void,
        length,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ocb_encrypt_message(
    mut key: *const ocb_key,
    mut cipher: *const libc::c_void,
    mut f: Option<nettle_cipher_func>,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut tlength: size_t,
    mut clength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut ctx: ocb_ctx = ocb_ctx {
        initial: nettle_block16 { b: [0; 16] },
        offset: nettle_block16 { b: [0; 16] },
        sum: nettle_block16 { b: [0; 16] },
        checksum: nettle_block16 { b: [0; 16] },
        data_count: 0,
        message_count: 0,
    };
    if clength >= tlength {} else {
        __assert_fail(
            b"clength >= tlength\0" as *const u8 as *const i8,
            b"ocb.c\0" as *const u8 as *const i8,
            408 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 186],
                &[i8; 186],
            >(
                b"void nettle_ocb_encrypt_message(const struct ocb_key *, const void *, nettle_cipher_func *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3163: {
        if clength >= tlength {} else {
            __assert_fail(
                b"clength >= tlength\0" as *const u8 as *const i8,
                b"ocb.c\0" as *const u8 as *const i8,
                408 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 186],
                    &[i8; 186],
                >(
                    b"void nettle_ocb_encrypt_message(const struct ocb_key *, const void *, nettle_cipher_func *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    nettle_ocb_set_nonce(&mut ctx, cipher, f, tlength, nlength, nonce);
    nettle_ocb_update(&mut ctx, key, cipher, f, alength, adata);
    nettle_ocb_encrypt(
        &mut ctx,
        key,
        cipher,
        f,
        clength.wrapping_sub(tlength),
        dst,
        src,
    );
    nettle_ocb_digest(
        &mut ctx,
        key,
        cipher,
        f,
        tlength,
        dst.offset(clength as isize).offset(-(tlength as isize)),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ocb_decrypt_message(
    mut key: *const ocb_key,
    mut encrypt_ctx: *const libc::c_void,
    mut encrypt: Option<nettle_cipher_func>,
    mut decrypt_ctx: *const libc::c_void,
    mut decrypt: Option<nettle_cipher_func>,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut tlength: size_t,
    mut mlength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) -> i32 {
    let mut ctx: ocb_ctx = ocb_ctx {
        initial: nettle_block16 { b: [0; 16] },
        offset: nettle_block16 { b: [0; 16] },
        sum: nettle_block16 { b: [0; 16] },
        checksum: nettle_block16 { b: [0; 16] },
        data_count: 0,
        message_count: 0,
    };
    let mut digest: nettle_block16 = nettle_block16 { b: [0; 16] };
    nettle_ocb_set_nonce(&mut ctx, encrypt_ctx, encrypt, tlength, nlength, nonce);
    nettle_ocb_update(&mut ctx, key, encrypt_ctx, encrypt, alength, adata);
    nettle_ocb_decrypt(
        &mut ctx,
        key,
        encrypt_ctx,
        encrypt,
        decrypt_ctx,
        decrypt,
        mlength,
        dst,
        src,
    );
    nettle_ocb_digest(
        &mut ctx,
        key,
        encrypt_ctx,
        encrypt,
        tlength,
        (digest.b).as_mut_ptr(),
    );
    return nettle_memeql_sec(
        (digest.b).as_mut_ptr() as *const libc::c_void,
        src.offset(mlength as isize) as *const libc::c_void,
        tlength,
    );
}