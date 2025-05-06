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
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn abort() -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_sha256_init(ctx: *mut sha256_ctx);
    fn nettle_sha256_update(ctx: *mut sha256_ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha256_digest(ctx: *mut sha256_ctx, length: size_t, digest: *mut uint8_t);
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
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yarrow_pool_id {
    YARROW_FAST = 0,
    YARROW_SLOW = 1,
}
impl yarrow_pool_id {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            yarrow_pool_id::YARROW_FAST => 0,
            yarrow_pool_id::YARROW_SLOW => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> yarrow_pool_id {
        match value {
            0 => yarrow_pool_id::YARROW_FAST,
            1 => yarrow_pool_id::YARROW_SLOW,
            _ => panic!("Invalid value for yarrow_pool_id: {}", value),
        }
    }
}
impl AddAssign<u32> for yarrow_pool_id {
    fn add_assign(&mut self, rhs: u32) {
        *self = yarrow_pool_id::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for yarrow_pool_id {
    fn sub_assign(&mut self, rhs: u32) {
        *self = yarrow_pool_id::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for yarrow_pool_id {
    fn mul_assign(&mut self, rhs: u32) {
        *self = yarrow_pool_id::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for yarrow_pool_id {
    fn div_assign(&mut self, rhs: u32) {
        *self = yarrow_pool_id::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for yarrow_pool_id {
    fn rem_assign(&mut self, rhs: u32) {
        *self = yarrow_pool_id::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for yarrow_pool_id {
    type Output = yarrow_pool_id;
    fn add(self, rhs: u32) -> yarrow_pool_id {
        yarrow_pool_id::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for yarrow_pool_id {
    type Output = yarrow_pool_id;
    fn sub(self, rhs: u32) -> yarrow_pool_id {
        yarrow_pool_id::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for yarrow_pool_id {
    type Output = yarrow_pool_id;
    fn mul(self, rhs: u32) -> yarrow_pool_id {
        yarrow_pool_id::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for yarrow_pool_id {
    type Output = yarrow_pool_id;
    fn div(self, rhs: u32) -> yarrow_pool_id {
        yarrow_pool_id::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for yarrow_pool_id {
    type Output = yarrow_pool_id;
    fn rem(self, rhs: u32) -> yarrow_pool_id {
        yarrow_pool_id::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yarrow_source {
    pub estimate: [uint32_t; 2],
    pub next: yarrow_pool_id,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yarrow256_ctx {
    pub pools: [sha256_ctx; 2],
    pub seeded: i32,
    pub key: aes256_ctx,
    pub counter: [uint8_t; 16],
    pub nsources: u32,
    pub sources: *mut yarrow_source,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_yarrow256_init(
    mut ctx: *mut yarrow256_ctx,
    mut n: u32,
    mut s: *mut yarrow_source,
) {
    let mut i: u32 = 0;
    nettle_sha256_init(&mut *((*ctx).pools).as_mut_ptr().offset(0 as i32 as isize));
    nettle_sha256_init(&mut *((*ctx).pools).as_mut_ptr().offset(1 as i32 as isize));
    (*ctx).seeded = 0 as i32;
    memset(
        ((*ctx).counter).as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[uint8_t; 16]>() as u64,
    );
    (*ctx).nsources = n;
    (*ctx).sources = s;
    i = 0 as i32 as u32;
    while i < n {
        (*((*ctx).sources).offset(i as isize))
            .estimate[yarrow_pool_id::YARROW_FAST as i32 as usize] = 0 as i32
            as uint32_t;
        (*((*ctx).sources).offset(i as isize))
            .estimate[yarrow_pool_id::YARROW_SLOW as i32 as usize] = 0 as i32
            as uint32_t;
        (*((*ctx).sources).offset(i as isize)).next = yarrow_pool_id::YARROW_FAST;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_yarrow256_seed(
    mut ctx: *mut yarrow256_ctx,
    mut length: size_t,
    mut seed_file: *const uint8_t,
) {
    if length > 0 as i32 as u64 {} else {
        __assert_fail(
            b"length > 0\0" as *const u8 as *const i8,
            b"yarrow256.c\0" as *const u8 as *const i8,
            115 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 76],
                &[i8; 76],
            >(
                b"void nettle_yarrow256_seed(struct yarrow256_ctx *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2237: {
        if length > 0 as i32 as u64 {} else {
            __assert_fail(
                b"length > 0\0" as *const u8 as *const i8,
                b"yarrow256.c\0" as *const u8 as *const i8,
                115 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 76],
                    &[i8; 76],
                >(
                    b"void nettle_yarrow256_seed(struct yarrow256_ctx *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    nettle_sha256_update(
        &mut *((*ctx).pools)
            .as_mut_ptr()
            .offset(yarrow_pool_id::YARROW_FAST as i32 as isize),
        length,
        seed_file,
    );
    nettle_yarrow256_fast_reseed(ctx);
}
unsafe extern "C" fn yarrow_generate_block(
    mut ctx: *mut yarrow256_ctx,
    mut block: *mut uint8_t,
) {
    let mut i: u32 = 0;
    nettle_aes256_encrypt(
        &mut (*ctx).key,
        ::core::mem::size_of::<[uint8_t; 16]>() as u64,
        block,
        ((*ctx).counter).as_mut_ptr(),
    );
    i = ::core::mem::size_of::<[uint8_t; 16]>() as u64 as u32;
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        (*ctx).counter[i as usize] = ((*ctx).counter[i as usize]).wrapping_add(1);
        if (*ctx).counter[i as usize] != 0 {
            break;
        }
    };
}
unsafe extern "C" fn yarrow_iterate(mut digest: *mut uint8_t) {
    let mut v0: [uint8_t; 32] = [0; 32];
    let mut i: u32 = 0;
    memcpy(
        v0.as_mut_ptr() as *mut libc::c_void,
        digest as *const libc::c_void,
        32 as i32 as u64,
    );
    i = 0 as i32 as u32;
    loop {
        i = i.wrapping_add(1);
        if !(i < 1500 as i32 as u32) {
            break;
        }
        let mut count: [uint8_t; 4] = [0; 4];
        let mut hash: sha256_ctx = sha256_ctx {
            state: [0; 8],
            count: 0,
            index: 0,
            block: [0; 64],
        };
        nettle_sha256_init(&mut hash);
        count[0 as i32 as usize] = (i >> 24 as i32 & 0xff as i32 as u32) as uint8_t;
        count[1 as i32 as usize] = (i >> 16 as i32 & 0xff as i32 as u32) as uint8_t;
        count[2 as i32 as usize] = (i >> 8 as i32 & 0xff as i32 as u32) as uint8_t;
        count[3 as i32 as usize] = (i & 0xff as i32 as u32) as uint8_t;
        nettle_sha256_update(&mut hash, 32 as i32 as size_t, digest);
        nettle_sha256_update(
            &mut hash,
            ::core::mem::size_of::<[uint8_t; 32]>() as u64,
            v0.as_mut_ptr(),
        );
        nettle_sha256_update(
            &mut hash,
            ::core::mem::size_of::<[uint8_t; 4]>() as u64,
            count.as_mut_ptr(),
        );
        nettle_sha256_digest(&mut hash, 32 as i32 as size_t, digest);
    };
}
#[no_mangle]
pub unsafe extern "C" fn nettle_yarrow256_fast_reseed(mut ctx: *mut yarrow256_ctx) {
    let mut digest: [uint8_t; 32] = [0; 32];
    let mut i: u32 = 0;
    if (*ctx).seeded != 0 {
        let mut blocks: [uint8_t; 32] = [0; 32];
        yarrow_generate_block(ctx, blocks.as_mut_ptr());
        yarrow_generate_block(ctx, blocks.as_mut_ptr().offset(16 as i32 as isize));
        nettle_sha256_update(
            &mut *((*ctx).pools)
                .as_mut_ptr()
                .offset(yarrow_pool_id::YARROW_FAST as i32 as isize),
            ::core::mem::size_of::<[uint8_t; 32]>() as u64,
            blocks.as_mut_ptr(),
        );
    }
    nettle_sha256_digest(
        &mut *((*ctx).pools)
            .as_mut_ptr()
            .offset(yarrow_pool_id::YARROW_FAST as i32 as isize),
        ::core::mem::size_of::<[uint8_t; 32]>() as u64,
        digest.as_mut_ptr(),
    );
    yarrow_iterate(digest.as_mut_ptr());
    nettle_aes256_set_encrypt_key(&mut (*ctx).key, digest.as_mut_ptr());
    (*ctx).seeded = 1 as i32;
    memset(
        ((*ctx).counter).as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[uint8_t; 16]>() as u64,
    );
    nettle_aes256_encrypt(
        &mut (*ctx).key,
        ::core::mem::size_of::<[uint8_t; 16]>() as u64,
        ((*ctx).counter).as_mut_ptr(),
        ((*ctx).counter).as_mut_ptr(),
    );
    i = 0 as i32 as u32;
    while i < (*ctx).nsources {
        (*((*ctx).sources).offset(i as isize))
            .estimate[yarrow_pool_id::YARROW_FAST as i32 as usize] = 0 as i32
            as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_yarrow256_slow_reseed(mut ctx: *mut yarrow256_ctx) {
    let mut digest: [uint8_t; 32] = [0; 32];
    let mut i: u32 = 0;
    nettle_sha256_digest(
        &mut *((*ctx).pools)
            .as_mut_ptr()
            .offset(yarrow_pool_id::YARROW_SLOW as i32 as isize),
        ::core::mem::size_of::<[uint8_t; 32]>() as u64,
        digest.as_mut_ptr(),
    );
    nettle_sha256_update(
        &mut *((*ctx).pools)
            .as_mut_ptr()
            .offset(yarrow_pool_id::YARROW_FAST as i32 as isize),
        ::core::mem::size_of::<[uint8_t; 32]>() as u64,
        digest.as_mut_ptr(),
    );
    nettle_yarrow256_fast_reseed(ctx);
    i = 0 as i32 as u32;
    while i < (*ctx).nsources {
        (*((*ctx).sources).offset(i as isize))
            .estimate[yarrow_pool_id::YARROW_SLOW as i32 as usize] = 0 as i32
            as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_yarrow256_update(
    mut ctx: *mut yarrow256_ctx,
    mut source_index: u32,
    mut entropy: u32,
    mut length: size_t,
    mut data: *const uint8_t,
) -> i32 {
    let mut current: yarrow_pool_id = yarrow_pool_id::YARROW_FAST;
    let mut source: *mut yarrow_source = 0 as *mut yarrow_source;
    if source_index < (*ctx).nsources {} else {
        __assert_fail(
            b"source_index < ctx->nsources\0" as *const u8 as *const i8,
            b"yarrow256.c\0" as *const u8 as *const i8,
            244 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 105],
                &[i8; 105],
            >(
                b"int nettle_yarrow256_update(struct yarrow256_ctx *, unsigned int, unsigned int, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2644: {
        if source_index < (*ctx).nsources {} else {
            __assert_fail(
                b"source_index < ctx->nsources\0" as *const u8 as *const i8,
                b"yarrow256.c\0" as *const u8 as *const i8,
                244 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 105],
                    &[i8; 105],
                >(
                    b"int nettle_yarrow256_update(struct yarrow256_ctx *, unsigned int, unsigned int, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length == 0 {
        return 0 as i32;
    }
    source = &mut *((*ctx).sources).offset(source_index as isize) as *mut yarrow_source;
    if (*ctx).seeded == 0 {
        current = yarrow_pool_id::YARROW_SLOW;
    } else {
        current = (*source).next;
        (*source).next = yarrow_pool_id::from_libc_c_uint(
            ((*source).next as u64 == 0) as i32 as u32,
        );
    }
    nettle_sha256_update(
        &mut *((*ctx).pools).as_mut_ptr().offset(current as isize),
        length,
        data,
    );
    if (*source).estimate[current as usize] < 0x100000 as i32 as u32 {
        if entropy > 0x100000 as i32 as u32 {
            entropy = 0x100000 as i32 as u32;
        }
        if length < (0x100000 as i32 / 4 as i32) as u64
            && entropy as u64 > (4 as i32 as u64).wrapping_mul(length)
        {
            entropy = (4 as i32 as u64).wrapping_mul(length) as u32;
        }
        entropy = entropy.wrapping_add((*source).estimate[current as usize]);
        if entropy > 0x100000 as i32 as u32 {
            entropy = 0x100000 as i32 as u32;
        }
        (*source).estimate[current as usize] = entropy;
    }
    match current as u32 {
        0 => {
            if (*source).estimate[yarrow_pool_id::YARROW_FAST as i32 as usize]
                >= 100 as i32 as u32
            {
                nettle_yarrow256_fast_reseed(ctx);
                return 1 as i32;
            } else {
                return 0 as i32
            }
        }
        1 => {
            if nettle_yarrow256_needed_sources(ctx) == 0 {
                nettle_yarrow256_slow_reseed(ctx);
                return 1 as i32;
            } else {
                return 0 as i32
            }
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn yarrow_gate(mut ctx: *mut yarrow256_ctx) {
    let mut key: [uint8_t; 32] = [0; 32];
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64) < ::core::mem::size_of::<[uint8_t; 32]>() as u64 {
        yarrow_generate_block(ctx, key.as_mut_ptr().offset(i as isize));
        i = i.wrapping_add(16 as i32 as u32);
    }
    nettle_aes256_set_encrypt_key(&mut (*ctx).key, key.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn nettle_yarrow256_random(
    mut ctx: *mut yarrow256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    if (*ctx).seeded != 0 {} else {
        __assert_fail(
            b"ctx->seeded\0" as *const u8 as *const i8,
            b"yarrow256.c\0" as *const u8 as *const i8,
            328 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[i8; 72],
            >(
                b"void nettle_yarrow256_random(struct yarrow256_ctx *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2838: {
        if (*ctx).seeded != 0 {} else {
            __assert_fail(
                b"ctx->seeded\0" as *const u8 as *const i8,
                b"yarrow256.c\0" as *const u8 as *const i8,
                328 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[i8; 72],
                >(
                    b"void nettle_yarrow256_random(struct yarrow256_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length >= 16 as i32 as u64 {
        yarrow_generate_block(ctx, dst);
        dst = dst.offset(16 as i32 as isize);
        length = (length as u64).wrapping_sub(16 as i32 as u64) as size_t as size_t;
    }
    if length != 0 {
        let mut buffer: [uint8_t; 16] = [0; 16];
        if length < 16 as i32 as u64 {} else {
            __assert_fail(
                b"length < AES_BLOCK_SIZE\0" as *const u8 as *const i8,
                b"yarrow256.c\0" as *const u8 as *const i8,
                340 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[i8; 72],
                >(
                    b"void nettle_yarrow256_random(struct yarrow256_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_2774: {
            if length < 16 as i32 as u64 {} else {
                __assert_fail(
                    b"length < AES_BLOCK_SIZE\0" as *const u8 as *const i8,
                    b"yarrow256.c\0" as *const u8 as *const i8,
                    340 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 72],
                        &[i8; 72],
                    >(
                        b"void nettle_yarrow256_random(struct yarrow256_ctx *, size_t, uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        yarrow_generate_block(ctx, buffer.as_mut_ptr());
        memcpy(
            dst as *mut libc::c_void,
            buffer.as_mut_ptr() as *const libc::c_void,
            length,
        );
    }
    yarrow_gate(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_yarrow256_is_seeded(mut ctx: *mut yarrow256_ctx) -> i32 {
    return (*ctx).seeded;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_yarrow256_needed_sources(
    mut ctx: *mut yarrow256_ctx,
) -> u32 {
    let mut k: u32 = 0;
    let mut i: u32 = 0;
    k = 0 as i32 as u32;
    i = k;
    while i < (*ctx).nsources {
        if (*((*ctx).sources).offset(i as isize))
            .estimate[yarrow_pool_id::YARROW_SLOW as i32 as usize] >= 160 as i32 as u32
        {
            k = k.wrapping_add(1);
            k;
        }
        i = i.wrapping_add(1);
        i;
    }
    return if k < 2 as i32 as u32 {
        (2 as i32 as u32).wrapping_sub(k)
    } else {
        0 as i32 as u32
    };
}