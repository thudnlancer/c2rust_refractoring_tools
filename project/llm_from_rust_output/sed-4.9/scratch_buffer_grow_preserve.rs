use std::alloc::{self, Layout};
use std::mem;
use std::ptr::{self, NonNull};
use std::slice;

const INITIAL_BUFFER_SIZE: usize = 1024;

#[derive(Debug)]
pub struct ScratchBuffer {
    data: Option<NonNull<u8>>,
    length: usize,
    stack_space: [u8; INITIAL_BUFFER_SIZE],
}

impl ScratchBuffer {
    pub fn new() -> Self {
        ScratchBuffer {
            data: None,
            length: INITIAL_BUFFER_SIZE,
            stack_space: [0; INITIAL_BUFFER_SIZE],
        }
    }

    pub fn grow_preserve(&mut self) -> Result<(), alloc::LayoutError> {
        let new_length = self.length.checked_mul(2).ok_or(alloc::LayoutError)?;
        let layout = Layout::from_size_align(new_length, mem::align_of::<u8>())?;

        let new_ptr = if let Some(ptr) = self.data {
            unsafe {
                let old_ptr = ptr.as_ptr();
                let new_ptr = alloc::realloc(old_ptr, layout, new_length);
                if new_ptr.is_null() {
                    alloc::dealloc(old_ptr, layout);
                    self.data = None;
                    self.length = INITIAL_BUFFER_SIZE;
                    return Err(alloc::LayoutError);
                }
                NonNull::new(new_ptr).ok_or(alloc::LayoutError)?
            }
        } else {
            let new_ptr = unsafe { alloc::alloc(layout) };
            if new_ptr.is_null() {
                return Err(alloc::LayoutError);
            }
            unsafe {
                ptr::copy_nonoverlapping(
                    self.stack_space.as_ptr(),
                    new_ptr,
                    self.stack_space.len(),
                );
            }
            NonNull::new(new_ptr).ok_or(alloc::LayoutError)?
        };

        self.data = Some(new_ptr);
        self.length = new_length;
        Ok(())
    }

    pub fn as_slice(&self) -> &[u8] {
        if let Some(ptr) = self.data {
            unsafe { slice::from_raw_parts(ptr.as_ptr(), self.length) }
        } else {
            &self.stack_space[..self.length]
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if let Some(ptr) = self.data {
            unsafe { slice::from_raw_parts_mut(ptr.as_ptr(), self.length) }
        } else {
            &mut self.stack_space[..self.length]
        }
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        if let Some(ptr) = self.data {
            unsafe {
                alloc::dealloc(
                    ptr.as_ptr(),
                    Layout::from_size_align(self.length, mem::align_of::<u8>())
                        .expect("Invalid layout"),
                );
            }
        }
    }
}