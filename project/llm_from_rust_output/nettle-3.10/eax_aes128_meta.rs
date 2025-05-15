use std::mem;
use std::os::raw::{c_uchar, c_uint, c_ulong};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint32_t = c_uint;
pub type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    pub b: [uint8_t; 16],
    pub w: [c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes128Ctx {
    pub keys: [uint32_t; 44],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EaxKey {
    pub pad_block: NettleBlock16,
    pub pad_partial: NettleBlock16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EaxCtx {
    pub omac_nonce: NettleBlock16,
    pub omac_data: NettleBlock16,
    pub omac_message: NettleBlock16,
    pub ctr: NettleBlock16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EaxAes128Ctx {
    pub key: EaxKey,
    pub eax: EaxCtx,
    pub cipher: Aes128Ctx,
}

pub struct NettleAead {
    pub name: &'static str,
    pub context_size: u32,
    pub block_size: u32,
    pub key_size: u32,
    pub nonce_size: u32,
    pub digest_size: u32,
}

impl EaxAes128Ctx {
    pub fn set_key(&mut self, key: &[u8]) {
        assert_eq!(key.len(), 16);
        // Safe because we've validated key length
        unsafe {
            nettle_eax_aes128_set_key(self as *mut _ as *mut _, key.as_ptr());
        }
    }

    pub fn set_nonce(&mut self, nonce: &[u8]) {
        assert_eq!(nonce.len(), 16);
        // Safe because we've validated nonce length
        unsafe {
            nettle_eax_aes128_set_nonce(self as *mut _ as *mut _, 16, nonce.as_ptr());
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        // Safe because we're passing valid pointers and lengths
        unsafe {
            nettle_eax_aes128_update(self as *mut _ as *mut _, data.len(), data.as_ptr());
        }
    }

    pub fn encrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        assert_eq!(dst.len(), src.len());
        // Safe because we've validated lengths match
        unsafe {
            nettle_eax_aes128_encrypt(
                self as *mut _ as *mut _,
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        assert_eq!(dst.len(), src.len());
        // Safe because we've validated lengths match
        unsafe {
            nettle_eax_aes128_decrypt(
                self as *mut _ as *mut _,
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        assert_eq!(digest.len(), 16);
        // Safe because we've validated digest length
        unsafe {
            nettle_eax_aes128_digest(self as *mut _ as *mut _, 16, digest.as_mut_ptr());
        }
    }
}

pub static NETTLE_EAX_AES128: NettleAead = NettleAead {
    name: "eax_aes128",
    context_size: mem::size_of::<EaxAes128Ctx>() as u32,
    block_size: 16,
    key_size: 16,
    nonce_size: 16,
    digest_size: 16,
};

extern "C" {
    fn nettle_eax_aes128_set_key(ctx: *mut EaxAes128Ctx, key: *const uint8_t);
    fn nettle_eax_aes128_set_nonce(ctx: *mut EaxAes128Ctx, length: size_t, iv: *const uint8_t);
    fn nettle_eax_aes128_update(ctx: *mut EaxAes128Ctx, length: size_t, data: *const uint8_t);
    fn nettle_eax_aes128_encrypt(
        ctx: *mut EaxAes128Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_eax_aes128_decrypt(
        ctx: *mut EaxAes128Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_eax_aes128_digest(
        ctx: *mut EaxAes128Ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
}