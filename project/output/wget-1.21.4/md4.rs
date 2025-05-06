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
}
pub type size_t = u64;
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type uintptr_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md4_ctx {
    pub A: uint32_t,
    pub B: uint32_t,
    pub C: uint32_t,
    pub D: uint32_t,
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
pub unsafe extern "C" fn md4_init_ctx(mut ctx: *mut md4_ctx) {
    (*ctx).A = 0x67452301 as i32 as uint32_t;
    (*ctx).B = 0xefcdab89 as u32;
    (*ctx).C = 0x98badcfe as u32;
    (*ctx).D = 0x10325476 as i32 as uint32_t;
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
pub unsafe extern "C" fn md4_read_ctx(
    mut ctx: *const md4_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut r: *mut i8 = resbuf as *mut i8;
    set_uint32(
        r
            .offset(
                (0 as i32 as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64)
                    as isize,
            ),
        (*ctx).A,
    );
    set_uint32(
        r
            .offset(
                (1 as i32 as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64)
                    as isize,
            ),
        (*ctx).B,
    );
    set_uint32(
        r
            .offset(
                (2 as i32 as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64)
                    as isize,
            ),
        (*ctx).C,
    );
    set_uint32(
        r
            .offset(
                (3 as i32 as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64)
                    as isize,
            ),
        (*ctx).D,
    );
    return resbuf;
}
#[no_mangle]
pub unsafe extern "C" fn md4_finish_ctx(
    mut ctx: *mut md4_ctx,
    mut resbuf: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut bytes: uint32_t = (*ctx).buflen;
    let mut pad: size_t = 0;
    (*ctx).total[0 as i32 as usize] = ((*ctx).total[0 as i32 as usize] as u32)
        .wrapping_add(bytes) as uint32_t as uint32_t;
    if (*ctx).total[0 as i32 as usize] < bytes {
        (*ctx).total[1 as i32 as usize] = ((*ctx).total[1 as i32 as usize])
            .wrapping_add(1);
        (*ctx).total[1 as i32 as usize];
    }
    pad = (if bytes >= 56 as i32 as u32 {
        ((64 as i32 + 56 as i32) as u32).wrapping_sub(bytes)
    } else {
        (56 as i32 as u32).wrapping_sub(bytes)
    }) as size_t;
    memcpy(
        &mut *(((*ctx).buffer).as_mut_ptr() as *mut i8).offset(bytes as isize) as *mut i8
            as *mut libc::c_void,
        fillbuf.as_ptr() as *const libc::c_void,
        pad,
    );
    (*ctx)
        .buffer[(bytes as u64).wrapping_add(pad).wrapping_div(4 as i32 as u64)
        as usize] = (*ctx).total[0 as i32 as usize] << 3 as i32;
    (*ctx)
        .buffer[(bytes as u64)
        .wrapping_add(pad)
        .wrapping_div(4 as i32 as u64)
        .wrapping_add(1 as i32 as u64) as usize] = (*ctx).total[1 as i32 as usize]
        << 3 as i32 | (*ctx).total[0 as i32 as usize] >> 29 as i32;
    md4_process_block(
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        (bytes as u64).wrapping_add(pad).wrapping_add(8 as i32 as u64),
        ctx,
    );
    return md4_read_ctx(ctx, resbuf);
}
#[no_mangle]
pub unsafe extern "C" fn md4_buffer(
    mut buffer: *const i8,
    mut len: size_t,
    mut resblock: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut ctx: md4_ctx = md4_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    md4_init_ctx(&mut ctx);
    md4_process_bytes(buffer as *const libc::c_void, len, &mut ctx);
    return md4_finish_ctx(&mut ctx, resblock);
}
#[no_mangle]
pub unsafe extern "C" fn md4_process_bytes(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md4_ctx,
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
            md4_process_block(
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
                md4_process_block(
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
            md4_process_block(buffer, len & !(63 as i32) as u64, ctx);
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
            md4_process_block(
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
pub unsafe extern "C" fn md4_process_block(
    mut buffer: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md4_ctx,
) {
    let mut words: *const uint32_t = buffer as *const uint32_t;
    let mut nwords: size_t = len.wrapping_div(::core::mem::size_of::<uint32_t>() as u64);
    let mut endp: *const uint32_t = words.offset(nwords as isize);
    let mut x: [uint32_t; 16] = [0; 16];
    let mut A: uint32_t = (*ctx).A;
    let mut B: uint32_t = (*ctx).B;
    let mut C: uint32_t = (*ctx).C;
    let mut D: uint32_t = (*ctx).D;
    let mut lolen: uint32_t = len as uint32_t;
    (*ctx).total[0 as i32 as usize] = ((*ctx).total[0 as i32 as usize] as u32)
        .wrapping_add(lolen) as uint32_t as uint32_t;
    (*ctx).total[1 as i32 as usize] = ((*ctx).total[1 as i32 as usize] as u64)
        .wrapping_add(
            (len >> 31 as i32 >> 1 as i32)
                .wrapping_add(((*ctx).total[0 as i32 as usize] < lolen) as i32 as u64),
        ) as uint32_t as uint32_t;
    while words < endp {
        let mut t: i32 = 0;
        t = 0 as i32;
        while t < 16 as i32 {
            x[t as usize] = *words;
            words = words.offset(1);
            words;
            t += 1;
            t;
        }
        A = A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[0 as i32 as usize])
            << 3 as i32
            | A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[0 as i32 as usize])
                >> 32 as i32 - 3 as i32;
        D = D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[1 as i32 as usize])
            << 7 as i32
            | D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[1 as i32 as usize])
                >> 32 as i32 - 7 as i32;
        C = C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[2 as i32 as usize])
            << 11 as i32
            | C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[2 as i32 as usize])
                >> 32 as i32 - 11 as i32;
        B = B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[3 as i32 as usize])
            << 19 as i32
            | B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[3 as i32 as usize])
                >> 32 as i32 - 19 as i32;
        A = A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[4 as i32 as usize])
            << 3 as i32
            | A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[4 as i32 as usize])
                >> 32 as i32 - 3 as i32;
        D = D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[5 as i32 as usize])
            << 7 as i32
            | D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[5 as i32 as usize])
                >> 32 as i32 - 7 as i32;
        C = C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[6 as i32 as usize])
            << 11 as i32
            | C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[6 as i32 as usize])
                >> 32 as i32 - 11 as i32;
        B = B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[7 as i32 as usize])
            << 19 as i32
            | B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[7 as i32 as usize])
                >> 32 as i32 - 19 as i32;
        A = A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[8 as i32 as usize])
            << 3 as i32
            | A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[8 as i32 as usize])
                >> 32 as i32 - 3 as i32;
        D = D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[9 as i32 as usize])
            << 7 as i32
            | D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[9 as i32 as usize])
                >> 32 as i32 - 7 as i32;
        C = C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[10 as i32 as usize])
            << 11 as i32
            | C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[10 as i32 as usize])
                >> 32 as i32 - 11 as i32;
        B = B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[11 as i32 as usize])
            << 19 as i32
            | B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[11 as i32 as usize])
                >> 32 as i32 - 19 as i32;
        A = A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[12 as i32 as usize])
            << 3 as i32
            | A.wrapping_add(D ^ B & (C ^ D)).wrapping_add(x[12 as i32 as usize])
                >> 32 as i32 - 3 as i32;
        D = D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[13 as i32 as usize])
            << 7 as i32
            | D.wrapping_add(C ^ A & (B ^ C)).wrapping_add(x[13 as i32 as usize])
                >> 32 as i32 - 7 as i32;
        C = C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[14 as i32 as usize])
            << 11 as i32
            | C.wrapping_add(B ^ D & (A ^ B)).wrapping_add(x[14 as i32 as usize])
                >> 32 as i32 - 11 as i32;
        B = B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[15 as i32 as usize])
            << 19 as i32
            | B.wrapping_add(A ^ C & (D ^ A)).wrapping_add(x[15 as i32 as usize])
                >> 32 as i32 - 19 as i32;
        A = A
            .wrapping_add(B & C | B & D | C & D)
            .wrapping_add(x[0 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 3 as i32
            | A
                .wrapping_add(B & C | B & D | C & D)
                .wrapping_add(x[0 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 3 as i32;
        D = D
            .wrapping_add(A & B | A & C | B & C)
            .wrapping_add(x[4 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 5 as i32
            | D
                .wrapping_add(A & B | A & C | B & C)
                .wrapping_add(x[4 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 5 as i32;
        C = C
            .wrapping_add(D & A | D & B | A & B)
            .wrapping_add(x[8 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 9 as i32
            | C
                .wrapping_add(D & A | D & B | A & B)
                .wrapping_add(x[8 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 9 as i32;
        B = B
            .wrapping_add(C & D | C & A | D & A)
            .wrapping_add(x[12 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 13 as i32
            | B
                .wrapping_add(C & D | C & A | D & A)
                .wrapping_add(x[12 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 13 as i32;
        A = A
            .wrapping_add(B & C | B & D | C & D)
            .wrapping_add(x[1 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 3 as i32
            | A
                .wrapping_add(B & C | B & D | C & D)
                .wrapping_add(x[1 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 3 as i32;
        D = D
            .wrapping_add(A & B | A & C | B & C)
            .wrapping_add(x[5 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 5 as i32
            | D
                .wrapping_add(A & B | A & C | B & C)
                .wrapping_add(x[5 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 5 as i32;
        C = C
            .wrapping_add(D & A | D & B | A & B)
            .wrapping_add(x[9 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 9 as i32
            | C
                .wrapping_add(D & A | D & B | A & B)
                .wrapping_add(x[9 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 9 as i32;
        B = B
            .wrapping_add(C & D | C & A | D & A)
            .wrapping_add(x[13 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 13 as i32
            | B
                .wrapping_add(C & D | C & A | D & A)
                .wrapping_add(x[13 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 13 as i32;
        A = A
            .wrapping_add(B & C | B & D | C & D)
            .wrapping_add(x[2 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 3 as i32
            | A
                .wrapping_add(B & C | B & D | C & D)
                .wrapping_add(x[2 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 3 as i32;
        D = D
            .wrapping_add(A & B | A & C | B & C)
            .wrapping_add(x[6 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 5 as i32
            | D
                .wrapping_add(A & B | A & C | B & C)
                .wrapping_add(x[6 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 5 as i32;
        C = C
            .wrapping_add(D & A | D & B | A & B)
            .wrapping_add(x[10 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 9 as i32
            | C
                .wrapping_add(D & A | D & B | A & B)
                .wrapping_add(x[10 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 9 as i32;
        B = B
            .wrapping_add(C & D | C & A | D & A)
            .wrapping_add(x[14 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 13 as i32
            | B
                .wrapping_add(C & D | C & A | D & A)
                .wrapping_add(x[14 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 13 as i32;
        A = A
            .wrapping_add(B & C | B & D | C & D)
            .wrapping_add(x[3 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 3 as i32
            | A
                .wrapping_add(B & C | B & D | C & D)
                .wrapping_add(x[3 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 3 as i32;
        D = D
            .wrapping_add(A & B | A & C | B & C)
            .wrapping_add(x[7 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 5 as i32
            | D
                .wrapping_add(A & B | A & C | B & C)
                .wrapping_add(x[7 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 5 as i32;
        C = C
            .wrapping_add(D & A | D & B | A & B)
            .wrapping_add(x[11 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 9 as i32
            | C
                .wrapping_add(D & A | D & B | A & B)
                .wrapping_add(x[11 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 9 as i32;
        B = B
            .wrapping_add(C & D | C & A | D & A)
            .wrapping_add(x[15 as i32 as usize])
            .wrapping_add(0x5a827999 as i32 as u32) << 13 as i32
            | B
                .wrapping_add(C & D | C & A | D & A)
                .wrapping_add(x[15 as i32 as usize])
                .wrapping_add(0x5a827999 as i32 as u32) >> 32 as i32 - 13 as i32;
        A = A
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(x[0 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 3 as i32
            | A
                .wrapping_add(B ^ C ^ D)
                .wrapping_add(x[0 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 3 as i32;
        D = D
            .wrapping_add(A ^ B ^ C)
            .wrapping_add(x[8 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 9 as i32
            | D
                .wrapping_add(A ^ B ^ C)
                .wrapping_add(x[8 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 9 as i32;
        C = C
            .wrapping_add(D ^ A ^ B)
            .wrapping_add(x[4 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 11 as i32
            | C
                .wrapping_add(D ^ A ^ B)
                .wrapping_add(x[4 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 11 as i32;
        B = B
            .wrapping_add(C ^ D ^ A)
            .wrapping_add(x[12 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 15 as i32
            | B
                .wrapping_add(C ^ D ^ A)
                .wrapping_add(x[12 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 15 as i32;
        A = A
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(x[2 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 3 as i32
            | A
                .wrapping_add(B ^ C ^ D)
                .wrapping_add(x[2 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 3 as i32;
        D = D
            .wrapping_add(A ^ B ^ C)
            .wrapping_add(x[10 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 9 as i32
            | D
                .wrapping_add(A ^ B ^ C)
                .wrapping_add(x[10 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 9 as i32;
        C = C
            .wrapping_add(D ^ A ^ B)
            .wrapping_add(x[6 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 11 as i32
            | C
                .wrapping_add(D ^ A ^ B)
                .wrapping_add(x[6 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 11 as i32;
        B = B
            .wrapping_add(C ^ D ^ A)
            .wrapping_add(x[14 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 15 as i32
            | B
                .wrapping_add(C ^ D ^ A)
                .wrapping_add(x[14 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 15 as i32;
        A = A
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(x[1 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 3 as i32
            | A
                .wrapping_add(B ^ C ^ D)
                .wrapping_add(x[1 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 3 as i32;
        D = D
            .wrapping_add(A ^ B ^ C)
            .wrapping_add(x[9 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 9 as i32
            | D
                .wrapping_add(A ^ B ^ C)
                .wrapping_add(x[9 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 9 as i32;
        C = C
            .wrapping_add(D ^ A ^ B)
            .wrapping_add(x[5 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 11 as i32
            | C
                .wrapping_add(D ^ A ^ B)
                .wrapping_add(x[5 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 11 as i32;
        B = B
            .wrapping_add(C ^ D ^ A)
            .wrapping_add(x[13 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 15 as i32
            | B
                .wrapping_add(C ^ D ^ A)
                .wrapping_add(x[13 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 15 as i32;
        A = A
            .wrapping_add(B ^ C ^ D)
            .wrapping_add(x[3 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 3 as i32
            | A
                .wrapping_add(B ^ C ^ D)
                .wrapping_add(x[3 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 3 as i32;
        D = D
            .wrapping_add(A ^ B ^ C)
            .wrapping_add(x[11 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 9 as i32
            | D
                .wrapping_add(A ^ B ^ C)
                .wrapping_add(x[11 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 9 as i32;
        C = C
            .wrapping_add(D ^ A ^ B)
            .wrapping_add(x[7 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 11 as i32
            | C
                .wrapping_add(D ^ A ^ B)
                .wrapping_add(x[7 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 11 as i32;
        B = B
            .wrapping_add(C ^ D ^ A)
            .wrapping_add(x[15 as i32 as usize])
            .wrapping_add(0x6ed9eba1 as i32 as u32) << 15 as i32
            | B
                .wrapping_add(C ^ D ^ A)
                .wrapping_add(x[15 as i32 as usize])
                .wrapping_add(0x6ed9eba1 as i32 as u32) >> 32 as i32 - 15 as i32;
        (*ctx).A = ((*ctx).A as u32).wrapping_add(A) as uint32_t as uint32_t;
        A = (*ctx).A;
        (*ctx).B = ((*ctx).B as u32).wrapping_add(B) as uint32_t as uint32_t;
        B = (*ctx).B;
        (*ctx).C = ((*ctx).C as u32).wrapping_add(C) as uint32_t as uint32_t;
        C = (*ctx).C;
        (*ctx).D = ((*ctx).D as u32).wrapping_add(D) as uint32_t as uint32_t;
        D = (*ctx).D;
    }
}