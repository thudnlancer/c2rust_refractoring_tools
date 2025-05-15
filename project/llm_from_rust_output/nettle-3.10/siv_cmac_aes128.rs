use std::mem::MaybeUninit;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac128_key {
    pub K1: nettle_block16,
    pub K2: nettle_block16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct siv_cmac_aes128_ctx {
    pub cmac_key: cmac128_key,
    pub cmac_cipher: aes128_ctx,
    pub ctr_cipher: aes128_ctx,
}

pub struct SivCmacAes128 {
    ctx: siv_cmac_aes128_ctx,
}

impl SivCmacAes128 {
    pub fn new(key: &[u8]) -> Self {
        let mut ctx = MaybeUninit::<siv_cmac_aes128_ctx>::uninit();
        unsafe {
            nettle_siv_cmac_aes128_set_key(ctx.as_mut_ptr(), key.as_ptr());
            Self {
                ctx: ctx.assume_init(),
            }
        }
    }

    pub fn encrypt_message(
        &self,
        nonce: &[u8],
        adata: &[u8],
        src: &[u8],
        dst: &mut [u8],
    ) {
        unsafe {
            nettle_siv_cmac_aes128_encrypt_message(
                &self.ctx,
                nonce.len(),
                nonce.as_ptr(),
                adata.len(),
                adata.as_ptr(),
                dst.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt_message(
        &self,
        nonce: &[u8],
        adata: &[u8],
        src: &[u8],
        dst: &mut [u8],
    ) -> Result<(), ()> {
        let res = unsafe {
            nettle_siv_cmac_aes128_decrypt_message(
                &self.ctx,
                nonce.len(),
                nonce.as_ptr(),
                adata.len(),
                adata.as_ptr(),
                dst.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            )
        };
        if res == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

// These would be implemented by linking to the actual Nettle library
extern "C" {
    fn nettle_siv_cmac_aes128_set_key(ctx: *mut siv_cmac_aes128_ctx, key: *const uint8_t);
    fn nettle_siv_cmac_aes128_encrypt_message(
        ctx: *const siv_cmac_aes128_ctx,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        clength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_siv_cmac_aes128_decrypt_message(
        ctx: *const siv_cmac_aes128_ctx,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        mlength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    ) -> i32;
}