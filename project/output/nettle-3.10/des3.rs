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
    fn nettle_des_decrypt(
        ctx: *const des_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_des_encrypt(
        ctx: *const des_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_des_set_key(ctx: *mut des_ctx, key: *const uint8_t) -> i32;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des_ctx {
    pub key: [uint32_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des3_ctx {
    pub des: [des_ctx; 3],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_des3_set_key(
    mut ctx: *mut des3_ctx,
    mut key: *const uint8_t,
) -> i32 {
    let mut i: u32 = 0;
    let mut is_good: i32 = 1 as i32;
    i = 0 as i32 as u32;
    while i < 3 as i32 as u32 {
        if nettle_des_set_key(&mut *((*ctx).des).as_mut_ptr().offset(i as isize), key)
            == 0
        {
            is_good = 0 as i32;
        }
        i = i.wrapping_add(1);
        i;
        key = key.offset(8 as i32 as isize);
    }
    return is_good;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_des3_encrypt(
    mut ctx: *const des3_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_des_encrypt(
        &*((*ctx).des).as_ptr().offset(0 as i32 as isize),
        length,
        dst,
        src,
    );
    nettle_des_decrypt(
        &*((*ctx).des).as_ptr().offset(1 as i32 as isize),
        length,
        dst,
        dst,
    );
    nettle_des_encrypt(
        &*((*ctx).des).as_ptr().offset(2 as i32 as isize),
        length,
        dst,
        dst,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_des3_decrypt(
    mut ctx: *const des3_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_des_decrypt(
        &*((*ctx).des).as_ptr().offset(2 as i32 as isize),
        length,
        dst,
        src,
    );
    nettle_des_encrypt(
        &*((*ctx).des).as_ptr().offset(1 as i32 as isize),
        length,
        dst,
        dst,
    );
    nettle_des_decrypt(
        &*((*ctx).des).as_ptr().offset(0 as i32 as isize),
        length,
        dst,
        dst,
    );
}