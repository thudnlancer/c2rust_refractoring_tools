use std::ptr;
use std::slice;
use std::alloc::{self, Layout};

pub type size_t = usize;
pub type uint8_t = u8;

pub type NettleReallocFunc = fn(*mut std::ffi::c_void, size_t) -> *mut std::ffi::c_void;

#[derive(Clone)]
pub struct NettleBuffer {
    contents: *mut uint8_t,
    alloc: size_t,
    realloc_ctx: *mut std::ffi::c_void,
    realloc: Option<NettleReallocFunc>,
    size: size_t,
}

impl NettleBuffer {
    pub fn new() -> Self {
        Self {
            contents: ptr::null_mut(),
            alloc: 0,
            realloc_ctx: ptr::null_mut(),
            realloc: None,
            size: 0,
        }
    }

    pub fn init_realloc(&mut self, realloc_ctx: *mut std::ffi::c_void, realloc: Option<NettleReallocFunc>) {
        self.contents = ptr::null_mut();
        self.alloc = 0;
        self.realloc = realloc;
        self.realloc_ctx = realloc_ctx;
        self.size = 0;
    }

    pub fn init_size(&mut self, length: size_t, space: *mut uint8_t) {
        self.contents = space;
        self.alloc = length;
        self.realloc = None;
        self.realloc_ctx = ptr::null_mut();
        self.size = 0;
    }

    pub fn clear(&mut self) {
        if let Some(realloc) = self.realloc {
            realloc(self.realloc_ctx, 0);
        }
        self.contents = ptr::null_mut();
        self.alloc = 0;
        self.size = 0;
    }

    pub fn reset(&mut self) {
        self.size = 0;
    }

    pub fn grow(&mut self, length: size_t) -> bool {
        assert!(self.size <= self.alloc);

        if self.size.wrapping_add(length) > self.alloc {
            if self.realloc.is_none() {
                return false;
            }

            let new_alloc = self.alloc
                .wrapping_mul(2)
                .wrapping_add(length)
                .wrapping_add(100);

            let new_ptr = if let Some(realloc) = self.realloc {
                realloc(self.realloc_ctx, new_alloc)
            } else {
                ptr::null_mut()
            };

            if new_ptr.is_null() {
                return false;
            }

            self.contents = new_ptr as *mut uint8_t;
            self.alloc = new_alloc;
        }
        true
    }

    pub fn space(&mut self, length: size_t) -> Option<&mut [u8]> {
        if !self.grow(length) {
            return None;
        }

        let start = self.size;
        self.size = self.size.wrapping_add(length);
        
        unsafe {
            Some(slice::from_raw_parts_mut(self.contents.add(start), length))
        }
    }

    pub fn write(&mut self, data: &[u8]) -> bool {
        if let Some(space) = self.space(data.len()) {
            space.copy_from_slice(data);
            true
        } else {
            false
        }
    }

    pub fn copy(&mut self, src: &Self) -> bool {
        self.write(unsafe { slice::from_raw_parts(src.contents, src.size) })
    }

    pub fn contents(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.contents, self.size) }
    }
}

impl Default for NettleBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for NettleBuffer {
    fn drop(&mut self) {
        self.clear();
    }
}