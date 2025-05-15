use std::mem;
use std::io::Error;
use std::io::ErrorKind;

/// A variable-sized buffer with on-stack default allocation.
pub struct ScratchBuffer {
    data: Vec<u8>,
    length: usize,
}

impl ScratchBuffer {
    /// Creates a new empty scratch buffer.
    pub fn new() -> Self {
        ScratchBuffer {
            data: Vec::new(),
            length: 0,
        }
    }

    /// Sets the buffer size to accommodate an array of `nelem` elements each of `size` bytes.
    /// Returns Ok(()) on success or Err(ENOMEM) on allocation failure.
    pub fn set_array_size(&mut self, nelem: usize, size: usize) -> Result<(), Error> {
        let new_length = match nelem.checked_mul(size) {
            Some(len) => len,
            None => {
                // Overflow occurred
                self.free();
                return Err(Error::new(ErrorKind::OutOfMemory, "Allocation size overflow"));
            }
        };

        if new_length <= self.length {
            return Ok(());
        }

        // Discard old buffer
        self.free();

        // Allocate new buffer
        let mut new_data = Vec::with_capacity(new_length);
        unsafe {
            new_data.set_len(new_length);
        }

        // Install new heap-based buffer
        self.data = new_data;
        self.length = new_length;
        Ok(())
    }

    /// Frees the allocated buffer, resetting to empty state.
    pub fn free(&mut self) {
        self.data.clear();
        self.length = 0;
    }

    /// Initializes the buffer to empty state.
    pub fn init(&mut self) {
        self.free();
    }
}

impl Default for ScratchBuffer {
    fn default() -> Self {
        Self::new()
    }
}