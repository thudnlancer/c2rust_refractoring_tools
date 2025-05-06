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
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn _nettle_umac_set_key(
        l1_key: *mut uint32_t,
        l2_key: *mut uint32_t,
        l3_key1: *mut uint64_t,
        l3_key2: *mut uint32_t,
        pad: *mut aes128_ctx,
        key: *const uint8_t,
        n: u32,
    );
    fn _nettle_umac_nh_n(
        out: *mut uint64_t,
        n: u32,
        key: *const uint32_t,
        length: u32,
        msg: *const uint8_t,
    );
    fn _nettle_umac_l2(
        key: *const uint32_t,
        state: *mut uint64_t,
        n: u32,
        count: uint64_t,
        m: *const uint64_t,
    );
    fn _nettle_umac_l2_final(
        key: *const uint32_t,
        state: *mut uint64_t,
        n: u32,
        count: uint64_t,
    );
    fn _nettle_umac_l3(key: *const uint64_t, m: *const uint64_t) -> uint32_t;
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
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct umac128_ctx {
    pub l1_key: [uint32_t; 268],
    pub l2_key: [uint32_t; 24],
    pub l3_key1: [uint64_t; 32],
    pub l3_key2: [uint32_t; 4],
    pub pdf_key: aes128_ctx,
    pub l2_state: [uint64_t; 12],
    pub nonce: [uint8_t; 16],
    pub nonce_length: libc::c_ushort,
    pub index: u32,
    pub count: uint64_t,
    pub block: [uint8_t; 1024],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_umac128_set_key(
    mut ctx: *mut umac128_ctx,
    mut key: *const uint8_t,
) {
    _nettle_umac_set_key(
        ((*ctx).l1_key).as_mut_ptr(),
        ((*ctx).l2_key).as_mut_ptr(),
        ((*ctx).l3_key1).as_mut_ptr(),
        ((*ctx).l3_key2).as_mut_ptr(),
        &mut (*ctx).pdf_key,
        key,
        4 as i32 as u32,
    );
    memset(
        ((*ctx).nonce).as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[uint8_t; 16]>() as u64,
    );
    (*ctx).nonce_length = ::core::mem::size_of::<[uint8_t; 16]>() as u64
        as libc::c_ushort;
    (*ctx).index = 0 as i32 as u32;
    (*ctx).count = (*ctx).index as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_umac128_set_nonce(
    mut ctx: *mut umac128_ctx,
    mut nonce_length: size_t,
    mut nonce: *const uint8_t,
) {
    if nonce_length > 0 as i32 as u64 {} else {
        __assert_fail(
            b"nonce_length > 0\0" as *const u8 as *const i8,
            b"umac128.c\0" as *const u8 as *const i8,
            62 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 77],
                &[i8; 77],
            >(
                b"void nettle_umac128_set_nonce(struct umac128_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_886: {
        if nonce_length > 0 as i32 as u64 {} else {
            __assert_fail(
                b"nonce_length > 0\0" as *const u8 as *const i8,
                b"umac128.c\0" as *const u8 as *const i8,
                62 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 77],
                    &[i8; 77],
                >(
                    b"void nettle_umac128_set_nonce(struct umac128_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if nonce_length <= 16 as i32 as u64 {} else {
        __assert_fail(
            b"nonce_length <= AES_BLOCK_SIZE\0" as *const u8 as *const i8,
            b"umac128.c\0" as *const u8 as *const i8,
            63 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 77],
                &[i8; 77],
            >(
                b"void nettle_umac128_set_nonce(struct umac128_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_843: {
        if nonce_length <= 16 as i32 as u64 {} else {
            __assert_fail(
                b"nonce_length <= AES_BLOCK_SIZE\0" as *const u8 as *const i8,
                b"umac128.c\0" as *const u8 as *const i8,
                63 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 77],
                    &[i8; 77],
                >(
                    b"void nettle_umac128_set_nonce(struct umac128_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    memcpy(
        ((*ctx).nonce).as_mut_ptr() as *mut libc::c_void,
        nonce as *const libc::c_void,
        nonce_length,
    );
    memset(
        ((*ctx).nonce).as_mut_ptr().offset(nonce_length as isize) as *mut libc::c_void,
        0 as i32,
        (16 as i32 as u64).wrapping_sub(nonce_length),
    );
    (*ctx).nonce_length = nonce_length as libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_umac128_update(
    mut ctx: *mut umac128_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block: u64;
    if !(length == 0) {
        if (*ctx).index != 0 {
            let mut __md_left: u32 = (::core::mem::size_of::<[uint8_t; 1024]>() as u64)
                .wrapping_sub((*ctx).index as u64) as u32;
            if length < __md_left as u64 {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    data as *const libc::c_void,
                    length,
                );
                (*ctx).index = ((*ctx).index as u64).wrapping_add(length) as u32 as u32;
                current_block = 14648156034262866959;
            } else {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    data as *const libc::c_void,
                    __md_left as u64,
                );
                let mut __umac128_y: [uint64_t; 4] = [0; 4];
                _nettle_umac_nh_n(
                    __umac128_y.as_mut_ptr(),
                    4 as i32 as u32,
                    ((*ctx).l1_key).as_mut_ptr(),
                    1024 as i32 as u32,
                    ((*ctx).block).as_mut_ptr(),
                );
                __umac128_y[0 as i32 as usize] = (__umac128_y[0 as i32 as usize] as u64)
                    .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                    as uint64_t;
                __umac128_y[1 as i32 as usize] = (__umac128_y[1 as i32 as usize] as u64)
                    .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                    as uint64_t;
                __umac128_y[2 as i32 as usize] = (__umac128_y[2 as i32 as usize] as u64)
                    .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                    as uint64_t;
                __umac128_y[3 as i32 as usize] = (__umac128_y[3 as i32 as usize] as u64)
                    .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                    as uint64_t;
                let fresh0 = (*ctx).count;
                (*ctx).count = ((*ctx).count).wrapping_add(1);
                _nettle_umac_l2(
                    ((*ctx).l2_key).as_mut_ptr(),
                    ((*ctx).l2_state).as_mut_ptr(),
                    4 as i32 as u32,
                    fresh0,
                    __umac128_y.as_mut_ptr(),
                );
                data = data.offset(__md_left as isize);
                length = (length as u64).wrapping_sub(__md_left as u64) as size_t
                    as size_t;
                current_block = 9606288038608642794;
            }
        } else {
            current_block = 9606288038608642794;
        }
        match current_block {
            14648156034262866959 => {}
            _ => {
                while length >= ::core::mem::size_of::<[uint8_t; 1024]>() as u64 {
                    let mut __umac128_y_0: [uint64_t; 4] = [0; 4];
                    _nettle_umac_nh_n(
                        __umac128_y_0.as_mut_ptr(),
                        4 as i32 as u32,
                        ((*ctx).l1_key).as_mut_ptr(),
                        1024 as i32 as u32,
                        data,
                    );
                    __umac128_y_0[0 as i32 as usize] = (__umac128_y_0[0 as i32 as usize]
                        as u64)
                        .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                        as uint64_t;
                    __umac128_y_0[1 as i32 as usize] = (__umac128_y_0[1 as i32 as usize]
                        as u64)
                        .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                        as uint64_t;
                    __umac128_y_0[2 as i32 as usize] = (__umac128_y_0[2 as i32 as usize]
                        as u64)
                        .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                        as uint64_t;
                    __umac128_y_0[3 as i32 as usize] = (__umac128_y_0[3 as i32 as usize]
                        as u64)
                        .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                        as uint64_t;
                    let fresh1 = (*ctx).count;
                    (*ctx).count = ((*ctx).count).wrapping_add(1);
                    _nettle_umac_l2(
                        ((*ctx).l2_key).as_mut_ptr(),
                        ((*ctx).l2_state).as_mut_ptr(),
                        4 as i32 as u32,
                        fresh1,
                        __umac128_y_0.as_mut_ptr(),
                    );
                    data = data
                        .offset(
                            ::core::mem::size_of::<[uint8_t; 1024]>() as u64 as isize,
                        );
                    length = (length as u64)
                        .wrapping_sub(::core::mem::size_of::<[uint8_t; 1024]>() as u64)
                        as size_t as size_t;
                }
                memcpy(
                    ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
                    data as *const libc::c_void,
                    length,
                );
                (*ctx).index = length as u32;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_umac128_digest(
    mut ctx: *mut umac128_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut tag: [uint32_t; 4] = [0; 4];
    let mut i: u32 = 0;
    if length > 0 as i32 as u64 {} else {
        __assert_fail(
            b"length > 0\0" as *const u8 as *const i8,
            b"umac128.c\0" as *const u8 as *const i8,
            96 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 68],
                &[i8; 68],
            >(b"void nettle_umac128_digest(struct umac128_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1771: {
        if length > 0 as i32 as u64 {} else {
            __assert_fail(
                b"length > 0\0" as *const u8 as *const i8,
                b"umac128.c\0" as *const u8 as *const i8,
                96 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 68],
                    &[i8; 68],
                >(
                    b"void nettle_umac128_digest(struct umac128_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length <= 16 as i32 as u64 {} else {
        __assert_fail(
            b"length <= 16\0" as *const u8 as *const i8,
            b"umac128.c\0" as *const u8 as *const i8,
            97 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 68],
                &[i8; 68],
            >(b"void nettle_umac128_digest(struct umac128_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1732: {
        if length <= 16 as i32 as u64 {} else {
            __assert_fail(
                b"length <= 16\0" as *const u8 as *const i8,
                b"umac128.c\0" as *const u8 as *const i8,
                97 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 68],
                    &[i8; 68],
                >(
                    b"void nettle_umac128_digest(struct umac128_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ctx).index > 0 as i32 as u32 || (*ctx).count == 0 as i32 as u64 {
        let mut y: [uint64_t; 4] = [0; 4];
        let mut pad: u32 = if (*ctx).index > 0 as i32 as u32 {
            31 as i32 as u32 & ((*ctx).index).wrapping_neg()
        } else {
            32 as i32 as u32
        };
        memset(
            ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                as *mut libc::c_void,
            0 as i32,
            pad as u64,
        );
        _nettle_umac_nh_n(
            y.as_mut_ptr(),
            4 as i32 as u32,
            ((*ctx).l1_key).as_mut_ptr(),
            ((*ctx).index).wrapping_add(pad),
            ((*ctx).block).as_mut_ptr(),
        );
        y[0 as i32 as usize] = (y[0 as i32 as usize] as u64)
            .wrapping_add((8 as i32 as u32).wrapping_mul((*ctx).index) as u64)
            as uint64_t as uint64_t;
        y[1 as i32 as usize] = (y[1 as i32 as usize] as u64)
            .wrapping_add((8 as i32 as u32).wrapping_mul((*ctx).index) as u64)
            as uint64_t as uint64_t;
        y[2 as i32 as usize] = (y[2 as i32 as usize] as u64)
            .wrapping_add((8 as i32 as u32).wrapping_mul((*ctx).index) as u64)
            as uint64_t as uint64_t;
        y[3 as i32 as usize] = (y[3 as i32 as usize] as u64)
            .wrapping_add((8 as i32 as u32).wrapping_mul((*ctx).index) as u64)
            as uint64_t as uint64_t;
        let fresh2 = (*ctx).count;
        (*ctx).count = ((*ctx).count).wrapping_add(1);
        _nettle_umac_l2(
            ((*ctx).l2_key).as_mut_ptr(),
            ((*ctx).l2_state).as_mut_ptr(),
            4 as i32 as u32,
            fresh2,
            y.as_mut_ptr(),
        );
    }
    if (*ctx).count > 0 as i32 as u64 {} else {
        __assert_fail(
            b"ctx->count > 0\0" as *const u8 as *const i8,
            b"umac128.c\0" as *const u8 as *const i8,
            113 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 68],
                &[i8; 68],
            >(b"void nettle_umac128_digest(struct umac128_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1528: {
        if (*ctx).count > 0 as i32 as u64 {} else {
            __assert_fail(
                b"ctx->count > 0\0" as *const u8 as *const i8,
                b"umac128.c\0" as *const u8 as *const i8,
                113 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 68],
                    &[i8; 68],
                >(
                    b"void nettle_umac128_digest(struct umac128_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    nettle_aes128_encrypt(
        &mut (*ctx).pdf_key,
        16 as i32 as size_t,
        tag.as_mut_ptr() as *mut uint8_t,
        ((*ctx).nonce).as_mut_ptr(),
    );
    let mut increment_i: u32 = ((*ctx).nonce_length as i32 - 1 as i32) as u32;
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
    _nettle_umac_l2_final(
        ((*ctx).l2_key).as_mut_ptr(),
        ((*ctx).l2_state).as_mut_ptr(),
        4 as i32 as u32,
        (*ctx).count,
    );
    i = 0 as i32 as u32;
    while i < 4 as i32 as u32 {
        tag[i as usize]
            ^= (*ctx).l3_key2[i as usize]
                ^ _nettle_umac_l3(
                    ((*ctx).l3_key1)
                        .as_mut_ptr()
                        .offset((8 as i32 as u32).wrapping_mul(i) as isize),
                    ((*ctx).l2_state)
                        .as_mut_ptr()
                        .offset((2 as i32 as u32).wrapping_mul(i) as isize),
                );
        i = i.wrapping_add(1);
        i;
    }
    memcpy(digest as *mut libc::c_void, tag.as_mut_ptr() as *const libc::c_void, length);
    (*ctx).index = 0 as i32 as u32;
    (*ctx).count = (*ctx).index as uint64_t;
}