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

pub struct AesContext {
    pub key_size: u32,
    pub ctx: AesCtx,
}

impl AesContext {
    pub fn set_encrypt_key(key_size: size_t, key: &[u8]) -> Result<Self, ()> {
        let ctx = match key_size {
            16 => {
                let mut ctx = MaybeUninit::<Aes128Ctx>::uninit();
                unsafe {
                    nettle_aes128_set_encrypt_key(ctx.as_mut_ptr(), key.as_ptr());
                    AesCtx::Aes128(ctx.assume_init())
                }
            }
            24 => {
                let mut ctx = MaybeUninit::<Aes192Ctx>::uninit();
                unsafe {
                    nettle_aes192_set_encrypt_key(ctx.as_mut_ptr(), key.as_ptr());
                    AesCtx::Aes192(ctx.assume_init())
                }
            }
            32 => {
                let mut ctx = MaybeUninit::<Aes256Ctx>::uninit();
                unsafe {
                    nettle_aes256_set_encrypt_key(ctx.as_mut_ptr(), key.as_ptr());
                    AesCtx::Aes256(ctx.assume_init())
                }
            }
            _ => return Err(()),
        };

        Ok(Self {
            key_size: key_size as u32,
            ctx,
        })
    }
}

extern "C" {
    fn nettle_aes128_set_encrypt_key(ctx: *mut Aes128Ctx, key: *const uint8_t);
    fn nettle_aes192_set_encrypt_key(ctx: *mut Aes192Ctx, key: *const uint8_t);
    fn nettle_aes256_set_encrypt_key(ctx: *mut Aes256Ctx, key: *const uint8_t);
}