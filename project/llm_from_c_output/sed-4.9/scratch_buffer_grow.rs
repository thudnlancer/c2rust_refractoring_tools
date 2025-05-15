use std::alloc::{self, Layout};
use std::mem;
use std::ptr;

/// A variable-sized buffer with on-stack default allocation.
pub struct ScratchBuffer {
    data: *mut u8,
    length: usize,
}

impl ScratchBuffer {
    /// Creates a new empty scratch buffer.
    pub fn new() -> Self {
        ScratchBuffer {
            data: ptr::null_mut(),
            length: 0,
        }
    }

    /// Frees the buffer's memory if allocated.
    pub fn free(&mut self) {
        if !self.data.is_null() {
            unsafe {
                alloc::dealloc(
                    self.data,
                    Layout::from_size_align(self.length, mem::align_of::<u8>()).unwrap(),
                );
            }
            self.data = ptr::null_mut();
            self.length = 0;
        }
    }

    /// Attempts to grow the buffer, doubling its size.
    /// Returns true if successful, false on allocation failure.
    pub fn grow(&mut self) -> bool {
        let new_length = match self.length.checked_mul(2) {
            Some(len) => len,
            None => {
                // On overflow, return failure
                return false;
            }
        };

        // Free old buffer
        self.free();

        // Allocate new buffer
        let layout = match Layout::from_size_align(new_length, mem::align_of::<u8>()) {
            Ok(layout) => layout,
            Err(_) => return false, // Invalid layout (size too large)
        };

        let new_ptr = unsafe { alloc::alloc(layout) };
        if new_ptr.is_null() {
            // Allocation failed, keep buffer in valid empty state
            return false;
        }

        // Install new buffer
        self.data = new_ptr;
        self.length = new_length;
        true
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        self.free();
    }
}