use std::num::NonZeroU32;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;

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

impl AesCtx {
    pub fn encrypt(&self, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]) {
        assert_eq!(src.len(), length);
        assert_eq!(dst.len(), length);
        
        match self {
            AesCtx::Aes128(ctx) => nettle_aes128_encrypt(ctx, length, dst, src),
            AesCtx::Aes192(ctx) => nettle_aes192_encrypt(ctx, length, dst, src),
            AesCtx::Aes256(ctx) => nettle_aes256_encrypt(ctx, length, dst, src),
        }
    }
}

fn nettle_aes128_encrypt(ctx: &Aes128Ctx, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]) {
    // Implementation of AES-128 encryption
    unimplemented!()
}

fn nettle_aes192_encrypt(ctx: &Aes192Ctx, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]) {
    // Implementation of AES-192 encryption
    unimplemented!()
}

fn nettle_aes256_encrypt(ctx: &Aes256Ctx, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]) {
    // Implementation of AES-256 encryption
    unimplemented!()
}