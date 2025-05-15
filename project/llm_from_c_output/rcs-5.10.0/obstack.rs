use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicPtr, Ordering};

const DEFAULT_ALIGNMENT: usize = mem::align_of::<usize>();
const DEFAULT_ROUNDING: usize = mem::size_of::<usize>();

struct Chunk {
    limit: *mut u8,
    prev: Option<Box<Chunk>>,
    contents: [u8; 0],
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
    chunk_alloc: Box<dyn Fn(usize) -> *mut u8>,
    chunk_free: Box<dyn Fn(*mut u8)>,
}

impl Obstack {
    pub fn new(
        size: usize,
        alignment: usize,
        chunk_alloc: impl Fn(usize) -> *mut u8 + 'static,
        chunk_free: impl Fn(*mut u8) + 'static,
    ) -> Self {
        let alignment = if alignment == 0 {
            DEFAULT_ALIGNMENT
        } else {
            alignment
        };

        let size = if size == 0 {
            4096 - (((12 + DEFAULT_ROUNDING - 1) & !(DEFAULT_ROUNDING - 1))
                + 4
                + DEFAULT_ROUNDING
                - 1
                & !(DEFAULT_ROUNDING - 1)
        } else {
            size
        };

        let chunk_alloc = Box::new(chunk_alloc);
        let chunk = Some(unsafe {
            let ptr = (chunk_alloc)(size);
            if ptr.is_null() {
                panic!("memory exhausted");
            }
            Box::from_raw(ptr as *mut Chunk)
        });

        let mut obstack = Self {
            chunk_size: size,
            chunk,
            object_base: ptr::null_mut(),
            next_free: ptr::null_mut(),
            chunk_limit: ptr::null_mut(),
            alignment_mask: alignment - 1,
            maybe_empty_object: false,
            alloc_failed: false,
            chunk_alloc,
            chunk_free: Box::new(chunk_free),
        };

        if let Some(chunk) = &obstack.chunk {
            let base = unsafe { ptr_align(chunk.contents.as_ptr(), obstack.alignment_mask) };
            obstack.object_base = base;
            obstack.next_free = base;
            obstack.chunk_limit = unsafe { chunk.contents.as_ptr().add(size) };
        }

        obstack
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
        self.chunk
            .as_ref()
            .map_or(true, |c| c.prev.is_none() && self.next_free == self.object_base)
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

    pub fn grow0(&mut self, data: &[u8]) {
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

    pub fn copy(&mut self, data: &[u8]) -> *mut u8 {
        self.grow(data);
        self.finish()
    }

    pub fn copy0(&mut self, data: &[u8]) -> *mut u8 {
        self.grow0(data);
        self.finish()
    }

    pub fn finish(&mut self) -> *mut u8 {
        let value = self.object_base;
        if self.next_free == value {
            self.maybe_empty_object = true;
        }

        self.next_free = unsafe {
            ptr_align(
                self.object_base,
                self.next_free,
                self.alignment_mask,
            )
        };

        if let Some(chunk) = &self.chunk {
            let chunk_ptr = chunk.contents.as_ptr() as *const u8 as *mut u8;
            let used = unsafe { self.next_free.offset_from(chunk_ptr) as usize };
            let limit = unsafe { self.chunk_limit.offset_from(chunk_ptr) as usize };
            if used > limit {
                self.next_free = self.chunk_limit;
            }
        }

        self.object_base = self.next_free;
        value
    }

    pub fn free(&mut self, obj: *mut u8) {
        if let Some(chunk) = &self.chunk {
            let chunk_ptr = chunk.contents.as_ptr() as *const u8 as *mut u8;
            if obj > chunk_ptr && obj < self.chunk_limit {
                self.next_free = obj;
                self.object_base = obj;
                return;
            }
        }
        self.free_chunks(obj);
    }

    fn new_chunk(&mut self, length: usize) {
        let old_chunk = self.chunk.take().unwrap();
        let obj_size = unsafe { self.next_free.offset_from(self.object_base) as usize };

        let sum1 = obj_size + length;
        let sum2 = sum1 + self.alignment_mask;
        let mut new_size = sum2 + (obj_size >> 3) + 100;
        new_size = new_size.max(sum2).max(self.chunk_size);

        let new_chunk = unsafe {
            let ptr = (self.chunk_alloc)(new_size);
            if ptr.is_null() {
                panic!("memory exhausted");
            }
            Box::from_raw(ptr as *mut Chunk)
        };

        new_chunk.prev = Some(old_chunk);
        self.chunk = Some(new_chunk);
        self.chunk_limit = unsafe { self.chunk.as_ref().unwrap().contents.as_ptr().add(new_size) };

        let object_base = unsafe {
            ptr_align(
                self.chunk.as_ref().unwrap().contents.as_ptr(),
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

        if !self.maybe_empty_object
            && self.object_base
                == unsafe {
                    ptr_align(
                        old_chunk.contents.as_ptr(),
                        self.alignment_mask,
                    )
                }
        {
            let prev = old_chunk.prev;
            (self.chunk_free)(Box::into_raw(old_chunk) as *mut u8);
            self.chunk.as_mut().unwrap().prev = prev;
        }

        self.object_base = object_base;
        self.next_free = unsafe { object_base.add(obj_size) };
        self.maybe_empty_object = false;
    }

    fn free_chunks(&mut self, obj: *mut u8) {
        let mut current = self.chunk.take();
        while let Some(chunk) = current {
            let chunk_ptr = chunk.contents.as_ptr() as *const u8 as *mut u8;
            if obj >= chunk_ptr && obj < chunk.limit {
                self.chunk = Some(chunk);
                self.object_base = obj;
                self.next_free = obj;
                self.chunk_limit = self.chunk.as_ref().unwrap().limit;
                return;
            }
            current = chunk.prev;
            (self.chunk_free)(Box::into_raw(chunk) as *mut u8);
            self.maybe_empty_object = true;
        }

        if !obj.is_null() {
            panic!("object not in any chunk");
        }
    }

    pub fn blank(&mut self, length: usize) {
        if self.room() < length {
            self.new_chunk(length);
        }
        unsafe {
            self.next_free = self.next_free.add(length);
        }
    }
}

unsafe fn ptr_align(base: *const u8, ptr: *const u8, mask: usize) -> *mut u8 {
    let base = base as usize;
    let ptr = ptr as usize;
    ((base + (ptr - base + mask) & !mask) as *mut u8
}

unsafe fn ptr_align(base: *const u8, mask: usize) -> *mut u8 {
    let base = base as usize;
    ((base + mask) & !mask) as *mut u8
}

static ALLOC_FAILED_HANDLER: AtomicPtr<()> = AtomicPtr::new(print_and_abort as *mut ());

extern "C" fn print_and_abort() {
    eprintln!("memory exhausted");
    std::process::exit(1);
}