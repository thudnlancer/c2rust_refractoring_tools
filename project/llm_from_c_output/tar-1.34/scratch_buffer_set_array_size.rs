use std::mem;
use std::io::{Error, ErrorKind};
use std::ptr::NonNull;

/// A variable-sized buffer with on-stack default allocation.
pub struct ScratchBuffer {
    data: Option<NonNull<u8>>,
    length: usize,
}

impl ScratchBuffer {
    /// Creates a new, empty scratch buffer.
    pub fn new() -> Self {
        ScratchBuffer {
            data: None,
            length: 0,
        }
    }

    /// Frees the allocated memory if any.
    pub fn free(&mut self) {
        if let Some(ptr) = self.data {
            unsafe {
                // SAFETY: We own the data and it was allocated with malloc
                let _ = Vec::from_raw_parts(ptr.as_ptr(), 0, self.length);
            }
        }
        self.data = None;
        self.length = 0;
    }

    /// Attempts to resize the buffer to hold `nelem` elements of `size` bytes each.
    pub fn set_array_size(&mut self, nelem: usize, size: usize) -> Result<(), Error> {
        let new_length = match nelem.checked_mul(size) {
            Some(len) => len,
            None => {
                self.free();
                return Err(Error::new(ErrorKind::OutOfMemory, "size overflow"));
            }
        };

        if new_length <= self.length {
            return Ok(());
        }

        self.free();

        let mut new_vec = match Vec::<u8>::try_with_capacity(new_length) {
            Ok(v) => v,
            Err(_) => {
                return Err(Error::new(ErrorKind::OutOfMemory, "allocation failed"));
            }
        };

        // Prevent Vec from dropping the memory when it goes out of scope
        new_vec.set_len(new_length);
        let ptr = NonNull::new(new_vec.as_mut_ptr()).unwrap();
        mem::forget(new_vec);

        self.data = Some(ptr);
        self.length = new_length;

        Ok(())
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        self.free();
    }
}