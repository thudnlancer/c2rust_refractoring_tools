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
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block8 {
    pub b: [uint8_t; 8],
    pub u64_0: uint64_t,
}
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac64_key {
    pub K1: nettle_block8,
    pub K2: nettle_block8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac64_ctx {
    pub X: nettle_block8,
    pub block: nettle_block8,
    pub index: size_t,
}
#[inline]
unsafe extern "C" fn block8_xor(mut r: *mut nettle_block8, mut x: *const nettle_block8) {
    (*r).u64_0 ^= (*x).u64_0;
}
#[inline]
unsafe extern "C" fn block8_xor3(
    mut r: *mut nettle_block8,
    mut x: *const nettle_block8,
    mut y: *const nettle_block8,
) {
    (*r).u64_0 = (*x).u64_0 ^ (*y).u64_0;
}
#[inline]
unsafe extern "C" fn block8_xor_bytes(
    mut r: *mut nettle_block8,
    mut x: *const nettle_block8,
    mut bytes: *const uint8_t,
) {
    nettle_memxor3(
        ((*r).b).as_mut_ptr() as *mut libc::c_void,
        ((*x).b).as_ptr() as *const libc::c_void,
        bytes as *const libc::c_void,
        8 as i32 as size_t,
    );
}
#[inline]
unsafe extern "C" fn block8_mulx_be(
    mut dst: *mut nettle_block8,
    mut src: *const nettle_block8,
) {
    let mut carry: uint64_t = ((*src).u64_0 & 0x80 as i32 as u64) >> 7 as i32;
    (*dst).u64_0 = (((*src).u64_0 & 0x7f7f7f7f7f7f7f7f as u64) << 1 as i32
        | ((*src).u64_0 & 0x8080808080808080 as u64) >> 15 as i32)
        ^ (0x1b as u64) << 56 as i32 & carry.wrapping_neg();
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cmac64_set_key(
    mut key: *mut cmac64_key,
    mut cipher: *const libc::c_void,
    mut encrypt: Option<nettle_cipher_func>,
) {
    static mut zero_block: nettle_block8 = nettle_block8 { b: [0; 8] };
    let mut L: nettle_block8 = nettle_block8 { b: [0; 8] };
    encrypt
        .expect(
            "non-null function pointer",
        )(cipher, 8 as i32 as size_t, (L.b).as_mut_ptr(), (zero_block.b).as_ptr());
    block8_mulx_be(&mut (*key).K1, &mut L);
    block8_mulx_be(&mut (*key).K2, &mut (*key).K1);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cmac64_init(mut ctx: *mut cmac64_ctx) {
    memset(
        &mut (*ctx).X as *mut nettle_block8 as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<nettle_block8>() as u64,
    );
    (*ctx).index = 0 as i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cmac64_update(
    mut ctx: *mut cmac64_ctx,
    mut cipher: *const libc::c_void,
    mut encrypt: Option<nettle_cipher_func>,
    mut msg_len: size_t,
    mut msg: *const uint8_t,
) {
    let mut Y: nettle_block8 = nettle_block8 { b: [0; 8] };
    if (*ctx).index < 8 as i32 as u64 {
        let mut len: size_t = if (8 as i32 as u64).wrapping_sub((*ctx).index) < msg_len {
            (8 as i32 as u64).wrapping_sub((*ctx).index)
        } else {
            msg_len
        };
        memcpy(
            &mut *((*ctx).block.b).as_mut_ptr().offset((*ctx).index as isize)
                as *mut uint8_t as *mut libc::c_void,
            msg as *const libc::c_void,
            len,
        );
        msg = msg.offset(len as isize);
        msg_len = (msg_len as u64).wrapping_sub(len) as size_t as size_t;
        (*ctx).index = ((*ctx).index as u64).wrapping_add(len) as size_t as size_t;
    }
    if msg_len == 0 as i32 as u64 {
        return;
    }
    block8_xor3(&mut Y, &mut (*ctx).X, &mut (*ctx).block);
    encrypt
        .expect(
            "non-null function pointer",
        )(cipher, 8 as i32 as size_t, ((*ctx).X.b).as_mut_ptr(), (Y.b).as_mut_ptr());
    while msg_len > 8 as i32 as u64 {
        block8_xor_bytes(&mut Y, &mut (*ctx).X, msg);
        encrypt
            .expect(
                "non-null function pointer",
            )(cipher, 8 as i32 as size_t, ((*ctx).X.b).as_mut_ptr(), (Y.b).as_mut_ptr());
        msg = msg.offset(8 as i32 as isize);
        msg_len = (msg_len as u64).wrapping_sub(8 as i32 as u64) as size_t as size_t;
    }
    memcpy(
        ((*ctx).block.b).as_mut_ptr() as *mut libc::c_void,
        msg as *const libc::c_void,
        msg_len,
    );
    (*ctx).index = msg_len;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cmac64_digest(
    mut ctx: *mut cmac64_ctx,
    mut key: *const cmac64_key,
    mut cipher: *const libc::c_void,
    mut encrypt: Option<nettle_cipher_func>,
    mut length: u32,
    mut dst: *mut uint8_t,
) {
    let mut Y: nettle_block8 = nettle_block8 { b: [0; 8] };
    memset(
        ((*ctx).block.b).as_mut_ptr().offset((*ctx).index as isize) as *mut libc::c_void,
        0 as i32,
        (::core::mem::size_of::<[uint8_t; 8]>() as u64).wrapping_sub((*ctx).index),
    );
    if (*ctx).index < 8 as i32 as u64 {
        (*ctx).block.b[(*ctx).index as usize] = 0x80 as i32 as uint8_t;
        block8_xor(&mut (*ctx).block, &(*key).K2);
    } else {
        block8_xor(&mut (*ctx).block, &(*key).K1);
    }
    block8_xor3(&mut Y, &mut (*ctx).block, &mut (*ctx).X);
    if length <= 8 as i32 as u32 {} else {
        __assert_fail(
            b"length <= 8\0" as *const u8 as *const i8,
            b"cmac64.c\0" as *const u8 as *const i8,
            140 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 135],
                &[i8; 135],
            >(
                b"void nettle_cmac64_digest(struct cmac64_ctx *, const struct cmac64_key *, const void *, nettle_cipher_func *, unsigned int, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2188: {
        if length <= 8 as i32 as u32 {} else {
            __assert_fail(
                b"length <= 8\0" as *const u8 as *const i8,
                b"cmac64.c\0" as *const u8 as *const i8,
                140 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 135],
                    &[i8; 135],
                >(
                    b"void nettle_cmac64_digest(struct cmac64_ctx *, const struct cmac64_key *, const void *, nettle_cipher_func *, unsigned int, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length == 8 as i32 as u32 {
        encrypt
            .expect(
                "non-null function pointer",
            )(cipher, 8 as i32 as size_t, dst, (Y.b).as_mut_ptr());
    } else {
        encrypt
            .expect(
                "non-null function pointer",
            )(
            cipher,
            8 as i32 as size_t,
            ((*ctx).block.b).as_mut_ptr(),
            (Y.b).as_mut_ptr(),
        );
        memcpy(
            dst as *mut libc::c_void,
            ((*ctx).block.b).as_mut_ptr() as *const libc::c_void,
            length as u64,
        );
    }
    memset(
        &mut (*ctx).X as *mut nettle_block8 as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<nettle_block8>() as u64,
    );
    (*ctx).index = 0 as i32 as size_t;
}