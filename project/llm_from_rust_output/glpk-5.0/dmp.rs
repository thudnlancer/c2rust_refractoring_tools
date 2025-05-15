use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{null_mut, NonNull};

const DEBUG: bool = false;
const BLOCK_SIZE: usize = 8000;

#[derive(Debug)]
struct Prefix {
    pool: NonNull<Dmp>,
    size: usize,
}

#[derive(Debug)]
pub struct Dmp {
    avail: [Option<NonNull<u8>>; 32],
    block: Option<NonNull<u8>>,
    used: usize,
    count: usize,
}

impl Dmp {
    pub fn new() -> Option<NonNull<Self>> {
        if DEBUG {
            println!("dmp_create_pool: warning: debug mode is on");
        }

        let layout = Layout::new::<Self>();
        unsafe {
            let ptr = alloc(layout) as *mut Self;
            if ptr.is_null() {
                return None;
            }

            let mut pool = NonNull::new_unchecked(ptr);
            pool.as_mut().avail = [None; 32];
            pool.as_mut().block = None;
            pool.as_mut().used = BLOCK_SIZE;
            pool.as_mut().count = 0;

            Some(pool)
        }
    }

    pub fn get_atom(&mut self, size: usize) -> Option<NonNull<u8>> {
        assert!(1 <= size && size <= 256, "Invalid size");

        let need = (size + 7) & !7;
        let k = (need >> 3) - 1;

        if self.avail[k].is_none() {
            let aligned_size = if DEBUG {
                need + ((std::mem::size_of::<Prefix>() + 7) & !7)
            } else {
                need
            };

            if self.used + aligned_size > BLOCK_SIZE {
                let layout = Layout::from_size_align(BLOCK_SIZE, 8).unwrap();
                unsafe {
                    let block = alloc(layout);
                    if block.is_null() {
                        return None;
                    }

                    let block_ptr = NonNull::new_unchecked(block);
                    *(block_ptr.as_ptr() as *mut Option<NonNull<u8>>) = self.block;
                    self.block = Some(block_ptr);
                    self.used = 8;
                }
            }

            unsafe {
                let atom = self.block.unwrap().as_ptr().add(self.used) as *mut u8;
                self.used += aligned_size;
                Some(NonNull::new_unchecked(atom))
            }
        } else {
            let atom = self.avail[k].unwrap();
            unsafe {
                self.avail[k] = Some(NonNull::new_unchecked(*(atom.as_ptr() as *mut *mut u8)));
            }
            Some(atom)
        }
    }

    pub fn free_atom(&mut self, mut atom: NonNull<u8>, size: usize) {
        assert!(1 <= size && size <= 256, "Invalid size");
        assert!(self.count > 0, "Pool count underflow");

        let k = ((size + 7) >> 3) - 1;

        if DEBUG {
            let prefix_offset = (std::mem::size_of::<Prefix>() + 7) & !7;
            atom = unsafe {
                NonNull::new_unchecked(atom.as_ptr().sub(prefix_offset) as *mut u8)
            };
            let prefix = unsafe { &*(atom.as_ptr() as *const Prefix) };
            assert!(prefix.pool.as_ptr() == self as *const _);
            assert!(prefix.size == size);
        }

        unsafe {
            *(atom.as_ptr() as *mut Option<NonNull<u8>>) = self.avail[k];
        }
        self.avail[k] = Some(atom);
        self.count -= 1;
    }

    pub fn in_use(&self) -> usize {
        self.count
    }
}

impl Drop for Dmp {
    fn drop(&mut self) {
        while let Some(block) = self.block {
            unsafe {
                self.block = *(block.as_ptr() as *mut Option<NonNull<u8>>);
                dealloc(block.as_ptr(), Layout::from_size_align(BLOCK_SIZE, 8).unwrap());
            }
        }
    }
}