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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn _nettle_ripemd160_compress(state: *mut uint32_t, data: *const uint8_t);
    fn _nettle_write_le32(length: size_t, dst: *mut uint8_t, src: *const uint32_t);
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
pub struct ripemd160_ctx {
    pub state: [uint32_t; 5],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ripemd160_init(mut ctx: *mut ripemd160_ctx) {
    static mut iv: [uint32_t; 5] = [
        0x67452301 as i32 as uint32_t,
        0xefcdab89 as u32,
        0x98badcfe as u32,
        0x10325476 as i32 as uint32_t,
        0xc3d2e1f0 as u32,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        iv.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 5]>() as u64,
    );
    (*ctx).count = 0 as i32 as uint64_t;
    (*ctx).index = 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_ripemd160_update(
    mut ctx: *mut ripemd160_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block: u64;
    if !(length == 0) {
        if (*ctx).index != 0 {
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
                current_block = 15652330335145281839;
            } else {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    data as *const libc::c_void,
                    __md_left as u64,
                );
                _nettle_ripemd160_compress(
                    ((*ctx).state).as_mut_ptr(),
                    ((*ctx).block).as_mut_ptr(),
                );
                (*ctx).count = ((*ctx).count).wrapping_add(1);
                (*ctx).count;
                data = data.offset(__md_left as isize);
                length = (length as u64).wrapping_sub(__md_left as u64) as size_t
                    as size_t;
                current_block = 11812396948646013369;
            }
        } else {
            current_block = 11812396948646013369;
        }
        match current_block {
            15652330335145281839 => {}
            _ => {
                while length >= ::core::mem::size_of::<[uint8_t; 64]>() as u64 {
                    _nettle_ripemd160_compress(((*ctx).state).as_mut_ptr(), data);
                    (*ctx).count = ((*ctx).count).wrapping_add(1);
                    (*ctx).count;
                    data = data
                        .offset(::core::mem::size_of::<[uint8_t; 64]>() as u64 as isize);
                    length = (length as u64)
                        .wrapping_sub(::core::mem::size_of::<[uint8_t; 64]>() as u64)
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
pub unsafe extern "C" fn nettle_ripemd160_digest(
    mut ctx: *mut ripemd160_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut bit_count: uint64_t = 0;
    if length <= 20 as i32 as u64 {} else {
        __assert_fail(
            b"length <= RIPEMD160_DIGEST_SIZE\0" as *const u8 as *const i8,
            b"ripemd160.c\0" as *const u8 as *const i8,
            190 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[i8; 72],
            >(
                b"void nettle_ripemd160_digest(struct ripemd160_ctx *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1168: {
        if length <= 20 as i32 as u64 {} else {
            __assert_fail(
                b"length <= RIPEMD160_DIGEST_SIZE\0" as *const u8 as *const i8,
                b"ripemd160.c\0" as *const u8 as *const i8,
                190 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[i8; 72],
                >(
                    b"void nettle_ripemd160_digest(struct ripemd160_ctx *, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut __md_i: u32 = 0;
    __md_i = (*ctx).index;
    if (__md_i as u64) < ::core::mem::size_of::<[uint8_t; 64]>() as u64 {} else {
        __assert_fail(
            b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const i8,
            b"ripemd160.c\0" as *const u8 as *const i8,
            192 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 72],
                &[i8; 72],
            >(
                b"void nettle_ripemd160_digest(struct ripemd160_ctx *, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_1108: {
        if (__md_i as u64) < ::core::mem::size_of::<[uint8_t; 64]>() as u64 {} else {
            __assert_fail(
                b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const i8,
                b"ripemd160.c\0" as *const u8 as *const i8,
                192 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 72],
                    &[i8; 72],
                >(
                    b"void nettle_ripemd160_digest(struct ripemd160_ctx *, size_t, uint8_t *)\0",
                ))
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
        _nettle_ripemd160_compress(
            ((*ctx).state).as_mut_ptr(),
            ((*ctx).block).as_mut_ptr(),
        );
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
    *((*ctx).block).as_mut_ptr().offset(56 as i32 as isize).offset(7 as i32 as isize) = (bit_count
        >> 56 as i32 & 0xff as i32 as u64) as uint8_t;
    *((*ctx).block).as_mut_ptr().offset(56 as i32 as isize).offset(6 as i32 as isize) = (bit_count
        >> 48 as i32 & 0xff as i32 as u64) as uint8_t;
    *((*ctx).block).as_mut_ptr().offset(56 as i32 as isize).offset(5 as i32 as isize) = (bit_count
        >> 40 as i32 & 0xff as i32 as u64) as uint8_t;
    *((*ctx).block).as_mut_ptr().offset(56 as i32 as isize).offset(4 as i32 as isize) = (bit_count
        >> 32 as i32 & 0xff as i32 as u64) as uint8_t;
    *((*ctx).block).as_mut_ptr().offset(56 as i32 as isize).offset(3 as i32 as isize) = (bit_count
        >> 24 as i32 & 0xff as i32 as u64) as uint8_t;
    *((*ctx).block).as_mut_ptr().offset(56 as i32 as isize).offset(2 as i32 as isize) = (bit_count
        >> 16 as i32 & 0xff as i32 as u64) as uint8_t;
    *((*ctx).block).as_mut_ptr().offset(56 as i32 as isize).offset(1 as i32 as isize) = (bit_count
        >> 8 as i32 & 0xff as i32 as u64) as uint8_t;
    *((*ctx).block).as_mut_ptr().offset(56 as i32 as isize).offset(0 as i32 as isize) = (bit_count
        & 0xff as i32 as u64) as uint8_t;
    _nettle_ripemd160_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
    _nettle_write_le32(length, digest, ((*ctx).state).as_mut_ptr());
    nettle_ripemd160_init(ctx);
}