use std::mem::MaybeUninit;

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
    pub fn key_size(&self) -> usize {
        match self {
            AesCtx::Aes128(_) => 16,
            AesCtx::Aes192(_) => 24,
            AesCtx::Aes256(_) => 32,
        }
    }

    pub fn invert_key(&self) -> Self {
        match self {
            AesCtx::Aes128(ctx) => {
                let mut dst = MaybeUninit::<Aes128Ctx>::uninit();
                unsafe {
                    nettle_aes128_invert_key(dst.as_mut_ptr(), ctx);
                    AesCtx::Aes128(dst.assume_init())
                }
            }
            AesCtx::Aes192(ctx) => {
                let mut dst = MaybeUninit::<Aes192Ctx>::uninit();
                unsafe {
                    nettle_aes192_invert_key(dst.as_mut_ptr(), ctx);
                    AesCtx::Aes192(dst.assume_init())
                }
            }
            AesCtx::Aes256(ctx) => {
                let mut dst = MaybeUninit::<Aes256Ctx>::uninit();
                unsafe {
                    nettle_aes256_invert_key(dst.as_mut_ptr(), ctx);
                    AesCtx::Aes256(dst.assume_init())
                }
            }
        }
    }
}

pub fn aes_set_decrypt_key(ctx: &mut AesCtx, keysize: size_t, key: &[uint8_t]) {
    aes_set_encrypt_key(ctx, keysize, key);
    *ctx = ctx.invert_key();
}

// These would be implemented in a safe wrapper around the FFI functions
unsafe fn nettle_aes128_invert_key(dst: *mut Aes128Ctx, src: *const Aes128Ctx) {
    // FFI implementation
}

unsafe fn nettle_aes192_invert_key(dst: *mut Aes192Ctx, src: *const Aes192Ctx) {
    // FFI implementation
}

unsafe fn nettle_aes256_invert_key(dst: *mut Aes256Ctx, src: *const Aes256Ctx) {
    // FFI implementation
}

fn aes_set_encrypt_key(ctx: &mut AesCtx, length: size_t, key: &[uint8_t]) {
    // Safe implementation or FFI wrapper
}