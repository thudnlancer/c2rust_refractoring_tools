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
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn nettle_memxor3(
        dst: *mut libc::c_void,
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type nettle_fill16_func = unsafe extern "C" fn(
    *mut uint8_t,
    size_t,
    *mut nettle_block16,
) -> ();
#[no_mangle]
pub unsafe extern "C" fn _nettle_ctr_crypt16(
    mut ctx: *const libc::c_void,
    mut f: Option<nettle_cipher_func>,
    mut fill: Option<nettle_fill16_func>,
    mut ctr: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if dst != src as *mut uint8_t
        && (dst as uintptr_t).wrapping_rem(::core::mem::size_of::<uint64_t>() as u64)
            == 0
    {
        let mut blocks: size_t = length.wrapping_div(16 as u32 as u64);
        let mut done: size_t = 0;
        fill
            .expect(
                "non-null function pointer",
            )(ctr, blocks, dst as *mut nettle_block16);
        done = blocks.wrapping_mul(16 as i32 as u64);
        f.expect("non-null function pointer")(ctx, done, dst, dst);
        nettle_memxor(dst as *mut libc::c_void, src as *const libc::c_void, done);
        length = (length as u64).wrapping_sub(done) as size_t as size_t;
        if length > 0 as i32 as u64 {
            let mut block: nettle_block16 = nettle_block16 { b: [0; 16] };
            dst = dst.offset(done as isize);
            src = src.offset(done as isize);
            if length < 16 as i32 as u64 {} else {
                __assert_fail(
                    b"length < 16\0" as *const u8 as *const i8,
                    b"ctr16.c\0" as *const u8 as *const i8,
                    71 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 130],
                        &[i8; 130],
                    >(
                        b"void _nettle_ctr_crypt16(const void *, nettle_cipher_func *, nettle_fill16_func *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_569: {
                if length < 16 as i32 as u64 {} else {
                    __assert_fail(
                        b"length < 16\0" as *const u8 as *const i8,
                        b"ctr16.c\0" as *const u8 as *const i8,
                        71 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 130],
                            &[i8; 130],
                        >(
                            b"void _nettle_ctr_crypt16(const void *, nettle_cipher_func *, nettle_fill16_func *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            fill
                .expect(
                    "non-null function pointer",
                )(ctr, 1 as i32 as size_t, &mut block);
            f
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                16 as i32 as size_t,
                (block.b).as_mut_ptr(),
                (block.b).as_mut_ptr(),
            );
            nettle_memxor3(
                dst as *mut libc::c_void,
                src as *const libc::c_void,
                (block.b).as_mut_ptr() as *const libc::c_void,
                length,
            );
        }
    } else {
        let mut current_block_22: u64;
        let mut buffer: *mut nettle_block16 = 0 as *mut nettle_block16;
        let mut blocks_0: size_t = length
            .wrapping_add(15 as i32 as u64)
            .wrapping_div(16 as u32 as u64);
        let mut i: size_t = 0;
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (::core::mem::size_of::<nettle_block16>() as u64)
                .wrapping_mul(
                    (if blocks_0 < (512 as i32 / 16 as i32) as u64 {
                        blocks_0
                    } else {
                        (512 as i32 / 16 as i32) as u64
                    }),
                ) as usize,
        );
        buffer = fresh0.as_mut_ptr() as *mut nettle_block16;
        i = 0 as i32 as size_t;
        loop {
            if !(blocks_0 >= (512 as i32 / 16 as i32) as u64) {
                current_block_22 = 5689001924483802034;
                break;
            }
            fill
                .expect(
                    "non-null function pointer",
                )(ctr, (512 as i32 / 16 as i32) as size_t, buffer);
            f
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                512 as i32 as size_t,
                ((*buffer).b).as_mut_ptr(),
                ((*buffer).b).as_mut_ptr(),
            );
            if length.wrapping_sub(i) < 512 as i32 as u64 {
                current_block_22 = 1648488492331626575;
                break;
            }
            nettle_memxor3(
                dst.offset(i as isize) as *mut libc::c_void,
                src.offset(i as isize) as *const libc::c_void,
                ((*buffer).b).as_mut_ptr() as *const libc::c_void,
                512 as i32 as size_t,
            );
            i = (i as u64).wrapping_add(512 as i32 as u64) as size_t as size_t;
            blocks_0 = (blocks_0 as u64).wrapping_sub((512 as i32 / 16 as i32) as u64)
                as size_t as size_t;
        }
        match current_block_22 {
            5689001924483802034 => {
                if blocks_0 > 0 as i32 as u64 {
                    if length.wrapping_sub(i) < 512 as i32 as u64 {} else {
                        __assert_fail(
                            b"length - i < CTR_BUFFER_LIMIT\0" as *const u8 as *const i8,
                            b"ctr16.c\0" as *const u8 as *const i8,
                            99 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 130],
                                &[i8; 130],
                            >(
                                b"void _nettle_ctr_crypt16(const void *, nettle_cipher_func *, nettle_fill16_func *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_334: {
                        if length.wrapping_sub(i) < 512 as i32 as u64 {} else {
                            __assert_fail(
                                b"length - i < CTR_BUFFER_LIMIT\0" as *const u8
                                    as *const i8,
                                b"ctr16.c\0" as *const u8 as *const i8,
                                99 as i32 as u32,
                                (*::core::mem::transmute::<
                                    &[u8; 130],
                                    &[i8; 130],
                                >(
                                    b"void _nettle_ctr_crypt16(const void *, nettle_cipher_func *, nettle_fill16_func *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    fill.expect("non-null function pointer")(ctr, blocks_0, buffer);
                    f
                        .expect(
                            "non-null function pointer",
                        )(
                        ctx,
                        blocks_0.wrapping_mul(16 as i32 as u64),
                        ((*buffer).b).as_mut_ptr(),
                        ((*buffer).b).as_mut_ptr(),
                    );
                    current_block_22 = 1648488492331626575;
                } else {
                    current_block_22 = 14576567515993809846;
                }
            }
            _ => {}
        }
        match current_block_22 {
            1648488492331626575 => {
                nettle_memxor3(
                    dst.offset(i as isize) as *mut libc::c_void,
                    src.offset(i as isize) as *const libc::c_void,
                    ((*buffer).b).as_mut_ptr() as *const libc::c_void,
                    length.wrapping_sub(i),
                );
            }
            _ => {}
        }
    };
}