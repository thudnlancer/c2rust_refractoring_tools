use std::alloc::{self, Layout};
use std::mem;
use std::ptr::{self, NonNull};
use std::io;

#[derive(Clone)]
pub struct ScratchBuffer {
    data: Option<NonNull<u8>>,
    length: usize,
    space: [u8; 1024],
}

impl ScratchBuffer {
    pub fn new() -> Self {
        ScratchBuffer {
            data: None,
            length: 0,
            space: [0; 1024],
        }
    }

    pub fn init(&mut self) {
        self.data = NonNull::new(self.space.as_mut_ptr());
        self.length = mem::size_of_val(&self.space);
    }

    pub fn grow_preserve(&mut self) -> io::Result<()> {
        let new_length = self.length.checked_mul(2).ok_or(io::Error::new(
            io::ErrorKind::Other,
            "size overflow",
        ))?;

        let new_ptr = if let Some(data) = self.data {
            if data.as_ptr() == self.space.as_mut_ptr() {
                // Currently using stack space, need to move to heap
                let layout = Layout::from_size_align(new_length, 1)?;
                let new_ptr = unsafe { alloc::alloc(layout) };
                if new_ptr.is_null() {
                    return Err(io::Error::new(io::ErrorKind::Other, "allocation failed"));
                }
                unsafe {
                    ptr::copy_nonoverlapping(
                        self.space.as_ptr(),
                        new_ptr,
                        self.length,
                    );
                }
                NonNull::new(new_ptr)
            } else {
                // Already on heap, realloc
                let layout = Layout::from_size_align(self.length, 1)?;
                let new_ptr = unsafe {
                    alloc::realloc(data.as_ptr(), layout, new_length)
                };
                if new_ptr.is_null() {
                    return Err(io::Error::new(io::ErrorKind::Other, "reallocation failed"));
                }
                NonNull::new(new_ptr)
            }
        } else {
            // Not initialized, just allocate
            let layout = Layout::from_size_align(new_length, 1)?;
            let new_ptr = unsafe { alloc::alloc(layout) };
            if new_ptr.is_null() {
                return Err(io::Error::new(io::ErrorKind::Other, "allocation failed"));
            }
            NonNull::new(new_ptr)
        };

        if let Some(ptr) = new_ptr {
            self.data = Some(ptr);
            self.length = new_length;
            Ok(())
        } else {
            self.init();
            Err(io::Error::new(io::ErrorKind::Other, "allocation failed"))
        }
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        if let Some(data) = self.data {
            if data.as_ptr() != self.space.as_mut_ptr() {
                let layout = Layout::from_size_align(self.length, 1).unwrap();
                unsafe { alloc::dealloc(data.as_ptr(), layout) };
            }
        }
    }
}