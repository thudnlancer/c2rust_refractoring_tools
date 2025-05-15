use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _nettle_write_le32(length: size_t, dst: *mut uint8_t, src: *const uint32_t);
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
pub struct md5_ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_md5_init(mut ctx: *mut md5_ctx) {
    let iv: [uint32_t; 4] = [
        0x67452301 as libc::c_int as uint32_t,
        0xefcdab89 as libc::c_uint,
        0x98badcfe as libc::c_uint,
        0x10325476 as libc::c_int as uint32_t,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        iv.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong,
    );
    (*ctx).count = 0 as libc::c_int as uint64_t;
    (*ctx).index = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_md5_update(
    mut ctx: *mut md5_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block: u64;
    if !(length == 0) {
        if (*ctx).index != 0 {
            let mut __md_left: libc::c_uint = (::core::mem::size_of::<[uint8_t; 64]>()
                as libc::c_ulong)
                .wrapping_sub((*ctx).index as libc::c_ulong) as libc::c_uint;
            if length < __md_left as libc::c_ulong {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    data as *const libc::c_void,
                    length,
                );
                (*ctx)
                    .index = ((*ctx).index as libc::c_ulong).wrapping_add(length)
                    as libc::c_uint as libc::c_uint;
                current_block = 15652330335145281839;
            } else {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    data as *const libc::c_void,
                    __md_left as libc::c_ulong,
                );
                nettle_md5_compress(
                    ((*ctx).state).as_mut_ptr(),
                    ((*ctx).block).as_mut_ptr(),
                );
                (*ctx).count = ((*ctx).count).wrapping_add(1);
                (*ctx).count;
                data = data.offset(__md_left as isize);
                length = (length as libc::c_ulong)
                    .wrapping_sub(__md_left as libc::c_ulong) as size_t as size_t;
                current_block = 11812396948646013369;
            }
        } else {
            current_block = 11812396948646013369;
        }
        match current_block {
            15652330335145281839 => {}
            _ => {
                while length >= ::core::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong
                {
                    nettle_md5_compress(((*ctx).state).as_mut_ptr(), data);
                    (*ctx).count = ((*ctx).count).wrapping_add(1);
                    (*ctx).count;
                    data = data
                        .offset(
                            ::core::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong
                                as isize,
                        );
                    length = (length as libc::c_ulong)
                        .wrapping_sub(
                            ::core::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong,
                        ) as size_t as size_t;
                }
                memcpy(
                    ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
                    data as *const libc::c_void,
                    length,
                );
                (*ctx).index = length as libc::c_uint;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_md5_digest(
    mut ctx: *mut md5_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut bit_count: uint64_t = 0;
    if length <= 16 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"length <= MD5_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
            b"md5.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void nettle_md5_digest(struct md5_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_4465: {
        if length <= 16 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"length <= MD5_DIGEST_SIZE\0" as *const u8 as *const libc::c_char,
                b"md5.c\0" as *const u8 as *const libc::c_char,
                81 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void nettle_md5_digest(struct md5_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __md_i: libc::c_uint = 0;
    __md_i = (*ctx).index;
    if (__md_i as libc::c_ulong)
        < ::core::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong
    {} else {
        __assert_fail(
            b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const libc::c_char,
            b"md5.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"void nettle_md5_digest(struct md5_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_4405: {
        if (__md_i as libc::c_ulong)
            < ::core::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong
        {} else {
            __assert_fail(
                b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const libc::c_char,
                b"md5.c\0" as *const u8 as *const libc::c_char,
                83 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"void nettle_md5_digest(struct md5_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let fresh0 = __md_i;
    __md_i = __md_i.wrapping_add(1);
    (*ctx).block[fresh0 as usize] = 0x80 as libc::c_int as uint8_t;
    if __md_i as libc::c_ulong
        > (::core::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)
    {
        memset(
            ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
                .wrapping_sub(__md_i as libc::c_ulong),
        );
        nettle_md5_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
        __md_i = 0 as libc::c_int as libc::c_uint;
    }
    memset(
        ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<[uint8_t; 64]>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(__md_i as libc::c_ulong),
    );
    bit_count = (*ctx).count << 9 as libc::c_int
        | ((*ctx).index << 3 as libc::c_int) as libc::c_ulong;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            7 as libc::c_int as isize,
        ) = (bit_count >> 56 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            6 as libc::c_int as isize,
        ) = (bit_count >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            5 as libc::c_int as isize,
        ) = (bit_count >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            4 as libc::c_int as isize,
        ) = (bit_count >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            3 as libc::c_int as isize,
        ) = (bit_count >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            2 as libc::c_int as isize,
        ) = (bit_count >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            1 as libc::c_int as isize,
        ) = (bit_count >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as libc::c_int - 8 as libc::c_int) as isize)
        .offset(
            0 as libc::c_int as isize,
        ) = (bit_count & 0xff as libc::c_int as libc::c_ulong) as uint8_t;
    nettle_md5_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
    _nettle_write_le32(length, digest, ((*ctx).state).as_mut_ptr());
    nettle_md5_init(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_md5_compress(
    mut digest: *mut uint32_t,
    mut input: *const uint8_t,
) {
    let mut data: [uint32_t; 16] = [0; 16];
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 16 as libc::c_int as libc::c_uint {
        data[i
            as usize] = (*input.offset(3 as libc::c_int as isize) as uint32_t)
            << 24 as libc::c_int
            | (*input.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*input.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | *input.offset(0 as libc::c_int as isize) as uint32_t;
        i = i.wrapping_add(1);
        i;
        input = input.offset(4 as libc::c_int as isize);
    }
    a = *digest.offset(0 as libc::c_int as isize);
    b = *digest.offset(1 as libc::c_int as isize);
    c = *digest.offset(2 as libc::c_int as isize);
    d = *digest.offset(3 as libc::c_int as isize);
    a = (a as libc::c_uint)
        .wrapping_add(
            (d ^ b & (c ^ d))
                .wrapping_add(data[0 as libc::c_int as usize])
                .wrapping_add(0xd76aa478 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (c ^ a & (b ^ c))
                .wrapping_add(data[1 as libc::c_int as usize])
                .wrapping_add(0xe8c7b756 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (b ^ d & (a ^ b))
                .wrapping_add(data[2 as libc::c_int as usize])
                .wrapping_add(0x242070db as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (a ^ c & (d ^ a))
                .wrapping_add(data[3 as libc::c_int as usize])
                .wrapping_add(0xc1bdceee as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (d ^ b & (c ^ d))
                .wrapping_add(data[4 as libc::c_int as usize])
                .wrapping_add(0xf57c0faf as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (c ^ a & (b ^ c))
                .wrapping_add(data[5 as libc::c_int as usize])
                .wrapping_add(0x4787c62a as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (b ^ d & (a ^ b))
                .wrapping_add(data[6 as libc::c_int as usize])
                .wrapping_add(0xa8304613 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (a ^ c & (d ^ a))
                .wrapping_add(data[7 as libc::c_int as usize])
                .wrapping_add(0xfd469501 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (d ^ b & (c ^ d))
                .wrapping_add(data[8 as libc::c_int as usize])
                .wrapping_add(0x698098d8 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (c ^ a & (b ^ c))
                .wrapping_add(data[9 as libc::c_int as usize])
                .wrapping_add(0x8b44f7af as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (b ^ d & (a ^ b))
                .wrapping_add(data[10 as libc::c_int as usize])
                .wrapping_add(0xffff5bb1 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (a ^ c & (d ^ a))
                .wrapping_add(data[11 as libc::c_int as usize])
                .wrapping_add(0x895cd7be as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (d ^ b & (c ^ d))
                .wrapping_add(data[12 as libc::c_int as usize])
                .wrapping_add(0x6b901122 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 7 as libc::c_int | a >> 32 as libc::c_int - 7 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (c ^ a & (b ^ c))
                .wrapping_add(data[13 as libc::c_int as usize])
                .wrapping_add(0xfd987193 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 12 as libc::c_int | d >> 32 as libc::c_int - 12 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (b ^ d & (a ^ b))
                .wrapping_add(data[14 as libc::c_int as usize])
                .wrapping_add(0xa679438e as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 17 as libc::c_int | c >> 32 as libc::c_int - 17 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (a ^ c & (d ^ a))
                .wrapping_add(data[15 as libc::c_int as usize])
                .wrapping_add(0x49b40821 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 22 as libc::c_int | b >> 32 as libc::c_int - 22 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d & (b ^ c))
                .wrapping_add(data[1 as libc::c_int as usize])
                .wrapping_add(0xf61e2562 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ c & (a ^ b))
                .wrapping_add(data[6 as libc::c_int as usize])
                .wrapping_add(0xc040b340 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ b & (d ^ a))
                .wrapping_add(data[11 as libc::c_int as usize])
                .wrapping_add(0x265e5a51 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ a & (c ^ d))
                .wrapping_add(data[0 as libc::c_int as usize])
                .wrapping_add(0xe9b6c7aa as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d & (b ^ c))
                .wrapping_add(data[5 as libc::c_int as usize])
                .wrapping_add(0xd62f105d as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ c & (a ^ b))
                .wrapping_add(data[10 as libc::c_int as usize])
                .wrapping_add(0x2441453 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ b & (d ^ a))
                .wrapping_add(data[15 as libc::c_int as usize])
                .wrapping_add(0xd8a1e681 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ a & (c ^ d))
                .wrapping_add(data[4 as libc::c_int as usize])
                .wrapping_add(0xe7d3fbc8 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d & (b ^ c))
                .wrapping_add(data[9 as libc::c_int as usize])
                .wrapping_add(0x21e1cde6 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ c & (a ^ b))
                .wrapping_add(data[14 as libc::c_int as usize])
                .wrapping_add(0xc33707d6 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ b & (d ^ a))
                .wrapping_add(data[3 as libc::c_int as usize])
                .wrapping_add(0xf4d50d87 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ a & (c ^ d))
                .wrapping_add(data[8 as libc::c_int as usize])
                .wrapping_add(0x455a14ed as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ d & (b ^ c))
                .wrapping_add(data[13 as libc::c_int as usize])
                .wrapping_add(0xa9e3e905 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 5 as libc::c_int | a >> 32 as libc::c_int - 5 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ c & (a ^ b))
                .wrapping_add(data[2 as libc::c_int as usize])
                .wrapping_add(0xfcefa3f8 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ b & (d ^ a))
                .wrapping_add(data[7 as libc::c_int as usize])
                .wrapping_add(0x676f02d9 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 14 as libc::c_int | c >> 32 as libc::c_int - 14 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ a & (c ^ d))
                .wrapping_add(data[12 as libc::c_int as usize])
                .wrapping_add(0x8d2a4c8a as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 20 as libc::c_int | b >> 32 as libc::c_int - 20 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(data[5 as libc::c_int as usize])
                .wrapping_add(0xfffa3942 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(data[8 as libc::c_int as usize])
                .wrapping_add(0x8771f681 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(data[11 as libc::c_int as usize])
                .wrapping_add(0x6d9d6122 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(data[14 as libc::c_int as usize])
                .wrapping_add(0xfde5380c as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(data[1 as libc::c_int as usize])
                .wrapping_add(0xa4beea44 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(data[4 as libc::c_int as usize])
                .wrapping_add(0x4bdecfa9 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(data[7 as libc::c_int as usize])
                .wrapping_add(0xf6bb4b60 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(data[10 as libc::c_int as usize])
                .wrapping_add(0xbebfbc70 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(data[13 as libc::c_int as usize])
                .wrapping_add(0x289b7ec6 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(data[0 as libc::c_int as usize])
                .wrapping_add(0xeaa127fa as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(data[3 as libc::c_int as usize])
                .wrapping_add(0xd4ef3085 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(data[6 as libc::c_int as usize])
                .wrapping_add(0x4881d05 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(data[9 as libc::c_int as usize])
                .wrapping_add(0xd9d4d039 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 4 as libc::c_int | a >> 32 as libc::c_int - 4 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(data[12 as libc::c_int as usize])
                .wrapping_add(0xe6db99e5 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 11 as libc::c_int | d >> 32 as libc::c_int - 11 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(data[15 as libc::c_int as usize])
                .wrapping_add(0x1fa27cf8 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 16 as libc::c_int | c >> 32 as libc::c_int - 16 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(data[2 as libc::c_int as usize])
                .wrapping_add(0xc4ac5665 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 23 as libc::c_int | b >> 32 as libc::c_int - 23 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(data[0 as libc::c_int as usize])
                .wrapping_add(0xf4292244 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(data[7 as libc::c_int as usize])
                .wrapping_add(0x432aff97 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(data[14 as libc::c_int as usize])
                .wrapping_add(0xab9423a7 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(data[5 as libc::c_int as usize])
                .wrapping_add(0xfc93a039 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(data[12 as libc::c_int as usize])
                .wrapping_add(0x655b59c3 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(data[3 as libc::c_int as usize])
                .wrapping_add(0x8f0ccc92 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(data[10 as libc::c_int as usize])
                .wrapping_add(0xffeff47d as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(data[1 as libc::c_int as usize])
                .wrapping_add(0x85845dd1 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(data[8 as libc::c_int as usize])
                .wrapping_add(0x6fa87e4f as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(data[15 as libc::c_int as usize])
                .wrapping_add(0xfe2ce6e0 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(data[6 as libc::c_int as usize])
                .wrapping_add(0xa3014314 as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(data[13 as libc::c_int as usize])
                .wrapping_add(0x4e0811a1 as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    a = (a as libc::c_uint)
        .wrapping_add(
            (c ^ (b | !d))
                .wrapping_add(data[4 as libc::c_int as usize])
                .wrapping_add(0xf7537e82 as libc::c_uint),
        ) as uint32_t as uint32_t;
    a = a << 6 as libc::c_int | a >> 32 as libc::c_int - 6 as libc::c_int;
    a = (a as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    d = (d as libc::c_uint)
        .wrapping_add(
            (b ^ (a | !c))
                .wrapping_add(data[11 as libc::c_int as usize])
                .wrapping_add(0xbd3af235 as libc::c_uint),
        ) as uint32_t as uint32_t;
    d = d << 10 as libc::c_int | d >> 32 as libc::c_int - 10 as libc::c_int;
    d = (d as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    c = (c as libc::c_uint)
        .wrapping_add(
            (a ^ (d | !b))
                .wrapping_add(data[2 as libc::c_int as usize])
                .wrapping_add(0x2ad7d2bb as libc::c_int as libc::c_uint),
        ) as uint32_t as uint32_t;
    c = c << 15 as libc::c_int | c >> 32 as libc::c_int - 15 as libc::c_int;
    c = (c as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
    b = (b as libc::c_uint)
        .wrapping_add(
            (d ^ (c | !a))
                .wrapping_add(data[9 as libc::c_int as usize])
                .wrapping_add(0xeb86d391 as libc::c_uint),
        ) as uint32_t as uint32_t;
    b = b << 21 as libc::c_int | b >> 32 as libc::c_int - 21 as libc::c_int;
    b = (b as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh1 = *digest.offset(0 as libc::c_int as isize);
    *fresh1 = (*fresh1 as libc::c_uint).wrapping_add(a) as uint32_t as uint32_t;
    let ref mut fresh2 = *digest.offset(1 as libc::c_int as isize);
    *fresh2 = (*fresh2 as libc::c_uint).wrapping_add(b) as uint32_t as uint32_t;
    let ref mut fresh3 = *digest.offset(2 as libc::c_int as isize);
    *fresh3 = (*fresh3 as libc::c_uint).wrapping_add(c) as uint32_t as uint32_t;
    let ref mut fresh4 = *digest.offset(3 as libc::c_int as isize);
    *fresh4 = (*fresh4 as libc::c_uint).wrapping_add(d) as uint32_t as uint32_t;
}
