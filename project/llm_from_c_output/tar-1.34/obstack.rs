use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{self, NonNull};
use std::mem;
use std::os::raw::c_void;
use std::cmp::max;

const DEFAULT_ALIGNMENT: usize = max(
    mem::align_of::<f64>(),
    max(mem::align_of::<usize>(), mem::align_of::<*mut c_void>()),
);

const DEFAULT_ROUNDING: usize = max(
    mem::size_of::<f64>(),
    max(mem::size_of::<usize>(), mem::size_of::<*mut c_void>()),
);

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
    chunk_alloc: Box<dyn Fn(usize) -> *mut c_void>,
    chunk_free: Box<dyn Fn(*mut c_void)>,
}

impl Obstack {
    pub fn new(
        size: usize,
        alignment: usize,
        alloc_fn: Box<dyn Fn(usize) -> *mut c_void>,
        free_fn: Box<dyn Fn(*mut c_void)>,
    ) -> Self {
        let alignment = if alignment == 0 {
            DEFAULT_ALIGNMENT
        } else {
            alignment
        };

        let size = if size == 0 {
            4096 - (((12 + DEFAULT_ROUNDING - 1) & !(DEFAULT_ROUNDING - 1))
                + 4 + DEFAULT_ROUNDING - 1
                & !(DEFAULT_ROUNDING - 1)
        } else {
            size
        };

        let chunk = (alloc_fn)(size);
        if chunk.is_null() {
            panic!("memory exhausted");
        }

        let chunk = unsafe { Box::from_raw(chunk as *mut Chunk) };
        let object_base = Self::align_ptr(
            chunk.contents.as_ptr() as *mut u8,
            alignment - 1,
        );
        let chunk_limit = unsafe { chunk.contents.as_ptr().add(size) as *mut u8 };

        Obstack {
            chunk_size: size,
            chunk: Some(chunk),
            object_base,
            next_free: object_base,
            chunk_limit,
            alignment_mask: alignment - 1,
            maybe_empty_object: false,
            alloc_failed: false,
            chunk_alloc: alloc_fn,
            chunk_free: free_fn,
        }
    }

    fn align_ptr(ptr: *mut u8, mask: usize) -> *mut u8 {
        ((ptr as usize + mask) & !mask) as *mut u8
    }

    pub fn base(&self) -> *mut u8 {
        self.object_base
    }

    pub fn next_free(&self) -> *mut u8 {
        self.next_free
    }

    pub fn alignment_mask(&self) -> usize {
        self.alignment_mask
    }

    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    pub fn memory_used(&self) -> usize {
        let mut total = 0;
        let mut chunk = self.chunk.as_ref();
        while let Some(c) = chunk {
            total += unsafe { c.limit.offset_from(c.contents.as_ptr()) } as usize;
            chunk = c.prev.as_ref();
        }
        total
    }

    pub fn new_chunk(&mut self, length: usize) {
        let old_chunk = self.chunk.take().unwrap();
        let obj_size = unsafe { self.next_free.offset_from(self.object_base) } as usize;

        let sum1 = obj_size + length;
        let sum2 = sum1 + self.alignment_mask;
        let mut new_size = sum2 + (obj_size >> 3) + 100;
        new_size = max(new_size, sum2);
        new_size = max(new_size, self.chunk_size);

        let new_chunk = (self.chunk_alloc)(new_size);
        if new_chunk.is_null() {
            panic!("memory exhausted");
        }

        let mut new_chunk = unsafe { Box::from_raw(new_chunk as *mut Chunk) };
        new_chunk.prev = Some(old_chunk);
        new_chunk.limit = unsafe { new_chunk.contents.as_ptr().add(new_size) } as *mut u8;

        let object_base = Self::align_ptr(
            new_chunk.contents.as_ptr() as *mut u8,
            self.alignment_mask,
        );

        unsafe {
            ptr::copy_nonoverlapping(
                self.object_base,
                object_base,
                obj_size,
            );
        }

        if !self.maybe_empty_object
            && self.object_base
                == Self::align_ptr(
                    new_chunk.contents.as_ptr() as *mut u8,
                    self.alignment_mask,
                )
        {
            let prev = new_chunk.prev.take().unwrap();
            (self.chunk_free)(Box::into_raw(prev) as *mut c_void);
        }

        self.chunk = Some(new_chunk);
        self.object_base = object_base;
        self.next_free = unsafe { object_base.add(obj_size) };
        self.chunk_limit = self.chunk.as_ref().unwrap().limit;
        self.maybe_empty_object = false;
    }

    pub fn room(&self) -> usize {
        unsafe { self.chunk_limit.offset_from(self.next_free) } as usize
    }

    pub fn make_room(&mut self, length: usize) {
        if self.room() < length {
            self.new_chunk(length);
        }
    }

    pub fn grow(&mut self, data: &[u8]) {
        let len = data.len();
        self.make_room(len);
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), self.next_free, len);
            self.next_free = self.next_free.add(len);
        }
    }

    pub fn grow0(&mut self, data: &[u8]) {
        let len = data.len();
        self.make_room(len + 1);
        unsafe {
            ptr::copy_nonoverlapping(data.as_ptr(), self.next_free, len);
            self.next_free = self.next_free.add(len);
            *self.next_free = 0;
            self.next_free = self.next_free.add(1);
        }
    }

    pub fn blank(&mut self, length: usize) {
        self.make_room(length);
        unsafe {
            self.next_free = self.next_free.add(length);
        }
    }

    pub fn alloc(&mut self, length: usize) -> *mut u8 {
        self.blank(length);
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

        self.next_free = Self::align_ptr(
            self.object_base,
            self.alignment_mask,
        );

        let chunk = self.chunk.as_ref().unwrap();
        if unsafe { self.next_free.offset_from(chunk.contents.as_ptr()) } as usize
            > unsafe { chunk.limit.offset_from(chunk.contents.as_ptr()) } as usize
        {
            self.next_free = chunk.limit;
        }

        self.object_base = self.next_free;
        value
    }

    pub fn free(&mut self, obj: *mut u8) {
        let mut chunk = self.chunk.take();
        while let Some(mut c) = chunk {
            if obj >= c.contents.as_ptr() as *mut u8
                && obj < c.limit
            {
                self.object_base = obj;
                self.next_free = obj;
                self.chunk_limit = c.limit;
                self.chunk = Some(c);
                return;
            }

            let prev = c.prev.take();
            (self.chunk_free)(Box::into_raw(c) as *mut c_void);
            chunk = prev;
            self.maybe_empty_object = true;
        }

        if !obj.is_null() {
            panic!("object not in obstack");
        }
    }
}

impl Drop for Obstack {
    fn drop(&mut self) {
        let mut chunk = self.chunk.take();
        while let Some(c) = chunk {
            let prev = c.prev;
            (self.chunk_free)(Box::into_raw(c) as *mut c_void);
            chunk = prev;
        }
    }
}

fn default_alloc(size: usize) -> *mut c_void {
    unsafe { alloc(Layout::from_size_align(size, DEFAULT_ALIGNMENT).unwrap() as *mut c_void }
}

fn default_free(ptr: *mut c_void) {
    unsafe {
        dealloc(
            ptr as *mut u8,
            Layout::from_size_align(0, DEFAULT_ALIGNMENT).unwrap(),
        );
    }
}

pub fn obstack_init() -> Obstack {
    Obstack::new(
        0,
        0,
        Box::new(default_alloc),
        Box::new(default_free),
    )
}