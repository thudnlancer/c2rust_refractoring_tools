#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    static _nettle_blowfish_initial_ctx: blowfish_ctx;
    fn _nettle_blowfish_encround(
        ctx: *const blowfish_ctx,
        ret_xl: *mut uint32_t,
        ret_xr: *mut uint32_t,
    );
    fn nettle_base64_encode_init(ctx: *mut base64_encode_ctx);
    fn nettle_base64_encode_update(
        ctx: *mut base64_encode_ctx,
        dst: *mut i8,
        length: size_t,
        src: *const uint8_t,
    ) -> size_t;
    fn nettle_base64_encode_final(ctx: *mut base64_encode_ctx, dst: *mut i8) -> size_t;
    fn nettle_base64_decode_init(ctx: *mut base64_decode_ctx);
    fn nettle_base64_decode_update(
        ctx: *mut base64_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const i8,
    ) -> i32;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blowfish_ctx {
    pub s: [[uint32_t; 256]; 4],
    pub p: [uint32_t; 18],
}
pub type bf_key = [uint32_t; 18];
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub salt: [uint32_t; 4],
    pub output: [uint32_t; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ctx: blowfish_ctx,
    pub expanded_key: bf_key,
    pub binary: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encode_ctx {
    pub alphabet: *const i8,
    pub word: libc::c_ushort,
    pub bits: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: u8,
    pub padding: u8,
}
#[inline]
unsafe extern "C" fn nettle_bswap32_n(mut n: u32, mut x: *mut uint32_t) {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while i < n {
        *x.offset(i as isize) = (*x.offset(i as isize)).swap_bytes();
        i = i.wrapping_add(1);
        i;
    }
}
static mut radix64_decode_table: [libc::c_schar; 256] = [
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(2 as i32) as libc::c_schar,
    -(2 as i32) as libc::c_schar,
    -(2 as i32) as libc::c_schar,
    -(2 as i32) as libc::c_schar,
    -(2 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(2 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    0 as i32 as libc::c_schar,
    1 as i32 as libc::c_schar,
    54 as i32 as libc::c_schar,
    55 as i32 as libc::c_schar,
    56 as i32 as libc::c_schar,
    57 as i32 as libc::c_schar,
    58 as i32 as libc::c_schar,
    59 as i32 as libc::c_schar,
    60 as i32 as libc::c_schar,
    61 as i32 as libc::c_schar,
    62 as i32 as libc::c_schar,
    63 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(3 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    2 as i32 as libc::c_schar,
    3 as i32 as libc::c_schar,
    4 as i32 as libc::c_schar,
    5 as i32 as libc::c_schar,
    6 as i32 as libc::c_schar,
    7 as i32 as libc::c_schar,
    8 as i32 as libc::c_schar,
    9 as i32 as libc::c_schar,
    10 as i32 as libc::c_schar,
    11 as i32 as libc::c_schar,
    12 as i32 as libc::c_schar,
    13 as i32 as libc::c_schar,
    14 as i32 as libc::c_schar,
    15 as i32 as libc::c_schar,
    16 as i32 as libc::c_schar,
    17 as i32 as libc::c_schar,
    18 as i32 as libc::c_schar,
    19 as i32 as libc::c_schar,
    20 as i32 as libc::c_schar,
    21 as i32 as libc::c_schar,
    22 as i32 as libc::c_schar,
    23 as i32 as libc::c_schar,
    24 as i32 as libc::c_schar,
    25 as i32 as libc::c_schar,
    26 as i32 as libc::c_schar,
    27 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    28 as i32 as libc::c_schar,
    29 as i32 as libc::c_schar,
    30 as i32 as libc::c_schar,
    31 as i32 as libc::c_schar,
    32 as i32 as libc::c_schar,
    33 as i32 as libc::c_schar,
    34 as i32 as libc::c_schar,
    35 as i32 as libc::c_schar,
    36 as i32 as libc::c_schar,
    37 as i32 as libc::c_schar,
    38 as i32 as libc::c_schar,
    39 as i32 as libc::c_schar,
    40 as i32 as libc::c_schar,
    41 as i32 as libc::c_schar,
    42 as i32 as libc::c_schar,
    43 as i32 as libc::c_schar,
    44 as i32 as libc::c_schar,
    45 as i32 as libc::c_schar,
    46 as i32 as libc::c_schar,
    47 as i32 as libc::c_schar,
    48 as i32 as libc::c_schar,
    49 as i32 as libc::c_schar,
    50 as i32 as libc::c_schar,
    51 as i32 as libc::c_schar,
    52 as i32 as libc::c_schar,
    53 as i32 as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
    -(1 as i32) as libc::c_schar,
];
static mut radix64_encode_table: [i8; 64] = unsafe {
    *::core::mem::transmute::<
        &[u8; 64],
        &[i8; 64],
    >(b"./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")
};
#[no_mangle]
pub unsafe extern "C" fn nettle_blowfish_bcrypt_verify(
    mut lenkey: size_t,
    mut key: *const uint8_t,
    mut lenhashed: size_t,
    mut hashed: *const uint8_t,
) -> i32 {
    let mut newhash: [uint8_t; 61] = [0; 61];
    return (nettle_blowfish_bcrypt_hash(
        newhash.as_mut_ptr(),
        lenkey,
        key,
        lenhashed,
        hashed,
        -(1 as i32),
        0 as *const uint8_t,
    ) != 0 && strcmp(newhash.as_mut_ptr() as *const i8, hashed as *const i8) == 0)
        as i32;
}
unsafe extern "C" fn encode_radix64(
    mut dst: *mut i8,
    mut len: size_t,
    mut src: *const uint8_t,
) -> *mut i8 {
    let mut ctx: base64_encode_ctx = base64_encode_ctx {
        alphabet: 0 as *const i8,
        word: 0,
        bits: 0,
    };
    nettle_base64_encode_init(&mut ctx);
    ctx.alphabet = radix64_encode_table.as_ptr();
    dst = dst.offset(nettle_base64_encode_update(&mut ctx, dst, len, src) as isize);
    dst = dst.offset(nettle_base64_encode_final(&mut ctx, dst) as isize);
    dst = dst.offset(-1);
    *dst = '\0' as i32 as i8;
    return dst;
}
static mut magic_w: [uint32_t; 6] = [
    0x4f727068 as i32 as uint32_t,
    0x65616e42 as i32 as uint32_t,
    0x65686f6c as i32 as uint32_t,
    0x64657253 as i32 as uint32_t,
    0x63727944 as i32 as uint32_t,
    0x6f756274 as i32 as uint32_t,
];
unsafe extern "C" fn set_xkey(
    mut lenkey: size_t,
    mut key: *const uint8_t,
    mut expanded: *mut uint32_t,
    mut initial: *mut uint32_t,
    mut bug: u32,
    mut safety: uint32_t,
) {
    let mut ptr: *const uint8_t = key;
    let mut n: size_t = lenkey;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut sign: uint32_t = 0;
    let mut diff: uint32_t = 0;
    let mut tmp: [uint32_t; 2] = [0; 2];
    diff = 0 as i32 as uint32_t;
    sign = diff;
    i = 0 as i32 as u32;
    while i < (16 as i32 + 2 as i32) as u32 {
        tmp[1 as i32 as usize] = 0 as i32 as uint32_t;
        tmp[0 as i32 as usize] = tmp[1 as i32 as usize];
        j = 0 as i32 as u32;
        while j < 4 as i32 as u32 {
            tmp[0 as i32 as usize] <<= 8 as i32;
            tmp[0 as i32 as usize] |= *ptr as u32;
            tmp[1 as i32 as usize] <<= 8 as i32;
            tmp[1 as i32 as usize] |= *ptr as libc::c_schar as u32;
            if j != 0 {
                sign |= tmp[1 as i32 as usize] & 0x80 as i32 as u32;
            }
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if fresh0 != 0 {
                ptr = ptr.offset(1);
                ptr;
            } else {
                ptr = key;
                n = lenkey;
            }
            j = j.wrapping_add(1);
            j;
        }
        diff |= tmp[0 as i32 as usize] ^ tmp[1 as i32 as usize];
        *expanded.offset(i as isize) = tmp[bug as usize];
        *initial.offset(i as isize) = _nettle_blowfish_initial_ctx.p[i as usize]
            ^ tmp[bug as usize];
        i = i.wrapping_add(1);
        i;
    }
    diff |= diff >> 16 as i32;
    diff &= 0xffff as i32 as u32;
    diff = (diff as u32).wrapping_add(0xffff as i32 as u32) as uint32_t as uint32_t;
    sign <<= 9 as i32;
    sign &= !diff & safety;
    let ref mut fresh1 = *initial.offset(0 as i32 as isize);
    *fresh1 ^= sign;
}
unsafe extern "C" fn ibcrypt(
    mut dst: *mut uint8_t,
    mut lenkey: size_t,
    mut key: *const uint8_t,
    mut lenscheme: size_t,
    mut scheme: *const uint8_t,
    mut minlog2rounds: i32,
    mut log2rounds: i32,
    mut salt: *const uint8_t,
) -> i32 {
    let mut data: C2RustUnnamed_0 = C2RustUnnamed_0 {
        ctx: blowfish_ctx {
            s: [[0; 256]; 4],
            p: [0; 18],
        },
        expanded_key: [0; 18],
        binary: C2RustUnnamed { salt: [0; 4] },
    };
    let mut psalt: [uint8_t; 16] = [0; 16];
    let mut L: uint32_t = 0;
    let mut R: uint32_t = 0;
    let mut ptr: *mut uint32_t = 0 as *mut uint32_t;
    let mut count: uint32_t = 0;
    let mut i: i32 = 0;
    let mut cscheme: u32 = 0;
    let mut bug: u32 = 0 as i32 as u32;
    let mut safety: uint32_t = 0 as i32 as uint32_t;
    if lenscheme < 2 as i32 as u64 {
        return 0 as i32;
    }
    if lenscheme >= 3 as i32 as u64
        && {
            let fresh2 = scheme;
            scheme = scheme.offset(1);
            *fresh2 as i32 != '$' as i32
        }
    {
        return 0 as i32;
    }
    let fresh3 = scheme;
    scheme = scheme.offset(1);
    if *fresh3 as i32 != '2' as i32 {
        return 0 as i32;
    }
    let fresh4 = scheme;
    scheme = scheme.offset(1);
    cscheme = *fresh4 as u32;
    match cscheme {
        97 => {
            safety = 0x10000 as i32 as uint32_t;
        }
        120 => {
            bug = 1 as i32 as u32;
        }
        98 | 121 => {}
        _ => return 0 as i32,
    }
    if lenscheme >= 4 as i32 as u64 {
        let fresh5 = scheme;
        scheme = scheme.offset(1);
        if *fresh5 as i32 != '$' as i32 {
            return 0 as i32;
        }
        if lenscheme >= 6 as i32 as u64 {
            if log2rounds < 0 as i32 {
                let fresh6 = scheme;
                scheme = scheme.offset(1);
                let mut c: u32 = (*fresh6 as i32 - '0' as i32) as u32;
                if c > 9 as i32 as u32 {
                    return 0 as i32;
                }
                log2rounds = c.wrapping_mul(10 as i32 as u32) as i32;
                let fresh7 = scheme;
                scheme = scheme.offset(1);
                c = (*fresh7 as i32 - '0' as i32) as u32;
                if c > 9 as i32 as u32 {
                    return 0 as i32;
                }
                log2rounds = (log2rounds as u32).wrapping_add(c) as i32 as i32;
            } else {
                scheme = scheme.offset(2 as i32 as isize);
            }
            if lenscheme >= 7 as i32 as u64
                && {
                    let fresh8 = scheme;
                    scheme = scheme.offset(1);
                    *fresh8 as i32 != '$' as i32
                }
            {
                return 0 as i32;
            }
            if lenscheme
                >= (7 as i32 + (16 as i32 * 8 as i32 + 5 as i32) / 6 as i32) as u64
                && salt.is_null()
            {
                let mut ctx: base64_decode_ctx = base64_decode_ctx {
                    table: 0 as *const libc::c_schar,
                    word: 0,
                    bits: 0,
                    padding: 0,
                };
                let mut saltlen: size_t = 16 as i32 as size_t;
                nettle_base64_decode_init(&mut ctx);
                ctx.table = radix64_decode_table.as_ptr();
                if nettle_base64_decode_update(
                    &mut ctx,
                    &mut saltlen,
                    (data.binary.salt).as_mut_ptr() as *mut uint8_t,
                    ((16 as i32 * 8 as i32 + 5 as i32) / 6 as i32) as size_t,
                    scheme as *const i8,
                ) == 0 || saltlen != 16 as i32 as u64
                {
                    return 0 as i32;
                }
            }
        }
    }
    if !salt.is_null() {
        memcpy(
            (data.binary.salt).as_mut_ptr() as *mut libc::c_void,
            salt as *const libc::c_void,
            16 as i32 as u64,
        );
    } else if lenscheme
        < (7 as i32 + (16 as i32 * 8 as i32 + 5 as i32) / 6 as i32) as u64
    {
        return 0 as i32
    }
    memcpy(
        psalt.as_mut_ptr() as *mut libc::c_void,
        (data.binary.salt).as_mut_ptr() as *const libc::c_void,
        16 as i32 as u64,
    );
    nettle_bswap32_n(4 as i32 as u32, (data.binary.salt).as_mut_ptr());
    if log2rounds < minlog2rounds || log2rounds > 31 as i32 {
        return 0 as i32;
    }
    count = (1 as i32 as uint32_t) << log2rounds;
    set_xkey(
        lenkey,
        key,
        (data.expanded_key).as_mut_ptr(),
        (data.ctx.p).as_mut_ptr(),
        bug,
        safety,
    );
    memcpy(
        (data.ctx.s).as_mut_ptr() as *mut libc::c_void,
        (_nettle_blowfish_initial_ctx.s).as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[[uint32_t; 256]; 4]>() as u64,
    );
    R = 0 as i32 as uint32_t;
    L = R;
    i = 0 as i32;
    while i < 16 as i32 + 2 as i32 {
        L ^= data.binary.salt[(i & 2 as i32) as usize];
        R ^= data.binary.salt[((i & 2 as i32) + 1 as i32) as usize];
        _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
        data.ctx.p[i as usize] = L;
        data.ctx.p[(i + 1 as i32) as usize] = R;
        i += 2 as i32;
    }
    ptr = (data.ctx.s[0 as i32 as usize]).as_mut_ptr();
    loop {
        ptr = ptr.offset(4 as i32 as isize);
        L ^= data.binary.salt[(16 as i32 + 2 as i32 & 3 as i32) as usize];
        R ^= data.binary.salt[(16 as i32 + 3 as i32 & 3 as i32) as usize];
        _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
        *ptr.offset(-(4 as i32 as isize)) = L;
        *ptr.offset(-(3 as i32 as isize)) = R;
        L ^= data.binary.salt[(16 as i32 + 4 as i32 & 3 as i32) as usize];
        R ^= data.binary.salt[(16 as i32 + 5 as i32 & 3 as i32) as usize];
        _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
        *ptr.offset(-(2 as i32 as isize)) = L;
        *ptr.offset(-(1 as i32 as isize)) = R;
        if !(ptr
            < &mut *(*(data.ctx.s).as_mut_ptr().offset(3 as i32 as isize))
                .as_mut_ptr()
                .offset(0xff as i32 as isize) as *mut uint32_t)
        {
            break;
        }
    }
    loop {
        let mut done: i32 = 0;
        i = 0 as i32;
        while i < 16 as i32 + 2 as i32 {
            data.ctx.p[i as usize] ^= data.expanded_key[i as usize];
            data.ctx.p[(i + 1 as i32) as usize]
                ^= data.expanded_key[(i + 1 as i32) as usize];
            i += 2 as i32;
        }
        done = 0 as i32;
        loop {
            let mut tmp1: uint32_t = 0;
            let mut tmp2: uint32_t = 0;
            let mut tmp3: uint32_t = 0;
            let mut tmp4: uint32_t = 0;
            R = 0 as i32 as uint32_t;
            L = R;
            ptr = (data.ctx.p).as_mut_ptr();
            loop {
                ptr = ptr.offset(2 as i32 as isize);
                _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
                *ptr.offset(-(2 as i32 as isize)) = L;
                *ptr.offset(-(1 as i32 as isize)) = R;
                if !(ptr
                    < &mut *(data.ctx.p)
                        .as_mut_ptr()
                        .offset((16 as i32 + 2 as i32) as isize) as *mut uint32_t)
                {
                    break;
                }
            }
            ptr = (data.ctx.s[0 as i32 as usize]).as_mut_ptr();
            loop {
                ptr = ptr.offset(2 as i32 as isize);
                _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
                *ptr.offset(-(2 as i32 as isize)) = L;
                *ptr.offset(-(1 as i32 as isize)) = R;
                if !(ptr
                    < &mut *(*(data.ctx.s).as_mut_ptr().offset(3 as i32 as isize))
                        .as_mut_ptr()
                        .offset(0xff as i32 as isize) as *mut uint32_t)
                {
                    break;
                }
            }
            if done != 0 {
                break;
            }
            done = 1 as i32;
            tmp1 = data.binary.salt[0 as i32 as usize];
            tmp2 = data.binary.salt[1 as i32 as usize];
            tmp3 = data.binary.salt[2 as i32 as usize];
            tmp4 = data.binary.salt[3 as i32 as usize];
            i = 0 as i32;
            while i < 16 as i32 {
                data.ctx.p[i as usize] ^= tmp1;
                data.ctx.p[(i + 1 as i32) as usize] ^= tmp2;
                data.ctx.p[(i + 2 as i32) as usize] ^= tmp3;
                data.ctx.p[(i + 3 as i32) as usize] ^= tmp4;
                i += 4 as i32;
            }
            data.ctx.p[16 as i32 as usize] ^= tmp1;
            data.ctx.p[17 as i32 as usize] ^= tmp2;
        }
        count = count.wrapping_sub(1);
        if !(count != 0) {
            break;
        }
    }
    i = 0 as i32;
    while i < 6 as i32 {
        L = magic_w[i as usize];
        R = magic_w[(i + 1 as i32) as usize];
        count = 64 as i32 as uint32_t;
        loop {
            _nettle_blowfish_encround(&mut data.ctx, &mut L, &mut R);
            count = count.wrapping_sub(1);
            if !(count != 0) {
                break;
            }
        }
        data.binary.output[i as usize] = L;
        data.binary.output[(i + 1 as i32) as usize] = R;
        i += 2 as i32;
    }
    let fresh9 = dst;
    dst = dst.offset(1);
    *fresh9 = '$' as i32 as uint8_t;
    let fresh10 = dst;
    dst = dst.offset(1);
    *fresh10 = '2' as i32 as uint8_t;
    let fresh11 = dst;
    dst = dst.offset(1);
    *fresh11 = cscheme as uint8_t;
    let fresh12 = dst;
    dst = dst.offset(1);
    *fresh12 = '$' as i32 as uint8_t;
    let fresh13 = dst;
    dst = dst.offset(1);
    *fresh13 = ('0' as i32 + log2rounds / 10 as i32) as uint8_t;
    let fresh14 = dst;
    dst = dst.offset(1);
    *fresh14 = ('0' as i32 + log2rounds % 10 as i32) as uint8_t;
    let fresh15 = dst;
    dst = dst.offset(1);
    *fresh15 = '$' as i32 as uint8_t;
    dst = (encode_radix64(dst as *mut i8, 16 as i32 as size_t, psalt.as_mut_ptr())
        as *mut uint8_t)
        .offset(-(1 as i32 as isize));
    nettle_bswap32_n(6 as i32 as u32, (data.binary.output).as_mut_ptr());
    encode_radix64(
        dst as *mut i8,
        23 as i32 as size_t,
        (data.binary.output).as_mut_ptr() as *mut uint8_t,
    );
    return cscheme as i32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_blowfish_bcrypt_hash(
    mut dst: *mut uint8_t,
    mut lenkey: size_t,
    mut key: *const uint8_t,
    mut lenscheme: size_t,
    mut scheme: *const uint8_t,
    mut log2rounds: i32,
    mut salt: *const uint8_t,
) -> i32 {
    let test_pw: [uint8_t; 10] = *::core::mem::transmute::<
        &[u8; 10],
        &[uint8_t; 10],
    >(b"8b \xD0\xC1\xD2\xCF\xCC\xD8\0");
    let test_scheme: [uint8_t; 30] = *::core::mem::transmute::<
        &[u8; 30],
        &[uint8_t; 30],
    >(b"$2a$00$abcdefghijklmnopqrstuu\0");
    static mut test_hashes: [*const i8; 2] = [
        b"i1D709vfamulimlGcq0qq3UvuUasvEa\0U\0" as *const u8 as *const i8,
        b"VUrPmXD6q/nVSSp7pNDhCR9071IfIRe\0U\0" as *const u8 as *const i8,
    ];
    let mut test_hash: *const i8 = test_hashes[0 as i32 as usize];
    let mut cscheme: i32 = 0;
    let mut ok: i32 = 0;
    let mut bufs: [uint8_t; 29] = [0; 29];
    let mut bufo: [uint8_t; 61] = [0; 61];
    *dst = '\0' as i32 as uint8_t;
    cscheme = ibcrypt(dst, lenkey, key, lenscheme, scheme, 4 as i32, log2rounds, salt);
    memcpy(
        bufs.as_mut_ptr() as *mut libc::c_void,
        test_scheme.as_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[uint8_t; 30]>() as u64).wrapping_sub(1 as i32 as u64),
    );
    if cscheme != 0 {
        bufs[2 as i32 as usize] = cscheme as uint8_t;
        test_hash = test_hashes[(bufs[2 as i32 as usize] as i32 == 'x' as i32) as i32
            as usize];
    }
    *bufo.as_mut_ptr() = 0 as i32 as uint8_t;
    ok = ibcrypt(
        bufo.as_mut_ptr(),
        (::core::mem::size_of::<[uint8_t; 10]>() as u64).wrapping_sub(1 as i32 as u64),
        test_pw.as_ptr(),
        ::core::mem::size_of::<[uint8_t; 29]>() as u64,
        bufs.as_mut_ptr(),
        0 as i32,
        -(1 as i32),
        0 as *const uint8_t,
    );
    ok = (ok != 0
        && memcmp(
            bufo.as_mut_ptr() as *const libc::c_void,
            bufs.as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[uint8_t; 29]>() as u64,
        ) == 0
        && memcmp(
            bufo
                .as_mut_ptr()
                .offset(
                    (7 as i32 + (16 as i32 * 8 as i32 + 5 as i32) / 6 as i32) as isize,
                ) as *const libc::c_void,
            test_hash as *const libc::c_void,
            (::core::mem::size_of::<*const i8>() as u64).wrapping_sub(1 as i32 as u64),
        ) == 0) as i32;
    let k: [uint8_t; 12] = *::core::mem::transmute::<
        &[u8; 12],
        &[uint8_t; 12],
    >(b"\xFF\xA334\xFF\xFF\xFF\xA3345\0");
    let mut ae: bf_key = [0; 18];
    let mut ai: bf_key = [0; 18];
    let mut ye: bf_key = [0; 18];
    let mut yi: bf_key = [0; 18];
    set_xkey(
        (::core::mem::size_of::<[uint8_t; 12]>() as u64).wrapping_sub(1 as i32 as u64),
        k.as_ptr(),
        ae.as_mut_ptr(),
        ai.as_mut_ptr(),
        0 as i32 as u32,
        0x10000 as i32 as uint32_t,
    );
    set_xkey(
        (::core::mem::size_of::<[uint8_t; 12]>() as u64).wrapping_sub(1 as i32 as u64),
        k.as_ptr(),
        ye.as_mut_ptr(),
        yi.as_mut_ptr(),
        0 as i32 as u32,
        0 as i32 as uint32_t,
    );
    ai[0 as i32 as usize] ^= 0x10000 as i32 as u32;
    ok = (ok != 0 && ai[0 as i32 as usize] == 0xdb9c59bc as u32
        && ye[17 as i32 as usize] == 0x33343500 as i32 as u32
        && memcmp(
            ae.as_mut_ptr() as *const libc::c_void,
            ye.as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<bf_key>() as u64,
        ) == 0
        && memcmp(
            ai.as_mut_ptr() as *const libc::c_void,
            yi.as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<bf_key>() as u64,
        ) == 0) as i32;
    return (ok != 0 && cscheme != 0) as i32;
}