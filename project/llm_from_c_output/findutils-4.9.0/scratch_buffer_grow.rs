use std::alloc::{alloc, Layout};
use std::mem;
use std::ptr;

/// A variable-sized buffer with on-stack default allocation.
pub struct ScratchBuffer {
    data: *mut u8,
    length: usize,
}

impl ScratchBuffer {
    /// Creates a new, empty scratch buffer.
    pub fn new() -> Self {
        ScratchBuffer {
            data: ptr::null_mut(),
            length: 0,
        }
    }

    /// Frees the buffer's memory if it was allocated on the heap.
    pub fn free(&mut self) {
        if !self.data.is_null() {
            unsafe {
                let layout = Layout::from_size_align_unchecked(self.length, mem::align_of::<u8>());
                alloc::dealloc(self.data, layout);
            }
            self.data = ptr::null_mut();
            self.length = 0;
        }
    }

    /// Attempts to grow the buffer by doubling its size.
    /// Returns true if successful, false if allocation failed.
    pub fn grow(&mut self) -> bool {
        let new_length = match self.length.checked_mul(2) {
            Some(len) => len,
            None => {
                // On overflow, set length to 0 and return failure
                self.free();
                return false;
            }
        };

        // Free old buffer before allocating new one
        self.free();

        let new_ptr = if new_length > 0 {
            let layout = match Layout::from_size_align(new_length, mem::align_of::<u8>()) {
                Ok(layout) => layout,
                Err(_) => {
                    // Invalid layout (shouldn't happen since we checked for overflow)
                    return false;
                }
            };

            unsafe { alloc::alloc(layout) }
        } else {
            ptr::null_mut()
        };

        if new_ptr.is_null() {
            // Allocation failed, keep buffer in valid state
            self.free();
            false
        } else {
            // Success, update buffer with new allocation
            self.data = new_ptr;
            self.length = new_length;
            true
        }
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        self.free();
    }
}