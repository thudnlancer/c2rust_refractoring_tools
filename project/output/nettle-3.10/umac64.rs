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
pub struct umac64_ctx {
    pub l1_key: [uint32_t; 260],
    pub l2_key: [uint32_t; 12],
    pub l3_key1: [uint64_t; 16],
    pub l3_key2: [uint32_t; 2],
    pub pdf_key: aes128_ctx,
    pub l2_state: [uint64_t; 6],
    pub nonce: [uint8_t; 16],
    pub nonce_length: libc::c_ushort,
    pub nonce_low: libc::c_ushort,
    pub pad_cache: [uint32_t; 4],
    pub index: u32,
    pub count: uint64_t,
    pub block: [uint8_t; 1024],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_umac64_set_key(
    mut ctx: *mut umac64_ctx,
    mut key: *const uint8_t,
) {
    _nettle_umac_set_key(
        ((*ctx).l1_key).as_mut_ptr(),
        ((*ctx).l2_key).as_mut_ptr(),
        ((*ctx).l3_key1).as_mut_ptr(),
        ((*ctx).l3_key2).as_mut_ptr(),
        &mut (*ctx).pdf_key,
        key,
        2 as i32 as u32,
    );
    memset(
        ((*ctx).nonce).as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[uint8_t; 16]>() as u64,
    );
    (*ctx).nonce_low = 0 as i32 as libc::c_ushort;
    (*ctx).nonce_length = ::core::mem::size_of::<[uint8_t; 16]>() as u64
        as libc::c_ushort;
    (*ctx).index = 0 as i32 as u32;
    (*ctx).count = (*ctx).index as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_umac64_set_nonce(
    mut ctx: *mut umac64_ctx,
    mut nonce_length: size_t,
    mut nonce: *const uint8_t,
) {
    if nonce_length > 0 as i32 as u64 {} else {
        __assert_fail(
            b"nonce_length > 0\0" as *const u8 as *const i8,
            b"umac64.c\0" as *const u8 as *const i8,
            63 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[i8; 75],
            >(
                b"void nettle_umac64_set_nonce(struct umac64_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_917: {
        if nonce_length > 0 as i32 as u64 {} else {
            __assert_fail(
                b"nonce_length > 0\0" as *const u8 as *const i8,
                b"umac64.c\0" as *const u8 as *const i8,
                63 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[i8; 75],
                >(
                    b"void nettle_umac64_set_nonce(struct umac64_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if nonce_length <= 16 as i32 as u64 {} else {
        __assert_fail(
            b"nonce_length <= AES_BLOCK_SIZE\0" as *const u8 as *const i8,
            b"umac64.c\0" as *const u8 as *const i8,
            64 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 75],
                &[i8; 75],
            >(
                b"void nettle_umac64_set_nonce(struct umac64_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_874: {
        if nonce_length <= 16 as i32 as u64 {} else {
            __assert_fail(
                b"nonce_length <= AES_BLOCK_SIZE\0" as *const u8 as *const i8,
                b"umac64.c\0" as *const u8 as *const i8,
                64 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[i8; 75],
                >(
                    b"void nettle_umac64_set_nonce(struct umac64_ctx *, size_t, const uint8_t *)\0",
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
    (*ctx).nonce_low = ((*ctx).nonce[nonce_length.wrapping_sub(1 as i32 as u64) as usize]
        as i32 & 1 as i32) as libc::c_ushort;
    (*ctx).nonce[nonce_length.wrapping_sub(1 as i32 as u64) as usize] = ((*ctx)
        .nonce[nonce_length.wrapping_sub(1 as i32 as u64) as usize] as i32 & !(1 as i32))
        as uint8_t;
    (*ctx).nonce_length = nonce_length as libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_umac64_update(
    mut ctx: *mut umac64_ctx,
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
                current_block = 18386322304582297246;
            } else {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    data as *const libc::c_void,
                    __md_left as u64,
                );
                let mut __umac64_y: [uint64_t; 2] = [0; 2];
                _nettle_umac_nh_n(
                    __umac64_y.as_mut_ptr(),
                    2 as i32 as u32,
                    ((*ctx).l1_key).as_mut_ptr(),
                    1024 as i32 as u32,
                    ((*ctx).block).as_mut_ptr(),
                );
                __umac64_y[0 as i32 as usize] = (__umac64_y[0 as i32 as usize] as u64)
                    .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                    as uint64_t;
                __umac64_y[1 as i32 as usize] = (__umac64_y[1 as i32 as usize] as u64)
                    .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                    as uint64_t;
                let fresh0 = (*ctx).count;
                (*ctx).count = ((*ctx).count).wrapping_add(1);
                _nettle_umac_l2(
                    ((*ctx).l2_key).as_mut_ptr(),
                    ((*ctx).l2_state).as_mut_ptr(),
                    2 as i32 as u32,
                    fresh0,
                    __umac64_y.as_mut_ptr(),
                );
                data = data.offset(__md_left as isize);
                length = (length as u64).wrapping_sub(__md_left as u64) as size_t
                    as size_t;
                current_block = 12599329904712511516;
            }
        } else {
            current_block = 12599329904712511516;
        }
        match current_block {
            18386322304582297246 => {}
            _ => {
                while length >= ::core::mem::size_of::<[uint8_t; 1024]>() as u64 {
                    let mut __umac64_y_0: [uint64_t; 2] = [0; 2];
                    _nettle_umac_nh_n(
                        __umac64_y_0.as_mut_ptr(),
                        2 as i32 as u32,
                        ((*ctx).l1_key).as_mut_ptr(),
                        1024 as i32 as u32,
                        data,
                    );
                    __umac64_y_0[0 as i32 as usize] = (__umac64_y_0[0 as i32 as usize]
                        as u64)
                        .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                        as uint64_t;
                    __umac64_y_0[1 as i32 as usize] = (__umac64_y_0[1 as i32 as usize]
                        as u64)
                        .wrapping_add((8 as i32 * 1024 as i32) as u64) as uint64_t
                        as uint64_t;
                    let fresh1 = (*ctx).count;
                    (*ctx).count = ((*ctx).count).wrapping_add(1);
                    _nettle_umac_l2(
                        ((*ctx).l2_key).as_mut_ptr(),
                        ((*ctx).l2_state).as_mut_ptr(),
                        2 as i32 as u32,
                        fresh1,
                        __umac64_y_0.as_mut_ptr(),
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
pub unsafe extern "C" fn nettle_umac64_digest(
    mut ctx: *mut umac64_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut tag: [uint32_t; 2] = [0; 2];
    let mut pad: *mut uint32_t = 0 as *mut uint32_t;
    if length > 0 as i32 as u64 {} else {
        __assert_fail(
            b"length > 0\0" as *const u8 as *const i8,
            b"umac64.c\0" as *const u8 as *const i8,
            97 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[i8; 66],
            >(b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1845: {
        if length > 0 as i32 as u64 {} else {
            __assert_fail(
                b"length > 0\0" as *const u8 as *const i8,
                b"umac64.c\0" as *const u8 as *const i8,
                97 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[i8; 66],
                >(
                    b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length <= 8 as i32 as u64 {} else {
        __assert_fail(
            b"length <= 8\0" as *const u8 as *const i8,
            b"umac64.c\0" as *const u8 as *const i8,
            98 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[i8; 66],
            >(b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1806: {
        if length <= 8 as i32 as u64 {} else {
            __assert_fail(
                b"length <= 8\0" as *const u8 as *const i8,
                b"umac64.c\0" as *const u8 as *const i8,
                98 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[i8; 66],
                >(
                    b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ctx).index > 0 as i32 as u32 || (*ctx).count == 0 as i32 as u64 {
        let mut y: [uint64_t; 2] = [0; 2];
        let mut pad_0: u32 = if (*ctx).index > 0 as i32 as u32 {
            31 as i32 as u32 & ((*ctx).index).wrapping_neg()
        } else {
            32 as i32 as u32
        };
        memset(
            ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                as *mut libc::c_void,
            0 as i32,
            pad_0 as u64,
        );
        _nettle_umac_nh_n(
            y.as_mut_ptr(),
            2 as i32 as u32,
            ((*ctx).l1_key).as_mut_ptr(),
            ((*ctx).index).wrapping_add(pad_0),
            ((*ctx).block).as_mut_ptr(),
        );
        y[0 as i32 as usize] = (y[0 as i32 as usize] as u64)
            .wrapping_add((8 as i32 as u32).wrapping_mul((*ctx).index) as u64)
            as uint64_t as uint64_t;
        y[1 as i32 as usize] = (y[1 as i32 as usize] as u64)
            .wrapping_add((8 as i32 as u32).wrapping_mul((*ctx).index) as u64)
            as uint64_t as uint64_t;
        let fresh2 = (*ctx).count;
        (*ctx).count = ((*ctx).count).wrapping_add(1);
        _nettle_umac_l2(
            ((*ctx).l2_key).as_mut_ptr(),
            ((*ctx).l2_state).as_mut_ptr(),
            2 as i32 as u32,
            fresh2,
            y.as_mut_ptr(),
        );
    }
    if (*ctx).count > 0 as i32 as u64 {} else {
        __assert_fail(
            b"ctx->count > 0\0" as *const u8 as *const i8,
            b"umac64.c\0" as *const u8 as *const i8,
            112 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[i8; 66],
            >(b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1630: {
        if (*ctx).count > 0 as i32 as u64 {} else {
            __assert_fail(
                b"ctx->count > 0\0" as *const u8 as *const i8,
                b"umac64.c\0" as *const u8 as *const i8,
                112 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[i8; 66],
                >(
                    b"void nettle_umac64_digest(struct umac64_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ctx).nonce_low as i32 & 0x80 as i32 == 0 {
        nettle_aes128_encrypt(
            &mut (*ctx).pdf_key,
            16 as i32 as size_t,
            ((*ctx).pad_cache).as_mut_ptr() as *mut uint8_t,
            ((*ctx).nonce).as_mut_ptr(),
        );
        (*ctx).nonce_low = ((*ctx).nonce_low as i32 | 0x80 as i32) as libc::c_ushort;
    }
    pad = ((*ctx).pad_cache)
        .as_mut_ptr()
        .offset((2 as i32 * ((*ctx).nonce_low as i32 & 1 as i32)) as isize);
    (*ctx).nonce_low = ((*ctx).nonce_low).wrapping_add(1);
    (*ctx).nonce_low;
    if (*ctx).nonce_low as i32 & 1 as i32 == 0 {
        let mut i: u32 = ((*ctx).nonce_length as i32 - 1 as i32) as u32;
        (*ctx).nonce_low = 0 as i32 as libc::c_ushort;
        (*ctx).nonce[i as usize] = ((*ctx).nonce[i as usize] as i32 + 2 as i32)
            as uint8_t;
        if (*ctx).nonce[i as usize] as i32 == 0 as i32 && i > 0 as i32 as u32 {
            let mut increment_i: u32 = i.wrapping_sub(1 as i32 as u32);
            (*ctx).nonce[increment_i as usize] = ((*ctx).nonce[increment_i as usize])
                .wrapping_add(1);
            if (*ctx).nonce[increment_i as usize] as i32 == 0 as i32 {
                while increment_i > 0 as i32 as u32
                    && {
                        increment_i = increment_i.wrapping_sub(1);
                        (*ctx).nonce[increment_i as usize] = ((*ctx)
                            .nonce[increment_i as usize])
                            .wrapping_add(1);
                        (*ctx).nonce[increment_i as usize] as i32 == 0 as i32
                    }
                {}
            }
        }
    }
    _nettle_umac_l2_final(
        ((*ctx).l2_key).as_mut_ptr(),
        ((*ctx).l2_state).as_mut_ptr(),
        2 as i32 as u32,
        (*ctx).count,
    );
    tag[0 as i32 as usize] = *pad.offset(0 as i32 as isize)
        ^ (*ctx).l3_key2[0 as i32 as usize]
        ^ _nettle_umac_l3(((*ctx).l3_key1).as_mut_ptr(), ((*ctx).l2_state).as_mut_ptr());
    tag[1 as i32 as usize] = *pad.offset(1 as i32 as isize)
        ^ (*ctx).l3_key2[1 as i32 as usize]
        ^ _nettle_umac_l3(
            ((*ctx).l3_key1).as_mut_ptr().offset(8 as i32 as isize),
            ((*ctx).l2_state).as_mut_ptr().offset(2 as i32 as isize),
        );
    memcpy(digest as *mut libc::c_void, tag.as_mut_ptr() as *const libc::c_void, length);
    (*ctx).index = 0 as i32 as u32;
    (*ctx).count = (*ctx).index as uint64_t;
}