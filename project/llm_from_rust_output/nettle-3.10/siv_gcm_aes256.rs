use std::ptr;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;

#[derive(Clone, Copy)]
pub struct nettle_cipher {
    pub name: &'static str,
    pub context_size: u32,
    pub block_size: u32,
    pub key_size: u32,
    pub set_encrypt_key: Option<fn(&mut [u8], &[u8])>,
    pub set_decrypt_key: Option<fn(&mut [u8], &[u8])>,
    pub encrypt: Option<fn(&[u8], &mut [u8], &[u8])>,
    pub decrypt: Option<fn(&[u8], &mut [u8], &[u8])>,
}

#[derive(Clone, Copy)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}

pub static NETTLE_AES256: nettle_cipher = nettle_cipher {
    name: "AES256",
    context_size: std::mem::size_of::<aes256_ctx>() as u32,
    block_size: 16,
    key_size: 32,
    set_encrypt_key: None,
    set_decrypt_key: None,
    encrypt: None,
    decrypt: None,
};

pub fn siv_gcm_encrypt_message(
    cipher: &nettle_cipher,
    ctx: &aes256_ctx,
    nlength: size_t,
    nonce: &[u8],
    alength: size_t,
    adata: &[u8],
    clength: size_t,
    dst: &mut [u8],
    src: &[u8],
) {
    let mut ctr_ctx = aes256_ctx { keys: [0; 60] };
    
    unsafe {
        let cipher_ptr = cipher as *const nettle_cipher;
        let ctx_ptr = ctx as *const aes256_ctx as *const libc::c_void;
        let ctr_ptr = &mut ctr_ctx as *mut aes256_ctx as *mut libc::c_void;
        
        nettle_siv_gcm_encrypt_message(
            cipher_ptr,
            ctx_ptr,
            ctr_ptr,
            nlength,
            nonce.as_ptr(),
            alength,
            adata.as_ptr(),
            clength,
            dst.as_mut_ptr(),
            src.as_ptr(),
        );
    }
}

pub fn siv_gcm_decrypt_message(
    cipher: &nettle_cipher,
    ctx: &aes256_ctx,
    nlength: size_t,
    nonce: &[u8],
    alength: size_t,
    adata: &[u8],
    mlength: size_t,
    dst: &mut [u8],
    src: &[u8],
) -> i32 {
    let mut ctr_ctx = aes256_ctx { keys: [0; 60] };
    
    unsafe {
        let cipher_ptr = cipher as *const nettle_cipher;
        let ctx_ptr = ctx as *const aes256_ctx as *const libc::c_void;
        let ctr_ptr = &mut ctr_ctx as *mut aes256_ctx as *mut libc::c_void;
        
        nettle_siv_gcm_decrypt_message(
            cipher_ptr,
            ctx_ptr,
            ctr_ptr,
            nlength,
            nonce.as_ptr(),
            alength,
            adata.as_ptr(),
            mlength,
            dst.as_mut_ptr(),
            src.as_ptr(),
        )
    }
}

extern "C" {
    fn nettle_siv_gcm_encrypt_message(
        nc: *const nettle_cipher,
        ctx: *const libc::c_void,
        ctr_ctx: *mut libc::c_void,
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
        ctx: *const libc::c_void,
        ctr_ctx: *mut libc::c_void,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        mlength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    ) -> libc::c_int;
}