use std::mem;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NettleBlock16 {
    pub b: [u8; 16],
    pub w: [u64; 2],
    pub u64_0: [u64; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes128Ctx {
    pub keys: [u32; 44],
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

pub struct SafeEaxAes128 {
    ctx: Box<EaxAes128Ctx>,
}

impl SafeEaxAes128 {
    pub fn new(key: &[u8; 16]) -> Self {
        unsafe {
            let mut ctx = Box::new(mem::zeroed::<EaxAes128Ctx>());
            nettle_aes128_set_encrypt_key(&mut ctx.cipher, key.as_ptr());
            nettle_eax_set_key(
                &mut ctx.key,
                &mut ctx.cipher as *mut Aes128Ctx as *const libc::c_void,
                Some(nettle_aes128_encrypt),
            );
            Self { ctx }
        }
    }

    pub fn set_nonce(&mut self, iv: &[u8]) {
        unsafe {
            nettle_eax_set_nonce(
                &mut self.ctx.eax,
                &mut self.ctx.key,
                &mut self.ctx.cipher as *mut Aes128Ctx as *const libc::c_void,
                Some(nettle_aes128_encrypt),
                iv.len(),
                iv.as_ptr(),
            );
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_eax_update(
                &mut self.ctx.eax,
                &mut self.ctx.key,
                &mut self.ctx.cipher as *mut Aes128Ctx as *const libc::c_void,
                Some(nettle_aes128_encrypt),
                data.len(),
                data.as_ptr(),
            );
        }
    }

    pub fn encrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        assert_eq!(dst.len(), src.len());
        unsafe {
            nettle_eax_encrypt(
                &mut self.ctx.eax,
                &mut self.ctx.key,
                &mut self.ctx.cipher as *mut Aes128Ctx as *const libc::c_void,
                Some(nettle_aes128_encrypt),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        assert_eq!(dst.len(), src.len());
        unsafe {
            nettle_eax_decrypt(
                &mut self.ctx.eax,
                &mut self.ctx.key,
                &mut self.ctx.cipher as *mut Aes128Ctx as *const libc::c_void,
                Some(nettle_aes128_encrypt),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        unsafe {
            nettle_eax_digest(
                &mut self.ctx.eax,
                &mut self.ctx.key,
                &mut self.ctx.cipher as *mut Aes128Ctx as *const libc::c_void,
                Some(nettle_aes128_encrypt),
                digest.len(),
                digest.as_mut_ptr(),
            );
        }
    }
}

// External FFI functions remain unsafe but are wrapped in safe interface
extern "C" {
    fn nettle_aes128_set_encrypt_key(ctx: *mut Aes128Ctx, key: *const u8);
    fn nettle_aes128_encrypt(
        ctx: *const Aes128Ctx,
        length: usize,
        dst: *mut u8,
        src: *const u8,
    );
    fn nettle_eax_set_key(
        key: *mut EaxKey,
        cipher: *const libc::c_void,
        f: Option<unsafe extern "C" fn(*const libc::c_void, usize, *mut u8, *const u8)>,
    );
    fn nettle_eax_set_nonce(
        eax: *mut EaxCtx,
        key: *const EaxKey,
        cipher: *const libc::c_void,
        f: Option<unsafe extern "C" fn(*const libc::c_void, usize, *mut u8, *const u8)>,
        nonce_length: usize,
        nonce: *const u8,
    );
    fn nettle_eax_update(
        eax: *mut EaxCtx,
        key: *const EaxKey,
        cipher: *const libc::c_void,
        f: Option<unsafe extern "C" fn(*const libc::c_void, usize, *mut u8, *const u8)>,
        data_length: usize,
        data: *const u8,
    );
    fn nettle_eax_encrypt(
        eax: *mut EaxCtx,
        key: *const EaxKey,
        cipher: *const libc::c_void,
        f: Option<unsafe extern "C" fn(*const libc::c_void, usize, *mut u8, *const u8)>,
        length: usize,
        dst: *mut u8,
        src: *const u8,
    );
    fn nettle_eax_decrypt(
        eax: *mut EaxCtx,
        key: *const EaxKey,
        cipher: *const libc::c_void,
        f: Option<unsafe extern "C" fn(*const libc::c_void, usize, *mut u8, *const u8)>,
        length: usize,
        dst: *mut u8,
        src: *const u8,
    );
    fn nettle_eax_digest(
        eax: *mut EaxCtx,
        key: *const EaxKey,
        cipher: *const libc::c_void,
        f: Option<unsafe extern "C" fn(*const libc::c_void, usize, *mut u8, *const u8)>,
        length: usize,
        digest: *mut u8,
    );
}