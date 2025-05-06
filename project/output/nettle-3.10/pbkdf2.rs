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
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
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
#[no_mangle]
pub unsafe extern "C" fn nettle_pbkdf2(
    mut mac_ctx: *mut libc::c_void,
    mut update: Option<nettle_hash_update_func>,
    mut digest: Option<nettle_hash_digest_func>,
    mut digest_size: size_t,
    mut iterations: u32,
    mut salt_length: size_t,
    mut salt: *const uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    let mut U: *mut uint8_t = 0 as *mut uint8_t;
    let mut T: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: u32 = 0;
    if iterations > 0 as i32 as u32 {} else {
        __assert_fail(
            b"iterations > 0\0" as *const u8 as *const i8,
            b"pbkdf2.c\0" as *const u8 as *const i8,
            61 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 147],
                &[i8; 147],
            >(
                b"void nettle_pbkdf2(void *, nettle_hash_update_func *, nettle_hash_digest_func *, size_t, unsigned int, size_t, const uint8_t *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1933: {
        if iterations > 0 as i32 as u32 {} else {
            __assert_fail(
                b"iterations > 0\0" as *const u8 as *const i8,
                b"pbkdf2.c\0" as *const u8 as *const i8,
                61 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 147],
                    &[i8; 147],
                >(
                    b"void nettle_pbkdf2(void *, nettle_hash_update_func *, nettle_hash_digest_func *, size_t, unsigned int, size_t, const uint8_t *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length == 0 as i32 as u64 {
        return;
    }
    let mut fresh0 = ::std::vec::from_elem(
        0,
        (::core::mem::size_of::<uint8_t>() as u64).wrapping_mul(digest_size) as usize,
    );
    U = fresh0.as_mut_ptr() as *mut uint8_t;
    let mut fresh1 = ::std::vec::from_elem(
        0,
        (::core::mem::size_of::<uint8_t>() as u64).wrapping_mul(digest_size) as usize,
    );
    T = fresh1.as_mut_ptr() as *mut uint8_t;
    i = 1 as i32 as u32;
    loop {
        let mut tmp: [uint8_t; 4] = [0; 4];
        let mut prev: *mut uint8_t = 0 as *mut uint8_t;
        let mut u: u32 = 0;
        tmp[0 as i32 as usize] = (i >> 24 as i32 & 0xff as i32 as u32) as uint8_t;
        tmp[1 as i32 as usize] = (i >> 16 as i32 & 0xff as i32 as u32) as uint8_t;
        tmp[2 as i32 as usize] = (i >> 8 as i32 & 0xff as i32 as u32) as uint8_t;
        tmp[3 as i32 as usize] = (i & 0xff as i32 as u32) as uint8_t;
        update.expect("non-null function pointer")(mac_ctx, salt_length, salt);
        update
            .expect(
                "non-null function pointer",
            )(mac_ctx, ::core::mem::size_of::<[uint8_t; 4]>() as u64, tmp.as_mut_ptr());
        digest.expect("non-null function pointer")(mac_ctx, digest_size, T);
        prev = T;
        u = 1 as i32 as u32;
        while u < iterations {
            update.expect("non-null function pointer")(mac_ctx, digest_size, prev);
            digest.expect("non-null function pointer")(mac_ctx, digest_size, U);
            nettle_memxor(T as *mut libc::c_void, U as *const libc::c_void, digest_size);
            u = u.wrapping_add(1);
            u;
            prev = U;
        }
        if length <= digest_size {
            memcpy(dst as *mut libc::c_void, T as *const libc::c_void, length);
            return;
        }
        memcpy(dst as *mut libc::c_void, T as *const libc::c_void, digest_size);
        i = i.wrapping_add(1);
        i;
        dst = dst.offset(digest_size as isize);
        length = (length as u64).wrapping_sub(digest_size) as size_t as size_t;
    };
}