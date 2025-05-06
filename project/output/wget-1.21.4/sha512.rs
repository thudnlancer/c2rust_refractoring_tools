#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm)]
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uintptr_t = u64;
pub type u64_0 = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx {
    pub state: [u64_0; 8],
    pub total: [u64_0; 2],
    pub buflen: size_t,
    pub buffer: [u64_0; 32],
}
static mut fillbuf: [u8; 128] = [
    0x80 as i32 as u8,
    0 as i32 as u8,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub unsafe extern "C" fn sha512_init_ctx(mut ctx: *mut sha512_ctx) {
    (*ctx).state[0 as i32 as usize] = ((0x6a09e667 as i32 as u64_0) << 32 as i32)
        .wrapping_add(0xf3bcc908 as u32 as u64);
    (*ctx).state[1 as i32 as usize] = ((0xbb67ae85 as u32 as u64_0) << 32 as i32)
        .wrapping_add(0x84caa73b as u32 as u64);
    (*ctx).state[2 as i32 as usize] = ((0x3c6ef372 as i32 as u64_0) << 32 as i32)
        .wrapping_add(0xfe94f82b as u32 as u64);
    (*ctx).state[3 as i32 as usize] = ((0xa54ff53a as u32 as u64_0) << 32 as i32)
        .wrapping_add(0x5f1d36f1 as i32 as u64);
    (*ctx).state[4 as i32 as usize] = ((0x510e527f as i32 as u64_0) << 32 as i32)
        .wrapping_add(0xade682d1 as u32 as u64);
    (*ctx).state[5 as i32 as usize] = ((0x9b05688c as u32 as u64_0) << 32 as i32)
        .wrapping_add(0x2b3e6c1f as i32 as u64);
    (*ctx).state[6 as i32 as usize] = ((0x1f83d9ab as i32 as u64_0) << 32 as i32)
        .wrapping_add(0xfb41bd6b as u32 as u64);
    (*ctx).state[7 as i32 as usize] = ((0x5be0cd19 as i32 as u64_0) << 32 as i32)
        .wrapping_add(0x137e2179 as i32 as u64);
    (*ctx).total[1 as i32 as usize] = 0 as i32 as u64_0;
    (*ctx).total[0 as i32 as usize] = (*ctx).total[1 as i32 as usize];
    (*ctx).buflen = 0 as i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn sha384_init_ctx(mut ctx: *mut sha512_ctx) {
    (*ctx).state[0 as i32 as usize] = ((0xcbbb9d5d as u32 as u64_0) << 32 as i32)
        .wrapping_add(0xc1059ed8 as u32 as u64);
    (*ctx).state[1 as i32 as usize] = ((0x629a292a as i32 as u64_0) << 32 as i32)
        .wrapping_add(0x367cd507 as i32 as u64);
    (*ctx).state[2 as i32 as usize] = ((0x9159015a as u32 as u64_0) << 32 as i32)
        .wrapping_add(0x3070dd17 as i32 as u64);
    (*ctx).state[3 as i32 as usize] = ((0x152fecd8 as i32 as u64_0) << 32 as i32)
        .wrapping_add(0xf70e5939 as u32 as u64);
    (*ctx).state[4 as i32 as usize] = ((0x67332667 as i32 as u64_0) << 32 as i32)
        .wrapping_add(0xffc00b31 as u32 as u64);
    (*ctx).state[5 as i32 as usize] = ((0x8eb44a87 as u32 as u64_0) << 32 as i32)
        .wrapping_add(0x68581511 as i32 as u64);
    (*ctx).state[6 as i32 as usize] = ((0xdb0c2e0d as u32 as u64_0) << 32 as i32)
        .wrapping_add(0x64f98fa7 as i32 as u64);
    (*ctx).state[7 as i32 as usize] = ((0x47b5481d as i32 as u64_0) << 32 as i32)
        .wrapping_add(0xbefa4fa4 as u32 as u64);
    (*ctx).total[1 as i32 as usize] = 0 as i32 as u64_0;
    (*ctx).total[0 as i32 as usize] = (*ctx).total[1 as i32 as usize];
    (*ctx).buflen = 0 as i32 as size_t;
}
unsafe extern "C" fn set_uint64(mut cp: *mut i8, mut v: u64_0) {
    memcpy(
        cp as *mut libc::c_void,
        &mut v as *mut u64_0 as *const libc::c_void,
        ::core::mem::size_of::<u64_0>() as u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sha512_read_ctx(
    mut ctx: *const sha512_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: i32 = 0;
    let mut r: *mut i8 = resbuf as *mut i8;
    i = 0 as i32;
    while i < 8 as i32 {
        set_uint64(
            r
                .offset(
                    (i as u64).wrapping_mul(::core::mem::size_of::<u64_0>() as u64)
                        as isize,
                ),
            ({
                let mut __v: __uint64_t = 0;
                let mut __x: __uint64_t = (*ctx).state[i as usize];
                if 0 != 0 {
                    __v = ((__x as libc::c_ulonglong
                        & 0xff00000000000000 as libc::c_ulonglong) >> 56 as i32
                        | (__x as libc::c_ulonglong
                            & 0xff000000000000 as libc::c_ulonglong) >> 40 as i32
                        | (__x as libc::c_ulonglong
                            & 0xff0000000000 as libc::c_ulonglong) >> 24 as i32
                        | (__x as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong)
                            >> 8 as i32
                        | (__x as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong)
                            << 8 as i32
                        | (__x as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong)
                            << 24 as i32
                        | (__x as libc::c_ulonglong & 0xff00 as libc::c_ulonglong)
                            << 40 as i32
                        | (__x as libc::c_ulonglong & 0xff as libc::c_ulonglong)
                            << 56 as i32) as __uint64_t;
                } else {
                    let fresh0 = &mut __v;
                    let fresh1;
                    let fresh2 = __x;
                    asm!(
                        "bswap {0:r}", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
                }
                __v
            }),
        );
        i += 1;
        i;
    }
    return resbuf;
}
#[no_mangle]
pub unsafe extern "C" fn sha384_read_ctx(
    mut ctx: *const sha512_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: i32 = 0;
    let mut r: *mut i8 = resbuf as *mut i8;
    i = 0 as i32;
    while i < 6 as i32 {
        set_uint64(
            r
                .offset(
                    (i as u64).wrapping_mul(::core::mem::size_of::<u64_0>() as u64)
                        as isize,
                ),
            ({
                let mut __v: __uint64_t = 0;
                let mut __x: __uint64_t = (*ctx).state[i as usize];
                if 0 != 0 {
                    __v = ((__x as libc::c_ulonglong
                        & 0xff00000000000000 as libc::c_ulonglong) >> 56 as i32
                        | (__x as libc::c_ulonglong
                            & 0xff000000000000 as libc::c_ulonglong) >> 40 as i32
                        | (__x as libc::c_ulonglong
                            & 0xff0000000000 as libc::c_ulonglong) >> 24 as i32
                        | (__x as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong)
                            >> 8 as i32
                        | (__x as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong)
                            << 8 as i32
                        | (__x as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong)
                            << 24 as i32
                        | (__x as libc::c_ulonglong & 0xff00 as libc::c_ulonglong)
                            << 40 as i32
                        | (__x as libc::c_ulonglong & 0xff as libc::c_ulonglong)
                            << 56 as i32) as __uint64_t;
                } else {
                    let fresh3 = &mut __v;
                    let fresh4;
                    let fresh5 = __x;
                    asm!(
                        "bswap {0:r}", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) => fresh4,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
                }
                __v
            }),
        );
        i += 1;
        i;
    }
    return resbuf;
}
unsafe extern "C" fn sha512_conclude_ctx(mut ctx: *mut sha512_ctx) {
    let mut bytes: size_t = (*ctx).buflen;
    let mut size: size_t = (if bytes < 112 as i32 as u64 {
        128 as i32 / 8 as i32
    } else {
        128 as i32 * 2 as i32 / 8 as i32
    }) as size_t;
    (*ctx).total[0 as i32 as usize] = ((*ctx).total[0 as i32 as usize])
        .wrapping_add(bytes);
    if (*ctx).total[0 as i32 as usize] < bytes {
        (*ctx).total[1 as i32 as usize] = ((*ctx).total[1 as i32 as usize])
            .wrapping_add(1 as i32 as u64_0);
    }
    set_uint64(
        &mut *((*ctx).buffer)
            .as_mut_ptr()
            .offset(size.wrapping_sub(2 as i32 as u64) as isize) as *mut u64_0
            as *mut i8,
        ({
            let mut __v: __uint64_t = 0;
            let mut __x: __uint64_t = (*ctx).total[1 as i32 as usize] << 3 as i32
                | (*ctx).total[0 as i32 as usize] >> 61 as i32;
            if 0 != 0 {
                __v = ((__x as libc::c_ulonglong
                    & 0xff00000000000000 as libc::c_ulonglong) >> 56 as i32
                    | (__x as libc::c_ulonglong & 0xff000000000000 as libc::c_ulonglong)
                        >> 40 as i32
                    | (__x as libc::c_ulonglong & 0xff0000000000 as libc::c_ulonglong)
                        >> 24 as i32
                    | (__x as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong)
                        >> 8 as i32
                    | (__x as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong)
                        << 8 as i32
                    | (__x as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong)
                        << 24 as i32
                    | (__x as libc::c_ulonglong & 0xff00 as libc::c_ulonglong)
                        << 40 as i32
                    | (__x as libc::c_ulonglong & 0xff as libc::c_ulonglong)
                        << 56 as i32) as __uint64_t;
            } else {
                let fresh6 = &mut __v;
                let fresh7;
                let fresh8 = __x;
                asm!(
                    "bswap {0:r}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8) => fresh7,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
            }
            __v
        }),
    );
    set_uint64(
        &mut *((*ctx).buffer)
            .as_mut_ptr()
            .offset(size.wrapping_sub(1 as i32 as u64) as isize) as *mut u64_0
            as *mut i8,
        ({
            let mut __v: __uint64_t = 0;
            let mut __x: __uint64_t = (*ctx).total[0 as i32 as usize] << 3 as i32;
            if 0 != 0 {
                __v = ((__x as libc::c_ulonglong
                    & 0xff00000000000000 as libc::c_ulonglong) >> 56 as i32
                    | (__x as libc::c_ulonglong & 0xff000000000000 as libc::c_ulonglong)
                        >> 40 as i32
                    | (__x as libc::c_ulonglong & 0xff0000000000 as libc::c_ulonglong)
                        >> 24 as i32
                    | (__x as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong)
                        >> 8 as i32
                    | (__x as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong)
                        << 8 as i32
                    | (__x as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong)
                        << 24 as i32
                    | (__x as libc::c_ulonglong & 0xff00 as libc::c_ulonglong)
                        << 40 as i32
                    | (__x as libc::c_ulonglong & 0xff as libc::c_ulonglong)
                        << 56 as i32) as __uint64_t;
            } else {
                let fresh9 = &mut __v;
                let fresh10;
                let fresh11 = __x;
                asm!(
                    "bswap {0:r}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11) => fresh10,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
            }
            __v
        }),
    );
    memcpy(
        &mut *(((*ctx).buffer).as_mut_ptr() as *mut i8).offset(bytes as isize) as *mut i8
            as *mut libc::c_void,
        fillbuf.as_ptr() as *const libc::c_void,
        size
            .wrapping_sub(2 as i32 as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_sub(bytes),
    );
    sha512_process_block(
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        size.wrapping_mul(8 as i32 as u64),
        ctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sha512_finish_ctx(
    mut ctx: *mut sha512_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    sha512_conclude_ctx(ctx);
    return sha512_read_ctx(ctx, resbuf);
}
#[no_mangle]
pub unsafe extern "C" fn sha384_finish_ctx(
    mut ctx: *mut sha512_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    sha512_conclude_ctx(ctx);
    return sha384_read_ctx(ctx, resbuf);
}
#[no_mangle]
pub unsafe extern "C" fn sha512_buffer(
    mut buffer: *const i8,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: sha512_ctx = sha512_ctx {
        state: [0; 8],
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    sha512_init_ctx(&mut ctx);
    sha512_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return sha512_finish_ctx(&mut ctx, resblock);
}
#[no_mangle]
pub unsafe extern "C" fn sha384_buffer(
    mut buffer: *const i8,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: sha512_ctx = sha512_ctx {
        state: [0; 8],
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    sha384_init_ctx(&mut ctx);
    sha512_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return sha384_finish_ctx(&mut ctx, resblock);
}
#[no_mangle]
pub unsafe extern "C" fn sha512_process_bytes(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha512_ctx,
) {
    if (*ctx).buflen != 0 as i32 as u64 {
        let mut left_over: size_t = (*ctx).buflen;
        let mut add: size_t = if (256 as i32 as u64).wrapping_sub(left_over) > len {
            len
        } else {
            (256 as i32 as u64).wrapping_sub(left_over)
        };
        memcpy(
            &mut *(((*ctx).buffer).as_mut_ptr() as *mut i8).offset(left_over as isize)
                as *mut i8 as *mut libc::c_void,
            buffer,
            add,
        );
        (*ctx).buflen = ((*ctx).buflen as u64).wrapping_add(add) as size_t as size_t;
        if (*ctx).buflen > 128 as i32 as u64 {
            sha512_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                (*ctx).buflen & !(127 as i32) as u64,
                ctx,
            );
            (*ctx).buflen &= 127 as i32 as u64;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *(((*ctx).buffer).as_mut_ptr() as *mut i8)
                    .offset(
                        (left_over.wrapping_add(add) & !(127 as i32) as u64) as isize,
                    ) as *mut i8 as *const libc::c_void,
                (*ctx).buflen,
            );
        }
        buffer = (buffer as *const i8).offset(add as isize) as *const libc::c_void;
        len = (len as u64).wrapping_sub(add) as size_t as size_t;
    }
    if len >= 128 as i32 as u64 {
        if (buffer as uintptr_t).wrapping_rem(8 as u64) != 0 as i32 as u64 {
            while len > 128 as i32 as u64 {
                sha512_process_block(
                    memcpy(
                        ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                        buffer,
                        128 as i32 as u64,
                    ),
                    128 as i32 as size_t,
                    ctx,
                );
                buffer = (buffer as *const i8).offset(128 as i32 as isize)
                    as *const libc::c_void;
                len = (len as u64).wrapping_sub(128 as i32 as u64) as size_t as size_t;
            }
        } else {
            sha512_process_block(buffer, len & !(127 as i32) as u64, ctx);
            buffer = (buffer as *const i8).offset((len & !(127 as i32) as u64) as isize)
                as *const libc::c_void;
            len &= 127 as i32 as u64;
        }
    }
    if len > 0 as i32 as u64 {
        let mut left_over_0: size_t = (*ctx).buflen;
        memcpy(
            &mut *(((*ctx).buffer).as_mut_ptr() as *mut i8).offset(left_over_0 as isize)
                as *mut i8 as *mut libc::c_void,
            buffer,
            len,
        );
        left_over_0 = (left_over_0 as u64).wrapping_add(len) as size_t as size_t;
        if left_over_0 >= 128 as i32 as u64 {
            sha512_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                128 as i32 as size_t,
                ctx,
            );
            left_over_0 = (left_over_0 as u64).wrapping_sub(128 as i32 as u64) as size_t
                as size_t;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *((*ctx).buffer).as_mut_ptr().offset(16 as i32 as isize)
                    as *mut u64_0 as *const libc::c_void,
                left_over_0,
            );
        }
        (*ctx).buflen = left_over_0;
    }
}
static mut sha512_round_constants: [u64_0; 80] = [0; 80];
#[no_mangle]
pub unsafe extern "C" fn sha512_process_block(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha512_ctx,
) {
    let mut words: *const u64_0 = buffer as *const u64_0;
    let mut endp: *const u64_0 = words
        .offset(len.wrapping_div(::core::mem::size_of::<u64_0>() as u64) as isize);
    let mut x: [u64_0; 16] = [0; 16];
    let mut a: u64_0 = (*ctx).state[0 as i32 as usize];
    let mut b: u64_0 = (*ctx).state[1 as i32 as usize];
    let mut c: u64_0 = (*ctx).state[2 as i32 as usize];
    let mut d: u64_0 = (*ctx).state[3 as i32 as usize];
    let mut e: u64_0 = (*ctx).state[4 as i32 as usize];
    let mut f: u64_0 = (*ctx).state[5 as i32 as usize];
    let mut g: u64_0 = (*ctx).state[6 as i32 as usize];
    let mut h: u64_0 = (*ctx).state[7 as i32 as usize];
    let mut lolen: u64_0 = len;
    (*ctx).total[0 as i32 as usize] = ((*ctx).total[0 as i32 as usize])
        .wrapping_add(lolen);
    (*ctx).total[1 as i32 as usize] = ((*ctx).total[1 as i32 as usize])
        .wrapping_add(
            (len >> 31 as i32 >> 31 as i32 >> 2 as i32)
                .wrapping_add(((*ctx).total[0 as i32 as usize] < lolen) as i32 as u64_0),
        );
    while words < endp {
        let mut t: i32 = 0;
        t = 0 as i32;
        while t < 16 as i32 {
            x[t as usize] = ({
                let mut __v: __uint64_t = 0;
                let mut __x: __uint64_t = *words;
                if 0 != 0 {
                    __v = ((__x as libc::c_ulonglong
                        & 0xff00000000000000 as libc::c_ulonglong) >> 56 as i32
                        | (__x as libc::c_ulonglong
                            & 0xff000000000000 as libc::c_ulonglong) >> 40 as i32
                        | (__x as libc::c_ulonglong
                            & 0xff0000000000 as libc::c_ulonglong) >> 24 as i32
                        | (__x as libc::c_ulonglong & 0xff00000000 as libc::c_ulonglong)
                            >> 8 as i32
                        | (__x as libc::c_ulonglong & 0xff000000 as libc::c_ulonglong)
                            << 8 as i32
                        | (__x as libc::c_ulonglong & 0xff0000 as libc::c_ulonglong)
                            << 24 as i32
                        | (__x as libc::c_ulonglong & 0xff00 as libc::c_ulonglong)
                            << 40 as i32
                        | (__x as libc::c_ulonglong & 0xff as libc::c_ulonglong)
                            << 56 as i32) as __uint64_t;
                } else {
                    let fresh12 = &mut __v;
                    let fresh13;
                    let fresh14 = __x;
                    asm!(
                        "bswap {0:r}", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14) => fresh13,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
                }
                __v
            });
            words = words.offset(1);
            words;
            t += 1;
            t;
        }
        let mut t0: u64_0 = ((a << 36 as i32 | a >> 64 as i32 - 36 as i32)
            ^ ((a << 30 as i32 | a >> 64 as i32 - 30 as i32)
                ^ (a << 25 as i32 | a >> 64 as i32 - 25 as i32)))
            .wrapping_add(a & b | c & (a | b));
        let mut t1: u64_0 = h
            .wrapping_add(
                ((e << 50 as i32 | e >> 64 as i32 - 50 as i32)
                    ^ ((e << 46 as i32 | e >> 64 as i32 - 46 as i32)
                        ^ (e << 23 as i32 | e >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[0 as i32 as usize])
                                    .wrapping_add(x[0 as i32 as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1);
        h = t0.wrapping_add(t1);
        let mut t0_0: u64_0 = ((h << 36 as i32 | h >> 64 as i32 - 36 as i32)
            ^ ((h << 30 as i32 | h >> 64 as i32 - 30 as i32)
                ^ (h << 25 as i32 | h >> 64 as i32 - 25 as i32)))
            .wrapping_add(h & a | b & (h | a));
        let mut t1_0: u64_0 = g
            .wrapping_add(
                ((d << 50 as i32 | d >> 64 as i32 - 50 as i32)
                    ^ ((d << 46 as i32 | d >> 64 as i32 - 46 as i32)
                        ^ (d << 23 as i32 | d >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[1 as i32 as usize])
                                    .wrapping_add(x[1 as i32 as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_0);
        g = t0_0.wrapping_add(t1_0);
        let mut t0_1: u64_0 = ((g << 36 as i32 | g >> 64 as i32 - 36 as i32)
            ^ ((g << 30 as i32 | g >> 64 as i32 - 30 as i32)
                ^ (g << 25 as i32 | g >> 64 as i32 - 25 as i32)))
            .wrapping_add(g & h | a & (g | h));
        let mut t1_1: u64_0 = f
            .wrapping_add(
                ((c << 50 as i32 | c >> 64 as i32 - 50 as i32)
                    ^ ((c << 46 as i32 | c >> 64 as i32 - 46 as i32)
                        ^ (c << 23 as i32 | c >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[2 as i32 as usize])
                                    .wrapping_add(x[2 as i32 as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_1);
        f = t0_1.wrapping_add(t1_1);
        let mut t0_2: u64_0 = ((f << 36 as i32 | f >> 64 as i32 - 36 as i32)
            ^ ((f << 30 as i32 | f >> 64 as i32 - 30 as i32)
                ^ (f << 25 as i32 | f >> 64 as i32 - 25 as i32)))
            .wrapping_add(f & g | h & (f | g));
        let mut t1_2: u64_0 = e
            .wrapping_add(
                ((b << 50 as i32 | b >> 64 as i32 - 50 as i32)
                    ^ ((b << 46 as i32 | b >> 64 as i32 - 46 as i32)
                        ^ (b << 23 as i32 | b >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[3 as i32 as usize])
                                    .wrapping_add(x[3 as i32 as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_2);
        e = t0_2.wrapping_add(t1_2);
        let mut t0_3: u64_0 = ((e << 36 as i32 | e >> 64 as i32 - 36 as i32)
            ^ ((e << 30 as i32 | e >> 64 as i32 - 30 as i32)
                ^ (e << 25 as i32 | e >> 64 as i32 - 25 as i32)))
            .wrapping_add(e & f | g & (e | f));
        let mut t1_3: u64_0 = d
            .wrapping_add(
                ((a << 50 as i32 | a >> 64 as i32 - 50 as i32)
                    ^ ((a << 46 as i32 | a >> 64 as i32 - 46 as i32)
                        ^ (a << 23 as i32 | a >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[4 as i32 as usize])
                                    .wrapping_add(x[4 as i32 as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_3);
        d = t0_3.wrapping_add(t1_3);
        let mut t0_4: u64_0 = ((d << 36 as i32 | d >> 64 as i32 - 36 as i32)
            ^ ((d << 30 as i32 | d >> 64 as i32 - 30 as i32)
                ^ (d << 25 as i32 | d >> 64 as i32 - 25 as i32)))
            .wrapping_add(d & e | f & (d | e));
        let mut t1_4: u64_0 = c
            .wrapping_add(
                ((h << 50 as i32 | h >> 64 as i32 - 50 as i32)
                    ^ ((h << 46 as i32 | h >> 64 as i32 - 46 as i32)
                        ^ (h << 23 as i32 | h >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[5 as i32 as usize])
                                    .wrapping_add(x[5 as i32 as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_4);
        c = t0_4.wrapping_add(t1_4);
        let mut t0_5: u64_0 = ((c << 36 as i32 | c >> 64 as i32 - 36 as i32)
            ^ ((c << 30 as i32 | c >> 64 as i32 - 30 as i32)
                ^ (c << 25 as i32 | c >> 64 as i32 - 25 as i32)))
            .wrapping_add(c & d | e & (c | d));
        let mut t1_5: u64_0 = b
            .wrapping_add(
                ((g << 50 as i32 | g >> 64 as i32 - 50 as i32)
                    ^ ((g << 46 as i32 | g >> 64 as i32 - 46 as i32)
                        ^ (g << 23 as i32 | g >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[6 as i32 as usize])
                                    .wrapping_add(x[6 as i32 as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_5);
        b = t0_5.wrapping_add(t1_5);
        let mut t0_6: u64_0 = ((b << 36 as i32 | b >> 64 as i32 - 36 as i32)
            ^ ((b << 30 as i32 | b >> 64 as i32 - 30 as i32)
                ^ (b << 25 as i32 | b >> 64 as i32 - 25 as i32)))
            .wrapping_add(b & c | d & (b | c));
        let mut t1_6: u64_0 = a
            .wrapping_add(
                ((f << 50 as i32 | f >> 64 as i32 - 50 as i32)
                    ^ ((f << 46 as i32 | f >> 64 as i32 - 46 as i32)
                        ^ (f << 23 as i32 | f >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[7 as i32 as usize])
                                    .wrapping_add(x[7 as i32 as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_6);
        a = t0_6.wrapping_add(t1_6);
        let mut t0_7: u64_0 = ((a << 36 as i32 | a >> 64 as i32 - 36 as i32)
            ^ ((a << 30 as i32 | a >> 64 as i32 - 30 as i32)
                ^ (a << 25 as i32 | a >> 64 as i32 - 25 as i32)))
            .wrapping_add(a & b | c & (a | b));
        let mut t1_7: u64_0 = h
            .wrapping_add(
                ((e << 50 as i32 | e >> 64 as i32 - 50 as i32)
                    ^ ((e << 46 as i32 | e >> 64 as i32 - 46 as i32)
                        ^ (e << 23 as i32 | e >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[8 as i32 as usize])
                                    .wrapping_add(x[8 as i32 as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_7);
        h = t0_7.wrapping_add(t1_7);
        let mut t0_8: u64_0 = ((h << 36 as i32 | h >> 64 as i32 - 36 as i32)
            ^ ((h << 30 as i32 | h >> 64 as i32 - 30 as i32)
                ^ (h << 25 as i32 | h >> 64 as i32 - 25 as i32)))
            .wrapping_add(h & a | b & (h | a));
        let mut t1_8: u64_0 = g
            .wrapping_add(
                ((d << 50 as i32 | d >> 64 as i32 - 50 as i32)
                    ^ ((d << 46 as i32 | d >> 64 as i32 - 46 as i32)
                        ^ (d << 23 as i32 | d >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[9 as i32 as usize])
                                    .wrapping_add(x[9 as i32 as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_8);
        g = t0_8.wrapping_add(t1_8);
        let mut t0_9: u64_0 = ((g << 36 as i32 | g >> 64 as i32 - 36 as i32)
            ^ ((g << 30 as i32 | g >> 64 as i32 - 30 as i32)
                ^ (g << 25 as i32 | g >> 64 as i32 - 25 as i32)))
            .wrapping_add(g & h | a & (g | h));
        let mut t1_9: u64_0 = f
            .wrapping_add(
                ((c << 50 as i32 | c >> 64 as i32 - 50 as i32)
                    ^ ((c << 46 as i32 | c >> 64 as i32 - 46 as i32)
                        ^ (c << 23 as i32 | c >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[10 as i32 as usize])
                                    .wrapping_add(x[10 as i32 as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_9);
        f = t0_9.wrapping_add(t1_9);
        let mut t0_10: u64_0 = ((f << 36 as i32 | f >> 64 as i32 - 36 as i32)
            ^ ((f << 30 as i32 | f >> 64 as i32 - 30 as i32)
                ^ (f << 25 as i32 | f >> 64 as i32 - 25 as i32)))
            .wrapping_add(f & g | h & (f | g));
        let mut t1_10: u64_0 = e
            .wrapping_add(
                ((b << 50 as i32 | b >> 64 as i32 - 50 as i32)
                    ^ ((b << 46 as i32 | b >> 64 as i32 - 46 as i32)
                        ^ (b << 23 as i32 | b >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[11 as i32 as usize])
                                    .wrapping_add(x[11 as i32 as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_10);
        e = t0_10.wrapping_add(t1_10);
        let mut t0_11: u64_0 = ((e << 36 as i32 | e >> 64 as i32 - 36 as i32)
            ^ ((e << 30 as i32 | e >> 64 as i32 - 30 as i32)
                ^ (e << 25 as i32 | e >> 64 as i32 - 25 as i32)))
            .wrapping_add(e & f | g & (e | f));
        let mut t1_11: u64_0 = d
            .wrapping_add(
                ((a << 50 as i32 | a >> 64 as i32 - 50 as i32)
                    ^ ((a << 46 as i32 | a >> 64 as i32 - 46 as i32)
                        ^ (a << 23 as i32 | a >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[12 as i32 as usize])
                                    .wrapping_add(x[12 as i32 as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_11);
        d = t0_11.wrapping_add(t1_11);
        let mut t0_12: u64_0 = ((d << 36 as i32 | d >> 64 as i32 - 36 as i32)
            ^ ((d << 30 as i32 | d >> 64 as i32 - 30 as i32)
                ^ (d << 25 as i32 | d >> 64 as i32 - 25 as i32)))
            .wrapping_add(d & e | f & (d | e));
        let mut t1_12: u64_0 = c
            .wrapping_add(
                ((h << 50 as i32 | h >> 64 as i32 - 50 as i32)
                    ^ ((h << 46 as i32 | h >> 64 as i32 - 46 as i32)
                        ^ (h << 23 as i32 | h >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[13 as i32 as usize])
                                    .wrapping_add(x[13 as i32 as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_12);
        c = t0_12.wrapping_add(t1_12);
        let mut t0_13: u64_0 = ((c << 36 as i32 | c >> 64 as i32 - 36 as i32)
            ^ ((c << 30 as i32 | c >> 64 as i32 - 30 as i32)
                ^ (c << 25 as i32 | c >> 64 as i32 - 25 as i32)))
            .wrapping_add(c & d | e & (c | d));
        let mut t1_13: u64_0 = b
            .wrapping_add(
                ((g << 50 as i32 | g >> 64 as i32 - 50 as i32)
                    ^ ((g << 46 as i32 | g >> 64 as i32 - 46 as i32)
                        ^ (g << 23 as i32 | g >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[14 as i32 as usize])
                                    .wrapping_add(x[14 as i32 as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_13);
        b = t0_13.wrapping_add(t1_13);
        let mut t0_14: u64_0 = ((b << 36 as i32 | b >> 64 as i32 - 36 as i32)
            ^ ((b << 30 as i32 | b >> 64 as i32 - 30 as i32)
                ^ (b << 25 as i32 | b >> 64 as i32 - 25 as i32)))
            .wrapping_add(b & c | d & (b | c));
        let mut t1_14: u64_0 = a
            .wrapping_add(
                ((f << 50 as i32 | f >> 64 as i32 - 50 as i32)
                    ^ ((f << 46 as i32 | f >> 64 as i32 - 46 as i32)
                        ^ (f << 23 as i32 | f >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[15 as i32 as usize])
                                    .wrapping_add(x[15 as i32 as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_14);
        a = t0_14.wrapping_add(t1_14);
        let mut t0_15: u64_0 = ((a << 36 as i32 | a >> 64 as i32 - 36 as i32)
            ^ ((a << 30 as i32 | a >> 64 as i32 - 30 as i32)
                ^ (a << 25 as i32 | a >> 64 as i32 - 25 as i32)))
            .wrapping_add(a & b | c & (a | b));
        x[(16 as i32 & 15 as i32) as usize] = (x[(16 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(16 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(16 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(16 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(16 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(16 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(16 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(16 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(16 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(16 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(16 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(16 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_15: u64_0 = h
            .wrapping_add(
                ((e << 50 as i32 | e >> 64 as i32 - 50 as i32)
                    ^ ((e << 46 as i32 | e >> 64 as i32 - 46 as i32)
                        ^ (e << 23 as i32 | e >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[16 as i32 as usize])
                                    .wrapping_add(x[(16 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_15);
        h = t0_15.wrapping_add(t1_15);
        let mut t0_16: u64_0 = ((h << 36 as i32 | h >> 64 as i32 - 36 as i32)
            ^ ((h << 30 as i32 | h >> 64 as i32 - 30 as i32)
                ^ (h << 25 as i32 | h >> 64 as i32 - 25 as i32)))
            .wrapping_add(h & a | b & (h | a));
        x[(17 as i32 & 15 as i32) as usize] = (x[(17 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(17 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(17 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(17 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(17 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(17 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(17 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(17 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(17 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(17 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(17 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(17 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_16: u64_0 = g
            .wrapping_add(
                ((d << 50 as i32 | d >> 64 as i32 - 50 as i32)
                    ^ ((d << 46 as i32 | d >> 64 as i32 - 46 as i32)
                        ^ (d << 23 as i32 | d >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[17 as i32 as usize])
                                    .wrapping_add(x[(17 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_16);
        g = t0_16.wrapping_add(t1_16);
        let mut t0_17: u64_0 = ((g << 36 as i32 | g >> 64 as i32 - 36 as i32)
            ^ ((g << 30 as i32 | g >> 64 as i32 - 30 as i32)
                ^ (g << 25 as i32 | g >> 64 as i32 - 25 as i32)))
            .wrapping_add(g & h | a & (g | h));
        x[(18 as i32 & 15 as i32) as usize] = (x[(18 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(18 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(18 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(18 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(18 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(18 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(18 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(18 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(18 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(18 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(18 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(18 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_17: u64_0 = f
            .wrapping_add(
                ((c << 50 as i32 | c >> 64 as i32 - 50 as i32)
                    ^ ((c << 46 as i32 | c >> 64 as i32 - 46 as i32)
                        ^ (c << 23 as i32 | c >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[18 as i32 as usize])
                                    .wrapping_add(x[(18 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_17);
        f = t0_17.wrapping_add(t1_17);
        let mut t0_18: u64_0 = ((f << 36 as i32 | f >> 64 as i32 - 36 as i32)
            ^ ((f << 30 as i32 | f >> 64 as i32 - 30 as i32)
                ^ (f << 25 as i32 | f >> 64 as i32 - 25 as i32)))
            .wrapping_add(f & g | h & (f | g));
        x[(19 as i32 & 15 as i32) as usize] = (x[(19 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(19 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(19 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(19 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(19 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(19 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(19 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(19 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(19 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(19 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(19 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(19 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_18: u64_0 = e
            .wrapping_add(
                ((b << 50 as i32 | b >> 64 as i32 - 50 as i32)
                    ^ ((b << 46 as i32 | b >> 64 as i32 - 46 as i32)
                        ^ (b << 23 as i32 | b >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[19 as i32 as usize])
                                    .wrapping_add(x[(19 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_18);
        e = t0_18.wrapping_add(t1_18);
        let mut t0_19: u64_0 = ((e << 36 as i32 | e >> 64 as i32 - 36 as i32)
            ^ ((e << 30 as i32 | e >> 64 as i32 - 30 as i32)
                ^ (e << 25 as i32 | e >> 64 as i32 - 25 as i32)))
            .wrapping_add(e & f | g & (e | f));
        x[(20 as i32 & 15 as i32) as usize] = (x[(20 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(20 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(20 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(20 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(20 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(20 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(20 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(20 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(20 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(20 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(20 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(20 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_19: u64_0 = d
            .wrapping_add(
                ((a << 50 as i32 | a >> 64 as i32 - 50 as i32)
                    ^ ((a << 46 as i32 | a >> 64 as i32 - 46 as i32)
                        ^ (a << 23 as i32 | a >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[20 as i32 as usize])
                                    .wrapping_add(x[(20 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_19);
        d = t0_19.wrapping_add(t1_19);
        let mut t0_20: u64_0 = ((d << 36 as i32 | d >> 64 as i32 - 36 as i32)
            ^ ((d << 30 as i32 | d >> 64 as i32 - 30 as i32)
                ^ (d << 25 as i32 | d >> 64 as i32 - 25 as i32)))
            .wrapping_add(d & e | f & (d | e));
        x[(21 as i32 & 15 as i32) as usize] = (x[(21 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(21 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(21 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(21 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(21 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(21 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(21 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(21 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(21 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(21 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(21 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(21 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_20: u64_0 = c
            .wrapping_add(
                ((h << 50 as i32 | h >> 64 as i32 - 50 as i32)
                    ^ ((h << 46 as i32 | h >> 64 as i32 - 46 as i32)
                        ^ (h << 23 as i32 | h >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[21 as i32 as usize])
                                    .wrapping_add(x[(21 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_20);
        c = t0_20.wrapping_add(t1_20);
        let mut t0_21: u64_0 = ((c << 36 as i32 | c >> 64 as i32 - 36 as i32)
            ^ ((c << 30 as i32 | c >> 64 as i32 - 30 as i32)
                ^ (c << 25 as i32 | c >> 64 as i32 - 25 as i32)))
            .wrapping_add(c & d | e & (c | d));
        x[(22 as i32 & 15 as i32) as usize] = (x[(22 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(22 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(22 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(22 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(22 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(22 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(22 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(22 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(22 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(22 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(22 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(22 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_21: u64_0 = b
            .wrapping_add(
                ((g << 50 as i32 | g >> 64 as i32 - 50 as i32)
                    ^ ((g << 46 as i32 | g >> 64 as i32 - 46 as i32)
                        ^ (g << 23 as i32 | g >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[22 as i32 as usize])
                                    .wrapping_add(x[(22 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_21);
        b = t0_21.wrapping_add(t1_21);
        let mut t0_22: u64_0 = ((b << 36 as i32 | b >> 64 as i32 - 36 as i32)
            ^ ((b << 30 as i32 | b >> 64 as i32 - 30 as i32)
                ^ (b << 25 as i32 | b >> 64 as i32 - 25 as i32)))
            .wrapping_add(b & c | d & (b | c));
        x[(23 as i32 & 15 as i32) as usize] = (x[(23 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(23 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(23 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(23 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(23 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(23 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(23 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(23 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(23 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(23 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(23 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(23 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_22: u64_0 = a
            .wrapping_add(
                ((f << 50 as i32 | f >> 64 as i32 - 50 as i32)
                    ^ ((f << 46 as i32 | f >> 64 as i32 - 46 as i32)
                        ^ (f << 23 as i32 | f >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[23 as i32 as usize])
                                    .wrapping_add(x[(23 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_22);
        a = t0_22.wrapping_add(t1_22);
        let mut t0_23: u64_0 = ((a << 36 as i32 | a >> 64 as i32 - 36 as i32)
            ^ ((a << 30 as i32 | a >> 64 as i32 - 30 as i32)
                ^ (a << 25 as i32 | a >> 64 as i32 - 25 as i32)))
            .wrapping_add(a & b | c & (a | b));
        x[(24 as i32 & 15 as i32) as usize] = (x[(24 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(24 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(24 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(24 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(24 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(24 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(24 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(24 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(24 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(24 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(24 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(24 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_23: u64_0 = h
            .wrapping_add(
                ((e << 50 as i32 | e >> 64 as i32 - 50 as i32)
                    ^ ((e << 46 as i32 | e >> 64 as i32 - 46 as i32)
                        ^ (e << 23 as i32 | e >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[24 as i32 as usize])
                                    .wrapping_add(x[(24 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_23);
        h = t0_23.wrapping_add(t1_23);
        let mut t0_24: u64_0 = ((h << 36 as i32 | h >> 64 as i32 - 36 as i32)
            ^ ((h << 30 as i32 | h >> 64 as i32 - 30 as i32)
                ^ (h << 25 as i32 | h >> 64 as i32 - 25 as i32)))
            .wrapping_add(h & a | b & (h | a));
        x[(25 as i32 & 15 as i32) as usize] = (x[(25 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(25 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(25 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(25 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(25 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(25 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(25 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(25 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(25 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(25 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(25 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(25 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_24: u64_0 = g
            .wrapping_add(
                ((d << 50 as i32 | d >> 64 as i32 - 50 as i32)
                    ^ ((d << 46 as i32 | d >> 64 as i32 - 46 as i32)
                        ^ (d << 23 as i32 | d >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[25 as i32 as usize])
                                    .wrapping_add(x[(25 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_24);
        g = t0_24.wrapping_add(t1_24);
        let mut t0_25: u64_0 = ((g << 36 as i32 | g >> 64 as i32 - 36 as i32)
            ^ ((g << 30 as i32 | g >> 64 as i32 - 30 as i32)
                ^ (g << 25 as i32 | g >> 64 as i32 - 25 as i32)))
            .wrapping_add(g & h | a & (g | h));
        x[(26 as i32 & 15 as i32) as usize] = (x[(26 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(26 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(26 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(26 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(26 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(26 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(26 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(26 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(26 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(26 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(26 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(26 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_25: u64_0 = f
            .wrapping_add(
                ((c << 50 as i32 | c >> 64 as i32 - 50 as i32)
                    ^ ((c << 46 as i32 | c >> 64 as i32 - 46 as i32)
                        ^ (c << 23 as i32 | c >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[26 as i32 as usize])
                                    .wrapping_add(x[(26 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_25);
        f = t0_25.wrapping_add(t1_25);
        let mut t0_26: u64_0 = ((f << 36 as i32 | f >> 64 as i32 - 36 as i32)
            ^ ((f << 30 as i32 | f >> 64 as i32 - 30 as i32)
                ^ (f << 25 as i32 | f >> 64 as i32 - 25 as i32)))
            .wrapping_add(f & g | h & (f | g));
        x[(27 as i32 & 15 as i32) as usize] = (x[(27 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(27 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(27 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(27 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(27 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(27 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(27 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(27 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(27 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(27 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(27 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(27 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_26: u64_0 = e
            .wrapping_add(
                ((b << 50 as i32 | b >> 64 as i32 - 50 as i32)
                    ^ ((b << 46 as i32 | b >> 64 as i32 - 46 as i32)
                        ^ (b << 23 as i32 | b >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[27 as i32 as usize])
                                    .wrapping_add(x[(27 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_26);
        e = t0_26.wrapping_add(t1_26);
        let mut t0_27: u64_0 = ((e << 36 as i32 | e >> 64 as i32 - 36 as i32)
            ^ ((e << 30 as i32 | e >> 64 as i32 - 30 as i32)
                ^ (e << 25 as i32 | e >> 64 as i32 - 25 as i32)))
            .wrapping_add(e & f | g & (e | f));
        x[(28 as i32 & 15 as i32) as usize] = (x[(28 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(28 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(28 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(28 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(28 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(28 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(28 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(28 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(28 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(28 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(28 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(28 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_27: u64_0 = d
            .wrapping_add(
                ((a << 50 as i32 | a >> 64 as i32 - 50 as i32)
                    ^ ((a << 46 as i32 | a >> 64 as i32 - 46 as i32)
                        ^ (a << 23 as i32 | a >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[28 as i32 as usize])
                                    .wrapping_add(x[(28 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_27);
        d = t0_27.wrapping_add(t1_27);
        let mut t0_28: u64_0 = ((d << 36 as i32 | d >> 64 as i32 - 36 as i32)
            ^ ((d << 30 as i32 | d >> 64 as i32 - 30 as i32)
                ^ (d << 25 as i32 | d >> 64 as i32 - 25 as i32)))
            .wrapping_add(d & e | f & (d | e));
        x[(29 as i32 & 15 as i32) as usize] = (x[(29 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(29 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(29 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(29 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(29 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(29 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(29 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(29 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(29 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(29 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(29 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(29 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_28: u64_0 = c
            .wrapping_add(
                ((h << 50 as i32 | h >> 64 as i32 - 50 as i32)
                    ^ ((h << 46 as i32 | h >> 64 as i32 - 46 as i32)
                        ^ (h << 23 as i32 | h >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[29 as i32 as usize])
                                    .wrapping_add(x[(29 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_28);
        c = t0_28.wrapping_add(t1_28);
        let mut t0_29: u64_0 = ((c << 36 as i32 | c >> 64 as i32 - 36 as i32)
            ^ ((c << 30 as i32 | c >> 64 as i32 - 30 as i32)
                ^ (c << 25 as i32 | c >> 64 as i32 - 25 as i32)))
            .wrapping_add(c & d | e & (c | d));
        x[(30 as i32 & 15 as i32) as usize] = (x[(30 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(30 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(30 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(30 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(30 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(30 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(30 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(30 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(30 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(30 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(30 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(30 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_29: u64_0 = b
            .wrapping_add(
                ((g << 50 as i32 | g >> 64 as i32 - 50 as i32)
                    ^ ((g << 46 as i32 | g >> 64 as i32 - 46 as i32)
                        ^ (g << 23 as i32 | g >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[30 as i32 as usize])
                                    .wrapping_add(x[(30 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_29);
        b = t0_29.wrapping_add(t1_29);
        let mut t0_30: u64_0 = ((b << 36 as i32 | b >> 64 as i32 - 36 as i32)
            ^ ((b << 30 as i32 | b >> 64 as i32 - 30 as i32)
                ^ (b << 25 as i32 | b >> 64 as i32 - 25 as i32)))
            .wrapping_add(b & c | d & (b | c));
        x[(31 as i32 & 15 as i32) as usize] = (x[(31 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(31 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(31 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(31 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(31 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(31 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(31 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(31 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(31 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(31 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(31 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(31 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_30: u64_0 = a
            .wrapping_add(
                ((f << 50 as i32 | f >> 64 as i32 - 50 as i32)
                    ^ ((f << 46 as i32 | f >> 64 as i32 - 46 as i32)
                        ^ (f << 23 as i32 | f >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[31 as i32 as usize])
                                    .wrapping_add(x[(31 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_30);
        a = t0_30.wrapping_add(t1_30);
        let mut t0_31: u64_0 = ((a << 36 as i32 | a >> 64 as i32 - 36 as i32)
            ^ ((a << 30 as i32 | a >> 64 as i32 - 30 as i32)
                ^ (a << 25 as i32 | a >> 64 as i32 - 25 as i32)))
            .wrapping_add(a & b | c & (a | b));
        x[(32 as i32 & 15 as i32) as usize] = (x[(32 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(32 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(32 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(32 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(32 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(32 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(32 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(32 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(32 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(32 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(32 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(32 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_31: u64_0 = h
            .wrapping_add(
                ((e << 50 as i32 | e >> 64 as i32 - 50 as i32)
                    ^ ((e << 46 as i32 | e >> 64 as i32 - 46 as i32)
                        ^ (e << 23 as i32 | e >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[32 as i32 as usize])
                                    .wrapping_add(x[(32 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_31);
        h = t0_31.wrapping_add(t1_31);
        let mut t0_32: u64_0 = ((h << 36 as i32 | h >> 64 as i32 - 36 as i32)
            ^ ((h << 30 as i32 | h >> 64 as i32 - 30 as i32)
                ^ (h << 25 as i32 | h >> 64 as i32 - 25 as i32)))
            .wrapping_add(h & a | b & (h | a));
        x[(33 as i32 & 15 as i32) as usize] = (x[(33 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(33 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(33 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(33 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(33 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(33 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(33 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(33 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(33 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(33 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(33 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(33 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_32: u64_0 = g
            .wrapping_add(
                ((d << 50 as i32 | d >> 64 as i32 - 50 as i32)
                    ^ ((d << 46 as i32 | d >> 64 as i32 - 46 as i32)
                        ^ (d << 23 as i32 | d >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[33 as i32 as usize])
                                    .wrapping_add(x[(33 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_32);
        g = t0_32.wrapping_add(t1_32);
        let mut t0_33: u64_0 = ((g << 36 as i32 | g >> 64 as i32 - 36 as i32)
            ^ ((g << 30 as i32 | g >> 64 as i32 - 30 as i32)
                ^ (g << 25 as i32 | g >> 64 as i32 - 25 as i32)))
            .wrapping_add(g & h | a & (g | h));
        x[(34 as i32 & 15 as i32) as usize] = (x[(34 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(34 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(34 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(34 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(34 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(34 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(34 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(34 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(34 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(34 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(34 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(34 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_33: u64_0 = f
            .wrapping_add(
                ((c << 50 as i32 | c >> 64 as i32 - 50 as i32)
                    ^ ((c << 46 as i32 | c >> 64 as i32 - 46 as i32)
                        ^ (c << 23 as i32 | c >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[34 as i32 as usize])
                                    .wrapping_add(x[(34 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_33);
        f = t0_33.wrapping_add(t1_33);
        let mut t0_34: u64_0 = ((f << 36 as i32 | f >> 64 as i32 - 36 as i32)
            ^ ((f << 30 as i32 | f >> 64 as i32 - 30 as i32)
                ^ (f << 25 as i32 | f >> 64 as i32 - 25 as i32)))
            .wrapping_add(f & g | h & (f | g));
        x[(35 as i32 & 15 as i32) as usize] = (x[(35 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(35 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(35 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(35 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(35 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(35 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(35 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(35 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(35 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(35 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(35 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(35 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_34: u64_0 = e
            .wrapping_add(
                ((b << 50 as i32 | b >> 64 as i32 - 50 as i32)
                    ^ ((b << 46 as i32 | b >> 64 as i32 - 46 as i32)
                        ^ (b << 23 as i32 | b >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[35 as i32 as usize])
                                    .wrapping_add(x[(35 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_34);
        e = t0_34.wrapping_add(t1_34);
        let mut t0_35: u64_0 = ((e << 36 as i32 | e >> 64 as i32 - 36 as i32)
            ^ ((e << 30 as i32 | e >> 64 as i32 - 30 as i32)
                ^ (e << 25 as i32 | e >> 64 as i32 - 25 as i32)))
            .wrapping_add(e & f | g & (e | f));
        x[(36 as i32 & 15 as i32) as usize] = (x[(36 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(36 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(36 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(36 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(36 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(36 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(36 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(36 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(36 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(36 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(36 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(36 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_35: u64_0 = d
            .wrapping_add(
                ((a << 50 as i32 | a >> 64 as i32 - 50 as i32)
                    ^ ((a << 46 as i32 | a >> 64 as i32 - 46 as i32)
                        ^ (a << 23 as i32 | a >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[36 as i32 as usize])
                                    .wrapping_add(x[(36 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_35);
        d = t0_35.wrapping_add(t1_35);
        let mut t0_36: u64_0 = ((d << 36 as i32 | d >> 64 as i32 - 36 as i32)
            ^ ((d << 30 as i32 | d >> 64 as i32 - 30 as i32)
                ^ (d << 25 as i32 | d >> 64 as i32 - 25 as i32)))
            .wrapping_add(d & e | f & (d | e));
        x[(37 as i32 & 15 as i32) as usize] = (x[(37 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(37 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(37 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(37 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(37 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(37 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(37 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(37 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(37 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(37 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(37 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(37 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_36: u64_0 = c
            .wrapping_add(
                ((h << 50 as i32 | h >> 64 as i32 - 50 as i32)
                    ^ ((h << 46 as i32 | h >> 64 as i32 - 46 as i32)
                        ^ (h << 23 as i32 | h >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[37 as i32 as usize])
                                    .wrapping_add(x[(37 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_36);
        c = t0_36.wrapping_add(t1_36);
        let mut t0_37: u64_0 = ((c << 36 as i32 | c >> 64 as i32 - 36 as i32)
            ^ ((c << 30 as i32 | c >> 64 as i32 - 30 as i32)
                ^ (c << 25 as i32 | c >> 64 as i32 - 25 as i32)))
            .wrapping_add(c & d | e & (c | d));
        x[(38 as i32 & 15 as i32) as usize] = (x[(38 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(38 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(38 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(38 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(38 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(38 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(38 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(38 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(38 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(38 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(38 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(38 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_37: u64_0 = b
            .wrapping_add(
                ((g << 50 as i32 | g >> 64 as i32 - 50 as i32)
                    ^ ((g << 46 as i32 | g >> 64 as i32 - 46 as i32)
                        ^ (g << 23 as i32 | g >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[38 as i32 as usize])
                                    .wrapping_add(x[(38 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_37);
        b = t0_37.wrapping_add(t1_37);
        let mut t0_38: u64_0 = ((b << 36 as i32 | b >> 64 as i32 - 36 as i32)
            ^ ((b << 30 as i32 | b >> 64 as i32 - 30 as i32)
                ^ (b << 25 as i32 | b >> 64 as i32 - 25 as i32)))
            .wrapping_add(b & c | d & (b | c));
        x[(39 as i32 & 15 as i32) as usize] = (x[(39 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(39 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(39 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(39 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(39 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(39 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(39 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(39 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(39 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(39 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(39 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(39 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_38: u64_0 = a
            .wrapping_add(
                ((f << 50 as i32 | f >> 64 as i32 - 50 as i32)
                    ^ ((f << 46 as i32 | f >> 64 as i32 - 46 as i32)
                        ^ (f << 23 as i32 | f >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[39 as i32 as usize])
                                    .wrapping_add(x[(39 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_38);
        a = t0_38.wrapping_add(t1_38);
        let mut t0_39: u64_0 = ((a << 36 as i32 | a >> 64 as i32 - 36 as i32)
            ^ ((a << 30 as i32 | a >> 64 as i32 - 30 as i32)
                ^ (a << 25 as i32 | a >> 64 as i32 - 25 as i32)))
            .wrapping_add(a & b | c & (a | b));
        x[(40 as i32 & 15 as i32) as usize] = (x[(40 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(40 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(40 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(40 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(40 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(40 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(40 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(40 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(40 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(40 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(40 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(40 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_39: u64_0 = h
            .wrapping_add(
                ((e << 50 as i32 | e >> 64 as i32 - 50 as i32)
                    ^ ((e << 46 as i32 | e >> 64 as i32 - 46 as i32)
                        ^ (e << 23 as i32 | e >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[40 as i32 as usize])
                                    .wrapping_add(x[(40 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_39);
        h = t0_39.wrapping_add(t1_39);
        let mut t0_40: u64_0 = ((h << 36 as i32 | h >> 64 as i32 - 36 as i32)
            ^ ((h << 30 as i32 | h >> 64 as i32 - 30 as i32)
                ^ (h << 25 as i32 | h >> 64 as i32 - 25 as i32)))
            .wrapping_add(h & a | b & (h | a));
        x[(41 as i32 & 15 as i32) as usize] = (x[(41 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(41 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(41 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(41 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(41 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(41 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(41 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(41 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(41 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(41 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(41 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(41 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_40: u64_0 = g
            .wrapping_add(
                ((d << 50 as i32 | d >> 64 as i32 - 50 as i32)
                    ^ ((d << 46 as i32 | d >> 64 as i32 - 46 as i32)
                        ^ (d << 23 as i32 | d >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[41 as i32 as usize])
                                    .wrapping_add(x[(41 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_40);
        g = t0_40.wrapping_add(t1_40);
        let mut t0_41: u64_0 = ((g << 36 as i32 | g >> 64 as i32 - 36 as i32)
            ^ ((g << 30 as i32 | g >> 64 as i32 - 30 as i32)
                ^ (g << 25 as i32 | g >> 64 as i32 - 25 as i32)))
            .wrapping_add(g & h | a & (g | h));
        x[(42 as i32 & 15 as i32) as usize] = (x[(42 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(42 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(42 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(42 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(42 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(42 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(42 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(42 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(42 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(42 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(42 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(42 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_41: u64_0 = f
            .wrapping_add(
                ((c << 50 as i32 | c >> 64 as i32 - 50 as i32)
                    ^ ((c << 46 as i32 | c >> 64 as i32 - 46 as i32)
                        ^ (c << 23 as i32 | c >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[42 as i32 as usize])
                                    .wrapping_add(x[(42 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_41);
        f = t0_41.wrapping_add(t1_41);
        let mut t0_42: u64_0 = ((f << 36 as i32 | f >> 64 as i32 - 36 as i32)
            ^ ((f << 30 as i32 | f >> 64 as i32 - 30 as i32)
                ^ (f << 25 as i32 | f >> 64 as i32 - 25 as i32)))
            .wrapping_add(f & g | h & (f | g));
        x[(43 as i32 & 15 as i32) as usize] = (x[(43 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(43 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(43 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(43 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(43 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(43 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(43 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(43 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(43 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(43 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(43 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(43 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_42: u64_0 = e
            .wrapping_add(
                ((b << 50 as i32 | b >> 64 as i32 - 50 as i32)
                    ^ ((b << 46 as i32 | b >> 64 as i32 - 46 as i32)
                        ^ (b << 23 as i32 | b >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[43 as i32 as usize])
                                    .wrapping_add(x[(43 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_42);
        e = t0_42.wrapping_add(t1_42);
        let mut t0_43: u64_0 = ((e << 36 as i32 | e >> 64 as i32 - 36 as i32)
            ^ ((e << 30 as i32 | e >> 64 as i32 - 30 as i32)
                ^ (e << 25 as i32 | e >> 64 as i32 - 25 as i32)))
            .wrapping_add(e & f | g & (e | f));
        x[(44 as i32 & 15 as i32) as usize] = (x[(44 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(44 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(44 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(44 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(44 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(44 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(44 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(44 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(44 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(44 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(44 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(44 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_43: u64_0 = d
            .wrapping_add(
                ((a << 50 as i32 | a >> 64 as i32 - 50 as i32)
                    ^ ((a << 46 as i32 | a >> 64 as i32 - 46 as i32)
                        ^ (a << 23 as i32 | a >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[44 as i32 as usize])
                                    .wrapping_add(x[(44 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_43);
        d = t0_43.wrapping_add(t1_43);
        let mut t0_44: u64_0 = ((d << 36 as i32 | d >> 64 as i32 - 36 as i32)
            ^ ((d << 30 as i32 | d >> 64 as i32 - 30 as i32)
                ^ (d << 25 as i32 | d >> 64 as i32 - 25 as i32)))
            .wrapping_add(d & e | f & (d | e));
        x[(45 as i32 & 15 as i32) as usize] = (x[(45 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(45 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(45 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(45 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(45 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(45 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(45 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(45 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(45 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(45 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(45 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(45 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_44: u64_0 = c
            .wrapping_add(
                ((h << 50 as i32 | h >> 64 as i32 - 50 as i32)
                    ^ ((h << 46 as i32 | h >> 64 as i32 - 46 as i32)
                        ^ (h << 23 as i32 | h >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[45 as i32 as usize])
                                    .wrapping_add(x[(45 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_44);
        c = t0_44.wrapping_add(t1_44);
        let mut t0_45: u64_0 = ((c << 36 as i32 | c >> 64 as i32 - 36 as i32)
            ^ ((c << 30 as i32 | c >> 64 as i32 - 30 as i32)
                ^ (c << 25 as i32 | c >> 64 as i32 - 25 as i32)))
            .wrapping_add(c & d | e & (c | d));
        x[(46 as i32 & 15 as i32) as usize] = (x[(46 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(46 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(46 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(46 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(46 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(46 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(46 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(46 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(46 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(46 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(46 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(46 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_45: u64_0 = b
            .wrapping_add(
                ((g << 50 as i32 | g >> 64 as i32 - 50 as i32)
                    ^ ((g << 46 as i32 | g >> 64 as i32 - 46 as i32)
                        ^ (g << 23 as i32 | g >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[46 as i32 as usize])
                                    .wrapping_add(x[(46 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_45);
        b = t0_45.wrapping_add(t1_45);
        let mut t0_46: u64_0 = ((b << 36 as i32 | b >> 64 as i32 - 36 as i32)
            ^ ((b << 30 as i32 | b >> 64 as i32 - 30 as i32)
                ^ (b << 25 as i32 | b >> 64 as i32 - 25 as i32)))
            .wrapping_add(b & c | d & (b | c));
        x[(47 as i32 & 15 as i32) as usize] = (x[(47 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(47 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(47 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(47 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(47 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(47 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(47 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(47 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(47 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(47 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(47 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(47 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_46: u64_0 = a
            .wrapping_add(
                ((f << 50 as i32 | f >> 64 as i32 - 50 as i32)
                    ^ ((f << 46 as i32 | f >> 64 as i32 - 46 as i32)
                        ^ (f << 23 as i32 | f >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[47 as i32 as usize])
                                    .wrapping_add(x[(47 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_46);
        a = t0_46.wrapping_add(t1_46);
        let mut t0_47: u64_0 = ((a << 36 as i32 | a >> 64 as i32 - 36 as i32)
            ^ ((a << 30 as i32 | a >> 64 as i32 - 30 as i32)
                ^ (a << 25 as i32 | a >> 64 as i32 - 25 as i32)))
            .wrapping_add(a & b | c & (a | b));
        x[(48 as i32 & 15 as i32) as usize] = (x[(48 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(48 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(48 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(48 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(48 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(48 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(48 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(48 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(48 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(48 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(48 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(48 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_47: u64_0 = h
            .wrapping_add(
                ((e << 50 as i32 | e >> 64 as i32 - 50 as i32)
                    ^ ((e << 46 as i32 | e >> 64 as i32 - 46 as i32)
                        ^ (e << 23 as i32 | e >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[48 as i32 as usize])
                                    .wrapping_add(x[(48 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_47);
        h = t0_47.wrapping_add(t1_47);
        let mut t0_48: u64_0 = ((h << 36 as i32 | h >> 64 as i32 - 36 as i32)
            ^ ((h << 30 as i32 | h >> 64 as i32 - 30 as i32)
                ^ (h << 25 as i32 | h >> 64 as i32 - 25 as i32)))
            .wrapping_add(h & a | b & (h | a));
        x[(49 as i32 & 15 as i32) as usize] = (x[(49 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(49 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(49 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(49 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(49 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(49 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(49 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(49 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(49 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(49 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(49 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(49 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_48: u64_0 = g
            .wrapping_add(
                ((d << 50 as i32 | d >> 64 as i32 - 50 as i32)
                    ^ ((d << 46 as i32 | d >> 64 as i32 - 46 as i32)
                        ^ (d << 23 as i32 | d >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[49 as i32 as usize])
                                    .wrapping_add(x[(49 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_48);
        g = t0_48.wrapping_add(t1_48);
        let mut t0_49: u64_0 = ((g << 36 as i32 | g >> 64 as i32 - 36 as i32)
            ^ ((g << 30 as i32 | g >> 64 as i32 - 30 as i32)
                ^ (g << 25 as i32 | g >> 64 as i32 - 25 as i32)))
            .wrapping_add(g & h | a & (g | h));
        x[(50 as i32 & 15 as i32) as usize] = (x[(50 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(50 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(50 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(50 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(50 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(50 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(50 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(50 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(50 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(50 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(50 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(50 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_49: u64_0 = f
            .wrapping_add(
                ((c << 50 as i32 | c >> 64 as i32 - 50 as i32)
                    ^ ((c << 46 as i32 | c >> 64 as i32 - 46 as i32)
                        ^ (c << 23 as i32 | c >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[50 as i32 as usize])
                                    .wrapping_add(x[(50 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_49);
        f = t0_49.wrapping_add(t1_49);
        let mut t0_50: u64_0 = ((f << 36 as i32 | f >> 64 as i32 - 36 as i32)
            ^ ((f << 30 as i32 | f >> 64 as i32 - 30 as i32)
                ^ (f << 25 as i32 | f >> 64 as i32 - 25 as i32)))
            .wrapping_add(f & g | h & (f | g));
        x[(51 as i32 & 15 as i32) as usize] = (x[(51 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(51 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(51 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(51 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(51 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(51 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(51 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(51 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(51 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(51 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(51 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(51 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_50: u64_0 = e
            .wrapping_add(
                ((b << 50 as i32 | b >> 64 as i32 - 50 as i32)
                    ^ ((b << 46 as i32 | b >> 64 as i32 - 46 as i32)
                        ^ (b << 23 as i32 | b >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[51 as i32 as usize])
                                    .wrapping_add(x[(51 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_50);
        e = t0_50.wrapping_add(t1_50);
        let mut t0_51: u64_0 = ((e << 36 as i32 | e >> 64 as i32 - 36 as i32)
            ^ ((e << 30 as i32 | e >> 64 as i32 - 30 as i32)
                ^ (e << 25 as i32 | e >> 64 as i32 - 25 as i32)))
            .wrapping_add(e & f | g & (e | f));
        x[(52 as i32 & 15 as i32) as usize] = (x[(52 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(52 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(52 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(52 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(52 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(52 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(52 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(52 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(52 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(52 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(52 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(52 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_51: u64_0 = d
            .wrapping_add(
                ((a << 50 as i32 | a >> 64 as i32 - 50 as i32)
                    ^ ((a << 46 as i32 | a >> 64 as i32 - 46 as i32)
                        ^ (a << 23 as i32 | a >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[52 as i32 as usize])
                                    .wrapping_add(x[(52 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_51);
        d = t0_51.wrapping_add(t1_51);
        let mut t0_52: u64_0 = ((d << 36 as i32 | d >> 64 as i32 - 36 as i32)
            ^ ((d << 30 as i32 | d >> 64 as i32 - 30 as i32)
                ^ (d << 25 as i32 | d >> 64 as i32 - 25 as i32)))
            .wrapping_add(d & e | f & (d | e));
        x[(53 as i32 & 15 as i32) as usize] = (x[(53 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(53 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(53 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(53 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(53 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(53 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(53 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(53 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(53 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(53 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(53 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(53 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_52: u64_0 = c
            .wrapping_add(
                ((h << 50 as i32 | h >> 64 as i32 - 50 as i32)
                    ^ ((h << 46 as i32 | h >> 64 as i32 - 46 as i32)
                        ^ (h << 23 as i32 | h >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[53 as i32 as usize])
                                    .wrapping_add(x[(53 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_52);
        c = t0_52.wrapping_add(t1_52);
        let mut t0_53: u64_0 = ((c << 36 as i32 | c >> 64 as i32 - 36 as i32)
            ^ ((c << 30 as i32 | c >> 64 as i32 - 30 as i32)
                ^ (c << 25 as i32 | c >> 64 as i32 - 25 as i32)))
            .wrapping_add(c & d | e & (c | d));
        x[(54 as i32 & 15 as i32) as usize] = (x[(54 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(54 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(54 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(54 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(54 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(54 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(54 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(54 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(54 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(54 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(54 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(54 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_53: u64_0 = b
            .wrapping_add(
                ((g << 50 as i32 | g >> 64 as i32 - 50 as i32)
                    ^ ((g << 46 as i32 | g >> 64 as i32 - 46 as i32)
                        ^ (g << 23 as i32 | g >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[54 as i32 as usize])
                                    .wrapping_add(x[(54 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_53);
        b = t0_53.wrapping_add(t1_53);
        let mut t0_54: u64_0 = ((b << 36 as i32 | b >> 64 as i32 - 36 as i32)
            ^ ((b << 30 as i32 | b >> 64 as i32 - 30 as i32)
                ^ (b << 25 as i32 | b >> 64 as i32 - 25 as i32)))
            .wrapping_add(b & c | d & (b | c));
        x[(55 as i32 & 15 as i32) as usize] = (x[(55 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(55 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(55 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(55 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(55 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(55 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(55 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(55 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(55 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(55 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(55 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(55 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_54: u64_0 = a
            .wrapping_add(
                ((f << 50 as i32 | f >> 64 as i32 - 50 as i32)
                    ^ ((f << 46 as i32 | f >> 64 as i32 - 46 as i32)
                        ^ (f << 23 as i32 | f >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[55 as i32 as usize])
                                    .wrapping_add(x[(55 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_54);
        a = t0_54.wrapping_add(t1_54);
        let mut t0_55: u64_0 = ((a << 36 as i32 | a >> 64 as i32 - 36 as i32)
            ^ ((a << 30 as i32 | a >> 64 as i32 - 30 as i32)
                ^ (a << 25 as i32 | a >> 64 as i32 - 25 as i32)))
            .wrapping_add(a & b | c & (a | b));
        x[(56 as i32 & 15 as i32) as usize] = (x[(56 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(56 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(56 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(56 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(56 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(56 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(56 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(56 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(56 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(56 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(56 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(56 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_55: u64_0 = h
            .wrapping_add(
                ((e << 50 as i32 | e >> 64 as i32 - 50 as i32)
                    ^ ((e << 46 as i32 | e >> 64 as i32 - 46 as i32)
                        ^ (e << 23 as i32 | e >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[56 as i32 as usize])
                                    .wrapping_add(x[(56 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_55);
        h = t0_55.wrapping_add(t1_55);
        let mut t0_56: u64_0 = ((h << 36 as i32 | h >> 64 as i32 - 36 as i32)
            ^ ((h << 30 as i32 | h >> 64 as i32 - 30 as i32)
                ^ (h << 25 as i32 | h >> 64 as i32 - 25 as i32)))
            .wrapping_add(h & a | b & (h | a));
        x[(57 as i32 & 15 as i32) as usize] = (x[(57 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(57 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(57 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(57 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(57 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(57 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(57 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(57 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(57 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(57 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(57 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(57 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_56: u64_0 = g
            .wrapping_add(
                ((d << 50 as i32 | d >> 64 as i32 - 50 as i32)
                    ^ ((d << 46 as i32 | d >> 64 as i32 - 46 as i32)
                        ^ (d << 23 as i32 | d >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[57 as i32 as usize])
                                    .wrapping_add(x[(57 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_56);
        g = t0_56.wrapping_add(t1_56);
        let mut t0_57: u64_0 = ((g << 36 as i32 | g >> 64 as i32 - 36 as i32)
            ^ ((g << 30 as i32 | g >> 64 as i32 - 30 as i32)
                ^ (g << 25 as i32 | g >> 64 as i32 - 25 as i32)))
            .wrapping_add(g & h | a & (g | h));
        x[(58 as i32 & 15 as i32) as usize] = (x[(58 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(58 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(58 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(58 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(58 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(58 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(58 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(58 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(58 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(58 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(58 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(58 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_57: u64_0 = f
            .wrapping_add(
                ((c << 50 as i32 | c >> 64 as i32 - 50 as i32)
                    ^ ((c << 46 as i32 | c >> 64 as i32 - 46 as i32)
                        ^ (c << 23 as i32 | c >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[58 as i32 as usize])
                                    .wrapping_add(x[(58 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_57);
        f = t0_57.wrapping_add(t1_57);
        let mut t0_58: u64_0 = ((f << 36 as i32 | f >> 64 as i32 - 36 as i32)
            ^ ((f << 30 as i32 | f >> 64 as i32 - 30 as i32)
                ^ (f << 25 as i32 | f >> 64 as i32 - 25 as i32)))
            .wrapping_add(f & g | h & (f | g));
        x[(59 as i32 & 15 as i32) as usize] = (x[(59 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(59 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(59 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(59 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(59 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(59 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(59 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(59 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(59 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(59 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(59 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(59 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_58: u64_0 = e
            .wrapping_add(
                ((b << 50 as i32 | b >> 64 as i32 - 50 as i32)
                    ^ ((b << 46 as i32 | b >> 64 as i32 - 46 as i32)
                        ^ (b << 23 as i32 | b >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[59 as i32 as usize])
                                    .wrapping_add(x[(59 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_58);
        e = t0_58.wrapping_add(t1_58);
        let mut t0_59: u64_0 = ((e << 36 as i32 | e >> 64 as i32 - 36 as i32)
            ^ ((e << 30 as i32 | e >> 64 as i32 - 30 as i32)
                ^ (e << 25 as i32 | e >> 64 as i32 - 25 as i32)))
            .wrapping_add(e & f | g & (e | f));
        x[(60 as i32 & 15 as i32) as usize] = (x[(60 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(60 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(60 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(60 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(60 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(60 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(60 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(60 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(60 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(60 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(60 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(60 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_59: u64_0 = d
            .wrapping_add(
                ((a << 50 as i32 | a >> 64 as i32 - 50 as i32)
                    ^ ((a << 46 as i32 | a >> 64 as i32 - 46 as i32)
                        ^ (a << 23 as i32 | a >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[60 as i32 as usize])
                                    .wrapping_add(x[(60 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_59);
        d = t0_59.wrapping_add(t1_59);
        let mut t0_60: u64_0 = ((d << 36 as i32 | d >> 64 as i32 - 36 as i32)
            ^ ((d << 30 as i32 | d >> 64 as i32 - 30 as i32)
                ^ (d << 25 as i32 | d >> 64 as i32 - 25 as i32)))
            .wrapping_add(d & e | f & (d | e));
        x[(61 as i32 & 15 as i32) as usize] = (x[(61 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(61 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(61 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(61 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(61 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(61 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(61 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(61 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(61 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(61 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(61 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(61 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_60: u64_0 = c
            .wrapping_add(
                ((h << 50 as i32 | h >> 64 as i32 - 50 as i32)
                    ^ ((h << 46 as i32 | h >> 64 as i32 - 46 as i32)
                        ^ (h << 23 as i32 | h >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[61 as i32 as usize])
                                    .wrapping_add(x[(61 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_60);
        c = t0_60.wrapping_add(t1_60);
        let mut t0_61: u64_0 = ((c << 36 as i32 | c >> 64 as i32 - 36 as i32)
            ^ ((c << 30 as i32 | c >> 64 as i32 - 30 as i32)
                ^ (c << 25 as i32 | c >> 64 as i32 - 25 as i32)))
            .wrapping_add(c & d | e & (c | d));
        x[(62 as i32 & 15 as i32) as usize] = (x[(62 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(62 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(62 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(62 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(62 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(62 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(62 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(62 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(62 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(62 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(62 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(62 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_61: u64_0 = b
            .wrapping_add(
                ((g << 50 as i32 | g >> 64 as i32 - 50 as i32)
                    ^ ((g << 46 as i32 | g >> 64 as i32 - 46 as i32)
                        ^ (g << 23 as i32 | g >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[62 as i32 as usize])
                                    .wrapping_add(x[(62 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_61);
        b = t0_61.wrapping_add(t1_61);
        let mut t0_62: u64_0 = ((b << 36 as i32 | b >> 64 as i32 - 36 as i32)
            ^ ((b << 30 as i32 | b >> 64 as i32 - 30 as i32)
                ^ (b << 25 as i32 | b >> 64 as i32 - 25 as i32)))
            .wrapping_add(b & c | d & (b | c));
        x[(63 as i32 & 15 as i32) as usize] = (x[(63 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(63 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(63 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(63 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(63 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(63 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(63 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(63 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(63 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(63 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(63 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(63 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_62: u64_0 = a
            .wrapping_add(
                ((f << 50 as i32 | f >> 64 as i32 - 50 as i32)
                    ^ ((f << 46 as i32 | f >> 64 as i32 - 46 as i32)
                        ^ (f << 23 as i32 | f >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[63 as i32 as usize])
                                    .wrapping_add(x[(63 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_62);
        a = t0_62.wrapping_add(t1_62);
        let mut t0_63: u64_0 = ((a << 36 as i32 | a >> 64 as i32 - 36 as i32)
            ^ ((a << 30 as i32 | a >> 64 as i32 - 30 as i32)
                ^ (a << 25 as i32 | a >> 64 as i32 - 25 as i32)))
            .wrapping_add(a & b | c & (a | b));
        x[(64 as i32 & 15 as i32) as usize] = (x[(64 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(64 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(64 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(64 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(64 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(64 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(64 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(64 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(64 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(64 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(64 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(64 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_63: u64_0 = h
            .wrapping_add(
                ((e << 50 as i32 | e >> 64 as i32 - 50 as i32)
                    ^ ((e << 46 as i32 | e >> 64 as i32 - 46 as i32)
                        ^ (e << 23 as i32 | e >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[64 as i32 as usize])
                                    .wrapping_add(x[(64 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_63);
        h = t0_63.wrapping_add(t1_63);
        let mut t0_64: u64_0 = ((h << 36 as i32 | h >> 64 as i32 - 36 as i32)
            ^ ((h << 30 as i32 | h >> 64 as i32 - 30 as i32)
                ^ (h << 25 as i32 | h >> 64 as i32 - 25 as i32)))
            .wrapping_add(h & a | b & (h | a));
        x[(65 as i32 & 15 as i32) as usize] = (x[(65 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(65 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(65 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(65 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(65 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(65 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(65 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(65 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(65 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(65 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(65 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(65 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_64: u64_0 = g
            .wrapping_add(
                ((d << 50 as i32 | d >> 64 as i32 - 50 as i32)
                    ^ ((d << 46 as i32 | d >> 64 as i32 - 46 as i32)
                        ^ (d << 23 as i32 | d >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[65 as i32 as usize])
                                    .wrapping_add(x[(65 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_64);
        g = t0_64.wrapping_add(t1_64);
        let mut t0_65: u64_0 = ((g << 36 as i32 | g >> 64 as i32 - 36 as i32)
            ^ ((g << 30 as i32 | g >> 64 as i32 - 30 as i32)
                ^ (g << 25 as i32 | g >> 64 as i32 - 25 as i32)))
            .wrapping_add(g & h | a & (g | h));
        x[(66 as i32 & 15 as i32) as usize] = (x[(66 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(66 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(66 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(66 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(66 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(66 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(66 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(66 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(66 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(66 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(66 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(66 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_65: u64_0 = f
            .wrapping_add(
                ((c << 50 as i32 | c >> 64 as i32 - 50 as i32)
                    ^ ((c << 46 as i32 | c >> 64 as i32 - 46 as i32)
                        ^ (c << 23 as i32 | c >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[66 as i32 as usize])
                                    .wrapping_add(x[(66 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_65);
        f = t0_65.wrapping_add(t1_65);
        let mut t0_66: u64_0 = ((f << 36 as i32 | f >> 64 as i32 - 36 as i32)
            ^ ((f << 30 as i32 | f >> 64 as i32 - 30 as i32)
                ^ (f << 25 as i32 | f >> 64 as i32 - 25 as i32)))
            .wrapping_add(f & g | h & (f | g));
        x[(67 as i32 & 15 as i32) as usize] = (x[(67 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(67 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(67 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(67 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(67 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(67 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(67 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(67 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(67 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(67 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(67 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(67 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_66: u64_0 = e
            .wrapping_add(
                ((b << 50 as i32 | b >> 64 as i32 - 50 as i32)
                    ^ ((b << 46 as i32 | b >> 64 as i32 - 46 as i32)
                        ^ (b << 23 as i32 | b >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[67 as i32 as usize])
                                    .wrapping_add(x[(67 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_66);
        e = t0_66.wrapping_add(t1_66);
        let mut t0_67: u64_0 = ((e << 36 as i32 | e >> 64 as i32 - 36 as i32)
            ^ ((e << 30 as i32 | e >> 64 as i32 - 30 as i32)
                ^ (e << 25 as i32 | e >> 64 as i32 - 25 as i32)))
            .wrapping_add(e & f | g & (e | f));
        x[(68 as i32 & 15 as i32) as usize] = (x[(68 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(68 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(68 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(68 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(68 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(68 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(68 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(68 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(68 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(68 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(68 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(68 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_67: u64_0 = d
            .wrapping_add(
                ((a << 50 as i32 | a >> 64 as i32 - 50 as i32)
                    ^ ((a << 46 as i32 | a >> 64 as i32 - 46 as i32)
                        ^ (a << 23 as i32 | a >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[68 as i32 as usize])
                                    .wrapping_add(x[(68 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_67);
        d = t0_67.wrapping_add(t1_67);
        let mut t0_68: u64_0 = ((d << 36 as i32 | d >> 64 as i32 - 36 as i32)
            ^ ((d << 30 as i32 | d >> 64 as i32 - 30 as i32)
                ^ (d << 25 as i32 | d >> 64 as i32 - 25 as i32)))
            .wrapping_add(d & e | f & (d | e));
        x[(69 as i32 & 15 as i32) as usize] = (x[(69 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(69 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(69 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(69 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(69 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(69 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(69 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(69 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(69 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(69 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(69 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(69 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_68: u64_0 = c
            .wrapping_add(
                ((h << 50 as i32 | h >> 64 as i32 - 50 as i32)
                    ^ ((h << 46 as i32 | h >> 64 as i32 - 46 as i32)
                        ^ (h << 23 as i32 | h >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[69 as i32 as usize])
                                    .wrapping_add(x[(69 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_68);
        c = t0_68.wrapping_add(t1_68);
        let mut t0_69: u64_0 = ((c << 36 as i32 | c >> 64 as i32 - 36 as i32)
            ^ ((c << 30 as i32 | c >> 64 as i32 - 30 as i32)
                ^ (c << 25 as i32 | c >> 64 as i32 - 25 as i32)))
            .wrapping_add(c & d | e & (c | d));
        x[(70 as i32 & 15 as i32) as usize] = (x[(70 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(70 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(70 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(70 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(70 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(70 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(70 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(70 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(70 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(70 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(70 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(70 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_69: u64_0 = b
            .wrapping_add(
                ((g << 50 as i32 | g >> 64 as i32 - 50 as i32)
                    ^ ((g << 46 as i32 | g >> 64 as i32 - 46 as i32)
                        ^ (g << 23 as i32 | g >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[70 as i32 as usize])
                                    .wrapping_add(x[(70 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_69);
        b = t0_69.wrapping_add(t1_69);
        let mut t0_70: u64_0 = ((b << 36 as i32 | b >> 64 as i32 - 36 as i32)
            ^ ((b << 30 as i32 | b >> 64 as i32 - 30 as i32)
                ^ (b << 25 as i32 | b >> 64 as i32 - 25 as i32)))
            .wrapping_add(b & c | d & (b | c));
        x[(71 as i32 & 15 as i32) as usize] = (x[(71 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(71 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(71 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(71 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(71 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(71 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(71 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(71 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(71 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(71 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(71 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(71 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_70: u64_0 = a
            .wrapping_add(
                ((f << 50 as i32 | f >> 64 as i32 - 50 as i32)
                    ^ ((f << 46 as i32 | f >> 64 as i32 - 46 as i32)
                        ^ (f << 23 as i32 | f >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[71 as i32 as usize])
                                    .wrapping_add(x[(71 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_70);
        a = t0_70.wrapping_add(t1_70);
        let mut t0_71: u64_0 = ((a << 36 as i32 | a >> 64 as i32 - 36 as i32)
            ^ ((a << 30 as i32 | a >> 64 as i32 - 30 as i32)
                ^ (a << 25 as i32 | a >> 64 as i32 - 25 as i32)))
            .wrapping_add(a & b | c & (a | b));
        x[(72 as i32 & 15 as i32) as usize] = (x[(72 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(72 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(72 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(72 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(72 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(72 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(72 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(72 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(72 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(72 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(72 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(72 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_71: u64_0 = h
            .wrapping_add(
                ((e << 50 as i32 | e >> 64 as i32 - 50 as i32)
                    ^ ((e << 46 as i32 | e >> 64 as i32 - 46 as i32)
                        ^ (e << 23 as i32 | e >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (g ^ e & (f ^ g))
                            .wrapping_add(
                                (sha512_round_constants[72 as i32 as usize])
                                    .wrapping_add(x[(72 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        d = d.wrapping_add(t1_71);
        h = t0_71.wrapping_add(t1_71);
        let mut t0_72: u64_0 = ((h << 36 as i32 | h >> 64 as i32 - 36 as i32)
            ^ ((h << 30 as i32 | h >> 64 as i32 - 30 as i32)
                ^ (h << 25 as i32 | h >> 64 as i32 - 25 as i32)))
            .wrapping_add(h & a | b & (h | a));
        x[(73 as i32 & 15 as i32) as usize] = (x[(73 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(73 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(73 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(73 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(73 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(73 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(73 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(73 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(73 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(73 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(73 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(73 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_72: u64_0 = g
            .wrapping_add(
                ((d << 50 as i32 | d >> 64 as i32 - 50 as i32)
                    ^ ((d << 46 as i32 | d >> 64 as i32 - 46 as i32)
                        ^ (d << 23 as i32 | d >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (f ^ d & (e ^ f))
                            .wrapping_add(
                                (sha512_round_constants[73 as i32 as usize])
                                    .wrapping_add(x[(73 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        c = c.wrapping_add(t1_72);
        g = t0_72.wrapping_add(t1_72);
        let mut t0_73: u64_0 = ((g << 36 as i32 | g >> 64 as i32 - 36 as i32)
            ^ ((g << 30 as i32 | g >> 64 as i32 - 30 as i32)
                ^ (g << 25 as i32 | g >> 64 as i32 - 25 as i32)))
            .wrapping_add(g & h | a & (g | h));
        x[(74 as i32 & 15 as i32) as usize] = (x[(74 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(74 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(74 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(74 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(74 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(74 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(74 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(74 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(74 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(74 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(74 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(74 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_73: u64_0 = f
            .wrapping_add(
                ((c << 50 as i32 | c >> 64 as i32 - 50 as i32)
                    ^ ((c << 46 as i32 | c >> 64 as i32 - 46 as i32)
                        ^ (c << 23 as i32 | c >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (e ^ c & (d ^ e))
                            .wrapping_add(
                                (sha512_round_constants[74 as i32 as usize])
                                    .wrapping_add(x[(74 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        b = b.wrapping_add(t1_73);
        f = t0_73.wrapping_add(t1_73);
        let mut t0_74: u64_0 = ((f << 36 as i32 | f >> 64 as i32 - 36 as i32)
            ^ ((f << 30 as i32 | f >> 64 as i32 - 30 as i32)
                ^ (f << 25 as i32 | f >> 64 as i32 - 25 as i32)))
            .wrapping_add(f & g | h & (f | g));
        x[(75 as i32 & 15 as i32) as usize] = (x[(75 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(75 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(75 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(75 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(75 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(75 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(75 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(75 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(75 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(75 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(75 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(75 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_74: u64_0 = e
            .wrapping_add(
                ((b << 50 as i32 | b >> 64 as i32 - 50 as i32)
                    ^ ((b << 46 as i32 | b >> 64 as i32 - 46 as i32)
                        ^ (b << 23 as i32 | b >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (d ^ b & (c ^ d))
                            .wrapping_add(
                                (sha512_round_constants[75 as i32 as usize])
                                    .wrapping_add(x[(75 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        a = a.wrapping_add(t1_74);
        e = t0_74.wrapping_add(t1_74);
        let mut t0_75: u64_0 = ((e << 36 as i32 | e >> 64 as i32 - 36 as i32)
            ^ ((e << 30 as i32 | e >> 64 as i32 - 30 as i32)
                ^ (e << 25 as i32 | e >> 64 as i32 - 25 as i32)))
            .wrapping_add(e & f | g & (e | f));
        x[(76 as i32 & 15 as i32) as usize] = (x[(76 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(76 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(76 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(76 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(76 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(76 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(76 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(76 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(76 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(76 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(76 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(76 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_75: u64_0 = d
            .wrapping_add(
                ((a << 50 as i32 | a >> 64 as i32 - 50 as i32)
                    ^ ((a << 46 as i32 | a >> 64 as i32 - 46 as i32)
                        ^ (a << 23 as i32 | a >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (c ^ a & (b ^ c))
                            .wrapping_add(
                                (sha512_round_constants[76 as i32 as usize])
                                    .wrapping_add(x[(76 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        h = h.wrapping_add(t1_75);
        d = t0_75.wrapping_add(t1_75);
        let mut t0_76: u64_0 = ((d << 36 as i32 | d >> 64 as i32 - 36 as i32)
            ^ ((d << 30 as i32 | d >> 64 as i32 - 30 as i32)
                ^ (d << 25 as i32 | d >> 64 as i32 - 25 as i32)))
            .wrapping_add(d & e | f & (d | e));
        x[(77 as i32 & 15 as i32) as usize] = (x[(77 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(77 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(77 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(77 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(77 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(77 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(77 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(77 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(77 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(77 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(77 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(77 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_76: u64_0 = c
            .wrapping_add(
                ((h << 50 as i32 | h >> 64 as i32 - 50 as i32)
                    ^ ((h << 46 as i32 | h >> 64 as i32 - 46 as i32)
                        ^ (h << 23 as i32 | h >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (b ^ h & (a ^ b))
                            .wrapping_add(
                                (sha512_round_constants[77 as i32 as usize])
                                    .wrapping_add(x[(77 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        g = g.wrapping_add(t1_76);
        c = t0_76.wrapping_add(t1_76);
        let mut t0_77: u64_0 = ((c << 36 as i32 | c >> 64 as i32 - 36 as i32)
            ^ ((c << 30 as i32 | c >> 64 as i32 - 30 as i32)
                ^ (c << 25 as i32 | c >> 64 as i32 - 25 as i32)))
            .wrapping_add(c & d | e & (c | d));
        x[(78 as i32 & 15 as i32) as usize] = (x[(78 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(78 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(78 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(78 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(78 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(78 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(78 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(78 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(78 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(78 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(78 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(78 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_77: u64_0 = b
            .wrapping_add(
                ((g << 50 as i32 | g >> 64 as i32 - 50 as i32)
                    ^ ((g << 46 as i32 | g >> 64 as i32 - 46 as i32)
                        ^ (g << 23 as i32 | g >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (a ^ g & (h ^ a))
                            .wrapping_add(
                                (sha512_round_constants[78 as i32 as usize])
                                    .wrapping_add(x[(78 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        f = f.wrapping_add(t1_77);
        b = t0_77.wrapping_add(t1_77);
        let mut t0_78: u64_0 = ((b << 36 as i32 | b >> 64 as i32 - 36 as i32)
            ^ ((b << 30 as i32 | b >> 64 as i32 - 30 as i32)
                ^ (b << 25 as i32 | b >> 64 as i32 - 25 as i32)))
            .wrapping_add(b & c | d & (b | c));
        x[(79 as i32 & 15 as i32) as usize] = (x[(79 as i32 & 15 as i32) as usize])
            .wrapping_add(
                ((x[(79 as i32 - 2 as i32 & 15 as i32) as usize] << 45 as i32
                    | x[(79 as i32 - 2 as i32 & 15 as i32) as usize]
                        >> 64 as i32 - 45 as i32)
                    ^ ((x[(79 as i32 - 2 as i32 & 15 as i32) as usize] << 3 as i32
                        | x[(79 as i32 - 2 as i32 & 15 as i32) as usize]
                            >> 64 as i32 - 3 as i32)
                        ^ x[(79 as i32 - 2 as i32 & 15 as i32) as usize] >> 6 as i32))
                    .wrapping_add(
                        (x[(79 as i32 - 7 as i32 & 15 as i32) as usize])
                            .wrapping_add(
                                (x[(79 as i32 - 15 as i32 & 15 as i32) as usize]
                                    << 63 as i32
                                    | x[(79 as i32 - 15 as i32 & 15 as i32) as usize]
                                        >> 64 as i32 - 63 as i32)
                                    ^ ((x[(79 as i32 - 15 as i32 & 15 as i32) as usize]
                                        << 56 as i32
                                        | x[(79 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 64 as i32 - 56 as i32)
                                        ^ x[(79 as i32 - 15 as i32 & 15 as i32) as usize]
                                            >> 7 as i32),
                            ),
                    ),
            );
        let mut t1_78: u64_0 = a
            .wrapping_add(
                ((f << 50 as i32 | f >> 64 as i32 - 50 as i32)
                    ^ ((f << 46 as i32 | f >> 64 as i32 - 46 as i32)
                        ^ (f << 23 as i32 | f >> 64 as i32 - 23 as i32)))
                    .wrapping_add(
                        (h ^ f & (g ^ h))
                            .wrapping_add(
                                (sha512_round_constants[79 as i32 as usize])
                                    .wrapping_add(x[(79 as i32 & 15 as i32) as usize]),
                            ),
                    ),
            );
        e = e.wrapping_add(t1_78);
        a = t0_78.wrapping_add(t1_78);
        (*ctx).state[0 as i32 as usize] = ((*ctx).state[0 as i32 as usize])
            .wrapping_add(a);
        a = (*ctx).state[0 as i32 as usize];
        (*ctx).state[1 as i32 as usize] = ((*ctx).state[1 as i32 as usize])
            .wrapping_add(b);
        b = (*ctx).state[1 as i32 as usize];
        (*ctx).state[2 as i32 as usize] = ((*ctx).state[2 as i32 as usize])
            .wrapping_add(c);
        c = (*ctx).state[2 as i32 as usize];
        (*ctx).state[3 as i32 as usize] = ((*ctx).state[3 as i32 as usize])
            .wrapping_add(d);
        d = (*ctx).state[3 as i32 as usize];
        (*ctx).state[4 as i32 as usize] = ((*ctx).state[4 as i32 as usize])
            .wrapping_add(e);
        e = (*ctx).state[4 as i32 as usize];
        (*ctx).state[5 as i32 as usize] = ((*ctx).state[5 as i32 as usize])
            .wrapping_add(f);
        f = (*ctx).state[5 as i32 as usize];
        (*ctx).state[6 as i32 as usize] = ((*ctx).state[6 as i32 as usize])
            .wrapping_add(g);
        g = (*ctx).state[6 as i32 as usize];
        (*ctx).state[7 as i32 as usize] = ((*ctx).state[7 as i32 as usize])
            .wrapping_add(h);
        h = (*ctx).state[7 as i32 as usize];
    }
}
unsafe extern "C" fn run_static_initializers() {
    sha512_round_constants = [
        ((0x428a2f98 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xd728ae22 as u32 as u64),
        ((0x71374491 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x23ef65cd as i32 as u64),
        ((0xb5c0fbcf as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xec4d3b2f as u32 as u64),
        ((0xe9b5dba5 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x8189dbbc as u32 as u64),
        ((0x3956c25b as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xf348b538 as u32 as u64),
        ((0x59f111f1 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xb605d019 as u32 as u64),
        ((0x923f82a4 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xaf194f9b as u32 as u64),
        ((0xab1c5ed5 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xda6d8118 as u32 as u64),
        ((0xd807aa98 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xa3030242 as u32 as u64),
        ((0x12835b01 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x45706fbe as i32 as u64),
        ((0x243185be as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x4ee4b28c as i32 as u64),
        ((0x550c7dc3 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xd5ffb4e2 as u32 as u64),
        ((0x72be5d74 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xf27b896f as u32 as u64),
        ((0x80deb1fe as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x3b1696b1 as i32 as u64),
        ((0x9bdc06a7 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x25c71235 as i32 as u64),
        ((0xc19bf174 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xcf692694 as u32 as u64),
        ((0xe49b69c1 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x9ef14ad2 as u32 as u64),
        ((0xefbe4786 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x384f25e3 as i32 as u64),
        ((0xfc19dc6 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x8b8cd5b5 as u32 as u64),
        ((0x240ca1cc as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x77ac9c65 as i32 as u64),
        ((0x2de92c6f as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x592b0275 as i32 as u64),
        ((0x4a7484aa as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x6ea6e483 as i32 as u64),
        ((0x5cb0a9dc as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xbd41fbd4 as u32 as u64),
        ((0x76f988da as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x831153b5 as u32 as u64),
        ((0x983e5152 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xee66dfab as u32 as u64),
        ((0xa831c66d as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x2db43210 as i32 as u64),
        ((0xb00327c8 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x98fb213f as u32 as u64),
        ((0xbf597fc7 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xbeef0ee4 as u32 as u64),
        ((0xc6e00bf3 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x3da88fc2 as i32 as u64),
        ((0xd5a79147 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x930aa725 as u32 as u64),
        ((0x6ca6351 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xe003826f as u32 as u64),
        ((0x14292967 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xa0e6e70 as i32 as u64),
        ((0x27b70a85 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x46d22ffc as i32 as u64),
        ((0x2e1b2138 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x5c26c926 as i32 as u64),
        ((0x4d2c6dfc as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x5ac42aed as i32 as u64),
        ((0x53380d13 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x9d95b3df as u32 as u64),
        ((0x650a7354 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x8baf63de as u32 as u64),
        ((0x766a0abb as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x3c77b2a8 as i32 as u64),
        ((0x81c2c92e as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x47edaee6 as i32 as u64),
        ((0x92722c85 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x1482353b as i32 as u64),
        ((0xa2bfe8a1 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x4cf10364 as i32 as u64),
        ((0xa81a664b as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xbc423001 as u32 as u64),
        ((0xc24b8b70 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xd0f89791 as u32 as u64),
        ((0xc76c51a3 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x654be30 as i32 as u64),
        ((0xd192e819 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xd6ef5218 as u32 as u64),
        ((0xd6990624 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x5565a910 as i32 as u64),
        ((0xf40e3585 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x5771202a as i32 as u64),
        ((0x106aa070 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x32bbd1b8 as i32 as u64),
        ((0x19a4c116 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xb8d2d0c8 as u32 as u64),
        ((0x1e376c08 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x5141ab53 as i32 as u64),
        ((0x2748774c as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xdf8eeb99 as u32 as u64),
        ((0x34b0bcb5 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xe19b48a8 as u32 as u64),
        ((0x391c0cb3 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xc5c95a63 as u32 as u64),
        ((0x4ed8aa4a as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xe3418acb as u32 as u64),
        ((0x5b9cca4f as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x7763e373 as i32 as u64),
        ((0x682e6ff3 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xd6b2b8a3 as u32 as u64),
        ((0x748f82ee as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x5defb2fc as i32 as u64),
        ((0x78a5636f as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x43172f60 as i32 as u64),
        ((0x84c87814 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xa1f0ab72 as u32 as u64),
        ((0x8cc70208 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x1a6439ec as i32 as u64),
        ((0x90befffa as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x23631e28 as i32 as u64),
        ((0xa4506ceb as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xde82bde9 as u32 as u64),
        ((0xbef9a3f7 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xb2c67915 as u32 as u64),
        ((0xc67178f2 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xe372532b as u32 as u64),
        ((0xca273ece as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xea26619c as u32 as u64),
        ((0xd186b8c7 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0x21c0c207 as i32 as u64),
        ((0xeada7dd6 as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xcde0eb1e as u32 as u64),
        ((0xf57d4f7f as u32 as u64_0) << 32 as i32)
            .wrapping_add(0xee6ed178 as u32 as u64),
        ((0x6f067aa as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x72176fba as i32 as u64),
        ((0xa637dc5 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xa2c898a6 as u32 as u64),
        ((0x113f9804 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xbef90dae as u32 as u64),
        ((0x1b710b35 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x131c471b as i32 as u64),
        ((0x28db77f5 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x23047d84 as i32 as u64),
        ((0x32caab7b as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x40c72493 as i32 as u64),
        ((0x3c9ebe0a as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x15c9bebc as i32 as u64),
        ((0x431d67c4 as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x9c100d4c as u32 as u64),
        ((0x4cc5d4be as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xcb3e42b6 as u32 as u64),
        ((0x597f299c as i32 as u64_0) << 32 as i32)
            .wrapping_add(0xfc657e2a as u32 as u64),
        ((0x5fcb6fab as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x3ad6faec as i32 as u64),
        ((0x6c44198c as i32 as u64_0) << 32 as i32)
            .wrapping_add(0x4a475817 as i32 as u64),
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];