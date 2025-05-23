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
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes128_decrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes192_encrypt(
        ctx: *const aes192_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes192_decrypt(
        ctx: *const aes192_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes256_decrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_memeql_sec(
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> i32;
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
pub union nettle_block8 {
    pub b: [uint8_t; 8],
    pub u64_0: uint64_t,
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
#[no_mangle]
pub unsafe extern "C" fn nettle_nist_keywrap16(
    mut ctx: *const libc::c_void,
    mut encrypt: Option<nettle_cipher_func>,
    mut iv: *const uint8_t,
    mut ciphertext_length: size_t,
    mut ciphertext: *mut uint8_t,
    mut cleartext: *const uint8_t,
) {
    let mut I: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut B: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut A: nettle_block8 = nettle_block8 { b: [0; 8] };
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut n: size_t = 0;
    let mut R: *mut uint8_t = ciphertext.offset(8 as i32 as isize);
    if ciphertext_length >= 16 as i32 as u64 {} else {
        __assert_fail(
            b"ciphertext_length >= 16\0" as *const u8 as *const i8,
            b"nist-keywrap.c\0" as *const u8 as *const i8,
            61 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 116],
                &[i8; 116],
            >(
                b"void nettle_nist_keywrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_867: {
        if ciphertext_length >= 16 as i32 as u64 {} else {
            __assert_fail(
                b"ciphertext_length >= 16\0" as *const u8 as *const i8,
                b"nist-keywrap.c\0" as *const u8 as *const i8,
                61 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 116],
                    &[i8; 116],
                >(
                    b"void nettle_nist_keywrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ciphertext_length.wrapping_rem(8 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!(ciphertext_length % 8)\0" as *const u8 as *const i8,
            b"nist-keywrap.c\0" as *const u8 as *const i8,
            62 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 116],
                &[i8; 116],
            >(
                b"void nettle_nist_keywrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_820: {
        if ciphertext_length.wrapping_rem(8 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!(ciphertext_length % 8)\0" as *const u8 as *const i8,
                b"nist-keywrap.c\0" as *const u8 as *const i8,
                62 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 116],
                    &[i8; 116],
                >(
                    b"void nettle_nist_keywrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    n = ciphertext_length.wrapping_sub(8 as i32 as u64).wrapping_div(8 as i32 as u64);
    memcpy(
        R as *mut libc::c_void,
        cleartext as *const libc::c_void,
        ciphertext_length.wrapping_sub(8 as i32 as u64),
    );
    memcpy(
        (A.b).as_mut_ptr() as *mut libc::c_void,
        iv as *const libc::c_void,
        8 as i32 as u64,
    );
    j = 0 as i32 as size_t;
    while j < 6 as i32 as u64 {
        i = 0 as i32 as size_t;
        while i < n {
            I.u64_0[0 as i32 as usize] = A.u64_0;
            memcpy(
                (I.b).as_mut_ptr().offset(8 as i32 as isize) as *mut libc::c_void,
                R.offset(i.wrapping_mul(8 as i32 as u64) as isize)
                    as *const libc::c_void,
                8 as i32 as u64,
            );
            encrypt
                .expect(
                    "non-null function pointer",
                )(ctx, 16 as i32 as size_t, (B.b).as_mut_ptr(), (I.b).as_mut_ptr());
            A.u64_0 = (B.u64_0[0 as i32 as usize] as libc::c_ulonglong
                ^ (n.wrapping_mul(j).wrapping_add(i.wrapping_add(1 as i32 as u64))
                    as libc::c_ulonglong)
                    .swap_bytes()) as uint64_t;
            memcpy(
                R.offset(i.wrapping_mul(8 as i32 as u64) as isize) as *mut libc::c_void,
                (B.b).as_mut_ptr().offset(8 as i32 as isize) as *const libc::c_void,
                8 as i32 as u64,
            );
            i = i.wrapping_add(1);
            i;
        }
        j = j.wrapping_add(1);
        j;
    }
    memcpy(
        ciphertext as *mut libc::c_void,
        (A.b).as_mut_ptr() as *const libc::c_void,
        8 as i32 as u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_nist_keyunwrap16(
    mut ctx: *const libc::c_void,
    mut decrypt: Option<nettle_cipher_func>,
    mut iv: *const uint8_t,
    mut cleartext_length: size_t,
    mut cleartext: *mut uint8_t,
    mut ciphertext: *const uint8_t,
) -> i32 {
    let mut I: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut B: nettle_block16 = nettle_block16 { b: [0; 16] };
    let mut A: nettle_block8 = nettle_block8 { b: [0; 8] };
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: size_t = 0;
    let mut R: *mut uint8_t = cleartext;
    if cleartext_length >= 8 as i32 as u64 {} else {
        __assert_fail(
            b"cleartext_length >= 8\0" as *const u8 as *const i8,
            b"nist-keywrap.c\0" as *const u8 as *const i8,
            103 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[i8; 117],
            >(
                b"int nettle_nist_keyunwrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1156: {
        if cleartext_length >= 8 as i32 as u64 {} else {
            __assert_fail(
                b"cleartext_length >= 8\0" as *const u8 as *const i8,
                b"nist-keywrap.c\0" as *const u8 as *const i8,
                103 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 117],
                    &[i8; 117],
                >(
                    b"int nettle_nist_keyunwrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if cleartext_length.wrapping_rem(8 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!(cleartext_length % 8)\0" as *const u8 as *const i8,
            b"nist-keywrap.c\0" as *const u8 as *const i8,
            104 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 117],
                &[i8; 117],
            >(
                b"int nettle_nist_keyunwrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1113: {
        if cleartext_length.wrapping_rem(8 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!(cleartext_length % 8)\0" as *const u8 as *const i8,
                b"nist-keywrap.c\0" as *const u8 as *const i8,
                104 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 117],
                    &[i8; 117],
                >(
                    b"int nettle_nist_keyunwrap16(const void *, nettle_cipher_func *, const uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    n = cleartext_length.wrapping_div(8 as i32 as u64);
    memcpy(
        (A.b).as_mut_ptr() as *mut libc::c_void,
        ciphertext as *const libc::c_void,
        8 as i32 as u64,
    );
    memcpy(
        R as *mut libc::c_void,
        ciphertext.offset(8 as i32 as isize) as *const libc::c_void,
        cleartext_length,
    );
    j = 5 as i32;
    while j >= 0 as i32 {
        i = n.wrapping_sub(1 as i32 as u64) as i32;
        while i >= 0 as i32 {
            I.u64_0[0 as i32 as usize] = (A.u64_0 as libc::c_ulonglong
                ^ (n.wrapping_mul(j as u64).wrapping_add((i + 1 as i32) as u64)
                    as libc::c_ulonglong)
                    .swap_bytes()) as uint64_t;
            memcpy(
                (I.b).as_mut_ptr().offset(8 as i32 as isize) as *mut libc::c_void,
                R.offset((i * 8 as i32) as isize) as *const libc::c_void,
                8 as i32 as u64,
            );
            decrypt
                .expect(
                    "non-null function pointer",
                )(ctx, 16 as i32 as size_t, (B.b).as_mut_ptr(), (I.b).as_mut_ptr());
            A.u64_0 = B.u64_0[0 as i32 as usize];
            memcpy(
                R.offset((i * 8 as i32) as isize) as *mut libc::c_void,
                (B.b).as_mut_ptr().offset(8 as i32 as isize) as *const libc::c_void,
                8 as i32 as u64,
            );
            i -= 1;
            i;
        }
        j -= 1;
        j;
    }
    return nettle_memeql_sec(
        (A.b).as_mut_ptr() as *const libc::c_void,
        iv as *const libc::c_void,
        8 as i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes128_keywrap(
    mut ctx: *mut aes128_ctx,
    mut iv: *const uint8_t,
    mut ciphertext_length: size_t,
    mut ciphertext: *mut uint8_t,
    mut cleartext: *const uint8_t,
) {
    nettle_nist_keywrap16(
        ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const aes128_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option<nettle_cipher_func>,
        >(
            Some(
                nettle_aes128_encrypt
                    as unsafe extern "C" fn(
                        *const aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        iv,
        ciphertext_length,
        ciphertext,
        cleartext,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes192_keywrap(
    mut ctx: *mut aes192_ctx,
    mut iv: *const uint8_t,
    mut ciphertext_length: size_t,
    mut ciphertext: *mut uint8_t,
    mut cleartext: *const uint8_t,
) {
    nettle_nist_keywrap16(
        ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const aes192_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option<nettle_cipher_func>,
        >(
            Some(
                nettle_aes192_encrypt
                    as unsafe extern "C" fn(
                        *const aes192_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        iv,
        ciphertext_length,
        ciphertext,
        cleartext,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes256_keywrap(
    mut ctx: *mut aes256_ctx,
    mut iv: *const uint8_t,
    mut ciphertext_length: size_t,
    mut ciphertext: *mut uint8_t,
    mut cleartext: *const uint8_t,
) {
    nettle_nist_keywrap16(
        ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option<nettle_cipher_func>,
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
        iv,
        ciphertext_length,
        ciphertext,
        cleartext,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes128_keyunwrap(
    mut ctx: *mut aes128_ctx,
    mut iv: *const uint8_t,
    mut cleartext_length: size_t,
    mut cleartext: *mut uint8_t,
    mut ciphertext: *const uint8_t,
) -> i32 {
    return nettle_nist_keyunwrap16(
        ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const aes128_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option<nettle_cipher_func>,
        >(
            Some(
                nettle_aes128_decrypt
                    as unsafe extern "C" fn(
                        *const aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        iv,
        cleartext_length,
        cleartext,
        ciphertext,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes192_keyunwrap(
    mut ctx: *mut aes192_ctx,
    mut iv: *const uint8_t,
    mut cleartext_length: size_t,
    mut cleartext: *mut uint8_t,
    mut ciphertext: *const uint8_t,
) -> i32 {
    return nettle_nist_keyunwrap16(
        ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const aes192_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option<nettle_cipher_func>,
        >(
            Some(
                nettle_aes192_decrypt
                    as unsafe extern "C" fn(
                        *const aes192_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        iv,
        cleartext_length,
        cleartext,
        ciphertext,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes256_keyunwrap(
    mut ctx: *mut aes256_ctx,
    mut iv: *const uint8_t,
    mut cleartext_length: size_t,
    mut cleartext: *mut uint8_t,
    mut ciphertext: *const uint8_t,
) -> i32 {
    return nettle_nist_keyunwrap16(
        ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const aes256_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option<nettle_cipher_func>,
        >(
            Some(
                nettle_aes256_decrypt
                    as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        iv,
        cleartext_length,
        cleartext,
        ciphertext,
    );
}