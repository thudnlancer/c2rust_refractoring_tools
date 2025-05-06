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
    fn _nettle_aes_decrypt(
        rounds: u32,
        keys: *const uint32_t,
        T: *const aes_table,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    static _nettle_aes_decrypt_table: aes_table;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes_table {
    pub sbox: [uint8_t; 256],
    pub table: [[uint32_t; 256]; 4],
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_aes256_decrypt_c(
    mut ctx: *const aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(16 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!(length % AES_BLOCK_SIZE)\0" as *const u8 as *const i8,
            b"aes256-decrypt.c\0" as *const u8 as *const i8,
            56 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 93],
                &[i8; 93],
            >(
                b"void _nettle_aes256_decrypt_c(const struct aes256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_411: {
        if length.wrapping_rem(16 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!(length % AES_BLOCK_SIZE)\0" as *const u8 as *const i8,
                b"aes256-decrypt.c\0" as *const u8 as *const i8,
                56 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 93],
                    &[i8; 93],
                >(
                    b"void _nettle_aes256_decrypt_c(const struct aes256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    _nettle_aes_decrypt(
        14 as i32 as u32,
        ((*ctx).keys).as_ptr().offset((4 as i32 * 14 as i32) as isize),
        &_nettle_aes_decrypt_table,
        length,
        dst,
        src,
    );
}