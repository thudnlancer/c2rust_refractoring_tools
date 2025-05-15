use libc::{c_uchar, c_uint, c_ulong};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint32_t = c_uint;

#[derive(Copy, Clone)]
pub struct Aes128Ctx {
    pub keys: [uint32_t; 44],
}

#[derive(Copy, Clone)]
pub struct Aes192Ctx {
    pub keys: [uint32_t; 52],
}

#[derive(Copy, Clone)]
pub struct Aes256Ctx {
    pub keys: [uint32_t; 60],
}

#[derive(Copy, Clone)]
pub enum AesCtx {
    Aes128(Aes128Ctx),
    Aes192(Aes192Ctx),
    Aes256(Aes256Ctx),
}

pub struct AesContext {
    key_size: u32,
    ctx: AesCtx,
}

impl AesContext {
    pub fn decrypt(&self, length: size_t, dst: &mut [u8], src: &[u8]) {
        match self.ctx {
            AesCtx::Aes128(ref ctx) => {
                // Assume safe wrapper exists for nettle_aes128_decrypt
                unsafe {
                    nettle_aes128_decrypt(ctx, length, dst.as_mut_ptr(), src.as_ptr());
                }
            }
            AesCtx::Aes192(ref ctx) => {
                // Assume safe wrapper exists for nettle_aes192_decrypt
                unsafe {
                    nettle_aes192_decrypt(ctx, length, dst.as_mut_ptr(), src.as_ptr());
                }
            }
            AesCtx::Aes256(ref ctx) => {
                // Assume safe wrapper exists for nettle_aes256_decrypt
                unsafe {
                    nettle_aes256_decrypt(ctx, length, dst.as_mut_ptr(), src.as_ptr());
                }
            }
        }
    }
}

// These would be properly wrapped in safe Rust functions in practice
extern "C" {
    fn nettle_aes128_decrypt(
        ctx: *const Aes128Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes192_decrypt(
        ctx: *const Aes192Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes256_decrypt(
        ctx: *const Aes256Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}