use std::mem;

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

pub type nettle_cipher_func = fn(
    cipher: &dyn std::any::Any,
    length: size_t,
    dst: &mut [uint8_t],
    src: &[uint8_t],
);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_ctx {
    pub iv: nettle_block16,
    pub ctr: nettle_block16,
    pub x: nettle_block16,
    pub auth_size: uint64_t,
    pub data_size: uint64_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_aes128_ctx {
    pub key: gcm_key,
    pub gcm: gcm_ctx,
    pub cipher: aes128_ctx,
}

impl gcm_aes128_ctx {
    pub fn set_key(&mut self, key: &[uint8_t]) {
        unsafe {
            nettle_aes128_set_encrypt_key(&mut self.cipher, key.as_ptr());
        }
        
        let cipher_ptr = &self.cipher as *const aes128_ctx as *const libc::c_void;
        let encrypt_fn = nettle_aes128_encrypt_wrapper as nettle_cipher_func;
        
        unsafe {
            nettle_gcm_set_key(
                &mut self.key,
                cipher_ptr,
                Some(encrypt_fn),
            );
        }
    }

    pub fn set_iv(&mut self, iv: &[uint8_t]) {
        unsafe {
            nettle_gcm_set_iv(
                &mut self.gcm,
                &self.key,
                iv.len(),
                iv.as_ptr(),
            );
        }
    }

    pub fn update(&mut self, data: &[uint8_t]) {
        unsafe {
            nettle_gcm_update(
                &mut self.gcm,
                &self.key,
                data.len(),
                data.as_ptr(),
            );
        }
    }

    pub fn encrypt(&mut self, src: &[uint8_t], dst: &mut [uint8_t]) {
        assert_eq!(src.len(), dst.len());
        
        self.gcm.data_size += src.len() as u64;
        
        unsafe {
            nettle_gcm_encrypt(
                &mut self.gcm,
                &self.key,
                &self.cipher as *const aes128_ctx as *const libc::c_void,
                Some(nettle_aes128_encrypt_wrapper as nettle_cipher_func),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt(&mut self, src: &[uint8_t], dst: &mut [uint8_t]) {
        assert_eq!(src.len(), dst.len());
        
        self.gcm.data_size += src.len() as u64;
        
        unsafe {
            nettle_gcm_decrypt(
                &mut self.gcm,
                &self.key,
                &self.cipher as *const aes128_ctx as *const libc::c_void,
                Some(nettle_aes128_encrypt_wrapper as nettle_cipher_func),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [uint8_t]) {
        unsafe {
            nettle_gcm_digest(
                &mut self.gcm,
                &self.key,
                &self.cipher as *const aes128_ctx as *const libc::c_void,
                Some(nettle_aes128_encrypt_wrapper as nettle_cipher_func),
                digest.len(),
                digest.as_mut_ptr(),
            );
        }
    }
}

fn nettle_aes128_encrypt_wrapper(
    cipher: &dyn std::any::Any,
    length: size_t,
    dst: &mut [uint8_t],
    src: &[uint8_t],
) {
    let cipher = cipher.downcast_ref::<aes128_ctx>().unwrap();
    unsafe {
        nettle_aes128_encrypt(
            cipher,
            length,
            dst.as_mut_ptr(),
            src.as_ptr(),
        );
    }
}

// External C functions - these would need to be properly bound or reimplemented in Rust
extern "C" {
    fn nettle_aes128_set_encrypt_key(ctx: *mut aes128_ctx, key: *const uint8_t);
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_set_key(
        key: *mut gcm_key,
        cipher: *const libc::c_void,
        f: Option<nettle_cipher_func>,
    );
    fn nettle_gcm_set_iv(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        length: size_t,
        iv: *const uint8_t,
    );
    fn nettle_gcm_update(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_gcm_encrypt(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_decrypt(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_digest(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        digest: *mut uint8_t,
    );
}