use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{self, NonNull};
use std::mem;
use std::fmt;

const DEFAULT_ALIGNMENT: usize = mem::align_of::<usize>();
const DEFAULT_ROUNDING: usize = mem::size_of::<usize>();

struct Chunk {
    limit: *mut u8,
    prev: Option<Box<Chunk>>,
    data: [u8; 0], // Flexible array member
}

pub struct Obstack {
    chunk_size: usize,
    chunk: Option<Box<Chunk>>,
    object_base: *mut u8,
    next_free: *mut u8,
    chunk_limit: *mut u8,
    alignment_mask: usize,
    chunk_alloc: Box<dyn Fn(usize) -> *mut u8>,
    chunk_free: Box<dyn Fn(*mut u8, usize)>,
}

impl Obstack {
    pub fn new(
        chunk_size: usize,
        alignment: usize,
        chunk_alloc: impl Fn(usize) -> *mut u8 + 'static,
        chunk_free: impl Fn(*mut u8, usize) + 'static,
    ) -> Self {
        let alignment_mask = if alignment == 0 {
            DEFAULT_ALIGNMENT - 1
        } else {
            alignment - 1
        };

        let size = if chunk_size == 0 {
            4096 - (((12 + DEFAULT_ROUNDING - 1) & !(DEFAULT_ROUNDING - 1)) + 4 + DEFAULT_ROUNDING - 1) & !(DEFAULT_ROUNDING - 1)
        } else {
            chunk_size
        };

        let chunk_ptr = (chunk_alloc)(size);
        if chunk_ptr.is_null() {
            panic!("memory exhausted");
        }

        let chunk = Box::new(Chunk {
            limit: unsafe { chunk_ptr.add(size) },
            prev: None,
            data: [],
        });

        let base = unsafe {
            Self::align_ptr(
                chunk_ptr.add(mem::size_of::<Chunk>()),
                chunk_ptr.add(mem::size_of::<Chunk>()),
                alignment_mask,
            )
        };

        Obstack {
            chunk_size: size,
            chunk: Some(chunk),
            object_base: base,
            next_free: base,
            chunk_limit: unsafe { chunk_ptr.add(size) },
            alignment_mask,
            chunk_alloc: Box::new(chunk_alloc),
            chunk_free: Box::new(chunk_free),
        }
    }

    fn align_ptr(base: *mut u8, ptr: *mut u8, mask: usize) -> *mut u8 {
        unsafe { base.add((ptr as usize - base as usize + mask) & !mask) }
    }

    pub fn base(&self) -> *mut u8 {
        self.object_base
    }

    pub fn size(&self) -> usize {
        unsafe { self.next_free.offset_from(self.object_base) as usize }
    }

