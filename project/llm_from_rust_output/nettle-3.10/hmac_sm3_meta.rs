use std::mem::size_of;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Clone, Copy)]
pub struct sm3_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}

#[derive(Clone, Copy)]
pub struct hmac_sm3_ctx {
    pub outer: sm3_ctx,
    pub inner: sm3_ctx,
    pub state: sm3_ctx,
}

pub struct HmacSm3 {
    ctx: hmac_sm3_ctx,
}

impl HmacSm3 {
    pub fn new() -> Self {
        Self {
            ctx: hmac_sm3_ctx {
                outer: sm3_ctx {
                    state: [0; 8],
                    count: 0,
                    index: 0,
                    block: [0; 64],
                },
                inner: sm3_ctx {
                    state: [0; 8],
                    count: 0,
                    index: 0,
                    block: [0; 64],
                },
                state: sm3_ctx {
                    state: [0; 8],
                    count: 0,
                    index: 0,
                    block: [0; 64],
                },
            },
        }
    }

    pub fn set_key(&mut self, key: &[uint8_t]) {
        assert_eq!(key.len(), 32);
        // TODO: Implement actual key setting logic
    }

    pub fn update(&mut self, data: &[uint8_t]) {
        // TODO: Implement actual update logic
    }

    pub fn digest(&mut self, output: &mut [uint8_t]) {
        assert_eq!(output.len(), 32);
        // TODO: Implement actual digest logic
    }
}

pub const HMAC_SM3_DIGEST_SIZE: usize = 32;
pub const HMAC_SM3_KEY_SIZE: usize = 32;