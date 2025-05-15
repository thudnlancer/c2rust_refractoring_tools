use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

#[derive(Debug)]
struct Chunk {
    limit: *mut u8,
    prev: Option<Box<Chunk>>,
    contents: Vec<u8>,
}

#[derive(Debug)]
pub struct Obstack {
    chunk_size: usize,
    chunk: Option<Box<Chunk>>,
    object_base: *mut u8,
    next_free: *mut u8,
    chunk_limit: *mut u8,
    alignment_mask: usize,
    use_extra_arg: bool,
    maybe_empty_object: bool,
    alloc_failed: bool,
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

        let mut chunk = Chunk::new(size, alignment)?;
        let object_base = chunk.aligned_start(alignment);
        let next_free = object_base;
        let chunk_limit = unsafe { chunk.limit };

        Ok(Obstack {
            chunk_size: size,
            chunk: Some(Box::new(chunk)),
            object_base,
            next_free,
            chunk_limit,
            alignment_mask: alignment - 1,
            use_extra_arg: false,
            maybe_empty_object: false,
            alloc_failed: false,
        })
    }

    pub fn alloc(&mut self, length: usize) -> Result<*mut u8, &'static str> {
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

        if obj_size > sum1 || sum1 > sum2 {
            return Err("Invalid size calculation");
        }

        let mut new_chunk = Chunk::new(new_size, self.alignment_mask + 1)?;
        let old_chunk = self.chunk.take().unwrap();

        new_chunk.prev = Some(old_chunk);
        let object_base = new_chunk.aligned_start(self.alignment_mask + 1);

        unsafe {
            ptr::copy_nonoverlapping(
                self.object_base,
                object_base,
                obj_size,
            );
        }

        if !self.maybe_empty_object && 
            self.object_base == old_chunk.aligned_start(self.alignment_mask + 1) 
        {
            new_chunk.prev = old_chunk.prev;
        }

        self.chunk = Some(Box::new(new_chunk));
        self.chunk_limit = unsafe { self.chunk.as_ref().unwrap().limit };
        self.object_base = object_base;
        self.next_free = unsafe { self.object_base.add(obj_size) };
        self.maybe_empty_object = false;

        Ok(self.next_free)
    }

    pub fn free(&mut self, obj: *mut u8) {
        let mut current = self.chunk.take();
        while let Some(mut chunk) = current {
            if chunk.contains_ptr(obj) {
                self.next_free = obj;
                self.object_base = obj;
                self.chunk_limit = unsafe { chunk.limit };
                self.chunk = Some(chunk);
                return;
            }
            current = chunk.prev.take();
        }

        if !obj.is_null() {
            panic!("Attempt to free object not in obstack");
        }
    }

    pub fn memory_used(&self) -> usize {
        let mut total = 0;
        let mut current = &self.chunk;
        while let Some(chunk) = current {
            total += unsafe { chunk.limit.offset_from(chunk.start()) as usize };
            current = &chunk.prev;
        }
        total
    }
}

impl Chunk {
    fn new(size: usize, alignment: usize) -> Result<Self, &'static str> {
        let layout = Layout::from_size_align(size, alignment)
            .map_err(|_| "Invalid layout")?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err("Allocation failed");
        }

        Ok(Chunk {
            limit: unsafe { ptr.add(size) },
            prev: None,
            contents: Vec::with_capacity(size),
        })
    }

    fn aligned_start(&self, alignment: usize) -> *mut u8 {
        let start = self.contents.as_ptr() as usize;
        let aligned = (start + alignment - 1) & !(alignment - 1);
        aligned as *mut u8
    }

    fn contains_ptr(&self, ptr: *mut u8) -> bool {
        let start = self.contents.as_ptr() as *mut u8;
        ptr >= start && ptr < self.limit
    }

    fn start(&self) -> *mut u8 {
        self.contents.as_ptr() as *mut u8
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(
            unsafe { self.limit.offset_from(self.start()) as usize },
            1,
        ).unwrap();
        unsafe { dealloc(self.contents.as_mut_ptr(), layout) };
    }
}

fn print_and_abort() -> ! {
    eprintln!("memory exhausted");
    std::process::exit(1);
}

static OBSTACK_ALLOC_FAILED_HANDLER: fn() -> ! = print_and_abort;