    pub fn room(&self) -> usize {
        unsafe { self.chunk_limit.offset_from(self.next_free) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.chunk.as_ref().map_or(true, |chunk| {
            chunk.prev.is_none()
                && self.next_free
                    == Self::align_ptr(
                        chunk as *const _ as *mut u8,
                        chunk.data.as_ptr() as *mut u8,
                        self.alignment_mask,
                    )
        })
    }

    pub fn grow(&mut self, data: &[u8]) {
        let len = data.len();
        if self.room() < len {
            self.new_chunk(len);
        }
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), self.next_free, len);
            self.next_free = self.next_free.add(len);
        }
    }

    pub fn grow_zero(&mut self, data: &[u8]) {
        let len = data.len();
        if self.room() < len + 1 {
            self.new_chunk(len + 1);
        }
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), self.next_free, len);
            self.next_free = self.next_free.add(len);
            *self.next_free = 0;
            self.next_free = self.next_free.add(1);
        }
    }

    pub fn alloc(&mut self, size: usize) -> *mut u8 {
        self.blank(size);
        self.finish()
    }

    pub fn blank(&mut self, size: usize) {
        if self.room() < size {
            self.new_chunk(size);
        }
        unsafe {
            self.next_free = self.next_free.add(size);
        }
    }

    pub fn finish(&mut self) -> *mut u8 {
        let result = self.object_base;
        if self.next_free == self.object_base {
            // Mark as possibly containing empty object
        }

        self.next_free = Self::align_ptr(
            self.object_base,
            self.next_free,
            self.alignment_mask,
        );

        if unsafe { self.next_free.offset_from(self.chunk.as_ref().unwrap().as_ref() as *const _ as *mut u8) as usize }
            > unsafe { self.chunk_limit.offset_from(self.chunk.as_ref().unwrap().as_ref() as *const _ as *mut u8) as usize }
        {
            self.next_free = self.chunk_limit;
        }

        self.object_base = self.next_free;
        result
    }

    fn new_chunk(&mut self, length: usize) {
        let old_chunk = self.chunk.take().unwrap();
        let obj_size = unsafe { self.next_free.offset_from(self.object_base) as usize };

        let sum1 = obj_size + length;
        let sum2 = sum1 + self.alignment_mask;
        let mut new_size = sum2 + (obj_size >> 3) + 100;
        if new_size < sum2 {
            new_size = sum2;
        }
        if new_size < self.chunk_size {
            new_size = self.chunk_size;
        }

        let new_chunk_ptr = (self.chunk_alloc)(new_size);
        if new_chunk_ptr.is_null() {
            panic!("memory exhausted");
        }

        let mut new_chunk = Box::new(Chunk {
            limit: unsafe { new_chunk_ptr.add(new_size) },
            prev: Some(old_chunk),
            data: [],
        });

        let object_base = unsafe {
            Self::align_ptr(
                new_chunk_ptr.add(mem::size_of::<Chunk>()),
                new_chunk_ptr.add(mem::size_of::<Chunk>()),
                self.alignment_mask,
            )
        };

        unsafe {
            ptr::copy_nonoverlapping(
                self.object_base,
                object_base,
                obj_size,
            );
        }

        self.chunk = Some(new_chunk);
        self.object_base = object_base;
        self.next_free = unsafe { object_base.add(obj_size) };
        self.chunk_limit = unsafe { new_chunk_ptr.add(new_size) };
    }

    pub fn free(&mut self, obj: *mut u8) {
        let mut current_chunk = self.chunk.take();
        while let Some(mut chunk) = current_chunk {
            if obj >= chunk.as_ref() as *const _ as *mut u8
                && obj < chunk.limit
            {
                self.object_base = obj;
                self.next_free = obj;
                self.chunk_limit = chunk.limit;
                self.chunk = Some(chunk);
                return;
            }

            current_chunk = chunk.prev;
            (self.chunk_free)(chunk.as_mut() as *mut _ as *mut u8, self.chunk_size);
        }

        if !obj.is_null() {
            panic!("object not in obstack");
        }
    }

    pub fn memory_used(&self) -> usize {
        let mut total = 0;
        let mut current = self.chunk.as_ref();
        while let Some(chunk) = current {
            total += unsafe { chunk.limit.offset_from(chunk as *const _ as *mut u8) as usize };
            current = chunk.prev.as_ref();
        }
        total
    }
}

impl Drop for Obstack {
    fn drop(&mut self) {
        let mut current = self.chunk.take();
        while let Some(chunk) = current {
            current = chunk.prev;
            (self.chunk_free)(chunk.as_ref() as *const _ as *mut u8, self.chunk_size);
        }
    }
}

// Default allocator functions
fn default_alloc(size: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align(size, DEFAULT_ALIGNMENT).unwrap();
        alloc(layout)
    }
}

fn default_free(ptr: *mut u8, size: usize) {
    unsafe {
        let layout = Layout::from_size_align(size, DEFAULT_ALIGNMENT).unwrap();
        dealloc(ptr, layout);
    }
}

pub fn obstack_init() -> Obstack {
    Obstack::new(0, 0, default_alloc, default_free)
}