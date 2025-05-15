use std::mem;
use std::ptr;
use std::alloc::{self, Layout};

/// A variable-sized buffer with on-stack default allocation.
pub struct ScratchBuffer {
    data: *mut u8,
    length: usize,
    space: [u8; 1024], // Adjust size as needed for default stack allocation
}

impl ScratchBuffer {
    /// Creates a new scratch buffer with default stack allocation.
    pub fn new() -> Self {
        Self {
            data: ptr::null_mut(),
            length: 0,
            space: [0; 1024],
        }
    }

    /// Grows the buffer while preserving existing content.
    pub fn grow_preserve(&mut self) -> Result<(), alloc::LayoutError> {
        let new_length = match self.length.checked_mul(2) {
            Some(len) => len,
            None => return Err(alloc::LayoutError),
        };

        let new_ptr = if self.data.is_null() {
            // Move buffer to the heap
            let layout = Layout::array::<u8>(new_length)?;
            let new_ptr = unsafe { alloc::alloc(layout) };
            if new_ptr.is_null() {
                return Err(alloc::LayoutError);
            }
            unsafe {
                ptr::copy_nonoverlapping(
                    self.space.as_ptr(),
                    new_ptr,
                    self.space.len().min(new_length),
                );
            }
            new_ptr
        } else {
            // Buffer was already on the heap
            let layout = Layout::array::<u8>(new_length)?;
            let new_ptr = unsafe { alloc::realloc(self.data, layout, new_length) };
            if new_ptr.is_null() {
                // Deallocate but keep buffer valid
                unsafe { alloc::dealloc(self.data, Layout::array::<u8>(self.length).unwrap()) };
                *self = Self::new();
                return Err(alloc::LayoutError);
            }
            new_ptr
        };

        // Install new heap-based buffer
        self.data = new_ptr;
        self.length = new_length;
        Ok(())
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                alloc::dealloc(
                    self.data,
                    Layout::array::<u8>(self.length).unwrap(),
                );
            }
        }
    }
}