use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{self, NonNull};
use std::mem;
use std::os::raw::c_void;
use std::cmp::max;

const DEFAULT_ALIGNMENT: usize = max(
    mem::align_of::<f64>(), // approximation for long double
    max(
        mem::align_of::<usize>(), // approximation for uintmax_t
        mem::align_of::<*mut c_void>(),
    ),
);

const DEFAULT_ROUNDING: usize = max(
    mem::size_of::<f64>(), // approximation for long double
    max(
        mem::size_of::<usize>(), // approximation for uintmax_t
        mem::size_of::<*mut c_void>(),
    ),
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
        chunk_alloc: Box<dyn Fn(usize) -> *mut c_void>,
        chunk_free: Box<dyn Fn(*mut c_void)>,
    ) -> Result<Self, &'static str> {
        let alignment = if alignment == 0 {
            DEFAULT_ALIGNMENT
        } else {
            alignment
        };

        let size = if size == 0 {
            // Default size calculation
            let extra = (((12 + DEFAULT_ROUNDING - 1) & !(DEFAULT_ROUNDING - 1))
                + 4 + DEFAULT_ROUNDING - 1)
                & !(DEFAULT_ROUNDING - 1);
            4096 - extra
        } else {
            size
        };

        let mut obstack = Obstack {
            chunk_size: size,
            chunk: None,
            object_base: ptr::null_mut(),
            next_free: ptr::null_mut(),
            chunk_limit: ptr::null_mut(),
            alignment_mask: alignment - 1,
            maybe_empty_object: false,
            alloc_failed: false,
            chunk_alloc,
            chunk_free,
        };

        obstack.new_chunk(size)?;
        Ok(obstack)
    }

    fn new_chunk(&mut self, size: usize) -> Result<(), &'static str> {
        let layout = Layout::from_size_align(size, 1).unwrap();
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err("memory allocation failed");
        }

        let chunk = Box::new(Chunk {
            limit: unsafe { ptr.add(size) },
            prev: self.chunk.take(),
            contents: [],
        });

        let contents_ptr = ptr as *mut u8;
        self.object_base = Self::align_ptr(contents_ptr, self.alignment_mask);
        self.next_free = self.object_base;
        self.chunk_limit = unsafe { ptr.add(size) };
        self.chunk = Some(chunk);
        self.maybe_empty_object = false;

        Ok(())
    }

    fn align_ptr(ptr: *mut u8, mask: usize) -> *mut u8 {
        ((ptr as usize + mask) & !mask) as *mut u8
    }

    pub fn base(&self) -> *mut c_void {
        self.object_base as *mut c_void
    }

    pub fn size(&self) -> usize {
        unsafe { self.next_free.offset_from(self.object_base) as usize }
    }

    pub fn room(&self) -> usize {
        unsafe { self.chunk_limit.offset_from(self.next_free) as usize }
    }

    pub fn make_room(&mut self, length: usize) -> Result<(), &'static str> {
        if self.room() < length {
            self.grow(length)?;
        }
        Ok(())
    }

    fn grow(&mut self, length: usize) -> Result<(), &'static str> {
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
            return Err("memory allocation failed");
        }

        let new_chunk = Box::new(Chunk {
            limit: unsafe { new_chunk_ptr.add(new_size) as *mut u8 },
            prev: Some(old_chunk),
            contents: [],
        });

        let new_contents = new_chunk_ptr as *mut u8;
        let new_object_base = Self::align_ptr(new_contents, self.alignment_mask);

        unsafe {
            ptr::copy_nonoverlapping(
                self.object_base,
                new_object_base,
                obj_size,
            );
        }

        self.chunk = Some(new_chunk);
        self.object_base = new_object_base;
        self.next_free = unsafe { new_object_base.add(obj_size) };
        self.chunk_limit = unsafe { new_chunk_ptr.add(new_size) as *mut u8 };
        self.maybe_empty_object = false;

        Ok(())
    }

    pub fn finish(&mut self) -> *mut c_void {
        let value = self.object_base as *mut c_void;
        if self.next_free == self.object_base {
            self.maybe_empty_object = true;
        }

        self.next_free = Self::align_ptr(self.object_base, self.alignment_mask);
        if unsafe { self.next_free.offset_from(self.chunk.unwrap().contents.as_ptr() as *mut u8) as usize }
            > unsafe { self.chunk_limit.offset_from(self.chunk.unwrap().contents.as_ptr() as *mut u8) as usize }
        {
            self.next_free = self.chunk_limit;
        }

        self.object_base = self.next_free;
        value
    }

    pub fn free(&mut self, obj: *mut c_void) {
        let obj_ptr = obj as *mut u8;
        if obj_ptr > self.chunk.as_ref().unwrap().contents.as_ptr() as *mut u8
            && obj_ptr < self.chunk.as_ref().unwrap().limit
        {
            self.next_free = obj_ptr;
            self.object_base = obj_ptr;
        } else {
            self.free_all(obj);
        }
    }

    fn free_all(&mut self, obj: *mut c_void) {
        let mut current_chunk = self.chunk.take();
        while let Some(mut chunk) = current_chunk {
            if (chunk.contents.as_ptr() as *mut u8 >= obj as *mut u8)
                || (chunk.limit < obj as *mut u8)
            {
                current_chunk = chunk.prev.take();
                (self.chunk_free)(chunk as *mut _ as *mut c_void);
                self.maybe_empty_object = true;
            } else {
                self.chunk = Some(chunk);
                break;
            }
        }

        if let Some(chunk) = &self.chunk {
            self.object_base = obj as *mut u8;
            self.next_free = obj as *mut u8;
            self.chunk_limit = chunk.limit;
        } else if !obj.is_null() {
            panic!("object not in any chunk");
        }
    }
}

impl Drop for Obstack {
    fn drop(&mut self) {
        self.free_all(ptr::null_mut());
    }
}