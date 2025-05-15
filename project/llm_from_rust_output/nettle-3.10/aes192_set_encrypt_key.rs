use std::mem::MaybeUninit;

pub type uint8_t = u8;
pub type uint32_t = u32;

#[derive(Clone, Copy)]
pub struct Aes192Ctx {
    keys: [uint32_t; 52],
}

impl Aes192Ctx {
    pub fn new() -> Self {
        Self { keys: [0; 52] }
    }

    pub fn set_encrypt_key(&mut self, key: &[uint8_t; 24]) {
        unsafe {
            nettle_aes_set_key(
                12,
                6,
                self.keys.as_mut_ptr(),
                key.as_ptr(),
            );
        }
    }
}

extern "C" {
    fn nettle_aes_set_key(
        nr: u32,
        nk: u32,
        subkeys: *mut uint32_t,
        key: *const uint8_t,
    );
}