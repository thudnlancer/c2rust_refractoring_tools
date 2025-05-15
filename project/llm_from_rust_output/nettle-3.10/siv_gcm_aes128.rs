use std::os::raw::{c_ulong, c_uchar, c_uint, c_void, c_char};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint32_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_cipher {
    pub name: *const c_char,
    pub context_size: c_uint,
    pub block_size: c_uint,
    pub key_size: c_uint,
    pub set_encrypt_key: Option<nettle_set_key_func>,
    pub set_decrypt_key: Option<nettle_set_key_func>,
    pub encrypt: Option<nettle_cipher_func>,
    pub decrypt: Option<nettle_cipher_func>,
}

pub type nettle_set_key_func = unsafe extern "C" fn(*mut c_void, *const uint8_t);
pub type nettle_cipher_func = unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}

extern "C" {
    static nettle_aes128: nettle_cipher;
    fn nettle_siv_gcm_encrypt_message(
        nc: *const nettle_cipher,
        ctx: *const c_void,
        ctr_ctx: *mut c_void,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        clength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_siv_gcm_decrypt_message(
        nc: *const nettle_cipher,
        ctx: *const c_void,
        ctr_ctx: *mut c_void,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        mlength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    ) -> i32,
}

pub fn siv_gcm_aes128_encrypt_message(
    ctx: &aes128_ctx,
    nonce: &[u8],
    adata: &[u8],
    src: &[u8],
    dst: &mut [u8],
) {
    unsafe {
        let mut ctr_ctx = aes128_ctx { keys: [0; 44] };
        nettle_siv_gcm_encrypt_message(
            &nettle_aes128,
            ctx as *const _ as *const c_void,
            &mut ctr_ctx as *mut _ as *mut c_void,
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

pub fn siv_gcm_aes128_decrypt_message(
    ctx: &aes128_ctx,
    nonce: &[u8],
    adata: &[u8],
    src: &[u8],
    dst: &mut [u8],
) -> Result<(), ()> {
    unsafe {
        let mut ctr_ctx = aes128_ctx { keys: [0; 44] };
        let result = nettle_siv_gcm_decrypt_message(
            &nettle_aes128,
            ctx as *const _ as *const c_void,
            &mut ctr_ctx as *mut _ as *mut c_void,
            nonce.len(),
            nonce.as_ptr(),
            adata.len(),
            adata.as_ptr(),
            dst.len(),
            dst.as_mut_ptr(),
            src.as_ptr(),
        );
        if result == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}