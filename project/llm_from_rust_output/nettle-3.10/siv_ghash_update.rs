use std::convert::TryInto;

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

pub fn siv_ghash_update(
    ctx: &GcmKey,
    state: &mut NettleBlock16,
    mut blocks: usize,
    data: &[u8],
) -> &[u8] {
    let mut offset = 0;
    while blocks > 0 {
        let mut b = NettleBlock16 { b: [0; 16] };
        
        // Process first 8 bytes
        let first_word = u64::from_be_bytes(data[offset..offset+8].try_into().unwrap());
        unsafe {
            b.u64_0[1] = first_word;
        }
        
        // Process next 8 bytes
        let second_word = u64::from_be_bytes(data[offset+8..offset+16].try_into().unwrap());
        unsafe {
            b.u64_0[0] = second_word;
        }
        
        // Call the external function (still needs unsafe but isolated)
        unsafe {
            _nettle_ghash_update(
                ctx,
                state,
                1,
                b.b.as_ptr(),
            );
        }
        
        offset += 16;
        blocks -= 1;
    }
    &data[offset..]
}

extern "C" {
    fn _nettle_ghash_update(
        ctx: *const GcmKey,
        state: *mut NettleBlock16,
        blocks: usize,
        data: *const u8,
    ) -> *const u8;
}