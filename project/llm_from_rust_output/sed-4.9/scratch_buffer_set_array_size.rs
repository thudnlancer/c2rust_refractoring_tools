use std::alloc::{alloc, dealloc, Layout};
use std::mem::{size_of, MaybeUninit};
use std::ptr::NonNull;

#[derive(Clone)]
pub struct ScratchBuffer {
    data: Option<NonNull<u8>>,
    length: usize,
    space: MaybeUninit<[u8; 1024]>,
}

impl ScratchBuffer {
    pub fn new() -> Self {
        Self {
            data: None,
            length: 0,
            space: MaybeUninit::uninit(),
        }
    }

    pub fn init(&mut self) {
        self.data = NonNull::new(self.space.as_mut_ptr() as *mut u8);
        self.length = size_of::<[u8; 1024]>();
    }

    pub fn free(&mut self) {
        if let Some(ptr) = self.data {
            if ptr.as_ptr() != self.space.as_mut_ptr() as *mut u8 {
                unsafe {
                    dealloc(ptr.as_ptr(), Layout::from_size_align(self.length, 1).unwrap());
                }
            }
        }
        self.data = None;
        self.length = 0;
    }

    pub fn set_array_size(&mut self, nelem: usize, size: usize) -> Result<(), std::io::Error> {
        let new_length = nelem.checked_mul(size).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "size overflow when calculating new length",
            )
        })?;

        if new_length <= self.length {
            return Ok(());
        }

        self.free();

        let layout = Layout::from_size_align(new_length, 1).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "invalid layout parameters")
        })?;

        unsafe {
            let new_ptr = alloc(layout);
            if new_ptr.is_null() {
                self.init();
                return Err(std::io::Error::new(
                    std::io::ErrorKind::OutOfMemory,
                    "memory allocation failed",
                ));
            }
            self.data = NonNull::new(new_ptr);
            self.length = new_length;
        }

        Ok(())
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        self.free();
    }
}