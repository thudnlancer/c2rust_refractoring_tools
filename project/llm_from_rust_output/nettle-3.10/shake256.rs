use std::mem;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sha3State {
    pub a: [u64; 25],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sha3256Ctx {
    pub state: Sha3State,
    pub index: u32,
    pub block: [u8; 136],
}

impl Sha3256Ctx {
    pub fn shake(&mut self, length: usize, dst: &mut [u8]) {
        unsafe {
            _nettle_sha3_shake(
                &mut self.state,
                mem::size_of::<[u8; 136]>() as u32,
                self.block.as_mut_ptr(),
                self.index,
                length,
                dst.as_mut_ptr(),
            );
        }
        self.init();
    }

    pub fn shake_output(&mut self, length: usize, digest: &mut [u8]) {
        unsafe {
            self.index = _nettle_sha3_shake_output(
                &mut self.state,
                mem::size_of::<[u8; 136]>() as u32,
                self.block.as_mut_ptr(),
                self.index,
                length,
                digest.as_mut_ptr(),
            );
        }
    }

    pub fn init(&mut self) {
        unsafe {
            nettle_sha3_256_init(self);
        }
    }
}

extern "C" {
    fn nettle_sha3_256_init(ctx: *mut Sha3256Ctx);
    fn _nettle_sha3_shake(
        state: *mut Sha3State,
        block_size: u32,
        block: *mut u8,
        index: u32,
        length: usize,
        dst: *mut u8,
    );
    fn _nettle_sha3_shake_output(
        state: *mut Sha3State,
        block_size: u32,
        block: *mut u8,
        index: u32,
        length: usize,
        dst: *mut u8,
    ) -> u32;
}