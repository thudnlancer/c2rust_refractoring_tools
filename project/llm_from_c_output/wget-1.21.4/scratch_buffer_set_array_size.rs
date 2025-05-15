use std::mem;
use std::io::{Error, ErrorKind};
use std::ptr;

struct ScratchBuffer {
    data: Vec<u8>,
    length: usize,
}

impl ScratchBuffer {
    fn new() -> Self {
        ScratchBuffer {
            data: Vec::new(),
            length: 0,
        }
    }

    fn free(&mut self) {
        self.data = Vec::new();
        self.length = 0;
    }

    fn init(&mut self) {
        self.free();
    }
}

fn scratch_buffer_set_array_size(buffer: &mut ScratchBuffer, nelem: usize, size: usize) -> Result<(), Error> {
    let new_length = match nelem.checked_mul(size) {
        Some(len) => len,
        None => {
            buffer.free();
            buffer.init();
            return Err(Error::new(ErrorKind::OutOfMemory, "size overflow"));
        }
    };

    if new_length <= buffer.length {
        return Ok(());
    }

    buffer.free();

    let mut new_data = Vec::with_capacity(new_length);
    if new_data.capacity() < new_length {
        buffer.init();
        return Err(Error::new(ErrorKind::OutOfMemory, "allocation failed"));
    }

    unsafe {
        new_data.set_len(new_length);
    }

    buffer.data = new_data;
    buffer.length = new_length;
    Ok(())
}