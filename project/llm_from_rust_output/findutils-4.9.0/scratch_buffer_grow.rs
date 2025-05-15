use std::alloc::{alloc, Layout};
use std::ptr::NonNull;
use std::mem::size_of;

#[derive(Clone)]
pub struct ScratchBuffer {
    data: Option<NonNull<u8>>,
    length: usize,
    space: [u8; 1024],
}

impl ScratchBuffer {
    pub fn new() -> Self {
        ScratchBuffer {
            data: None,
            length: size_of::<[u8; 1024]>(),
            space: [0; 1024],
        }
    }

    pub fn free(&mut self) {
        if let Some(ptr) = self.data.take() {
            unsafe {
                let layout = Layout::from_size_align(self.length, 1).unwrap();
                std::alloc::dealloc(ptr.as_ptr(), layout);
            }
        }
    }

    pub fn init(&mut self) {
        self.free();
        self.length = self.space.len();
    }

    pub fn grow(&mut self) -> bool {
        let new_length = match self.length.checked_mul(2) {
            Some(len) => len,
            None => {
                std::mem::replace(&mut self.length, self.space.len());
                return false;
            }
        };

        self.free();
        
        let layout = match Layout::from_size_align(new_length, 1) {
            Ok(l) => l,
            Err(_) => {
                self.init();
                return false;
            }
        };

        let new_ptr = unsafe { alloc(layout) };
        if new_ptr.is_null() {
            self.init();
            return false;
        }

        self.data = NonNull::new(new_ptr);
        self.length = new_length;
        true
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.map_or(self.space.as_ptr(), |p| p.as_ptr())
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.map_or(self.space.as_mut_ptr(), |p| p.as_ptr())
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        self.free();
    }
}