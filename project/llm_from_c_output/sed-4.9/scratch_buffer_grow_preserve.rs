use std::mem;
use std::ptr;
use std::alloc::{self, Layout};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ScratchBufferError;

impl Error for ScratchBufferError {}

impl fmt::Display for ScratchBufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "scratch buffer allocation failed")
    }
}

struct ScratchBuffer {
    data: *mut u8,
    length: usize,
    space: [u8; 1024], // Assuming default stack size, adjust as needed
}

impl ScratchBuffer {
    fn new() -> Self {
        ScratchBuffer {
            data: ptr::null_mut(),
            length: 0,
            space: [0; 1024],
        }
    }

    fn grow_preserve(&mut self) -> Result<(), ScratchBufferError> {
        let new_length = match self.length.checked_mul(2) {
            Some(len) => len,
            None => return Err(ScratchBufferError),
        };

        let new_ptr = if self.data.is_null() {
            // Move buffer to the heap
            let layout = Layout::from_size_align(new_length, 1).map_err(|_| ScratchBufferError)?;
            let new_ptr = unsafe { alloc::alloc(layout) };
            if new_ptr.is_null() {
                return Err(ScratchBufferError);
            }
            unsafe {
                ptr::copy_nonoverlapping(self.space.as_ptr(), new_ptr, self.length);
            }
            new_ptr
        } else {
            // Buffer was already on the heap
            let layout = Layout::from_size_align(self.length, 1).map_err(|_| ScratchBufferError)?;
            let new_ptr = unsafe { alloc::realloc(self.data, layout, new_length) };
            if new_ptr.is_null() {
                unsafe {
                    alloc::dealloc(self.data, layout);
                }
                *self = ScratchBuffer::new();
                return Err(ScratchBufferError);
            }
            new_ptr
        };

        self.data = new_ptr;
        self.length = new_length;
        Ok(())
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        if !self.data.is_null() {
            let layout = Layout::from_size_align(self.length, 1).unwrap();
            unsafe {
                alloc::dealloc(self.data, layout);
            }
        }
    }
}