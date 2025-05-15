use std::mem;
use std::ptr;
use std::slice;

pub type NettleReallocFunc = fn(ctx: *mut u8, ptr: *mut u8, size: usize) -> *mut u8;

pub struct NettleBuffer {
    contents: *mut u8,
    alloc: usize,
    realloc_ctx: *mut u8,
    realloc: Option<NettleReallocFunc>,
    size: usize,
}

impl NettleBuffer {
    pub fn init(&mut self) {
        self.contents = ptr::null_mut();
        self.alloc = 0;
        self.realloc = Some(default_realloc);
        self.realloc_ctx = ptr::null_mut();
        self.size = 0;
    }

    pub fn init_realloc(&mut self, realloc_ctx: *mut u8, realloc: NettleReallocFunc) {
        self.contents = ptr::null_mut();
        self.alloc = 0;
        self.realloc = Some(realloc);
        self.realloc_ctx = realloc_ctx;
        self.size = 0;
    }

    pub fn init_size(&mut self, length: usize, space: *mut u8) {
        self.contents = space;
        self.alloc = length;
        self.realloc = None;
        self.realloc_ctx = ptr::null_mut();
        self.size = 0;
    }

    pub fn clear(&mut self) {
        if let Some(realloc) = self.realloc {
            realloc(self.realloc_ctx, self.contents, 0);
        }
        self.contents = ptr::null_mut();
        self.alloc = 0;
        self.size = 0;
    }

    pub fn reset(&mut self) {
        self.size = 0;
    }

    pub fn grow(&mut self, length: usize) -> bool {
        assert!(self.size <= self.alloc);

        if self.size + length > self.alloc {
            let Some(realloc) = self.realloc else {
                return false;
            };

            let alloc = self.alloc * 2 + length + 100;
            let p = realloc(self.realloc_ctx, self.contents, alloc);
            if p.is_null() {
                return false;
            }

            self.contents = p;
            self.alloc = alloc;
        }
        true
    }

    pub fn putc(&mut self, c: u8) -> bool {
        if self.size < self.alloc || self.grow(1) {
            unsafe {
                *self.contents.add(self.size) = c;
            }
            self.size += 1;
            true
        } else {
            false
        }
    }

    pub fn write(&mut self, data: &[u8]) -> bool {
        let length = data.len();
        if let Some(space) = self.space(length) {
            unsafe {
                ptr::copy_nonoverlapping(data.as_ptr(), space, length);
            }
            true
        } else {
            false
        }
    }

    pub fn space(&mut self, length: usize) -> Option<*mut u8> {
        if !self.grow(length) {
            return None;
        }
        let p = unsafe { self.contents.add(self.size) };
        self.size += length;
        Some(p)
    }

    pub fn copy(&mut self, src: &NettleBuffer) -> bool {
        self.write(unsafe { slice::from_raw_parts(src.contents, src.size) })
    }
}

fn default_realloc(_ctx: *mut u8, ptr: *mut u8, size: usize) -> *mut u8 {
    unsafe {
        if size == 0 {
            if !ptr.is_null() {
                libc::free(ptr as *mut libc::c_void);
            }
            ptr::null_mut()
        } else {
            libc::realloc(ptr as *mut libc::c_void, size) as *mut u8
        }
    }
}

impl Drop for NettleBuffer {
    fn drop(&mut self) {
        self.clear();
    }
}