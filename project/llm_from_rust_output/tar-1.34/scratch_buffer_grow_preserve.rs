use std::alloc::{self, Layout};
use std::mem;
use std::ptr::{self, NonNull};

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

    pub fn init(&mut self) {
        self.data = NonNull::new(self.space.as_mut_ptr());
        self.length = mem::size_of_val(&self.space);
    }

    pub fn grow_preserve(&mut self) -> Result<(), alloc::LayoutError> {
        let new_length = self.length.checked_mul(2).ok_or(alloc::LayoutError)?;
        let layout = Layout::from_size_align(new_length, 1)?;

        let new_ptr = if let Some(ptr) = self.data {
            if ptr.as_ptr() == self.space.as_mut_ptr() {
                // Currently using stack space, move to heap
                let new_ptr = unsafe { alloc::alloc(layout) };
                if new_ptr.is_null() {
                    return Err(alloc::LayoutError);
                }
                unsafe {
                    ptr::copy_nonoverlapping(self.space.as_ptr(), new_ptr, self.length);
                }
                NonNull::new(new_ptr)
            } else {
                // Already on heap, realloc
                let new_ptr = unsafe { alloc::realloc(ptr.as_ptr(), layout, new_length) };
                if new_ptr.is_null() {
                    unsafe { alloc::dealloc(ptr.as_ptr(), layout) };
                    self.init();
                    return Err(alloc::LayoutError);
                }
                NonNull::new(new_ptr)
            }
        } else {
            // Not initialized, just allocate
            let new_ptr = unsafe { alloc::alloc(layout) };
            if new_ptr.is_null() {
                return Err(alloc::LayoutError);
            }
            NonNull::new(new_ptr)
        };

        if let Some(ptr) = new_ptr {
            self.data = Some(ptr);
            self.length = new_length;
            Ok(())
        } else {
            Err(alloc::LayoutError)
        }
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        if let Some(ptr) = self.data {
            if ptr.as_ptr() != self.space.as_mut_ptr() {
                let layout = Layout::from_size_align(self.length, 1).unwrap();
                unsafe {
                    alloc::dealloc(ptr.as_ptr(), layout);
                }
            }
        }
    }
}