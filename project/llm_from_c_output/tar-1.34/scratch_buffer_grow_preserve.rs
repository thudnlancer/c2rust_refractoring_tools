use std::mem;
use std::ptr;
use std::alloc::{self, Layout};
use std::io::Error;

/// Variable-sized buffer with on-stack default allocation.
pub struct ScratchBuffer {
    data: *mut u8,
    length: usize,
    space: [u8; 1024], // Example stack-allocated space, adjust size as needed
}

impl ScratchBuffer {
    /// Initializes a new scratch buffer with stack-allocated space
    pub fn new() -> Self {
        let mut space = [0u8; 1024];
        ScratchBuffer {
            data: space.as_mut_ptr(),
            length: space.len(),
            space,
        }
    }

    /// Grows the buffer while preserving existing content
    pub fn grow_preserve(&mut self) -> Result<(), Error> {
        let new_length = match self.length.checked_mul(2) {
            Some(len) => len,
            None => return Err(Error::new(Error::from_raw_os_error(libc::ENOMEM), "overflow")),
        };

        let new_ptr = if self.data == self.space.as_mut_ptr() {
            // Move buffer to the heap
            let layout = Layout::from_size_align(new_length, 1).map_err(|_| Error::from_raw_os_error(libc::ENOMEM))?;
            let new_ptr = unsafe { alloc::alloc(layout) };
            if new_ptr.is_null() {
                return Err(Error::new(Error::from_raw_os_error(libc::ENOMEM), "allocation failed"));
            }
            unsafe {
                ptr::copy_nonoverlapping(self.data, new_ptr, self.length);
            }
            new_ptr
        } else {
            // Buffer was already on the heap
            let layout = Layout::from_size_align(self.length, 1).map_err(|_| Error::from_raw_os_error(libc::ENOMEM))?;
            let new_ptr = unsafe { alloc::realloc(self.data, layout, new_length) };
            if new_ptr.is_null() {
                unsafe {
                    alloc::dealloc(self.data, layout);
                }
                *self = Self::new();
                return Err(Error::new(Error::from_raw_os_error(libc::ENOMEM), "reallocation failed"));
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
        if self.data != self.space.as_mut_ptr() {
            let layout = Layout::from_size_align(self.length, 1).unwrap();
            unsafe {
                alloc::dealloc(self.data, layout);
            }
        }
    }
}