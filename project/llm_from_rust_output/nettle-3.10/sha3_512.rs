use std::mem;

type SizeT = usize;
type Uint8T = u8;
type Uint64T = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sha3State {
    pub a: [Uint64T; 25],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sha3512Ctx {
    pub state: Sha3State,
    pub index: u32,
    pub block: [Uint8T; 72],
}

impl Sha3512Ctx {
    pub fn new() -> Self {
        Sha3512Ctx {
            state: Sha3State { a: [0; 25] },
            index: 0,
            block: [0; 72],
        }
    }

    pub fn update(&mut self, data: &[Uint8T]) {
        self.index = unsafe {
            _nettle_sha3_update(
                &mut self.state,
                72,
                self.block.as_mut_ptr(),
                self.index,
                data.len(),
                data.as_ptr(),
            )
        };
    }

    pub fn digest(&mut self, digest: &mut [Uint8T]) {
        unsafe {
            _nettle_sha3_pad(
                &mut self.state,
                72,
                self.block.as_mut_ptr(),
                self.index,
                6,
            );
        }
        unsafe {
            nettle_sha3_permute(&mut self.state);
        }
        unsafe {
            _nettle_write_le64(digest.len(), digest.as_mut_ptr(), self.state.a.as_ptr());
        }
        *self = Self::new();
    }
}

fn _nettle_sha3_update(
    state: &mut Sha3State,
    block_size: u32,
    block: *mut Uint8T,
    pos: u32,
    length: SizeT,
    data: *const Uint8T,
) -> u32 {
    // Implementation of _nettle_sha3_update
    unimplemented!()
}

fn _nettle_sha3_pad(
    state: &mut Sha3State,
    block_size: u32,
    block: *mut Uint8T,
    pos: u32,
    magic: Uint8T,
) {
    // Implementation of _nettle_sha3_pad
    unimplemented!()
}

fn nettle_sha3_permute(state: &mut Sha3State) {
    // Implementation of nettle_sha3_permute
    unimplemented!()
}

fn _nettle_write_le64(length: SizeT, dst: *mut Uint8T, src: *const Uint64T) {
    // Implementation of _nettle_write_le64
    unimplemented!()
}