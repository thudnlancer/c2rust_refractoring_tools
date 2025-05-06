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
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type uintptr_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha1_ctx {
    pub A: uint32_t,
    pub B: uint32_t,
    pub C: uint32_t,
    pub D: uint32_t,
    pub E: uint32_t,
    pub total: [uint32_t; 2],
    pub buflen: uint32_t,
    pub buffer: [uint32_t; 32],
}
static mut fillbuf: [u8; 64] = [
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
];
#[no_mangle]
pub unsafe extern "C" fn sha1_init_ctx(mut ctx: *mut sha1_ctx) {
    (*ctx).A = 0x67452301 as i32 as uint32_t;
    (*ctx).B = 0xefcdab89 as u32;
    (*ctx).C = 0x98badcfe as u32;
    (*ctx).D = 0x10325476 as i32 as uint32_t;
    (*ctx).E = 0xc3d2e1f0 as u32;
    (*ctx).total[1 as i32 as usize] = 0 as i32 as uint32_t;
    (*ctx).total[0 as i32 as usize] = (*ctx).total[1 as i32 as usize];
    (*ctx).buflen = 0 as i32 as uint32_t;
}
unsafe extern "C" fn set_uint32(mut cp: *mut i8, mut v: uint32_t) {
    memcpy(
        cp as *mut libc::c_void,
        &mut v as *mut uint32_t as *const libc::c_void,
        ::core::mem::size_of::<uint32_t>() as u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sha1_read_ctx(
    mut ctx: *const sha1_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut r: *mut i8 = resbuf as *mut i8;
    set_uint32(
        r
            .offset(
                (0 as i32 as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64)
                    as isize,
            ),
        ({
            let mut __v: u32 = 0;
            let mut __x: u32 = (*ctx).A;
            if 0 != 0 {
                __v = (__x & 0xff000000 as u32) >> 24 as i32
                    | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                    | (__x & 0xff00 as i32 as u32) << 8 as i32
                    | (__x & 0xff as i32 as u32) << 24 as i32;
            } else {
                let fresh0 = &mut __v;
                let fresh1;
                let fresh2 = __x;
                asm!(
                    "bswap {0}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            }
            __v
        }),
    );
    set_uint32(
        r
            .offset(
                (1 as i32 as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64)
                    as isize,
            ),
        ({
            let mut __v: u32 = 0;
            let mut __x: u32 = (*ctx).B;
            if 0 != 0 {
                __v = (__x & 0xff000000 as u32) >> 24 as i32
                    | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                    | (__x & 0xff00 as i32 as u32) << 8 as i32
                    | (__x & 0xff as i32 as u32) << 24 as i32;
            } else {
                let fresh3 = &mut __v;
                let fresh4;
                let fresh5 = __x;
                asm!(
                    "bswap {0}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) => fresh4,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
            }
            __v
        }),
    );
    set_uint32(
        r
            .offset(
                (2 as i32 as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64)
                    as isize,
            ),
        ({
            let mut __v: u32 = 0;
            let mut __x: u32 = (*ctx).C;
            if 0 != 0 {
                __v = (__x & 0xff000000 as u32) >> 24 as i32
                    | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                    | (__x & 0xff00 as i32 as u32) << 8 as i32
                    | (__x & 0xff as i32 as u32) << 24 as i32;
            } else {
                let fresh6 = &mut __v;
                let fresh7;
                let fresh8 = __x;
                asm!(
                    "bswap {0}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8) => fresh7,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
            }
            __v
        }),
    );
    set_uint32(
        r
            .offset(
                (3 as i32 as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64)
                    as isize,
            ),
        ({
            let mut __v: u32 = 0;
            let mut __x: u32 = (*ctx).D;
            if 0 != 0 {
                __v = (__x & 0xff000000 as u32) >> 24 as i32
                    | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                    | (__x & 0xff00 as i32 as u32) << 8 as i32
                    | (__x & 0xff as i32 as u32) << 24 as i32;
            } else {
                let fresh9 = &mut __v;
                let fresh10;
                let fresh11 = __x;
                asm!(
                    "bswap {0}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11) => fresh10,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
            }
            __v
        }),
    );
    set_uint32(
        r
            .offset(
                (4 as i32 as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64)
                    as isize,
            ),
        ({
            let mut __v: u32 = 0;
            let mut __x: u32 = (*ctx).E;
            if 0 != 0 {
                __v = (__x & 0xff000000 as u32) >> 24 as i32
                    | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                    | (__x & 0xff00 as i32 as u32) << 8 as i32
                    | (__x & 0xff as i32 as u32) << 24 as i32;
            } else {
                let fresh12 = &mut __v;
                let fresh13;
                let fresh14 = __x;
                asm!(
                    "bswap {0}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14) => fresh13,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
            }
            __v
        }),
    );
    return resbuf;
}
#[no_mangle]
pub unsafe extern "C" fn sha1_finish_ctx(
    mut ctx: *mut sha1_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut bytes: uint32_t = (*ctx).buflen;
    let mut size: size_t = (if bytes < 56 as i32 as u32 {
        64 as i32 / 4 as i32
    } else {
        64 as i32 * 2 as i32 / 4 as i32
    }) as size_t;
    (*ctx).total[0 as i32 as usize] = ((*ctx).total[0 as i32 as usize] as u32)
        .wrapping_add(bytes) as uint32_t as uint32_t;
    if (*ctx).total[0 as i32 as usize] < bytes {
        (*ctx).total[1 as i32 as usize] = ((*ctx).total[1 as i32 as usize])
            .wrapping_add(1);
        (*ctx).total[1 as i32 as usize];
    }
    (*ctx).buffer[size.wrapping_sub(2 as i32 as u64) as usize] = ({
        let mut __v: u32 = 0;
        let mut __x: u32 = (*ctx).total[1 as i32 as usize] << 3 as i32
            | (*ctx).total[0 as i32 as usize] >> 29 as i32;
        if 0 != 0 {
            __v = (__x & 0xff000000 as u32) >> 24 as i32
                | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                | (__x & 0xff00 as i32 as u32) << 8 as i32
                | (__x & 0xff as i32 as u32) << 24 as i32;
        } else {
            let fresh15 = &mut __v;
            let fresh16;
            let fresh17 = __x;
            asm!(
                "bswap {0}", inlateout(reg) c2rust_asm_casts::AsmCast::cast_in(fresh15,
                fresh17) => fresh16, options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
        }
        __v
    });
    (*ctx).buffer[size.wrapping_sub(1 as i32 as u64) as usize] = ({
        let mut __v: u32 = 0;
        let mut __x: u32 = (*ctx).total[0 as i32 as usize] << 3 as i32;
        if 0 != 0 {
            __v = (__x & 0xff000000 as u32) >> 24 as i32
                | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                | (__x & 0xff00 as i32 as u32) << 8 as i32
                | (__x & 0xff as i32 as u32) << 24 as i32;
        } else {
            let fresh18 = &mut __v;
            let fresh19;
            let fresh20 = __x;
            asm!(
                "bswap {0}", inlateout(reg) c2rust_asm_casts::AsmCast::cast_in(fresh18,
                fresh20) => fresh19, options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
        }
        __v
    });
    memcpy(
        &mut *(((*ctx).buffer).as_mut_ptr() as *mut i8).offset(bytes as isize) as *mut i8
            as *mut libc::c_void,
        fillbuf.as_ptr() as *const libc::c_void,
        size
            .wrapping_sub(2 as i32 as u64)
            .wrapping_mul(4 as i32 as u64)
            .wrapping_sub(bytes as u64),
    );
    sha1_process_block(
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        size.wrapping_mul(4 as i32 as u64),
        ctx,
    );
    return sha1_read_ctx(ctx, resbuf);
}
#[no_mangle]
pub unsafe extern "C" fn sha1_buffer(
    mut buffer: *const i8,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: sha1_ctx = sha1_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        E: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    sha1_init_ctx(&mut ctx);
    sha1_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return sha1_finish_ctx(&mut ctx, resblock);
}
#[no_mangle]
pub unsafe extern "C" fn sha1_process_bytes(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha1_ctx,
) {
    if (*ctx).buflen != 0 as i32 as u32 {
        let mut left_over: size_t = (*ctx).buflen as size_t;
        let mut add: size_t = if (128 as i32 as u64).wrapping_sub(left_over) > len {
            len
        } else {
            (128 as i32 as u64).wrapping_sub(left_over)
        };
        memcpy(
            &mut *(((*ctx).buffer).as_mut_ptr() as *mut i8).offset(left_over as isize)
                as *mut i8 as *mut libc::c_void,
            buffer,
            add,
        );
        (*ctx).buflen = ((*ctx).buflen as u64).wrapping_add(add) as uint32_t as uint32_t;
        if (*ctx).buflen > 64 as i32 as u32 {
            sha1_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                ((*ctx).buflen & !(63 as i32) as u32) as size_t,
                ctx,
            );
            (*ctx).buflen &= 63 as i32 as u32;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *(((*ctx).buffer).as_mut_ptr() as *mut i8)
                    .offset((left_over.wrapping_add(add) & !(63 as i32) as u64) as isize)
                    as *mut i8 as *const libc::c_void,
                (*ctx).buflen as u64,
            );
        }
        buffer = (buffer as *const i8).offset(add as isize) as *const libc::c_void;
        len = (len as u64).wrapping_sub(add) as size_t as size_t;
    }
    if len >= 64 as i32 as u64 {
        if (buffer as uintptr_t).wrapping_rem(4 as u64) != 0 as i32 as u64 {
            while len > 64 as i32 as u64 {
                sha1_process_block(
                    memcpy(
                        ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                        buffer,
                        64 as i32 as u64,
                    ),
                    64 as i32 as size_t,
                    ctx,
                );
                buffer = (buffer as *const i8).offset(64 as i32 as isize)
                    as *const libc::c_void;
                len = (len as u64).wrapping_sub(64 as i32 as u64) as size_t as size_t;
            }
        } else {
            sha1_process_block(buffer, len & !(63 as i32) as u64, ctx);
            buffer = (buffer as *const i8).offset((len & !(63 as i32) as u64) as isize)
                as *const libc::c_void;
            len &= 63 as i32 as u64;
        }
    }
    if len > 0 as i32 as u64 {
        let mut left_over_0: size_t = (*ctx).buflen as size_t;
        memcpy(
            &mut *(((*ctx).buffer).as_mut_ptr() as *mut i8).offset(left_over_0 as isize)
                as *mut i8 as *mut libc::c_void,
            buffer,
            len,
        );
        left_over_0 = (left_over_0 as u64).wrapping_add(len) as size_t as size_t;
        if left_over_0 >= 64 as i32 as u64 {
            sha1_process_block(
                ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
                64 as i32 as size_t,
                ctx,
            );
            left_over_0 = (left_over_0 as u64).wrapping_sub(64 as i32 as u64) as size_t
                as size_t;
            memcpy(
                ((*ctx).buffer).as_mut_ptr() as *mut libc::c_void,
                &mut *((*ctx).buffer).as_mut_ptr().offset(16 as i32 as isize)
                    as *mut uint32_t as *const libc::c_void,
                left_over_0,
            );
        }
        (*ctx).buflen = left_over_0 as uint32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sha1_process_block(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut sha1_ctx,
) {
    let mut words: *const uint32_t = buffer as *const uint32_t;
    let mut nwords: size_t = len.wrapping_div(::core::mem::size_of::<uint32_t>() as u64);
    let mut endp: *const uint32_t = words.offset(nwords as isize);
    let mut x: [uint32_t; 16] = [0; 16];
    let mut a: uint32_t = (*ctx).A;
    let mut b: uint32_t = (*ctx).B;
    let mut c: uint32_t = (*ctx).C;
    let mut d: uint32_t = (*ctx).D;
    let mut e: uint32_t = (*ctx).E;
    let mut lolen: uint32_t = len as uint32_t;
    (*ctx).total[0 as i32 as usize] = ((*ctx).total[0 as i32 as usize] as u32)
        .wrapping_add(lolen) as uint32_t as uint32_t;
    (*ctx).total[1 as i32 as usize] = ((*ctx).total[1 as i32 as usize] as u64)
        .wrapping_add(
            (len >> 31 as i32 >> 1 as i32)
                .wrapping_add(((*ctx).total[0 as i32 as usize] < lolen) as i32 as u64),
        ) as uint32_t as uint32_t;
    while words < endp {
        let mut tm: uint32_t = 0;
        let mut t: i32 = 0;
        t = 0 as i32;
        while t < 16 as i32 {
            x[t as usize] = ({
                let mut __v: u32 = 0;
                let mut __x: u32 = *words;
                if 0 != 0 {
                    __v = (__x & 0xff000000 as u32) >> 24 as i32
                        | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                        | (__x & 0xff00 as i32 as u32) << 8 as i32
                        | (__x & 0xff as i32 as u32) << 24 as i32;
                } else {
                    let fresh21 = &mut __v;
                    let fresh22;
                    let fresh23 = __x;
                    asm!(
                        "bswap {0}", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23) => fresh22,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
                }
                __v
            });
            words = words.offset(1);
            words;
            t += 1;
            t;
        }
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ b & (c ^ d))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[0 as i32 as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ a & (b ^ c))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[1 as i32 as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ e & (a ^ b))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[2 as i32 as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ d & (e ^ a))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[3 as i32 as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ c & (d ^ e))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[4 as i32 as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ b & (c ^ d))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[5 as i32 as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ a & (b ^ c))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[6 as i32 as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ e & (a ^ b))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[7 as i32 as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ d & (e ^ a))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[8 as i32 as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ c & (d ^ e))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[9 as i32 as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ b & (c ^ d))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[10 as i32 as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ a & (b ^ c))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[11 as i32 as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ e & (a ^ b))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[12 as i32 as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ d & (e ^ a))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[13 as i32 as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ c & (d ^ e))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[14 as i32 as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ b & (c ^ d))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[15 as i32 as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(16 as i32 & 0xf as i32) as usize]
            ^ x[(16 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(16 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(16 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(16 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ a & (b ^ c))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[(16 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(17 as i32 & 0xf as i32) as usize]
            ^ x[(17 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(17 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(17 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(17 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ e & (a ^ b))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[(17 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(18 as i32 & 0xf as i32) as usize]
            ^ x[(18 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(18 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(18 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(18 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ d & (e ^ a))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[(18 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(19 as i32 & 0xf as i32) as usize]
            ^ x[(19 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(19 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(19 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(19 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ c & (d ^ e))
                    .wrapping_add(0x5a827999 as i32 as u32)
                    .wrapping_add(x[(19 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(20 as i32 & 0xf as i32) as usize]
            ^ x[(20 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(20 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(20 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(20 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(20 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(21 as i32 & 0xf as i32) as usize]
            ^ x[(21 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(21 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(21 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(21 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(21 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(22 as i32 & 0xf as i32) as usize]
            ^ x[(22 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(22 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(22 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(22 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(22 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(23 as i32 & 0xf as i32) as usize]
            ^ x[(23 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(23 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(23 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(23 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(23 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(24 as i32 & 0xf as i32) as usize]
            ^ x[(24 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(24 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(24 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(24 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(24 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(25 as i32 & 0xf as i32) as usize]
            ^ x[(25 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(25 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(25 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(25 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(25 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(26 as i32 & 0xf as i32) as usize]
            ^ x[(26 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(26 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(26 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(26 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(26 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(27 as i32 & 0xf as i32) as usize]
            ^ x[(27 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(27 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(27 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(27 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(27 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(28 as i32 & 0xf as i32) as usize]
            ^ x[(28 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(28 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(28 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(28 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(28 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(29 as i32 & 0xf as i32) as usize]
            ^ x[(29 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(29 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(29 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(29 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(29 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(30 as i32 & 0xf as i32) as usize]
            ^ x[(30 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(30 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(30 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(30 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(30 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(31 as i32 & 0xf as i32) as usize]
            ^ x[(31 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(31 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(31 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(31 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(31 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(32 as i32 & 0xf as i32) as usize]
            ^ x[(32 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(32 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(32 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(32 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(32 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(33 as i32 & 0xf as i32) as usize]
            ^ x[(33 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(33 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(33 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(33 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(33 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(34 as i32 & 0xf as i32) as usize]
            ^ x[(34 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(34 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(34 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(34 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(34 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(35 as i32 & 0xf as i32) as usize]
            ^ x[(35 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(35 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(35 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(35 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(35 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(36 as i32 & 0xf as i32) as usize]
            ^ x[(36 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(36 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(36 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(36 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(36 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(37 as i32 & 0xf as i32) as usize]
            ^ x[(37 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(37 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(37 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(37 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(37 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(38 as i32 & 0xf as i32) as usize]
            ^ x[(38 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(38 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(38 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(38 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(38 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(39 as i32 & 0xf as i32) as usize]
            ^ x[(39 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(39 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(39 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(39 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0x6ed9eba1 as i32 as u32)
                    .wrapping_add(x[(39 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(40 as i32 & 0xf as i32) as usize]
            ^ x[(40 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(40 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(40 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(40 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b & c | d & (b | c))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(40 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(41 as i32 & 0xf as i32) as usize]
            ^ x[(41 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(41 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(41 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(41 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a & b | c & (a | b))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(41 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(42 as i32 & 0xf as i32) as usize]
            ^ x[(42 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(42 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(42 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(42 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e & a | b & (e | a))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(42 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(43 as i32 & 0xf as i32) as usize]
            ^ x[(43 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(43 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(43 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(43 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d & e | a & (d | e))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(43 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(44 as i32 & 0xf as i32) as usize]
            ^ x[(44 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(44 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(44 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(44 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c & d | e & (c | d))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(44 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(45 as i32 & 0xf as i32) as usize]
            ^ x[(45 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(45 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(45 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(45 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b & c | d & (b | c))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(45 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(46 as i32 & 0xf as i32) as usize]
            ^ x[(46 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(46 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(46 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(46 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a & b | c & (a | b))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(46 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(47 as i32 & 0xf as i32) as usize]
            ^ x[(47 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(47 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(47 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(47 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e & a | b & (e | a))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(47 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(48 as i32 & 0xf as i32) as usize]
            ^ x[(48 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(48 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(48 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(48 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d & e | a & (d | e))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(48 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(49 as i32 & 0xf as i32) as usize]
            ^ x[(49 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(49 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(49 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(49 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c & d | e & (c | d))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(49 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(50 as i32 & 0xf as i32) as usize]
            ^ x[(50 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(50 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(50 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(50 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b & c | d & (b | c))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(50 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(51 as i32 & 0xf as i32) as usize]
            ^ x[(51 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(51 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(51 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(51 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a & b | c & (a | b))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(51 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(52 as i32 & 0xf as i32) as usize]
            ^ x[(52 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(52 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(52 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(52 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e & a | b & (e | a))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(52 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(53 as i32 & 0xf as i32) as usize]
            ^ x[(53 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(53 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(53 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(53 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d & e | a & (d | e))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(53 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(54 as i32 & 0xf as i32) as usize]
            ^ x[(54 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(54 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(54 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(54 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c & d | e & (c | d))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(54 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(55 as i32 & 0xf as i32) as usize]
            ^ x[(55 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(55 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(55 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(55 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b & c | d & (b | c))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(55 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(56 as i32 & 0xf as i32) as usize]
            ^ x[(56 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(56 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(56 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(56 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a & b | c & (a | b))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(56 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(57 as i32 & 0xf as i32) as usize]
            ^ x[(57 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(57 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(57 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(57 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e & a | b & (e | a))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(57 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(58 as i32 & 0xf as i32) as usize]
            ^ x[(58 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(58 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(58 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(58 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d & e | a & (d | e))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(58 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(59 as i32 & 0xf as i32) as usize]
            ^ x[(59 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(59 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(59 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(59 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c & d | e & (c | d))
                    .wrapping_add(0x8f1bbcdc as u32)
                    .wrapping_add(x[(59 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(60 as i32 & 0xf as i32) as usize]
            ^ x[(60 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(60 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(60 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(60 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(60 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(61 as i32 & 0xf as i32) as usize]
            ^ x[(61 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(61 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(61 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(61 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(61 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(62 as i32 & 0xf as i32) as usize]
            ^ x[(62 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(62 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(62 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(62 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(62 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(63 as i32 & 0xf as i32) as usize]
            ^ x[(63 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(63 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(63 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(63 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(63 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(64 as i32 & 0xf as i32) as usize]
            ^ x[(64 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(64 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(64 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(64 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(64 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(65 as i32 & 0xf as i32) as usize]
            ^ x[(65 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(65 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(65 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(65 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(65 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(66 as i32 & 0xf as i32) as usize]
            ^ x[(66 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(66 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(66 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(66 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(66 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(67 as i32 & 0xf as i32) as usize]
            ^ x[(67 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(67 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(67 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(67 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(67 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(68 as i32 & 0xf as i32) as usize]
            ^ x[(68 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(68 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(68 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(68 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(68 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(69 as i32 & 0xf as i32) as usize]
            ^ x[(69 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(69 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(69 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(69 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(69 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(70 as i32 & 0xf as i32) as usize]
            ^ x[(70 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(70 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(70 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(70 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(70 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(71 as i32 & 0xf as i32) as usize]
            ^ x[(71 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(71 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(71 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(71 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(71 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(72 as i32 & 0xf as i32) as usize]
            ^ x[(72 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(72 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(72 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(72 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(72 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(73 as i32 & 0xf as i32) as usize]
            ^ x[(73 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(73 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(73 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(73 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(73 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(74 as i32 & 0xf as i32) as usize]
            ^ x[(74 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(74 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(74 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(74 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(74 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        tm = x[(75 as i32 & 0xf as i32) as usize]
            ^ x[(75 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(75 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(75 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(75 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        e = (e as u32)
            .wrapping_add(
                (a << 5 as i32 | a >> 32 as i32 - 5 as i32)
                    .wrapping_add(b ^ c ^ d)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(75 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        b = b << 30 as i32 | b >> 32 as i32 - 30 as i32;
        tm = x[(76 as i32 & 0xf as i32) as usize]
            ^ x[(76 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(76 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(76 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(76 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        d = (d as u32)
            .wrapping_add(
                (e << 5 as i32 | e >> 32 as i32 - 5 as i32)
                    .wrapping_add(a ^ b ^ c)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(76 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        a = a << 30 as i32 | a >> 32 as i32 - 30 as i32;
        tm = x[(77 as i32 & 0xf as i32) as usize]
            ^ x[(77 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(77 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(77 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(77 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        c = (c as u32)
            .wrapping_add(
                (d << 5 as i32 | d >> 32 as i32 - 5 as i32)
                    .wrapping_add(e ^ a ^ b)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(77 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        e = e << 30 as i32 | e >> 32 as i32 - 30 as i32;
        tm = x[(78 as i32 & 0xf as i32) as usize]
            ^ x[(78 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(78 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(78 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(78 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        b = (b as u32)
            .wrapping_add(
                (c << 5 as i32 | c >> 32 as i32 - 5 as i32)
                    .wrapping_add(d ^ e ^ a)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(78 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        d = d << 30 as i32 | d >> 32 as i32 - 30 as i32;
        tm = x[(79 as i32 & 0xf as i32) as usize]
            ^ x[(79 as i32 - 14 as i32 & 0xf as i32) as usize]
            ^ x[(79 as i32 - 8 as i32 & 0xf as i32) as usize]
            ^ x[(79 as i32 - 3 as i32 & 0xf as i32) as usize];
        x[(79 as i32 & 0xf as i32) as usize] = tm << 1 as i32
            | tm >> 32 as i32 - 1 as i32;
        a = (a as u32)
            .wrapping_add(
                (b << 5 as i32 | b >> 32 as i32 - 5 as i32)
                    .wrapping_add(c ^ d ^ e)
                    .wrapping_add(0xca62c1d6 as u32)
                    .wrapping_add(x[(79 as i32 & 0xf as i32) as usize]),
            ) as uint32_t as uint32_t;
        c = c << 30 as i32 | c >> 32 as i32 - 30 as i32;
        (*ctx).A = ((*ctx).A as u32).wrapping_add(a) as uint32_t as uint32_t;
        a = (*ctx).A;
        (*ctx).B = ((*ctx).B as u32).wrapping_add(b) as uint32_t as uint32_t;
        b = (*ctx).B;
        (*ctx).C = ((*ctx).C as u32).wrapping_add(c) as uint32_t as uint32_t;
        c = (*ctx).C;
        (*ctx).D = ((*ctx).D as u32).wrapping_add(d) as uint32_t as uint32_t;
        d = (*ctx).D;
        (*ctx).E = ((*ctx).E as u32).wrapping_add(e) as uint32_t as uint32_t;
        e = (*ctx).E;
    }
}