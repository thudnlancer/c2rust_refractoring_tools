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
    fn nettle_sha3_permute(state: *mut sha3_state);
    fn _nettle_sha3_pad(
        state: *mut sha3_state,
        block_size: libc::c_uint,
        block: *mut uint8_t,
        pos: libc::c_uint,
        magic: uint8_t,
    );
    fn _nettle_write_le64(length: size_t, dst: *mut uint8_t, src: *const uint64_t);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
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
    mut block_size: libc::c_uint,
    mut block: *mut uint8_t,
    mut index: libc::c_uint,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    _nettle_sha3_pad(state, block_size, block, index, 0x1f as libc::c_int as uint8_t);
    while length > block_size as libc::c_ulong {
        nettle_sha3_permute(state);
        _nettle_write_le64(block_size as size_t, dst, ((*state).a).as_mut_ptr());
        length = (length as libc::c_ulong).wrapping_sub(block_size as libc::c_ulong)
            as size_t as size_t;
        dst = dst.offset(block_size as isize);
    }
    nettle_sha3_permute(state);
    _nettle_write_le64(length, dst, ((*state).a).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_sha3_shake_output(
    mut state: *mut sha3_state,
    mut block_size: libc::c_uint,
    mut block: *mut uint8_t,
    mut index: libc::c_uint,
    mut length: size_t,
    mut dst: *mut uint8_t,
) -> libc::c_uint {
    let mut left: libc::c_uint = 0;
    if index < block_size {
        _nettle_sha3_pad(
            state,
            block_size,
            block,
            index,
            0x1f as libc::c_int as uint8_t,
        );
        index = block_size;
    } else {
        index = !index;
    }
    if index <= block_size {} else {
        __assert_fail(
            b"index <= block_size\0" as *const u8 as *const libc::c_char,
            b"sha3-shake.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 118],
                &[libc::c_char; 118],
            >(
                b"unsigned int _nettle_sha3_shake_output(struct sha3_state *, unsigned int, uint8_t *, unsigned int, size_t, uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_865: {
        if index <= block_size {} else {
            __assert_fail(
                b"index <= block_size\0" as *const u8 as *const libc::c_char,
                b"sha3-shake.c\0" as *const u8 as *const libc::c_char,
                86 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 118],
                    &[libc::c_char; 118],
                >(
                    b"unsigned int _nettle_sha3_shake_output(struct sha3_state *, unsigned int, uint8_t *, unsigned int, size_t, uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    left = block_size.wrapping_sub(index);
    if length <= left as libc::c_ulong {
        memcpy(
            dst as *mut libc::c_void,
            block.offset(index as isize) as *const libc::c_void,
            length,
        );
        return !(index as libc::c_ulong).wrapping_add(length) as libc::c_uint;
    } else {
        memcpy(
            dst as *mut libc::c_void,
            block.offset(index as isize) as *const libc::c_void,
            left as libc::c_ulong,
        );
        length = (length as libc::c_ulong).wrapping_sub(left as libc::c_ulong) as size_t
            as size_t;
        dst = dst.offset(left as isize);
    }
    while length > block_size as libc::c_ulong {
        nettle_sha3_permute(state);
        _nettle_write_le64(block_size as size_t, dst, ((*state).a).as_mut_ptr());
        length = (length as libc::c_ulong).wrapping_sub(block_size as libc::c_ulong)
            as size_t as size_t;
        dst = dst.offset(block_size as isize);
    }
    nettle_sha3_permute(state);
    _nettle_write_le64(block_size as size_t, block, ((*state).a).as_mut_ptr());
    memcpy(dst as *mut libc::c_void, block as *const libc::c_void, length);
    return !length as libc::c_uint;
}
