use std::num::Wrapping;

type SizeT = usize;
type Uint8T = u8;
type Uint32T = u32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChachaCtx {
    pub state: [Uint32T; 16],
}

fn chacha_core(dst: &mut [Uint32T; 16], src: &[Uint32T; 16], rounds: u32) {
    unsafe {
        _nettle_chacha_core(dst.as_mut_ptr(), src.as_ptr(), rounds);
    }
}

extern "C" {
    fn _nettle_chacha_core(
        dst: *mut Uint32T,
        src: *const Uint32T,
        rounds: u32,
    );
    fn nettle_memxor3(
        dst: *mut std::ffi::c_void,
        a: *const std::ffi::c_void,
        b: *const std::ffi::c_void,
        n: SizeT,
    ) -> *mut std::ffi::c_void;
}

fn memxor3(dst: &mut [Uint8T], a: &[Uint8T], b: &[Uint8T]) {
    unsafe {
        nettle_memxor3(
            dst.as_mut_ptr() as *mut std::ffi::c_void,
            a.as_ptr() as *const std::ffi::c_void,
            b.as_ptr() as *const std::ffi::c_void,
            dst.len().min(a.len()).min(b.len()),
        );
    }
}

impl ChachaCtx {
    pub fn crypt(&mut self, length: SizeT, dst: &mut [Uint8T], src: &[Uint8T]) {
        if length == 0 || dst.is_empty() || src.is_empty() {
            return;
        }

        let mut remaining = length;
        let mut dst_offset = 0;
        let mut src_offset = 0;

        while remaining > 0 {
            let mut x = [0; 16];
            chacha_core(&mut x, &self.state, 20);

            self.state[12] = (Wrapping(self.state[12]) + Wrapping(1)).0;
            self.state[13] = (Wrapping(self.state[13]) + 
                Wrapping(if self.state[12] == 0 { 1 } else { 0 })).0;

            let block_size = remaining.min(64);
            let dst_block = &mut dst[dst_offset..dst_offset + block_size];
            let src_block = &src[src_offset..src_offset + block_size];
            let x_bytes = unsafe {
                std::slice::from_raw_parts(x.as_ptr() as *const Uint8T, 64)
            };

            memxor3(dst_block, src_block, &x_bytes[..block_size]);

            remaining -= block_size;
            dst_offset += block_size;
            src_offset += block_size;
        }
    }

    pub fn crypt32(&mut self, length: SizeT, dst: &mut [Uint8T], src: &[Uint8T]) {
        if length == 0 || dst.is_empty() || src.is_empty() {
            return;
        }

        let mut remaining = length;
        let mut dst_offset = 0;
        let mut src_offset = 0;

        while remaining > 0 {
            let mut x = [0; 16];
            chacha_core(&mut x, &self.state, 20);

            self.state[12] = (Wrapping(self.state[12]) + Wrapping(1)).0;

            let block_size = remaining.min(64);
            let dst_block = &mut dst[dst_offset..dst_offset + block_size];
            let src_block = &src[src_offset..src_offset + block_size];
            let x_bytes = unsafe {
                std::slice::from_raw_parts(x.as_ptr() as *const Uint8T, 64)
            };

            memxor3(dst_block, src_block, &x_bytes[..block_size]);

            remaining -= block_size;
            dst_offset += block_size;
            src_offset += block_size;
        }
    }
}