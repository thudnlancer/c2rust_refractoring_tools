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
    fn nettle_cmac128_set_key(
        key: *mut cmac128_key,
        cipher: *const libc::c_void,
        encrypt: Option<nettle_cipher_func>,
    );
    fn nettle_cmac128_init(ctx: *mut cmac128_ctx);
    fn nettle_cmac128_update(
        ctx: *mut cmac128_ctx,
        cipher: *const libc::c_void,
        encrypt: Option<nettle_cipher_func>,
        msg_len: size_t,
        msg: *const uint8_t,
    );
    fn nettle_cmac128_digest(
        ctx: *mut cmac128_ctx,
        key: *const cmac128_key,
        cipher: *const libc::c_void,
        encrypt: Option<nettle_cipher_func>,
        length: u32,
        digest: *mut uint8_t,
    );
    fn nettle_ctr_crypt(
        ctx: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        block_size: size_t,
        ctr: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
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
unsafe extern "C" fn _siv_s2v(
    mut nc: *const nettle_cipher,
    mut cmac_key: *const cmac128_key,
    mut cmac_cipher: *const libc::c_void,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut plength: size_t,
    mut pdata: *const uint8_t,
    mut v: *mut uint8_t,
) {
    let mut D: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut S: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut T: nettle_block16 = nettle_block16 { b: [0; 16] };
    static mut const_zero: nettle_block16 = nettle_block16 {
        b: [0 as i32 as uint8_t, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };
    let mut cmac_ctx: cmac128_ctx = cmac128_ctx {
        X: nettle_block16 { b: [0; 16] },
        block: nettle_block16 { b: [0; 16] },
        index: 0,
    };
    if nlength >= 1 as i32 as u64 {} else {
        __assert_fail(
            b"nlength >= SIV_MIN_NONCE_SIZE\0" as *const u8 as *const i8,
            b"siv-cmac.c\0" as *const u8 as *const i8,
            63 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 172],
                &[i8; 172],
            >(
                b"void _siv_s2v(const struct nettle_cipher *, const struct cmac128_key *, const void *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, const uint8_t *, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1662: {
        if nlength >= 1 as i32 as u64 {} else {
            __assert_fail(
                b"nlength >= SIV_MIN_NONCE_SIZE\0" as *const u8 as *const i8,
                b"siv-cmac.c\0" as *const u8 as *const i8,
                63 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 172],
                    &[i8; 172],
                >(
                    b"void _siv_s2v(const struct nettle_cipher *, const struct cmac128_key *, const void *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, const uint8_t *, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    nettle_cmac128_init(&mut cmac_ctx);
    nettle_cmac128_update(
        &mut cmac_ctx,
        cmac_cipher,
        (*nc).encrypt,
        16 as i32 as size_t,
        (const_zero.b).as_ptr(),
    );
    nettle_cmac128_digest(
        &mut cmac_ctx,
        cmac_key,
        cmac_cipher,
        (*nc).encrypt,
        16 as i32 as u32,
        (D.b).as_mut_ptr(),
    );
    block16_mulx_be(&mut D, &mut D);
    nettle_cmac128_update(&mut cmac_ctx, cmac_cipher, (*nc).encrypt, alength, adata);
    nettle_cmac128_digest(
        &mut cmac_ctx,
        cmac_key,
        cmac_cipher,
        (*nc).encrypt,
        16 as i32 as u32,
        (S.b).as_mut_ptr(),
    );
    block16_xor(&mut D, &mut S);
    block16_mulx_be(&mut D, &mut D);
    nettle_cmac128_update(&mut cmac_ctx, cmac_cipher, (*nc).encrypt, nlength, nonce);
    nettle_cmac128_digest(
        &mut cmac_ctx,
        cmac_key,
        cmac_cipher,
        (*nc).encrypt,
        16 as i32 as u32,
        (S.b).as_mut_ptr(),
    );
    block16_xor(&mut D, &mut S);
    if plength >= 16 as i32 as u64 {
        nettle_cmac128_update(
            &mut cmac_ctx,
            cmac_cipher,
            (*nc).encrypt,
            plength.wrapping_sub(16 as i32 as u64),
            pdata,
        );
        pdata = pdata.offset(plength.wrapping_sub(16 as i32 as u64) as isize);
        block16_xor_bytes(&mut T, &mut D, pdata);
    } else {
        let mut pad: nettle_block16 = nettle_block16 { b: [0; 16] };
        block16_mulx_be(&mut T, &mut D);
        memcpy(
            (pad.b).as_mut_ptr() as *mut libc::c_void,
            pdata as *const libc::c_void,
            plength,
        );
        pad.b[plength as usize] = 0x80 as i32 as uint8_t;
        if plength.wrapping_add(1 as i32 as u64) < 16 as i32 as u64 {
            memset(
                &mut *(pad.b)
                    .as_mut_ptr()
                    .offset(plength.wrapping_add(1 as i32 as u64) as isize)
                    as *mut uint8_t as *mut libc::c_void,
                0 as i32,
                (16 as i32 as u64).wrapping_sub(plength).wrapping_sub(1 as i32 as u64),
            );
        }
        block16_xor(&mut T, &mut pad);
    }
    nettle_cmac128_update(
        &mut cmac_ctx,
        cmac_cipher,
        (*nc).encrypt,
        16 as i32 as size_t,
        (T.b).as_mut_ptr(),
    );
    nettle_cmac128_digest(
        &mut cmac_ctx,
        cmac_key,
        cmac_cipher,
        (*nc).encrypt,
        16 as i32 as u32,
        v,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_siv_cmac_set_key(
    mut cmac_key: *mut cmac128_key,
    mut cmac_cipher: *mut libc::c_void,
    mut siv_cipher: *mut libc::c_void,
    mut nc: *const nettle_cipher,
    mut key: *const uint8_t,
) {
    ((*nc).set_encrypt_key).expect("non-null function pointer")(cmac_cipher, key);
    nettle_cmac128_set_key(cmac_key, cmac_cipher, (*nc).encrypt);
    ((*nc).set_encrypt_key)
        .expect(
            "non-null function pointer",
        )(siv_cipher, key.offset((*nc).key_size as isize));
}
#[no_mangle]
pub unsafe extern "C" fn nettle_siv_cmac_encrypt_message(
    mut cmac_key: *const cmac128_key,
    mut cmac_cipher: *const libc::c_void,
    mut nc: *const nettle_cipher,
    mut ctr_cipher: *const libc::c_void,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut clength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut siv: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut slength: size_t = 0;
    if clength >= 16 as i32 as u64 {} else {
        __assert_fail(
            b"clength >= SIV_DIGEST_SIZE\0" as *const u8 as *const i8,
            b"siv-cmac.c\0" as *const u8 as *const i8,
            126 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 209],
                &[i8; 209],
            >(
                b"void nettle_siv_cmac_encrypt_message(const struct cmac128_key *, const void *, const struct nettle_cipher *, const void *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1712: {
        if clength >= 16 as i32 as u64 {} else {
            __assert_fail(
                b"clength >= SIV_DIGEST_SIZE\0" as *const u8 as *const i8,
                b"siv-cmac.c\0" as *const u8 as *const i8,
                126 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 209],
                    &[i8; 209],
                >(
                    b"void nettle_siv_cmac_encrypt_message(const struct cmac128_key *, const void *, const struct nettle_cipher *, const void *, size_t, const uint8_t *, size_t, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    slength = clength.wrapping_sub(16 as i32 as u64);
    _siv_s2v(
        nc,
        cmac_key,
        cmac_cipher,
        alength,
        adata,
        nlength,
        nonce,
        slength,
        src,
        (siv.b).as_mut_ptr(),
    );
    memcpy(
        dst as *mut libc::c_void,
        (siv.b).as_mut_ptr() as *const libc::c_void,
        16 as i32 as u64,
    );
    siv.b[8 as i32 as usize] = (siv.b[8 as i32 as usize] as i32 & !(0x80 as i32))
        as uint8_t;
    siv.b[12 as i32 as usize] = (siv.b[12 as i32 as usize] as i32 & !(0x80 as i32))
        as uint8_t;
    nettle_ctr_crypt(
        ctr_cipher,
        (*nc).encrypt,
        16 as i32 as size_t,
        (siv.b).as_mut_ptr(),
        slength,
        dst.offset(16 as i32 as isize),
        src,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_siv_cmac_decrypt_message(
    mut cmac_key: *const cmac128_key,
    mut cmac_cipher: *const libc::c_void,
    mut nc: *const nettle_cipher,
    mut ctr_cipher: *const libc::c_void,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut mlength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) -> i32 {
    let mut siv: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut ctr: nettle_block16 = nettle_block16 { b: [0; 16] };
    memcpy(
        (ctr.b).as_mut_ptr() as *mut libc::c_void,
        src as *const libc::c_void,
        16 as i32 as u64,
    );
    ctr.b[8 as i32 as usize] = (ctr.b[8 as i32 as usize] as i32 & !(0x80 as i32))
        as uint8_t;
    ctr.b[12 as i32 as usize] = (ctr.b[12 as i32 as usize] as i32 & !(0x80 as i32))
        as uint8_t;
    nettle_ctr_crypt(
        ctr_cipher,
        (*nc).encrypt,
        16 as i32 as size_t,
        (ctr.b).as_mut_ptr(),
        mlength,
        dst,
        src.offset(16 as i32 as isize),
    );
    _siv_s2v(
        nc,
        cmac_key,
        cmac_cipher,
        alength,
        adata,
        nlength,
        nonce,
        mlength,
        dst,
        (siv.b).as_mut_ptr(),
    );
    return nettle_memeql_sec(
        (siv.b).as_mut_ptr() as *const libc::c_void,
        src as *const libc::c_void,
        16 as i32 as size_t,
    );
}