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
    fn _nettle_sha256_compress_n(
        state: *mut uint32_t,
        k: *const uint32_t,
        blocks: size_t,
        data: *const uint8_t,
    ) -> *const uint8_t;
    fn _nettle_write_be32(length: size_t, dst: *mut uint8_t, src: *const uint32_t);
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
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}
static mut K: [uint32_t; 64] = [
    0x428a2f98 as u64 as uint32_t,
    0x71374491 as u64 as uint32_t,
    0xb5c0fbcf as u64 as uint32_t,
    0xe9b5dba5 as u64 as uint32_t,
    0x3956c25b as u64 as uint32_t,
    0x59f111f1 as u64 as uint32_t,
    0x923f82a4 as u64 as uint32_t,
    0xab1c5ed5 as u64 as uint32_t,
    0xd807aa98 as u64 as uint32_t,
    0x12835b01 as u64 as uint32_t,
    0x243185be as u64 as uint32_t,
    0x550c7dc3 as u64 as uint32_t,
    0x72be5d74 as u64 as uint32_t,
    0x80deb1fe as u64 as uint32_t,
    0x9bdc06a7 as u64 as uint32_t,
    0xc19bf174 as u64 as uint32_t,
    0xe49b69c1 as u64 as uint32_t,
    0xefbe4786 as u64 as uint32_t,
    0xfc19dc6 as u64 as uint32_t,
    0x240ca1cc as u64 as uint32_t,
    0x2de92c6f as u64 as uint32_t,
    0x4a7484aa as u64 as uint32_t,
    0x5cb0a9dc as u64 as uint32_t,
    0x76f988da as u64 as uint32_t,
    0x983e5152 as u64 as uint32_t,
    0xa831c66d as u64 as uint32_t,
    0xb00327c8 as u64 as uint32_t,
    0xbf597fc7 as u64 as uint32_t,
    0xc6e00bf3 as u64 as uint32_t,
    0xd5a79147 as u64 as uint32_t,
    0x6ca6351 as u64 as uint32_t,
    0x14292967 as u64 as uint32_t,
    0x27b70a85 as u64 as uint32_t,
    0x2e1b2138 as u64 as uint32_t,
    0x4d2c6dfc as u64 as uint32_t,
    0x53380d13 as u64 as uint32_t,
    0x650a7354 as u64 as uint32_t,
    0x766a0abb as u64 as uint32_t,
    0x81c2c92e as u64 as uint32_t,
    0x92722c85 as u64 as uint32_t,
    0xa2bfe8a1 as u64 as uint32_t,
    0xa81a664b as u64 as uint32_t,
    0xc24b8b70 as u64 as uint32_t,
    0xc76c51a3 as u64 as uint32_t,
    0xd192e819 as u64 as uint32_t,
    0xd6990624 as u64 as uint32_t,
    0xf40e3585 as u64 as uint32_t,
    0x106aa070 as u64 as uint32_t,
    0x19a4c116 as u64 as uint32_t,
    0x1e376c08 as u64 as uint32_t,
    0x2748774c as u64 as uint32_t,
    0x34b0bcb5 as u64 as uint32_t,
    0x391c0cb3 as u64 as uint32_t,
    0x4ed8aa4a as u64 as uint32_t,
    0x5b9cca4f as u64 as uint32_t,
    0x682e6ff3 as u64 as uint32_t,
    0x748f82ee as u64 as uint32_t,
    0x78a5636f as u64 as uint32_t,
    0x84c87814 as u64 as uint32_t,
    0x8cc70208 as u64 as uint32_t,
    0x90befffa as u64 as uint32_t,
    0xa4506ceb as u64 as uint32_t,
    0xbef9a3f7 as u64 as uint32_t,
    0xc67178f2 as u64 as uint32_t,
];
#[no_mangle]
pub unsafe extern "C" fn nettle_sha256_compress(
    mut state: *mut uint32_t,
    mut input: *const uint8_t,
) {
    _nettle_sha256_compress_n(state, K.as_ptr(), 1 as i32 as size_t, input);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha256_init(mut ctx: *mut sha256_ctx) {
    static mut H0: [uint32_t; 8] = [
        0x6a09e667 as u64 as uint32_t,
        0xbb67ae85 as u64 as uint32_t,
        0x3c6ef372 as u64 as uint32_t,
        0xa54ff53a as u64 as uint32_t,
        0x510e527f as u64 as uint32_t,
        0x9b05688c as u64 as uint32_t,
        0x1f83d9ab as u64 as uint32_t,
        0x5be0cd19 as u64 as uint32_t,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        H0.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 8]>() as u64,
    );
    (*ctx).count = 0 as i32 as uint64_t;
    (*ctx).index = 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha256_update(
    mut ctx: *mut sha256_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut blocks: size_t = 0;
    if length == 0 {
        return;
    }
    if (*ctx).index > 0 as i32 as u32 {
        let mut __md_left: u32 = (::core::mem::size_of::<[uint8_t; 64]>() as u64)
            .wrapping_sub((*ctx).index as u64) as u32;
        if length < __md_left as u64 {
            memcpy(
                ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                    as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            (*ctx).index = ((*ctx).index as u64).wrapping_add(length) as u32 as u32;
            return;
        }
        memcpy(
            ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                as *mut libc::c_void,
            data as *const libc::c_void,
            __md_left as u64,
        );
        data = data.offset(__md_left as isize);
        length = (length as u64).wrapping_sub(__md_left as u64) as size_t as size_t;
        nettle_sha256_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
        (*ctx).count = ((*ctx).count).wrapping_add(1);
        (*ctx).count;
    }
    blocks = length >> 6 as i32;
    data = _nettle_sha256_compress_n(
        ((*ctx).state).as_mut_ptr(),
        K.as_ptr(),
        blocks,
        data,
    );
    (*ctx).count = ((*ctx).count as u64).wrapping_add(blocks) as uint64_t as uint64_t;
    length &= 63 as i32 as u64;
    memcpy(
        ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        length,
    );
    (*ctx).index = length as u32;
}
unsafe extern "C" fn sha256_write_digest(
    mut ctx: *mut sha256_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut bit_count: uint64_t = 0;
    if length <= 32 as i32 as u64 {} else {
        __assert_fail(
            b"length <= SHA256_DIGEST_SIZE\0" as *const u8 as *const i8,
            b"sha256.c\0" as *const u8 as *const i8,
            135 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[i8; 65],
            >(b"void sha256_write_digest(struct sha256_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_2342: {
        if length <= 32 as i32 as u64 {} else {
            __assert_fail(
                b"length <= SHA256_DIGEST_SIZE\0" as *const u8 as *const i8,
                b"sha256.c\0" as *const u8 as *const i8,
                135 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"void sha256_write_digest(struct sha256_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __md_i: u32 = 0;
    __md_i = (*ctx).index;
    if (__md_i as u64) < ::core::mem::size_of::<[uint8_t; 64]>() as u64 {} else {
        __assert_fail(
            b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const i8,
            b"sha256.c\0" as *const u8 as *const i8,
            137 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[i8; 65],
            >(b"void sha256_write_digest(struct sha256_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_2282: {
        if (__md_i as u64) < ::core::mem::size_of::<[uint8_t; 64]>() as u64 {} else {
            __assert_fail(
                b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const i8,
                b"sha256.c\0" as *const u8 as *const i8,
                137 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"void sha256_write_digest(struct sha256_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let fresh0 = __md_i;
    __md_i = __md_i.wrapping_add(1);
    (*ctx).block[fresh0 as usize] = 0x80 as i32 as uint8_t;
    if __md_i as u64
        > (::core::mem::size_of::<[uint8_t; 64]>() as u64).wrapping_sub(8 as i32 as u64)
    {
        memset(
            ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
            0 as i32,
            (::core::mem::size_of::<[uint8_t; 64]>() as u64).wrapping_sub(__md_i as u64),
        );
        nettle_sha256_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
        __md_i = 0 as i32 as u32;
    }
    memset(
        ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
        0 as i32,
        (::core::mem::size_of::<[uint8_t; 64]>() as u64)
            .wrapping_sub(8 as i32 as u64)
            .wrapping_sub(__md_i as u64),
    );
    bit_count = (*ctx).count << 9 as i32 | ((*ctx).index << 3 as i32) as u64;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(0 as i32 as isize) = (bit_count >> 56 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(1 as i32 as isize) = (bit_count >> 48 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(2 as i32 as isize) = (bit_count >> 40 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(3 as i32 as isize) = (bit_count >> 32 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(4 as i32 as isize) = (bit_count >> 24 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(5 as i32 as isize) = (bit_count >> 16 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(6 as i32 as isize) = (bit_count >> 8 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(7 as i32 as isize) = (bit_count & 0xff as i32 as u64) as uint8_t;
    nettle_sha256_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
    _nettle_write_be32(length, digest, ((*ctx).state).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha256_digest(
    mut ctx: *mut sha256_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    sha256_write_digest(ctx, length, digest);
    nettle_sha256_init(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha224_init(mut ctx: *mut sha256_ctx) {
    static mut H0: [uint32_t; 8] = [
        0xc1059ed8 as u32,
        0x367cd507 as i32 as uint32_t,
        0x3070dd17 as i32 as uint32_t,
        0xf70e5939 as u32,
        0xffc00b31 as u32,
        0x68581511 as i32 as uint32_t,
        0x64f98fa7 as i32 as uint32_t,
        0xbefa4fa4 as u32,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        H0.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 8]>() as u64,
    );
    (*ctx).count = 0 as i32 as uint64_t;
    (*ctx).index = 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha224_digest(
    mut ctx: *mut sha256_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    sha256_write_digest(ctx, length, digest);
    nettle_sha224_init(ctx);
}