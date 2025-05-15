use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{self, NonNull};
use std::mem;
use std::fmt;

const DEFAULT_ALIGNMENT: usize = mem::align_of::<usize>();
const DEFAULT_ROUNDING: usize = mem::size_of::<usize>();

struct Chunk {
    limit: *mut u8,
    prev: Option<Box<Chunk>>,
    storage: [u8; 0],
}

pub struct Obstack {
    chunk_size: usize,
    chunk: Option<Box<Chunk>>,
    object_base: *mut u8,
    next_free: *mut u8,
    chunk_limit: *mut u8,
    alignment_mask: usize,
    maybe_empty_object: bool,
    alloc_failed: bool,
}

impl Obstack {
    pub fn new() -> Self {
        Self::with_size(0)
    }

    pub fn with_size(size: usize) -> Self {
        let mut obstack = Obstack {
            chunk_size: 0,
            chunk: None,
            object_base: ptr::null_mut(),
            next_free: ptr::null_mut(),
            chunk_limit: ptr::null_mut(),
            alignment_mask: DEFAULT_ALIGNMENT - 1,
            maybe_empty_object: false,
            alloc_failed: false,
        };
        obstack.begin(size, DEFAULT_ALIGNMENT);
        obstack
    }

    fn begin(&mut self, size: usize, alignment: usize) {
        let alignment = if alignment == 0 { DEFAULT_ALIGNMENT } else { alignment };
        let size = if size == 0 {
            // Default size calculation
            4096 - (((12 + DEFAULT_ROUNDING - 1) & !(DEFAULT_ROUNDING - 1)) + 4 + DEFAULT_ROUNDING - 1) & !(DEFAULT_ROUNDING - 1)
        } else {
            size
        };

        self.chunk_size = size;
        self.alignment_mask = alignment - 1;

        if let Some(chunk) = self.new_chunk(size) {
            self.chunk = Some(chunk);
            let aligned_ptr = self.align_ptr(chunk.storage.as_ptr() as *mut u8, alignment);
            self.object_base = aligned_ptr;
            self.next_free = aligned_ptr;
            self.chunk_limit = unsafe { chunk.storage.as_ptr().add(size) } as *mut u8;
            self.maybe_empty_object = false;
            self.alloc_failed = false;
        } else {
            self.handle_alloc_failure();
        }
    }

    fn new_chunk(&self, size: usize) -> Option<Box<Chunk>> {
        let layout = Layout::from_size_align(size, DEFAULT_ALIGNMENT).ok()?;
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return None;
        }

        let chunk = Box::new(Chunk {
            limit: unsafe { ptr.add(size) },
            prev: None,
            storage: [],
        });

        Some(chunk)
    }

    fn align_ptr(&self, ptr: *mut u8, alignment: usize) -> *mut u8 {
        let mask = alignment - 1;
        ((ptr as usize + mask) & !mask) as *mut u8
    }

    fn handle_alloc_failure(&mut self) {
        self.alloc_failed = true;
        panic!("memory exhausted");
    }

    pub fn base(&self) -> *mut u8 {
        self.object_base
    }

    pub fn grow(&mut self, data: &[u8]) {
        let needed = data.len();
        if self.room() < needed {
            self.new_chunk_with_copy(needed);
        }
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), self.next_free, needed);
            self.next_free = self.next_free.add(needed);
        }
    }

    fn room(&self) -> usize {
        unsafe { self.chunk_limit.offset_from(self.next_free) as usize }
    }

    fn new_chunk_with_copy(&mut self, needed: usize) {
        let old_chunk = self.chunk.take().unwrap();
        let obj_size = unsafe { self.next_free.offset_from(self.object_base) } as usize;
        let new_size = (obj_size + needed + self.alignment_mask).max(self.chunk_size);

        if let Some(mut new_chunk) = self.new_chunk(new_size) {
            let aligned_ptr = self.align_ptr(new_chunk.storage.as_ptr() as *mut u8, self.alignment_mask + 1);
            
            unsafe {
                ptr::copy_nonoverlapping(self.object_base, aligned_ptr, obj_size);
            }

            new_chunk.prev = Some(old_chunk);
            self.chunk = Some(new_chunk);
            self.object_base = aligned_ptr;
            self.next_free = unsafe { aligned_ptr.add(obj_size) };
            self.chunk_limit = unsafe { new_chunk.storage.as_ptr().add(new_size) } as *mut u8;
            self.maybe_empty_object = false;
        } else {
            self.handle_alloc_failure();
        }
    }

    pub fn finish(&mut self) -> *mut u8 {
        let result = self.object_base;
        if self.next_free == self.object_base {
            self.maybe_empty_object = true;
        }

        self.next_free = self.align_ptr(self.object_base, self.alignment_mask + 1);
        if unsafe { self.next_free.offset_from(self.chunk.as_ref().unwrap().storage.as_ptr()) } as usize > 
           unsafe { self.chunk_limit.offset_from(self.chunk.as_ref().unwrap().storage.as_ptr()) } as usize {
            self.next_free = self.chunk_limit;
        }
        self.object_base = self.next_free;
        result
    }

    pub fn free(&mut self, obj: *mut u8) {
        if obj > self.chunk.as_ref().unwrap().storage.as_ptr() as *mut u8 && 
           obj < self.chunk_limit {
            self.next_free = obj;
            self.object_base = obj;
        } else {
            self.free_all(obj);
        }
    }

    fn free_all(&mut self, obj: *mut u8) {
        while let Some(mut chunk) = self.chunk.take() {
            if obj >= chunk.storage.as_ptr() as *mut u8 && obj < chunk.limit {
                self.chunk = Some(chunk);
                self.object_base = obj;
                self.next_free = obj;
                self.chunk_limit = chunk.limit;
                break;
            }
            self.chunk = chunk.prev.take();
            self.maybe_empty_object = true;
        }
    }
}

impl Drop for Obstack {
    fn drop(&mut self) {
        while let Some(chunk) = self.chunk.take() {
            let layout = Layout::from_size_align(unsafe { chunk.limit.offset_from(chunk.storage.as_ptr()) } as usize, 
                                               DEFAULT_ALIGNMENT).unwrap();
            unsafe {
                dealloc(chunk.storage.as_ptr() as *mut u8, layout);
            }
            self.chunk = chunk.prev;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_allocation() {
        let mut obstack = Obstack::new();
        let data = [1, 2, 3, 4];
        obstack.grow(&data);
        let ptr = obstack.finish();
        unsafe {
            assert_eq!(*ptr, 1);
            assert_eq!(*ptr.add(1), 2);
            assert_eq!(*ptr.add(2), 3);
            assert_eq!(*ptr.add(3), 4);
        }
    }

    #[test]
    fn multiple_chunks() {
        let mut obstack = Obstack::with_size(8); // Small chunk size for testing
        let data1 = [1, 2, 3, 4];
        let data2 = [5, 6, 7, 8, 9, 10]; // Will trigger new chunk
        obstack.grow(&data1);
        obstack.grow(&data2);
        let ptr = obstack.finish();
        unsafe {
            assert_eq!(*ptr.add(4), 5); // Verify data from second chunk
        }
    }
}