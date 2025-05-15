use std::alloc::{alloc, realloc, Layout};
use std::ptr::{self, NonNull};
use std::mem;

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
            length: 0,
            space: [0; 1024],
        }
    }

    pub fn dupfree(&mut self, size: usize) -> Option<NonNull<u8>> {
        let current_ptr = self.data
            .map(|p| p.as_ptr())
            .unwrap_or_else(|| self.space.as_mut_ptr());

        let layout = Layout::from_size_align(size, mem::align_of::<u8>()).ok()?;

        if self.data.is_none() {
            // Currently using stack space, need to allocate new memory
            unsafe {
                let new_ptr = alloc(layout);
                if new_ptr.is_null() {
                    return None;
                }
                ptr::copy_nonoverlapping(current_ptr, new_ptr, size.min(self.space.len()));
                Some(NonNull::new_unchecked(new_ptr))
            }
        } else {
            // Already using heap memory, realloc
            unsafe {
                let old_ptr = current_ptr;
                let new_ptr = realloc(old_ptr, layout, size);
                if new_ptr.is_null() {
                    // On failure, return the original pointer
                    Some(NonNull::new_unchecked(old_ptr))
                } else {
                    Some(NonNull::new_unchecked(new_ptr))
                }
            }
        }
    }
}