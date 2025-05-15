use ::libc;
extern "C" {
    fn nettle_aes_set_encrypt_key(
        ctx: *mut aes_ctx,
        length: size_t,
        key: *const uint8_t,
    );
    fn nettle_aes_encrypt(
        ctx: *const aes_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_set_key(
        key: *mut gcm_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
    );
    fn nettle_gcm_set_iv(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        length: size_t,
        iv: *const uint8_t,
    );
    fn nettle_gcm_update(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_gcm_encrypt(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_decrypt(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_digest(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        digest: *mut uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [libc::c_ulong; 2],
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
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes_ctx {
    pub key_size: libc::c_uint,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ctx128: aes128_ctx,
    pub ctx192: aes192_ctx,
    pub ctx256: aes256_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_ctx {
    pub iv: nettle_block16,
    pub ctr: nettle_block16,
    pub x: nettle_block16,
    pub auth_size: uint64_t,
    pub data_size: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_aes_ctx {
    pub key: gcm_key,
    pub gcm: gcm_ctx,
    pub cipher: aes_ctx,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_gcm_aes_set_key(
    mut ctx: *mut gcm_aes_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) {
    nettle_aes_set_encrypt_key(&mut (*ctx).cipher, length, key);
    nettle_gcm_set_key(
        &mut (*ctx).key,
        &mut (*ctx).cipher as *mut aes_ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes_encrypt
                    as unsafe extern "C" fn(
                        *const aes_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_gcm_aes_set_iv(
    mut ctx: *mut gcm_aes_ctx,
    mut length: size_t,
    mut iv: *const uint8_t,
) {
    nettle_gcm_set_iv(&mut (*ctx).gcm, &mut (*ctx).key, length, iv);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_gcm_aes_update(
    mut ctx: *mut gcm_aes_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    nettle_gcm_update(&mut (*ctx).gcm, &mut (*ctx).key, length, data);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_gcm_aes_encrypt(
    mut ctx: *mut gcm_aes_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if 0 as libc::c_int != 0 {
        nettle_aes_encrypt(
            &mut (*ctx).cipher,
            !(0 as libc::c_int as size_t),
            0 as *mut uint8_t,
            0 as *const uint8_t,
        );
    } else {
        nettle_gcm_encrypt(
            &mut (*ctx).gcm,
            &mut (*ctx).key,
            &mut (*ctx).cipher as *mut aes_ctx as *const libc::c_void,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const aes_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_aes_encrypt
                        as unsafe extern "C" fn(
                            *const aes_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            length,
            dst,
            src,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn nettle_gcm_aes_decrypt(
    mut ctx: *mut gcm_aes_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if 0 as libc::c_int != 0 {
        nettle_aes_encrypt(
            &mut (*ctx).cipher,
            !(0 as libc::c_int as size_t),
            0 as *mut uint8_t,
            0 as *const uint8_t,
        );
    } else {
        nettle_gcm_decrypt(
            &mut (*ctx).gcm,
            &mut (*ctx).key,
            &mut (*ctx).cipher as *mut aes_ctx as *const libc::c_void,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const aes_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_aes_encrypt
                        as unsafe extern "C" fn(
                            *const aes_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            length,
            dst,
            src,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn nettle_gcm_aes_digest(
    mut ctx: *mut gcm_aes_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    if 0 as libc::c_int != 0 {
        nettle_aes_encrypt(
            &mut (*ctx).cipher,
            !(0 as libc::c_int as size_t),
            0 as *mut uint8_t,
            0 as *const uint8_t,
        );
    } else {
        nettle_gcm_digest(
            &mut (*ctx).gcm,
            &mut (*ctx).key,
            &mut (*ctx).cipher as *mut aes_ctx as *const libc::c_void,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const aes_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_aes_encrypt
                        as unsafe extern "C" fn(
                            *const aes_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            length,
            digest,
        );
    };
}
