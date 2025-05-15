use std::alloc::{self, Layout};
use std::ptr::NonNull;
use std::mem;

#[derive(Clone)]
pub struct ScratchBuffer {
    data: Option<NonNull<u8>>,
    length: usize,
    space: [u8; 1024],
}

impl ScratchBuffer {
    pub fn new() -> Self {
        Self {
            data: None,
            length: mem::size_of::<[u8; 1024]>(),
            space: [0; 1024],
        }
    }

    pub fn grow(&mut self) -> bool {
        let new_length = match self.length.checked_mul(2) {
            Some(len) => len,
            None => {
                return false;
            }
        };

        if new_length <= self.space.len() {
            self.data = None;
            self.length = self.space.len();
            return true;
        }

        let layout = match Layout::from_size_align(new_length, mem::align_of::<u8>()) {
            Ok(layout) => layout,
            Err(_) => return false,
        };

        let new_ptr = unsafe { alloc::alloc(layout) };
        if new_ptr.is_null() {
            return false;
        }

        self.data = NonNull::new(new_ptr);
        self.length = new_length;
        true
    }

    pub fn as_ptr(&self) -> *const u8 {
        match self.data {
            Some(ptr) => ptr.as_ptr(),
            None => self.space.as_ptr(),
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        match self.data {
            Some(mut ptr) => ptr.as_mut(),
            None => self.space.as_mut_ptr(),
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        if let Some(ptr) = self.data {
            let layout = Layout::from_size_align(self.length, mem::align_of::<u8>()).unwrap();
            unsafe { alloc::dealloc(ptr.as_ptr(), layout) };
        }
    }
}