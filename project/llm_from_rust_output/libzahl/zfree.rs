use std::alloc::{self, Layout};
use std::ptr::NonNull;

const POOL_INITIAL_SIZE: usize = 128;
const POOL_GROWTH_FACTOR: f32 = 1.5;

type SizeT = usize;
type ZahlCharT = u64;

struct Pool {
    blocks: Vec<NonNull<ZahlCharT>>,
}

impl Pool {
    fn new() -> Self {
        Pool {
            blocks: Vec::with_capacity(POOL_INITIAL_SIZE),
        }
    }

    fn add_block(&mut self, block: NonNull<ZahlCharT>) -> Result<(), ()> {
        if self.blocks.len() == self.blocks.capacity() {
            let new_capacity = ((self.blocks.capacity() as f32) * POOL_GROWTH_FACTOR) as usize;
            self.blocks.reserve(new_capacity);
        }
        self.blocks.push(block);
        Ok(())
    }
}

struct Zahl {
    sign: i32,
    used: SizeT,
    alloced: SizeT,
    chars: Option<NonNull<ZahlCharT>>,
}

impl Zahl {
    fn free(&mut self, pools: &mut [Pool; 64]) {
        if let Some(chars) = self.chars.take() {
            let i = (8 * std::mem::size_of::<SizeT>() - 1).saturating_sub(self.alloced.leading_zeros() as usize);
            
            if let Err(_) = pools[i].add_block(chars) {
                unsafe {
                    let layout = Layout::array::<ZahlCharT>(self.alloced).unwrap();
                    alloc::dealloc(chars.as_ptr() as *mut u8, layout);
                }
            }
        }
    }
}

struct ZahlPoolManager {
    pools: [Pool; 64],
}

impl ZahlPoolManager {
    fn new() -> Self {
        ZahlPoolManager {
            pools: std::array::from_fn(|_| Pool::new()),
        }
    }

    fn free_zahl(&mut self, zahl: &mut Zahl) {
        zahl.free(&mut self.pools);
    }
}