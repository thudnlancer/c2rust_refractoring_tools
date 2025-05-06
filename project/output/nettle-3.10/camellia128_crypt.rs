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
    fn _nettle_camellia_crypt(
        nkeys: u32,
        keys: *const uint64_t,
        T: *const camellia_table,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    static _nettle_camellia_table: camellia_table;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia128_ctx {
    pub keys: [uint64_t; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia_table {
    pub sp1110: [uint32_t; 256],
    pub sp0222: [uint32_t; 256],
    pub sp3033: [uint32_t; 256],
    pub sp4404: [uint32_t; 256],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_camellia128_crypt(
    mut ctx: *const camellia128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length.wrapping_rem(16 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!(length % CAMELLIA_BLOCK_SIZE)\0" as *const u8 as *const i8,
            b"camellia128-crypt.c\0" as *const u8 as *const i8,
            50 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 98],
                &[i8; 98],
            >(
                b"void nettle_camellia128_crypt(const struct camellia128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_282: {
        if length.wrapping_rem(16 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!(length % CAMELLIA_BLOCK_SIZE)\0" as *const u8 as *const i8,
                b"camellia128-crypt.c\0" as *const u8 as *const i8,
                50 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 98],
                    &[i8; 98],
                >(
                    b"void nettle_camellia128_crypt(const struct camellia128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    _nettle_camellia_crypt(
        24 as i32 as u32,
        ((*ctx).keys).as_ptr(),
        &_nettle_camellia_table,
        length,
        dst,
        src,
    );
}