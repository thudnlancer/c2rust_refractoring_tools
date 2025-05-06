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
pub struct md4_ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_md4_init(mut ctx: *mut md4_ctx) {
    let iv: [uint32_t; 4] = [
        0x67452301 as i32 as uint32_t,
        0xefcdab89 as u32,
        0x98badcfe as u32,
        0x10325476 as i32 as uint32_t,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        iv.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 4]>() as u64,
    );
    (*ctx).count = 0 as i32 as uint64_t;
    (*ctx).index = 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_md4_update(
    mut ctx: *mut md4_ctx,
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
                md4_compress(ctx, ((*ctx).block).as_mut_ptr());
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
                    md4_compress(ctx, data);
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
pub unsafe extern "C" fn nettle_md4_digest(
    mut ctx: *mut md4_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut bit_count: uint64_t = 0;
    let mut data: [uint32_t; 16] = [0; 16];
    let mut i: u32 = 0;
    if length <= 16 as i32 as u64 {} else {
        __assert_fail(
            b"length <= MD4_DIGEST_SIZE\0" as *const u8 as *const i8,
            b"md4.c\0" as *const u8 as *const i8,
            93 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[i8; 60],
            >(b"void nettle_md4_digest(struct md4_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3499: {
        if length <= 16 as i32 as u64 {} else {
            __assert_fail(
                b"length <= MD4_DIGEST_SIZE\0" as *const u8 as *const i8,
                b"md4.c\0" as *const u8 as *const i8,
                93 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"void nettle_md4_digest(struct md4_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __md_i: u32 = 0;
    __md_i = (*ctx).index;
    if (__md_i as u64) < ::core::mem::size_of::<[uint8_t; 64]>() as u64 {} else {
        __assert_fail(
            b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const i8,
            b"md4.c\0" as *const u8 as *const i8,
            95 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[i8; 60],
            >(b"void nettle_md4_digest(struct md4_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3439: {
        if (__md_i as u64) < ::core::mem::size_of::<[uint8_t; 64]>() as u64 {} else {
            __assert_fail(
                b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const i8,
                b"md4.c\0" as *const u8 as *const i8,
                95 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"void nettle_md4_digest(struct md4_ctx *, size_t, uint8_t *)\0"))
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
        md4_compress(ctx, ((*ctx).block).as_mut_ptr());
        __md_i = 0 as i32 as u32;
    }
    memset(
        ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
        0 as i32,
        (::core::mem::size_of::<[uint8_t; 64]>() as u64)
            .wrapping_sub(8 as i32 as u64)
            .wrapping_sub(__md_i as u64),
    );
    i = 0 as i32 as u32;
    while i < (16 as i32 - 2 as i32) as u32 {
        data[i as usize] = (*((*ctx).block)
            .as_mut_ptr()
            .offset((4 as i32 as u32).wrapping_mul(i) as isize)
            .offset(3 as i32 as isize) as uint32_t) << 24 as i32
            | (*((*ctx).block)
                .as_mut_ptr()
                .offset((4 as i32 as u32).wrapping_mul(i) as isize)
                .offset(2 as i32 as isize) as uint32_t) << 16 as i32
            | (*((*ctx).block)
                .as_mut_ptr()
                .offset((4 as i32 as u32).wrapping_mul(i) as isize)
                .offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *((*ctx).block)
                .as_mut_ptr()
                .offset((4 as i32 as u32).wrapping_mul(i) as isize)
                .offset(0 as i32 as isize) as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
    bit_count = (*ctx).count << 9 as i32 | ((*ctx).index << 3 as i32) as u64;
    data[(16 as i32 - 2 as i32) as usize] = bit_count as uint32_t;
    data[(16 as i32 - 1 as i32) as usize] = (bit_count >> 32 as i32) as uint32_t;
    md4_transform(((*ctx).state).as_mut_ptr(), data.as_mut_ptr());
    _nettle_write_le32(length, digest, ((*ctx).state).as_mut_ptr());
    nettle_md4_init(ctx);
}
unsafe extern "C" fn md4_transform(
    mut digest: *mut uint32_t,
    mut data: *const uint32_t,
) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    a = *digest.offset(0 as i32 as isize);
    b = *digest.offset(1 as i32 as isize);
    c = *digest.offset(2 as i32 as isize);
    d = *digest.offset(3 as i32 as isize);
    a = (a as u32)
        .wrapping_add((c & b | d & !b).wrapping_add(*data.offset(0 as i32 as isize)))
        as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add((b & a | c & !a).wrapping_add(*data.offset(1 as i32 as isize)))
        as uint32_t as uint32_t;
    d = d << 7 as i32 | d >> 32 as i32 - 7 as i32;
    c = (c as u32)
        .wrapping_add((a & d | b & !d).wrapping_add(*data.offset(2 as i32 as isize)))
        as uint32_t as uint32_t;
    c = c << 11 as i32 | c >> 32 as i32 - 11 as i32;
    b = (b as u32)
        .wrapping_add((d & c | a & !c).wrapping_add(*data.offset(3 as i32 as isize)))
        as uint32_t as uint32_t;
    b = b << 19 as i32 | b >> 32 as i32 - 19 as i32;
    a = (a as u32)
        .wrapping_add((c & b | d & !b).wrapping_add(*data.offset(4 as i32 as isize)))
        as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add((b & a | c & !a).wrapping_add(*data.offset(5 as i32 as isize)))
        as uint32_t as uint32_t;
    d = d << 7 as i32 | d >> 32 as i32 - 7 as i32;
    c = (c as u32)
        .wrapping_add((a & d | b & !d).wrapping_add(*data.offset(6 as i32 as isize)))
        as uint32_t as uint32_t;
    c = c << 11 as i32 | c >> 32 as i32 - 11 as i32;
    b = (b as u32)
        .wrapping_add((d & c | a & !c).wrapping_add(*data.offset(7 as i32 as isize)))
        as uint32_t as uint32_t;
    b = b << 19 as i32 | b >> 32 as i32 - 19 as i32;
    a = (a as u32)
        .wrapping_add((c & b | d & !b).wrapping_add(*data.offset(8 as i32 as isize)))
        as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add((b & a | c & !a).wrapping_add(*data.offset(9 as i32 as isize)))
        as uint32_t as uint32_t;
    d = d << 7 as i32 | d >> 32 as i32 - 7 as i32;
    c = (c as u32)
        .wrapping_add((a & d | b & !d).wrapping_add(*data.offset(10 as i32 as isize)))
        as uint32_t as uint32_t;
    c = c << 11 as i32 | c >> 32 as i32 - 11 as i32;
    b = (b as u32)
        .wrapping_add((d & c | a & !c).wrapping_add(*data.offset(11 as i32 as isize)))
        as uint32_t as uint32_t;
    b = b << 19 as i32 | b >> 32 as i32 - 19 as i32;
    a = (a as u32)
        .wrapping_add((c & b | d & !b).wrapping_add(*data.offset(12 as i32 as isize)))
        as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add((b & a | c & !a).wrapping_add(*data.offset(13 as i32 as isize)))
        as uint32_t as uint32_t;
    d = d << 7 as i32 | d >> 32 as i32 - 7 as i32;
    c = (c as u32)
        .wrapping_add((a & d | b & !d).wrapping_add(*data.offset(14 as i32 as isize)))
        as uint32_t as uint32_t;
    c = c << 11 as i32 | c >> 32 as i32 - 11 as i32;
    b = (b as u32)
        .wrapping_add((d & c | a & !c).wrapping_add(*data.offset(15 as i32 as isize)))
        as uint32_t as uint32_t;
    b = b << 19 as i32 | b >> 32 as i32 - 19 as i32;
    a = (a as u32)
        .wrapping_add(
            (c & b | d & b | c & d)
                .wrapping_add(*data.offset(0 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add(
            (b & a | c & a | b & c)
                .wrapping_add(*data.offset(4 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    d = d << 5 as i32 | d >> 32 as i32 - 5 as i32;
    c = (c as u32)
        .wrapping_add(
            (a & d | b & d | a & b)
                .wrapping_add(*data.offset(8 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> 32 as i32 - 9 as i32;
    b = (b as u32)
        .wrapping_add(
            (d & c | a & c | d & a)
                .wrapping_add(*data.offset(12 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    b = b << 13 as i32 | b >> 32 as i32 - 13 as i32;
    a = (a as u32)
        .wrapping_add(
            (c & b | d & b | c & d)
                .wrapping_add(*data.offset(1 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add(
            (b & a | c & a | b & c)
                .wrapping_add(*data.offset(5 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    d = d << 5 as i32 | d >> 32 as i32 - 5 as i32;
    c = (c as u32)
        .wrapping_add(
            (a & d | b & d | a & b)
                .wrapping_add(*data.offset(9 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> 32 as i32 - 9 as i32;
    b = (b as u32)
        .wrapping_add(
            (d & c | a & c | d & a)
                .wrapping_add(*data.offset(13 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    b = b << 13 as i32 | b >> 32 as i32 - 13 as i32;
    a = (a as u32)
        .wrapping_add(
            (c & b | d & b | c & d)
                .wrapping_add(*data.offset(2 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add(
            (b & a | c & a | b & c)
                .wrapping_add(*data.offset(6 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    d = d << 5 as i32 | d >> 32 as i32 - 5 as i32;
    c = (c as u32)
        .wrapping_add(
            (a & d | b & d | a & b)
                .wrapping_add(*data.offset(10 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> 32 as i32 - 9 as i32;
    b = (b as u32)
        .wrapping_add(
            (d & c | a & c | d & a)
                .wrapping_add(*data.offset(14 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    b = b << 13 as i32 | b >> 32 as i32 - 13 as i32;
    a = (a as u32)
        .wrapping_add(
            (c & b | d & b | c & d)
                .wrapping_add(*data.offset(3 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add(
            (b & a | c & a | b & c)
                .wrapping_add(*data.offset(7 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    d = d << 5 as i32 | d >> 32 as i32 - 5 as i32;
    c = (c as u32)
        .wrapping_add(
            (a & d | b & d | a & b)
                .wrapping_add(*data.offset(11 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> 32 as i32 - 9 as i32;
    b = (b as u32)
        .wrapping_add(
            (d & c | a & c | d & a)
                .wrapping_add(*data.offset(15 as i32 as isize))
                .wrapping_add(0x5a827999 as i32 as u32),
        ) as uint32_t as uint32_t;
    b = b << 13 as i32 | b >> 32 as i32 - 13 as i32;
    a = (a as u32)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*data.offset(0 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*data.offset(8 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> 32 as i32 - 9 as i32;
    c = (c as u32)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*data.offset(4 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    c = c << 11 as i32 | c >> 32 as i32 - 11 as i32;
    b = (b as u32)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*data.offset(12 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    b = b << 15 as i32 | b >> 32 as i32 - 15 as i32;
    a = (a as u32)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*data.offset(2 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*data.offset(10 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> 32 as i32 - 9 as i32;
    c = (c as u32)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*data.offset(6 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    c = c << 11 as i32 | c >> 32 as i32 - 11 as i32;
    b = (b as u32)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*data.offset(14 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    b = b << 15 as i32 | b >> 32 as i32 - 15 as i32;
    a = (a as u32)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*data.offset(1 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*data.offset(9 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> 32 as i32 - 9 as i32;
    c = (c as u32)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*data.offset(5 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    c = c << 11 as i32 | c >> 32 as i32 - 11 as i32;
    b = (b as u32)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*data.offset(13 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    b = b << 15 as i32 | b >> 32 as i32 - 15 as i32;
    a = (a as u32)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(*data.offset(3 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    a = a << 3 as i32 | a >> 32 as i32 - 3 as i32;
    d = (d as u32)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(*data.offset(11 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> 32 as i32 - 9 as i32;
    c = (c as u32)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(*data.offset(7 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    c = c << 11 as i32 | c >> 32 as i32 - 11 as i32;
    b = (b as u32)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(*data.offset(15 as i32 as isize))
                .wrapping_add(0x6ed9eba1 as i32 as u32),
        ) as uint32_t as uint32_t;
    b = b << 15 as i32 | b >> 32 as i32 - 15 as i32;
    let ref mut fresh1 = *digest.offset(0 as i32 as isize);
    *fresh1 = (*fresh1 as u32).wrapping_add(a) as uint32_t as uint32_t;
    let ref mut fresh2 = *digest.offset(1 as i32 as isize);
    *fresh2 = (*fresh2 as u32).wrapping_add(b) as uint32_t as uint32_t;
    let ref mut fresh3 = *digest.offset(2 as i32 as isize);
    *fresh3 = (*fresh3 as u32).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh4 = *digest.offset(3 as i32 as isize);
    *fresh4 = (*fresh4 as u32).wrapping_add(d) as uint32_t as uint32_t;
}
unsafe extern "C" fn md4_compress(mut ctx: *mut md4_ctx, mut block: *const uint8_t) {
    let mut data: [uint32_t; 16] = [0; 16];
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while i < 16 as i32 as u32 {
        data[i as usize] = (*block.offset(3 as i32 as isize) as uint32_t) << 24 as i32
            | (*block.offset(2 as i32 as isize) as uint32_t) << 16 as i32
            | (*block.offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *block.offset(0 as i32 as isize) as uint32_t;
        i = i.wrapping_add(1);
        i;
        block = block.offset(4 as i32 as isize);
    }
    md4_transform(((*ctx).state).as_mut_ptr(), data.as_mut_ptr());
}