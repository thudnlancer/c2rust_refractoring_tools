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

        let mut obstack = Obstack {
            chunk_size: size,
            chunk: None,
            object_base: None,
            next_free: None,
            chunk_limit: None,
            alignment_mask: alignment - 1,
            alloc_failed: false,
        };

        obstack.allocate_new_chunk(size)?;
        Ok(obstack)
    }

    fn allocate_new_chunk(&mut self, size: usize) -> Result<(), &'static str> {
        let layout = Layout::from_size_align(size, self.alignment_mask + 1)
            .map_err(|_| "invalid layout")?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err("allocation failed");
        }

        let chunk_ptr = NonNull::new(ptr as *mut Chunk).ok_or("null pointer")?;
        let chunk = unsafe { &mut *chunk_ptr.as_ptr() };

        let contents_ptr = unsafe { ptr.add(mem::size_of::<Chunk>()) };
        let aligned_ptr = (contents_ptr as usize + self.alignment_mask) & !self.alignment_mask;
        let object_base = NonNull::new(aligned_ptr as *mut u8).ok_or("null pointer")?;

        chunk.limit = NonNull::new(unsafe { ptr.add(size) }).ok_or("null pointer")?;
        chunk.prev = self.chunk;

        self.chunk = Some(chunk_ptr);
        self.object_base = Some(object_base);
        self.next_free = Some(object_base);
        self.chunk_limit = Some(chunk.limit);
        self.alloc_failed = false;

        Ok(())
    }

    pub fn alloc(&mut self, length: usize) -> Result<NonNull<u8>, &'static str> {
        let obj_size = match (self.next_free, self.object_base) {
            (Some(next), Some(base)) => unsafe { next.as_ptr().offset_from(base.as_ptr()) as usize },
            _ => 0,
        };

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
            self.allocate_new_chunk(new_size)?;
        }

        let next_free = self.next_free.ok_or("no next free pointer")?;
        let result = next_free;
        self.next_free = NonNull::new(unsafe { next_free.as_ptr().add(length) })
            .ok_or("pointer overflow")?;

        Ok(result)
    }

    pub fn free(&mut self, obj: NonNull<u8>) -> Result<(), &'static str> {
        let mut current_chunk = self.chunk;
        while let Some(chunk_ptr) = current_chunk {
            let chunk = unsafe { chunk_ptr.as_ref() };
            let chunk_start = chunk_ptr.as_ptr() as *const u8;
            let chunk_end = chunk.limit.as_ptr();

            if obj.as_ptr() >= chunk_start && obj.as_ptr() < chunk_end {
                self.next_free = Some(obj);
                self.object_base = Some(obj);
                self.chunk_limit = Some(chunk.limit);
                self.chunk = Some(chunk_ptr);
                return Ok(());
            }

            current_chunk = chunk.prev;
            if let Some(prev_chunk) = chunk.prev {
                unsafe { dealloc(prev_chunk.as_ptr() as *mut u8, Layout::new::<Chunk>()) };
            }
        }

        Err("object not found in obstack")
    }

    pub fn memory_used(&self) -> usize {
        let mut total = 0;
        let mut current_chunk = self.chunk;

        while let Some(chunk_ptr) = current_chunk {
            let chunk = unsafe { chunk_ptr.as_ref() };
            total += unsafe { chunk.limit.as_ptr().offset_from(chunk_ptr.as_ptr() as *const u8) as usize };
            current_chunk = chunk.prev;
        }

        total
    }
}

impl Drop for Obstack {
    fn drop(&mut self) {
        let mut current_chunk = self.chunk;
        while let Some(chunk_ptr) = current_chunk {
            let chunk = unsafe { chunk_ptr.as_ref() };
            current_chunk = chunk.prev;
            unsafe { dealloc(chunk_ptr.as_ptr() as *mut u8, Layout::new::<Chunk>()) };
        }
    }
}

fn print_and_abort() -> ! {
    eprintln!("memory exhausted");
    std::process::abort();
}