use std::alloc::{self, Layout};
use std::mem;
use std::ptr;

/// Variable-sized buffer with on-stack default allocation.
pub struct ScratchBuffer {
    data: *mut u8,
    length: usize,
}

impl ScratchBuffer {
    /// Creates a new, empty scratch buffer.
    pub fn new() -> Self {
        Self {
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

    /// Sets the buffer size to accommodate an array of `nelem` elements each of `size` bytes.
    pub fn set_array_size(&mut self, nelem: usize, size: usize) -> Result<(), alloc::LayoutError> {
        let new_length = match nelem.checked_mul(size) {
            Some(len) => len,
            None => {
                self.free();
                return Err(alloc::LayoutError);
            }
        };

        if new_length <= self.length {
            return Ok(());
        }

        self.free();

        let layout = Layout::from_size_align(new_length, mem::align_of::<u8>())?;
        let new_ptr = unsafe { alloc::alloc(layout) };
        if new_ptr.is_null() {
            return Err(alloc::LayoutError);
        }

        self.data = new_ptr;
        self.length = new_length;
        Ok(())
    }
}

impl Default for ScratchBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        self.free();
    }
}