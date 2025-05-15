use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr::{self, NonNull};

#[derive(Debug)]
pub struct Obstack {
    chunk_size: usize,
    chunk: Option<NonNull<Chunk>>,
    object_base: Option<NonNull<u8>>,
    next_free: Option<NonNull<u8>>,
    chunk_limit: Option<NonNull<u8>>,
    alignment_mask: usize,
    alloc_failed: bool,
    maybe_empty_object: bool,
}

#[derive(Debug)]
struct Chunk {
    limit: NonNull<u8>,
    prev: Option<NonNull<Chunk>>,
    contents: [u8; 0],
}

impl Obstack {
    pub fn new(size: usize, alignment: usize) -> Result<Self, &'static str> {
        let alignment = if alignment == 0 {
            mem::align_of::<usize>()
        } else {
            alignment
        };

        let size = if size == 0 {
            4096 - (12 + mem::size_of::<usize>() - 1) & !(mem::size_of::<usize>() - 1)
        } else {
            size
        };

        let mut obstack = Self {
            chunk_size: size,
            chunk: None,
            object_base: None,
            next_free: None,
            chunk_limit: None,
            alignment_mask: alignment - 1,
            alloc_failed: false,
            maybe_empty_object: false,
        };

        obstack.alloc_chunk()?;
        Ok(obstack)
    }

    fn alloc_chunk(&mut self) -> Result<(), &'static str> {
        let layout = Layout::from_size_align(self.chunk_size, 1).map_err(|_| "invalid layout")?;
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err("allocation failed");
        }

        let chunk_ptr = ptr as *mut Chunk;
        let chunk = unsafe {
            let chunk = Chunk {
                limit: NonNull::new_unchecked(ptr.add(self.chunk_size)),
                prev: None,
                contents: [],
            };
            ptr::write(chunk_ptr, chunk);
            NonNull::new_unchecked(chunk_ptr)
        };

        let object_base = unsafe {
            let contents_ptr = (*chunk_ptr).contents.as_ptr();
            let aligned_ptr = (contents_ptr as usize + self.alignment_mask) & !self.alignment_mask;
            NonNull::new_unchecked(aligned_ptr as *mut u8)
        };

        self.chunk = Some(chunk);
        self.object_base = Some(object_base);
        self.next_free = Some(object_base);
        self.chunk_limit = Some(unsafe { (*chunk_ptr).limit });
        self.maybe_empty_object = false;

        Ok(())
    }

    pub fn alloc(&mut self, size: usize) -> Result<NonNull<u8>, &'static str> {
        if self.next_free.is_none() || self.chunk_limit.is_none() {
            return Err("obstack not initialized");
        }

        let next_free = unsafe { self.next_free.unwrap().as_ptr() };
        let chunk_limit = unsafe { self.chunk_limit.unwrap().as_ptr() };

        if next_free.add(size) > chunk_limit {
            self.grow(size)?;
        }

        let ptr = self.next_free.unwrap();
        self.next_free = NonNull::new(unsafe { ptr.as_ptr().add(size) });
        Ok(ptr)
    }

    fn grow(&mut self, length: usize) -> Result<(), &'static str> {
        let old_chunk = self.chunk.ok_or("no current chunk")?;
        let obj_size = unsafe {
            self.next_free.unwrap().as_ptr().offset_from(self.object_base.unwrap().as_ptr()) as usize
        };

        let sum1 = obj_size + length;
        let sum2 = sum1 + self.alignment_mask;
        let mut new_size = sum2 + (obj_size >> 3) + 100;

        new_size = new_size.max(sum2).max(self.chunk_size);

        let layout = Layout::from_size_align(new_size, 1).map_err(|_| "invalid layout")?;
        let new_ptr = unsafe { alloc(layout) };
        if new_ptr.is_null() {
            return Err("allocation failed");
        }

        let new_chunk_ptr = new_ptr as *mut Chunk;
        let new_chunk = unsafe {
            let new_chunk = Chunk {
                limit: NonNull::new_unchecked(new_ptr.add(new_size)),
                prev: Some(old_chunk),
                contents: [],
            };
            ptr::write(new_chunk_ptr, new_chunk);
            NonNull::new_unchecked(new_chunk_ptr)
        };

        let object_base = unsafe {
            let contents_ptr = (*new_chunk_ptr).contents.as_ptr();
            let aligned_ptr = (contents_ptr as usize + self.alignment_mask) & !self.alignment_mask;
            NonNull::new_unchecked(aligned_ptr as *mut u8)
        };

        unsafe {
            ptr::copy_nonoverlapping(
                self.object_base.unwrap().as_ptr(),
                object_base.as_ptr(),
                obj_size,
            );
        }

        self.chunk = Some(new_chunk);
        self.chunk_limit = Some(unsafe { (*new_chunk_ptr).limit });
        self.object_base = Some(object_base);
        self.next_free = NonNull::new(unsafe { object_base.as_ptr().add(obj_size) });
        self.maybe_empty_object = false;

        Ok(())
    }

    pub fn free(&mut self) {
        while let Some(chunk) = self.chunk {
            let prev = unsafe { chunk.as_ref().prev };
            let layout = Layout::from_size_align(self.chunk_size, 1).unwrap();
            unsafe {
                dealloc(chunk.as_ptr() as *mut u8, layout);
            }
            self.chunk = prev;
        }
        self.object_base = None;
        self.next_free = None;
        self.chunk_limit = None;
        self.maybe_empty_object = true;
    }
}

impl Drop for Obstack {
    fn drop(&mut self) {
        self.free();
    }
}