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
    fn _nettle_siv_ghash_set_key(ctx: *mut gcm_key, key: *const nettle_block16);
    fn _nettle_siv_ghash_update(
        ctx: *const gcm_key,
        state: *mut nettle_block16,
        blocks: size_t,
        data: *const uint8_t,
    ) -> *const uint8_t;
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
    fn nettle_memeql_sec(
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> i32;
    fn _nettle_ctr_crypt16(
        ctx: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        fill: Option<nettle_fill16_func>,
        ctr: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
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
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const uint8_t,
) -> ();
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_cipher {
    pub name: *const i8,
    pub context_size: u32,
    pub block_size: u32,
    pub key_size: u32,
    pub set_encrypt_key: Option<nettle_set_key_func>,
    pub set_decrypt_key: Option<nettle_set_key_func>,
    pub encrypt: Option<nettle_cipher_func>,
    pub decrypt: Option<nettle_cipher_func>,
}
pub type nettle_fill16_func = unsafe extern "C" fn(
    *mut uint8_t,
    size_t,
    *mut nettle_block16,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}
#[inline]
unsafe extern "C" fn block16_zero(mut r: *mut nettle_block16) {
    static mut zero_block: nettle_block16 = nettle_block16 { b: [0; 16] };
    *r = zero_block;
}
#[inline]
unsafe extern "C" fn block16_bswap(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    let mut t: uint64_t = ((*x).u64_0[0 as i32 as usize] as libc::c_ulonglong)
        .swap_bytes() as uint64_t;
    (*r).u64_0[0 as i32 as usize] = ((*x).u64_0[1 as i32 as usize] as libc::c_ulonglong)
        .swap_bytes() as uint64_t;
    (*r).u64_0[1 as i32 as usize] = t;
}
unsafe extern "C" fn siv_gcm_derive_keys(
    mut ctx: *const libc::c_void,
    mut f: Option<nettle_cipher_func>,
    mut key_size: size_t,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut auth_key: *mut nettle_block16,
    mut encryption_key: *mut uint8_t,
) {
    let mut block: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut out: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut i: size_t = 0;
    block16_zero(&mut block);
    memcpy(
        (block.b).as_mut_ptr().offset(4 as i32 as isize) as *mut libc::c_void,
        nonce as *const libc::c_void,
        if nlength < 12 as i32 as u64 { nlength } else { 12 as i32 as u64 },
    );
    f
        .expect(
            "non-null function pointer",
        )(ctx, 16 as i32 as size_t, (out.b).as_mut_ptr(), (block.b).as_mut_ptr());
    (*auth_key).u64_0[0 as i32 as usize] = out.u64_0[0 as i32 as usize];
    block.b[0 as i32 as usize] = 1 as i32 as uint8_t;
    f
        .expect(
            "non-null function pointer",
        )(ctx, 16 as i32 as size_t, (out.b).as_mut_ptr(), (block.b).as_mut_ptr());
    (*auth_key).u64_0[1 as i32 as usize] = out.u64_0[0 as i32 as usize];
    if key_size.wrapping_rem(8 as i32 as u64) == 0 as i32 as u64
        && key_size.wrapping_div(8 as i32 as u64).wrapping_add(2 as i32 as u64)
            <= 255 as i32 as u64
    {} else {
        __assert_fail(
            b"key_size % 8 == 0 && key_size / 8 + 2 <= UINT8_MAX\0" as *const u8
                as *const i8,
            b"siv-gcm.c\0" as *const u8 as *const i8,
            71 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 129],
                &[i8; 129],
            >(
                b"void siv_gcm_derive_keys(const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *, union nettle_block16 *, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1316: {
        if key_size.wrapping_rem(8 as i32 as u64) == 0 as i32 as u64
            && key_size.wrapping_div(8 as i32 as u64).wrapping_add(2 as i32 as u64)
                <= 255 as i32 as u64
        {} else {
            __assert_fail(
                b"key_size % 8 == 0 && key_size / 8 + 2 <= UINT8_MAX\0" as *const u8
                    as *const i8,
                b"siv-gcm.c\0" as *const u8 as *const i8,
                71 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 129],
                    &[i8; 129],
                >(
                    b"void siv_gcm_derive_keys(const void *, nettle_cipher_func *, size_t, size_t, const uint8_t *, union nettle_block16 *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = 0 as i32 as size_t;
    while i < key_size {
        block.b[0 as i32 as usize] = (block.b[0 as i32 as usize]).wrapping_add(1);
        block.b[0 as i32 as usize];
        f
            .expect(
                "non-null function pointer",
            )(ctx, 16 as i32 as size_t, (out.b).as_mut_ptr(), (block.b).as_mut_ptr());
        memcpy(
            encryption_key.offset(i as isize) as *mut libc::c_void,
            (out.b).as_mut_ptr() as *const libc::c_void,
            8 as i32 as u64,
        );
        i = (i as u64).wrapping_add(8 as i32 as u64) as size_t as size_t;
    }
}
unsafe extern "C" fn siv_gcm_fill(
    mut ctr: *mut uint8_t,
    mut blocks: size_t,
    mut buffer: *mut nettle_block16,
) {
    let mut c: uint32_t = 0;
    c = (*ctr.offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*ctr.offset(2 as i32 as isize) as uint32_t) << 16 as i32
        | (*ctr.offset(1 as i32 as isize) as uint32_t) << 8 as i32
        | *ctr.offset(0 as i32 as isize) as uint32_t;
    loop {
        let fresh0 = blocks;
        blocks = blocks.wrapping_sub(1);
        if !(fresh0 > 0 as i32 as u64) {
            break;
        }
        memcpy(
            ((*buffer).b).as_mut_ptr().offset(4 as i32 as isize) as *mut libc::c_void,
            ctr.offset(4 as i32 as isize) as *const libc::c_void,
            (16 as i32 - 4 as i32) as u64,
        );
        (*buffer).b[3 as i32 as usize] = (c >> 24 as i32 & 0xff as i32 as u32)
            as uint8_t;
        (*buffer).b[2 as i32 as usize] = (c >> 16 as i32 & 0xff as i32 as u32)
            as uint8_t;
        (*buffer).b[1 as i32 as usize] = (c >> 8 as i32 & 0xff as i32 as u32) as uint8_t;
        (*buffer).b[0 as i32 as usize] = (c & 0xff as i32 as u32) as uint8_t;
        buffer = buffer.offset(1);
        buffer;
        c = c.wrapping_add(1);
        c;
    }
    *ctr.offset(3 as i32 as isize) = (c >> 24 as i32 & 0xff as i32 as u32) as uint8_t;
    *ctr.offset(2 as i32 as isize) = (c >> 16 as i32 & 0xff as i32 as u32) as uint8_t;
    *ctr.offset(1 as i32 as isize) = (c >> 8 as i32 & 0xff as i32 as u32) as uint8_t;
    *ctr.offset(0 as i32 as isize) = (c & 0xff as i32 as u32) as uint8_t;
}
unsafe extern "C" fn siv_ghash_pad_update(
    mut ctx: *mut gcm_key,
    mut state: *mut nettle_block16,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut blocks: size_t = 0;
    blocks = length.wrapping_div(16 as i32 as u64);
    if blocks > 0 as i32 as u64 {
        data = _nettle_siv_ghash_update(ctx, state, blocks, data);
        length &= 0xf as i32 as u64;
    }
    if length > 0 as i32 as u64 {
        let mut block: [uint8_t; 16] = [0; 16];
        memset(
            block.as_mut_ptr().offset(length as isize) as *mut libc::c_void,
            0 as i32,
            (16 as i32 as u64).wrapping_sub(length),
        );
        memcpy(
            block.as_mut_ptr() as *mut libc::c_void,
            data as *const libc::c_void,
            length,
        );
        _nettle_siv_ghash_update(ctx, state, 1 as i32 as size_t, block.as_mut_ptr());
    }
}
unsafe extern "C" fn siv_gcm_authenticate(
    mut ctx: *const libc::c_void,
    mut nc: *const nettle_cipher,
    mut authentication_key: *const nettle_block16,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut mlength: size_t,
    mut mdata: *const uint8_t,
    mut tag: *mut uint8_t,
) {
    let mut state: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut siv_ghash_key: gcm_key = gcm_key {
        h: [nettle_block16 { b: [0; 16] }; 256],
    };
    let mut block: nettle_block16 = nettle_block16 { b: [0; 16] };
    _nettle_siv_ghash_set_key(&mut siv_ghash_key, authentication_key);
    block16_zero(&mut state);
    siv_ghash_pad_update(&mut siv_ghash_key, &mut state, alength, adata);
    siv_ghash_pad_update(&mut siv_ghash_key, &mut state, mlength, mdata);
    block.u64_0[0 as i32 as usize] = alength.wrapping_mul(8 as i32 as u64);
    block.u64_0[1 as i32 as usize] = mlength.wrapping_mul(8 as i32 as u64);
    _nettle_siv_ghash_update(
        &mut siv_ghash_key,
        &mut state,
        1 as i32 as size_t,
        (block.b).as_mut_ptr(),
    );
    block16_bswap(&mut state, &mut state);
    nettle_memxor(
        (state.b).as_mut_ptr() as *mut libc::c_void,
        nonce as *const libc::c_void,
        12 as i32 as size_t,
    );
    state.b[15 as i32 as usize] = (state.b[15 as i32 as usize] as i32 & 0x7f as i32)
        as uint8_t;
    ((*nc).encrypt)
        .expect(
            "non-null function pointer",
        )(ctx, 16 as i32 as size_t, tag, (state.b).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn nettle_siv_gcm_encrypt_message(
    mut nc: *const nettle_cipher,
    mut ctx: *const libc::c_void,
    mut ctr_ctx: *mut libc::c_void,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut clength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut authentication_key: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut encryption_key: *mut uint8_t = 0 as *mut uint8_t;
    let mut ctr: [uint8_t; 16] = [0; 16];
    let mut tag: *mut uint8_t = dst
        .offset(clength as isize)
        .offset(-(16 as i32 as isize));
    if clength >= 16 as i32 as u64 {} else {
        __assert_fail(
            b"clength >= SIV_GCM_DIGEST_SIZE\0" as *const u8 as *const i8,
            b"siv-gcm.c\0" as *const u8 as *const i8,
            165 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 174],
                &[i8; 174],
            >(
                b"void nettle_siv_gcm_encrypt_message(const struct nettle_cipher *, const void *, void *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1556: {
        if clength >= 16 as i32 as u64 {} else {
            __assert_fail(
                b"clength >= SIV_GCM_DIGEST_SIZE\0" as *const u8 as *const i8,
                b"siv-gcm.c\0" as *const u8 as *const i8,
                165 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 174],
                    &[i8; 174],
                >(
                    b"void nettle_siv_gcm_encrypt_message(const struct nettle_cipher *, const void *, void *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if nlength == 12 as i32 as u64 {} else {
        __assert_fail(
            b"nlength == SIV_GCM_NONCE_SIZE\0" as *const u8 as *const i8,
            b"siv-gcm.c\0" as *const u8 as *const i8,
            166 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 174],
                &[i8; 174],
            >(
                b"void nettle_siv_gcm_encrypt_message(const struct nettle_cipher *, const void *, void *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1516: {
        if nlength == 12 as i32 as u64 {} else {
            __assert_fail(
                b"nlength == SIV_GCM_NONCE_SIZE\0" as *const u8 as *const i8,
                b"siv-gcm.c\0" as *const u8 as *const i8,
                166 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 174],
                    &[i8; 174],
                >(
                    b"void nettle_siv_gcm_encrypt_message(const struct nettle_cipher *, const void *, void *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::core::mem::size_of::<uint8_t>() as u64).wrapping_mul((*nc).key_size as u64)
            as usize,
    );
    encryption_key = fresh1.as_mut_ptr() as *mut uint8_t;
    siv_gcm_derive_keys(
        ctx,
        (*nc).encrypt,
        (*nc).key_size as size_t,
        nlength,
        nonce,
        &mut authentication_key,
        encryption_key,
    );
    ((*nc).set_encrypt_key).expect("non-null function pointer")(ctr_ctx, encryption_key);
    siv_gcm_authenticate(
        ctr_ctx,
        nc,
        &mut authentication_key,
        nonce,
        alength,
        adata,
        clength.wrapping_sub(16 as i32 as u64),
        src,
        tag,
    );
    memcpy(
        ctr.as_mut_ptr() as *mut libc::c_void,
        tag as *const libc::c_void,
        16 as i32 as u64,
    );
    ctr[15 as i32 as usize] = (ctr[15 as i32 as usize] as i32 | 0x80 as i32) as uint8_t;
    _nettle_ctr_crypt16(
        ctr_ctx,
        (*nc).encrypt,
        Some(siv_gcm_fill as nettle_fill16_func),
        ctr.as_mut_ptr(),
        clength.wrapping_sub(16 as i32 as u64),
        dst,
        src,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_siv_gcm_decrypt_message(
    mut nc: *const nettle_cipher,
    mut ctx: *const libc::c_void,
    mut ctr_ctx: *mut libc::c_void,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut mlength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) -> i32 {
    let mut authentication_key: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut encryption_key: *mut uint8_t = 0 as *mut uint8_t;
    let mut state: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut tag: [uint8_t; 16] = [0; 16];
    if nlength == 12 as i32 as u64 {} else {
        __assert_fail(
            b"nlength == SIV_GCM_NONCE_SIZE\0" as *const u8 as *const i8,
            b"siv-gcm.c\0" as *const u8 as *const i8,
            204 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 173],
                &[i8; 173],
            >(
                b"int nettle_siv_gcm_decrypt_message(const struct nettle_cipher *, const void *, void *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1764: {
        if nlength == 12 as i32 as u64 {} else {
            __assert_fail(
                b"nlength == SIV_GCM_NONCE_SIZE\0" as *const u8 as *const i8,
                b"siv-gcm.c\0" as *const u8 as *const i8,
                204 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 173],
                    &[i8; 173],
                >(
                    b"int nettle_siv_gcm_decrypt_message(const struct nettle_cipher *, const void *, void *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::core::mem::size_of::<uint8_t>() as u64).wrapping_mul((*nc).key_size as u64)
            as usize,
    );
    encryption_key = fresh2.as_mut_ptr() as *mut uint8_t;
    siv_gcm_derive_keys(
        ctx,
        (*nc).encrypt,
        (*nc).key_size as size_t,
        nlength,
        nonce,
        &mut authentication_key,
        encryption_key,
    );
    memcpy(
        (state.b).as_mut_ptr() as *mut libc::c_void,
        src.offset(mlength as isize) as *const libc::c_void,
        16 as i32 as u64,
    );
    state.b[15 as i32 as usize] = (state.b[15 as i32 as usize] as i32 | 0x80 as i32)
        as uint8_t;
    ((*nc).set_encrypt_key).expect("non-null function pointer")(ctr_ctx, encryption_key);
    _nettle_ctr_crypt16(
        ctr_ctx,
        (*nc).encrypt,
        Some(siv_gcm_fill as nettle_fill16_func),
        (state.b).as_mut_ptr(),
        mlength,
        dst,
        src,
    );
    siv_gcm_authenticate(
        ctr_ctx,
        nc,
        &mut authentication_key,
        nonce,
        alength,
        adata,
        mlength,
        dst,
        tag.as_mut_ptr(),
    );
    return nettle_memeql_sec(
        tag.as_mut_ptr() as *const libc::c_void,
        src.offset(mlength as isize) as *const libc::c_void,
        16 as i32 as size_t,
    );
}