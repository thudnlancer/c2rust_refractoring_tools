use std::alloc::{self, Layout};
use std::mem;
use std::ptr;

#[derive(Clone)]
pub struct ScratchBuffer {
    data: *mut u8,
    length: usize,
    space: [u8; 1024],
}

impl ScratchBuffer {
    pub fn new() -> Self {
        let mut buffer = ScratchBuffer {
            data: ptr::null_mut(),
            length: 0,
            space: [0; 1024],
        };
        buffer.init();
        buffer
    }

    fn init(&mut self) {
        self.data = self.space.as_mut_ptr();
        self.length = mem::size_of_val(&self.space);
    }

    pub fn free(&mut self) {
        if self.data != self.space.as_mut_ptr() && !self.data.is_null() {
            unsafe {
                alloc::dealloc(
                    self.data,
                    Layout::from_size_align(self.length, mem::align_of::<u8>()).unwrap(),
                );
            }
        }
    }

    pub fn set_array_size(&mut self, nelem: usize, size: usize) -> Result<(), ()> {
        let new_length = nelem.checked_mul(size).ok_or(())?;

        if (nelem | size) > (usize::MAX >> 1) && nelem != 0 && size != new_length / nelem {
            self.free();
            self.init();
            return Err(());
        }

        if new_length <= self.length {
            return Ok(());
        }

        self.free();

        let layout = Layout::from_size_align(new_length, mem::align_of::<u8>()).map_err(|_| ())?;
        let new_ptr = unsafe { alloc::alloc(layout) };

        if new_ptr.is_null() {
            self.init();
            Err(())
        } else {
            self.data = new_ptr;
            self.length = new_length;
            Ok(())
        }
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        self.free();
    }
}