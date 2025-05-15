use std::mem;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    pub b: [uint8_t; 16],
    pub w: [usize; 2],
    pub u64_0: [uint64_t; 2],
}

impl Default for NettleBlock16 {
    fn default() -> Self {
        NettleBlock16 { b: [0; 16] }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes256Ctx {
    pub keys: [uint32_t; 60],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrbgCtrAes256Ctx {
    pub key: Aes256Ctx,
    pub V: NettleBlock16,
}

impl DrbgCtrAes256Ctx {
    pub fn init(&mut self, seed_material: &[uint8_t]) {
        let zero_key = [0u8; 32];
        self.key = Aes256Ctx { keys: [0; 60] };
        self.V = NettleBlock16::default();
        
        unsafe {
            nettle_aes256_set_encrypt_key(&mut self.key, zero_key.as_ptr());
            self.update(Some(seed_material));
        }
    }

    pub fn random(&mut self, n: size_t, dst: &mut [uint8_t]) {
        unsafe {
            self.output(n, dst.as_mut_ptr());
            self.update(None);
        }
    }

    unsafe fn output(&mut self, n: size_t, dst: *mut uint8_t) {
        let mut remaining = n;
        let mut output_ptr = dst;
        
        while remaining >= 16 {
            self.increment_counter();
            nettle_aes256_encrypt(&self.key, 16, output_ptr, self.V.b.as_ptr());
            remaining -= 16;
            output_ptr = output_ptr.add(16);
        }
        
        if remaining > 0 {
            let mut block = NettleBlock16::default();
            self.increment_counter();
            nettle_aes256_encrypt(&self.key, 16, block.b.as_mut_ptr(), self.V.b.as_ptr());
            std::ptr::copy_nonoverlapping(block.b.as_ptr(), output_ptr, remaining);
        }
    }

    unsafe fn update(&mut self, provided_data: Option<&[uint8_t]>) {
        let mut tmp = [NettleBlock16::default(); 3];
        self.output(16 + 32, tmp[0].b.as_mut_ptr());
        
        if let Some(data) = provided_data {
            nettle_memxor(
                tmp[0].b.as_mut_ptr() as *mut libc::c_void,
                data.as_ptr() as *const libc::c_void,
                16 + 32,
            );
        }
        
        nettle_aes256_set_encrypt_key(&mut self.key, tmp[0].b.as_ptr());
        self.V = tmp[2];
    }

    fn increment_counter(&mut self) {
        unsafe {
            let mut i = 15;
            self.V.b[i] = self.V.b[i].wrapping_add(1);
            if self.V.b[i] == 0 {
                while i > 0 {
                    i -= 1;
                    self.V.b[i] = self.V.b[i].wrapping_add(1);
                    if self.V.b[i] != 0 {
                        break;
                    }
                }
            }
        }
    }
}

extern "C" {
    fn nettle_aes256_set_encrypt_key(ctx: *mut Aes256Ctx, key: *const uint8_t);
    fn nettle_aes256_encrypt(
        ctx: *const Aes256Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
}