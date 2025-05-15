use std::alloc::{alloc, realloc, Layout};
use std::ptr::copy_nonoverlapping;
use std::mem::{size_of, align_of};

#[derive(Clone)]
pub struct ScratchBuffer {
    data: *mut u8,
    length: usize,
    space: [u8; 1024],
}

impl ScratchBuffer {
    pub fn dupfree(&mut self, size: usize) -> *mut u8 {
        if self.data == self.space.as_mut_ptr() {
            // Data is in the stack space, need to allocate and copy
            let layout = Layout::from_size_align(size, align_of::<u8>()).unwrap();
            unsafe {
                let new_ptr = alloc(layout);
                if !new_ptr.is_null() {
                    copy_nonoverlapping(self.data, new_ptr, self.length.min(size));
                    new_ptr
                } else {
                    std::ptr::null_mut()
                }
            }
        } else {
            // Data is already on heap, realloc
            let layout = Layout::from_size_align(self.length, align_of::<u8>()).unwrap();
            unsafe {
                realloc(self.data, layout, size)
            }
        }
    }
}