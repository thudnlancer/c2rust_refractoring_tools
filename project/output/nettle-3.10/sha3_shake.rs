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
    fn nettle_sha3_permute(state: *mut sha3_state);
    fn _nettle_sha3_pad(
        state: *mut sha3_state,
        block_size: u32,
        block: *mut uint8_t,
        pos: u32,
        magic: uint8_t,
    );
    fn _nettle_write_le64(length: size_t, dst: *mut uint8_t, src: *const uint64_t);
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
#[no_mangle]
pub unsafe extern "C" fn _nettle_sha3_shake(
    mut state: *mut sha3_state,
    mut block_size: u32,
    mut block: *mut uint8_t,
    mut index: u32,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    _nettle_sha3_pad(state, block_size, block, index, 0x1f as i32 as uint8_t);
    while length > block_size as u64 {
        nettle_sha3_permute(state);
        _nettle_write_le64(block_size as size_t, dst, ((*state).a).as_mut_ptr());
        length = (length as u64).wrapping_sub(block_size as u64) as size_t as size_t;
        dst = dst.offset(block_size as isize);
    }
    nettle_sha3_permute(state);
    _nettle_write_le64(length, dst, ((*state).a).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_sha3_shake_output(
    mut state: *mut sha3_state,
    mut block_size: u32,
    mut block: *mut uint8_t,
    mut index: u32,
    mut length: size_t,
    mut dst: *mut uint8_t,
) -> u32 {
    let mut left: u32 = 0;
    if index < block_size {
        _nettle_sha3_pad(state, block_size, block, index, 0x1f as i32 as uint8_t);
        index = block_size;
    } else {
        index = !index;
    }
    if index <= block_size {} else {
        __assert_fail(
            b"index <= block_size\0" as *const u8 as *const i8,
            b"sha3-shake.c\0" as *const u8 as *const i8,
            86 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 118],
                &[i8; 118],
            >(
                b"unsigned int _nettle_sha3_shake_output(struct sha3_state *, unsigned int, uint8_t *, unsigned int, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_865: {
        if index <= block_size {} else {
            __assert_fail(
                b"index <= block_size\0" as *const u8 as *const i8,
                b"sha3-shake.c\0" as *const u8 as *const i8,
                86 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 118],
                    &[i8; 118],
                >(
                    b"unsigned int _nettle_sha3_shake_output(struct sha3_state *, unsigned int, uint8_t *, unsigned int, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    left = block_size.wrapping_sub(index);
    if length <= left as u64 {
        memcpy(
            dst as *mut libc::c_void,
            block.offset(index as isize) as *const libc::c_void,
            length,
        );
        return !(index as u64).wrapping_add(length) as u32;
    } else {
        memcpy(
            dst as *mut libc::c_void,
            block.offset(index as isize) as *const libc::c_void,
            left as u64,
        );
        length = (length as u64).wrapping_sub(left as u64) as size_t as size_t;
        dst = dst.offset(left as isize);
    }
    while length > block_size as u64 {
        nettle_sha3_permute(state);
        _nettle_write_le64(block_size as size_t, dst, ((*state).a).as_mut_ptr());
        length = (length as u64).wrapping_sub(block_size as u64) as size_t as size_t;
        dst = dst.offset(block_size as isize);
    }
    nettle_sha3_permute(state);
    _nettle_write_le64(block_size as size_t, block, ((*state).a).as_mut_ptr());
    memcpy(dst as *mut libc::c_void, block as *const libc::c_void, length);
    return !length as u32;
}