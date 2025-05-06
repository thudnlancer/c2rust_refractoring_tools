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
    fn nettle_sha3_permute(state: *mut sha3_state);
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
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
pub struct sha3_state {
    pub a: [uint64_t; 25],
}
unsafe extern "C" fn sha3_absorb(
    mut state: *mut sha3_state,
    mut length: u32,
    mut data: *const uint8_t,
) {
    nettle_memxor(
        ((*state).a).as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        length as size_t,
    );
    nettle_sha3_permute(state);
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_sha3_update(
    mut state: *mut sha3_state,
    mut block_size: u32,
    mut block: *mut uint8_t,
    mut pos: u32,
    mut length: size_t,
    mut data: *const uint8_t,
) -> u32 {
    if pos < block_size {} else {
        __assert_fail(
            b"pos < block_size\0" as *const u8 as *const i8,
            b"sha3.c\0" as *const u8 as *const i8,
            76 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 118],
                &[i8; 118],
            >(
                b"unsigned int _nettle_sha3_update(struct sha3_state *, unsigned int, uint8_t *, unsigned int, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_796: {
        if pos < block_size {} else {
            __assert_fail(
                b"pos < block_size\0" as *const u8 as *const i8,
                b"sha3.c\0" as *const u8 as *const i8,
                76 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 118],
                    &[i8; 118],
                >(
                    b"unsigned int _nettle_sha3_update(struct sha3_state *, unsigned int, uint8_t *, unsigned int, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if length == 0 {
        return pos;
    }
    if pos > 0 as i32 as u32 {
        let mut __md_left: u32 = block_size.wrapping_sub(pos);
        if length < __md_left as u64 {
            memcpy(
                block.offset(pos as isize) as *mut libc::c_void,
                data as *const libc::c_void,
                length,
            );
            return (pos as u64).wrapping_add(length) as u32;
        }
        memcpy(
            block.offset(pos as isize) as *mut libc::c_void,
            data as *const libc::c_void,
            __md_left as u64,
        );
        data = data.offset(__md_left as isize);
        length = (length as u64).wrapping_sub(__md_left as u64) as size_t as size_t;
        sha3_absorb(state, block_size, block);
    }
    while length >= block_size as u64 {
        sha3_absorb(state, block_size, data);
        length = (length as u64).wrapping_sub(block_size as u64) as size_t as size_t;
        data = data.offset(block_size as isize);
    }
    memcpy(block as *mut libc::c_void, data as *const libc::c_void, length);
    return length as u32;
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_sha3_pad(
    mut state: *mut sha3_state,
    mut block_size: u32,
    mut block: *mut uint8_t,
    mut pos: u32,
    mut magic: uint8_t,
) {
    if pos < block_size {} else {
        __assert_fail(
            b"pos < block_size\0" as *const u8 as *const i8,
            b"sha3.c\0" as *const u8 as *const i8,
            97 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"void _nettle_sha3_pad(struct sha3_state *, unsigned int, uint8_t *, unsigned int, uint8_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_901: {
        if pos < block_size {} else {
            __assert_fail(
                b"pos < block_size\0" as *const u8 as *const i8,
                b"sha3.c\0" as *const u8 as *const i8,
                97 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"void _nettle_sha3_pad(struct sha3_state *, unsigned int, uint8_t *, unsigned int, uint8_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let fresh0 = pos;
    pos = pos.wrapping_add(1);
    *block.offset(fresh0 as isize) = magic;
    memset(
        block.offset(pos as isize) as *mut libc::c_void,
        0 as i32,
        block_size.wrapping_sub(pos) as u64,
    );
    let ref mut fresh1 = *block
        .offset(block_size.wrapping_sub(1 as i32 as u32) as isize);
    *fresh1 = (*fresh1 as i32 | 0x80 as i32) as uint8_t;
    nettle_memxor(
        ((*state).a).as_mut_ptr() as *mut libc::c_void,
        block as *const libc::c_void,
        block_size as size_t,
    );
}