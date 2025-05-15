use libc::{c_ulong, c_uchar, c_uint, c_int};
use std::ptr;

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint32_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DesCtx {
    pub key: [uint32_t; 32],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Des3Ctx {
    pub des: [DesCtx; 3],
}

extern "C" {
    fn nettle_des_decrypt(ctx: *const DesCtx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn nettle_des_encrypt(ctx: *const DesCtx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn nettle_des_set_key(ctx: *mut DesCtx, key: *const uint8_t) -> c_int;
}

pub fn nettle_des3_set_key(ctx: &mut Des3Ctx, key: &[uint8_t]) -> c_int {
    let mut is_good = 1;
    for i in 0..3 {
        let des_key = &mut ctx.des[i];
        let key_ptr = unsafe { key.as_ptr().add(i * 8) };
        if unsafe { nettle_des_set_key(des_key, key_ptr) } == 0 {
            is_good = 0;
        }
    }
    is_good
}

pub fn nettle_des3_encrypt(ctx: &Des3Ctx, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]) {
    unsafe {
        nettle_des_encrypt(&ctx.des[0], length, dst.as_mut_ptr(), src.as_ptr());
        nettle_des_decrypt(&ctx.des[1], length, dst.as_mut_ptr(), dst.as_ptr());
        nettle_des_encrypt(&ctx.des[2], length, dst.as_mut_ptr(), dst.as_ptr());
    }
}

pub fn nettle_des3_decrypt(ctx: &Des3Ctx, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]) {
    unsafe {
        nettle_des_decrypt(&ctx.des[2], length, dst.as_mut_ptr(), src.as_ptr());
        nettle_des_encrypt(&ctx.des[1], length, dst.as_mut_ptr(), dst.as_ptr());
        nettle_des_decrypt(&ctx.des[0], length, dst.as_mut_ptr(), dst.as_ptr());
    }
}