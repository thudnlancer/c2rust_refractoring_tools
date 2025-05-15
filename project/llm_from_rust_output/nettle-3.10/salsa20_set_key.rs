use std::mem::MaybeUninit;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Salsa20Ctx {
    pub input: [uint32_t; 16],
}

pub enum Salsa20Key {
    Key128([uint8_t; 16]),
    Key256([uint8_t; 32]),
}

impl Salsa20Ctx {
    pub fn set_key(&mut self, key: Salsa20Key) {
        match key {
            Salsa20Key::Key128(key) => self.set_128_key(&key),
            Salsa20Key::Key256(key) => self.set_256_key(&key),
        }
    }

    fn set_128_key(&mut self, key: &[uint8_t; 16]) {
        // Safe because we're passing properly aligned and initialized data
        unsafe {
            nettle_salsa20_128_set_key(self, key.as_ptr());
        }
    }

    fn set_256_key(&mut self, key: &[uint8_t; 32]) {
        // Safe because we're passing properly aligned and initialized data
        unsafe {
            nettle_salsa20_256_set_key(self, key.as_ptr());
        }
    }
}

extern "C" {
    fn nettle_salsa20_128_set_key(ctx: *mut Salsa20Ctx, key: *const uint8_t);
    fn nettle_salsa20_256_set_key(ctx: *mut Salsa20Ctx, key: *const uint8_t);
}