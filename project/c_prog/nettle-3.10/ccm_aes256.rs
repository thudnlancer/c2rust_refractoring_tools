use ::libc;
extern "C" {
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ccm_set_nonce(
        ctx: *mut ccm_ctx,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        noncelen: size_t,
        nonce: *const uint8_t,
        authlen: size_t,
        msglen: size_t,
        taglen: size_t,
    );
    fn nettle_ccm_update(
        ctx: *mut ccm_ctx,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_ccm_encrypt(
        ctx: *mut ccm_ctx,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ccm_decrypt(
        ctx: *mut ccm_ctx,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ccm_digest(
        ctx: *mut ccm_ctx,
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_ccm_encrypt_message(
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        tlength: size_t,
        clength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ccm_decrypt_message(
        cipher: *const libc::c_void,
        f: Option::<nettle_cipher_func>,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        tlength: size_t,
        mlength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    ) -> libc::c_int;
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
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ccm_ctx {
    pub ctr: nettle_block16,
    pub tag: nettle_block16,
    pub blength: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ccm_aes256_ctx {
    pub ccm: ccm_ctx,
    pub cipher: aes256_ctx,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ccm_aes256_set_key(
    mut ctx: *mut ccm_aes256_ctx,
    mut key: *const uint8_t,
) {
    nettle_aes256_set_encrypt_key(&mut (*ctx).cipher, key);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ccm_aes256_set_nonce(
    mut ctx: *mut ccm_aes256_ctx,
    mut length: size_t,
    mut nonce: *const uint8_t,
    mut authlen: size_t,
    mut msglen: size_t,
    mut taglen: size_t,
) {
    nettle_ccm_set_nonce(
        &mut (*ctx).ccm,
        &mut (*ctx).cipher as *mut aes256_ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_encrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        length,
        nonce,
        authlen,
        msglen,
        taglen,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ccm_aes256_update(
    mut ctx: *mut ccm_aes256_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    nettle_ccm_update(
        &mut (*ctx).ccm,
        &mut (*ctx).cipher as *mut aes256_ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_encrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        length,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ccm_aes256_encrypt(
    mut ctx: *mut ccm_aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_ccm_encrypt(
        &mut (*ctx).ccm,
        &mut (*ctx).cipher as *mut aes256_ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_encrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
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
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ccm_aes256_decrypt(
    mut ctx: *mut ccm_aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_ccm_decrypt(
        &mut (*ctx).ccm,
        &mut (*ctx).cipher as *mut aes256_ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_encrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
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
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ccm_aes256_digest(
    mut ctx: *mut ccm_aes256_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    nettle_ccm_digest(
        &mut (*ctx).ccm,
        &mut (*ctx).cipher as *mut aes256_ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_encrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        length,
        digest,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ccm_aes256_encrypt_message(
    mut ctx: *mut ccm_aes256_ctx,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut tlength: size_t,
    mut clength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_ccm_encrypt_message(
        &mut (*ctx).cipher as *mut aes256_ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_encrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        nlength,
        nonce,
        alength,
        adata,
        tlength,
        clength,
        dst,
        src,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ccm_aes256_decrypt_message(
    mut ctx: *mut ccm_aes256_ctx,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut tlength: size_t,
    mut mlength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) -> libc::c_int {
    return nettle_ccm_decrypt_message(
        &mut (*ctx).cipher as *mut aes256_ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option::<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_encrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        nlength,
        nonce,
        alength,
        adata,
        tlength,
        mlength,
        dst,
        src,
    );
}
