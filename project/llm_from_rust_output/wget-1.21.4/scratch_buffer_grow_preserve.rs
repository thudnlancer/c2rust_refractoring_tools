use std::alloc::{alloc, realloc, Layout};
use std::mem::{size_of, MaybeUninit};
use std::ptr::{self, NonNull};

#[derive(Clone)]
pub struct ScratchBuffer {
    data: Option<NonNull<u8>>,
    length: usize,
    space: MaybeUninit<[u8; 1024]>,
}

impl ScratchBuffer {
    pub fn new() -> Self {
        let mut buffer = ScratchBuffer {
            data: None,
            length: 0,
            space: MaybeUninit::uninit(),
        };
        buffer.init();
        buffer
    }

    fn init(&mut self) {
        self.data = NonNull::new(self.space.as_mut_ptr() as *mut u8);
        self.length = size_of::<[u8; 1024]>();
    }

    pub fn grow_preserve(&mut self) -> bool {
        let new_length = self.length.checked_mul(2).unwrap_or(0);
        if new_length == 0 {
            return false;
        }

        let new_ptr = if let Some(ptr) = self.data {
            if ptr.as_ptr() == self.space.as_mut_ptr() as *mut u8 {
                unsafe {
                    let layout = Layout::from_size_align(new_length, 1).unwrap();
                    let new_ptr = alloc(layout);
                    if new_ptr.is_null() {
                        return false;
                    }
                    ptr::copy_nonoverlapping(
                        self.space.as_ptr() as *const u8,
                        new_ptr,
                        self.length,
                    );
                    NonNull::new(new_ptr)
                }
            } else {
                unsafe {
                    let layout = Layout::from_size_align(self.length, 1).unwrap();
                    let new_ptr = realloc(ptr.as_ptr(), layout, new_length);
                    if new_ptr.is_null() {
                        self.free();
                        self.init();
                        return false;
                    }
                    NonNull::new(new_ptr)
                }
            }
        } else {
            None
        };

        if let Some(ptr) = new_ptr {
            self.data = Some(ptr);
            self.length = new_length;
            true
        } else {
            false
        }
    }

    fn free(&mut self) {
        if let Some(ptr) = self.data {
            if ptr.as_ptr() != self.space.as_mut_ptr() as *mut u8 {
                unsafe {
                    let layout = Layout::from_size_align(self.length, 1).unwrap();
                    std::alloc::dealloc(ptr.as_ptr(), layout);
                }
            }
        }
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        self.free();
    }
}