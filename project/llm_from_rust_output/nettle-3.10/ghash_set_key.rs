use std::mem;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    pub b: [u8; 16],
    pub w: [u64; 2],
    pub u64_0: [u64; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GcmKey {
    pub h: [NettleBlock16; 256],
}

impl NettleBlock16 {
    fn set(&mut self, x: &NettleBlock16) {
        unsafe {
            self.u64_0[0] = x.u64_0[0];
            self.u64_0[1] = x.u64_0[1];
        }
    }

    fn mulx_ghash(&mut self, x: &NettleBlock16) {
        unsafe {
            let mask = ((x.u64_0[1] >> 56) & 1).wrapping_neg();
            
            self.u64_0[1] = (x.u64_0[1] & 0xfefefefefefefefe) >> 1
                | (x.u64_0[1] & 0x0101010101010101) << 15
                | (x.u64_0[0] >> 49) & 0x80;
            
            self.u64_0[0] = ((x.u64_0[0] & 0xfefefefefefefefe) >> 1
                | (x.u64_0[0] & 0x0101010101010101) << 15) ^ (mask & 0xe1);
        }
    }
}

pub fn ghash_set_key(ctx: &mut GcmKey, key: &NettleBlock16) {
    // Initialize first element
    ctx.h[2 * 7].set(key);

    // First loop
    for i in 1..64 {
        let src_idx = 2 * ((i - 1) ^ 7);
        let dst_idx = 2 * (i ^ 7);
        ctx.h[dst_idx].mulx_ghash(&ctx.h[src_idx]);
    }

    // Special case
    let src_idx = 2 * (63 ^ 7);
    let dst_idx = 2 * 7 + 1;
    ctx.h[dst_idx].mulx_ghash(&ctx.h[src_idx]);

    // Second loop
    for i in 1..64 {
        let src_idx = 2 * ((i - 1) ^ 7) + 1;
        let dst_idx = 2 * (i ^ 7) + 1;
        ctx.h[dst_idx].mulx_ghash(&ctx.h[src_idx]);
    }
}