use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sha512Ctx {
    pub state: [u64; 8],
    pub count_low: u64,
    pub count_high: u64,
    pub index: u32,
    pub block: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HmacSha512Ctx {
    pub outer: Sha512Ctx,
    pub inner: Sha512Ctx,
    pub state: Sha512Ctx,
}

pub struct HmacSha512 {
    ctx: HmacSha512Ctx,
}

impl HmacSha512 {
    pub fn new(key: &[u8]) -> Self {
        assert!(key.len() <= 64, "Key length must be <= 64 bytes");
        
        let mut ctx = unsafe { mem::zeroed::<HmacSha512Ctx>() };
        unsafe {
            nettle_hmac_sha512_set_key(
                &mut ctx as *mut _,
                key.len(),
                key.as_ptr(),
            );
        }
        
        Self { ctx }
    }
    
    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_hmac_sha512_update(
                &mut self.ctx as *mut _,
                data.len(),
                data.as_ptr(),
            );
        }
    }
    
    pub fn digest(mut self, output: &mut [u8]) {
        unsafe {
            nettle_hmac_sha512_digest(
                &mut self.ctx as *mut _,
                output.len(),
                output.as_mut_ptr(),
            );
        }
    }
}

extern "C" {
    fn nettle_hmac_sha512_set_key(
        ctx: *mut HmacSha512Ctx,
        key_length: usize,
        key: *const u8,
    );
    
    fn nettle_hmac_sha512_update(
        ctx: *mut HmacSha512Ctx,
        length: usize,
        data: *const u8,
    );
    
    fn nettle_hmac_sha512_digest(
        ctx: *mut HmacSha512Ctx,
        length: usize,
        digest: *mut u8,
    );
}