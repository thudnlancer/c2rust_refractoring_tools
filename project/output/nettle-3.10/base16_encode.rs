#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = u64;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
static mut hex_digits: [uint8_t; 16] = unsafe {
    *::core::mem::transmute::<&[u8; 16], &[uint8_t; 16]>(b"0123456789abcdef")
};
#[no_mangle]
pub unsafe extern "C" fn nettle_base16_encode_single(
    mut dst: *mut i8,
    mut src: uint8_t,
) {
    *dst.offset(0 as i32 as isize) = hex_digits[(src as i32 / 0x10 as i32 & 0xf as i32)
        as usize] as i8;
    *dst.offset(1 as i32 as isize) = hex_digits[(src as i32 & 0xf as i32) as usize]
        as i8;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base16_encode_update(
    mut dst: *mut i8,
    mut length: size_t,
    mut src: *const uint8_t,
) {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < length {
        nettle_base16_encode_single(dst, *src.offset(i as isize));
        i = i.wrapping_add(1);
        i;
        dst = dst.offset(2 as i32 as isize);
    }
}