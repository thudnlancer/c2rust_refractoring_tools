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

pub type NettleCipherFunc = fn(ctx: &Aes128Ctx, length: size_t, dst: &mut [u8], src: &[u8]);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes128Ctx {
    pub keys: [uint32_t; 44],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CcmCtx {
    pub ctr: nettle_block16,
    pub tag: nettle_block16,
    pub blength: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CcmAes128Ctx {
    pub ccm: CcmCtx,
    pub cipher: Aes128Ctx,
}

impl CcmAes128Ctx {
    pub fn set_key(&mut self, key: &[u8]) {
        unsafe {
            nettle_aes128_set_encrypt_key(&mut self.cipher, key.as_ptr());
        }
    }

    pub fn set_nonce(
        &mut self,
        length: size_t,
        nonce: &[u8],
        authlen: size_t,
        msglen: size_t,
        taglen: size_t,
    ) {
        unsafe {
            nettle_ccm_set_nonce(
                &mut self.ccm,
                &self.cipher as *const _ as *const libc::c_void,
                Some(mem::transmute(nettle_aes128_encrypt as NettleCipherFunc)),
                length,
                nonce.as_ptr(),
                authlen,
                msglen,
                taglen,
            );
        }
    }

    pub fn update(&mut self, length: size_t, data: &[u8]) {
        unsafe {
            nettle_ccm_update(
                &mut self.ccm,
                &self.cipher as *const _ as *const libc::c_void,
                Some(mem::transmute(nettle_aes128_encrypt as NettleCipherFunc)),
                length,
                data.as_ptr(),
            );
        }
    }

    pub fn encrypt(&mut self, length: size_t, dst: &mut [u8], src: &[u8]) {
        unsafe {
            nettle_ccm_encrypt(
                &mut self.ccm,
                &self.cipher as *const _ as *const libc::c_void,
                Some(mem::transmute(nettle_aes128_encrypt as NettleCipherFunc)),
                length,
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt(&mut self, length: size_t, dst: &mut [u8], src: &[u8]) {
        unsafe {
            nettle_ccm_decrypt(
                &mut self.ccm,
                &self.cipher as *const _ as *const libc::c_void,
                Some(mem::transmute(nettle_aes128_encrypt as NettleCipherFunc)),
                length,
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, length: size_t, digest: &mut [u8]) {
        unsafe {
            nettle_ccm_digest(
                &mut self.ccm,
                &self.cipher as *const _ as *const libc::c_void,
                Some(mem::transmute(nettle_aes128_encrypt as NettleCipherFunc)),
                length,
                digest.as_mut_ptr(),
            );
        }
    }

    pub fn encrypt_message(
        &mut self,
        nlength: size_t,
        nonce: &[u8],
        alength: size_t,
        adata: &[u8],
        tlength: size_t,
        clength: size_t,
        dst: &mut [u8],
        src: &[u8],
    ) {
        unsafe {
            nettle_ccm_encrypt_message(
                &self.cipher as *const _ as *const libc::c_void,
                Some(mem::transmute(nettle_aes128_encrypt as NettleCipherFunc)),
                nlength,
                nonce.as_ptr(),
                alength,
                adata.as_ptr(),
                tlength,
                clength,
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt_message(
        &mut self,
        nlength: size_t,
        nonce: &[u8],
        alength: size_t,
        adata: &[u8],
        tlength: size_t,
        mlength: size_t,
        dst: &mut [u8],
        src: &[u8],
    ) -> i32 {
        unsafe {
            nettle_ccm_decrypt_message(
                &self.cipher as *const _ as *const libc::c_void,
                Some(mem::transmute(nettle_aes128_encrypt as NettleCipherFunc)),
                nlength,
                nonce.as_ptr(),
                alength,
                adata.as_ptr(),
                tlength,
                mlength,
                dst.as_mut_ptr(),
                src.as_ptr(),
            )
        }
    }
}

extern "C" {
    fn nettle_aes128_set_encrypt_key(ctx: *mut Aes128Ctx, key: *const uint8_t);
    fn nettle_aes128_encrypt(ctx: *const Aes128Ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn nettle_ccm_set_nonce(
        ctx: *mut CcmCtx,
        cipher: *const libc::c_void,
        f: Option<NettleCipherFunc>,
        noncelen: size_t,
        nonce: *const uint8_t,
        authlen: size_t,
        msglen: size_t,
        taglen: size_t,
    );
    fn nettle_ccm_update(
        ctx: *mut CcmCtx,
        cipher: *const libc::c_void,
        f: Option<NettleCipherFunc>,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_ccm_encrypt(
        ctx: *mut CcmCtx,
        cipher: *const libc::c_void,
        f: Option<NettleCipherFunc>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ccm_decrypt(
        ctx: *mut CcmCtx,
        cipher: *const libc::c_void,
        f: Option<NettleCipherFunc>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ccm_digest(
        ctx: *mut CcmCtx,
        cipher: *const libc::c_void,
        f: Option<NettleCipherFunc>,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_ccm_encrypt_message(
        cipher: *const libc::c_void,
        f: Option<NettleCipherFunc>,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        tlength: size_t,
        clength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ccm_decrypt_message(
        cipher: *const libc::c_void,
        f: Option<NettleCipherFunc>,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        tlength: size_t,
        mlength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    ) -> libc::c_int;
}