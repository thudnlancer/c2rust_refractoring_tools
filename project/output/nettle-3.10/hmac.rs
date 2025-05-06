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
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_hash {
    pub name: *const i8,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: Option<nettle_hash_init_func>,
    pub update: Option<nettle_hash_update_func>,
    pub digest: Option<nettle_hash_digest_func>,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_set_key(
    mut outer: *mut libc::c_void,
    mut inner: *mut libc::c_void,
    mut state: *mut libc::c_void,
    mut hash: *const nettle_hash,
    mut key_length: size_t,
    mut key: *const uint8_t,
) {
    let mut pad: *mut uint8_t = 0 as *mut uint8_t;
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::core::mem::size_of::<uint8_t>() as u64)
            .wrapping_mul((*hash).block_size as u64) as usize,
    );
    pad = fresh0.as_mut_ptr() as *mut uint8_t;
    ((*hash).init).expect("non-null function pointer")(outer);
    ((*hash).init).expect("non-null function pointer")(inner);
    if key_length > (*hash).block_size as u64 {
        let mut digest: *mut uint8_t = 0 as *mut uint8_t;
        let mut fresh1 = ::std::vec::from_elem(
            0,
            (::core::mem::size_of::<uint8_t>() as u64)
                .wrapping_mul((*hash).digest_size as u64) as usize,
        );
        digest = fresh1.as_mut_ptr() as *mut uint8_t;
        ((*hash).init).expect("non-null function pointer")(state);
        ((*hash).update).expect("non-null function pointer")(state, key_length, key);
        ((*hash).digest)
            .expect(
                "non-null function pointer",
            )(state, (*hash).digest_size as size_t, digest);
        key = digest;
        key_length = (*hash).digest_size as size_t;
    }
    if key_length <= (*hash).block_size as u64 {} else {
        __assert_fail(
            b"key_length <= hash->block_size\0" as *const u8 as *const i8,
            b"hmac.c\0" as *const u8 as *const i8,
            76 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 102],
                &[i8; 102],
            >(
                b"void nettle_hmac_set_key(void *, void *, void *, const struct nettle_hash *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_999: {
        if key_length <= (*hash).block_size as u64 {} else {
            __assert_fail(
                b"key_length <= hash->block_size\0" as *const u8 as *const i8,
                b"hmac.c\0" as *const u8 as *const i8,
                76 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 102],
                    &[i8; 102],
                >(
                    b"void nettle_hmac_set_key(void *, void *, void *, const struct nettle_hash *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    memset(pad as *mut libc::c_void, 0x5c as i32, (*hash).block_size as u64);
    nettle_memxor(pad as *mut libc::c_void, key as *const libc::c_void, key_length);
    ((*hash).update)
        .expect("non-null function pointer")(outer, (*hash).block_size as size_t, pad);
    memset(pad as *mut libc::c_void, 0x36 as i32, (*hash).block_size as u64);
    nettle_memxor(pad as *mut libc::c_void, key as *const libc::c_void, key_length);
    ((*hash).update)
        .expect("non-null function pointer")(inner, (*hash).block_size as size_t, pad);
    memcpy(state, inner, (*hash).context_size as u64);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_update(
    mut state: *mut libc::c_void,
    mut hash: *const nettle_hash,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    ((*hash).update).expect("non-null function pointer")(state, length, data);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_digest(
    mut outer: *const libc::c_void,
    mut inner: *const libc::c_void,
    mut state: *mut libc::c_void,
    mut hash: *const nettle_hash,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    let mut digest: *mut uint8_t = 0 as *mut uint8_t;
    let mut fresh2 = ::std::vec::from_elem(
        0,
        (::core::mem::size_of::<uint8_t>() as u64)
            .wrapping_mul((*hash).digest_size as u64) as usize,
    );
    digest = fresh2.as_mut_ptr() as *mut uint8_t;
    ((*hash).digest)
        .expect(
            "non-null function pointer",
        )(state, (*hash).digest_size as size_t, digest);
    memcpy(state, outer, (*hash).context_size as u64);
    ((*hash).update)
        .expect(
            "non-null function pointer",
        )(state, (*hash).digest_size as size_t, digest);
    ((*hash).digest).expect("non-null function pointer")(state, length, dst);
    memcpy(state, inner, (*hash).context_size as u64);
}