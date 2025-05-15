use std::alloc::{self, Layout};
use std::ptr::NonNull;
use std::mem::MaybeUninit;

pub struct ScratchBuffer {
    data: Option<NonNull<u8>>,
    length: usize,
    space: MaybeUninit<[u8; 1024]>,
}

impl ScratchBuffer {
    pub fn new() -> Self {
        let mut space = MaybeUninit::<[u8; 1024]>::uninit();
        let data_ptr = unsafe { space.as_mut_ptr() as *mut u8 };
        
        Self {
            data: NonNull::new(data_ptr),
            length: 1024,
            space,
        }
    }

    pub fn free(&mut self) {
        if let Some(ptr) = self.data {
            if !self.is_internal_buffer(ptr) {
                unsafe {
                    let layout = Layout::from_size_align(self.length, 1).unwrap();
                    alloc::dealloc(ptr.as_ptr(), layout);
                }
            }
            self.data = None;
        }
    }

    fn is_internal_buffer(&self, ptr: NonNull<u8>) -> bool {
        let internal_ptr = self.space.as_ptr() as *const u8;
        ptr.as_ptr() as *const u8 == internal_ptr
    }

    pub fn grow(&mut self) -> bool {
        let new_length = match self.length.checked_mul(2) {
            Some(len) => len,
            None => {
                return false;
            }
        };

        self.free();

        let layout = match Layout::from_size_align(new_length, 1) {
            Ok(l) => l,
            Err(_) => {
                self.init_internal_buffer();
                return false;
            }
        };

        let new_ptr = unsafe { alloc::alloc(layout) };
        if new_ptr.is_null() {
            self.init_internal_buffer();
            return false;
        }

        self.data = NonNull::new(new_ptr);
        self.length = new_length;
        true
    }

    fn init_internal_buffer(&mut self) {
        let data_ptr = unsafe { self.space.as_mut_ptr() as *mut u8 };
        self.data = NonNull::new(data_ptr);
        self.length = 1024;
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        self.free();
    }
}