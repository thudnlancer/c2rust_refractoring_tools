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
    use_extra_arg: bool,
    maybe_empty_object: bool,
    alloc_failed: bool,
    extra_arg: Option<NonNull<u8>>,
}

#[derive(Debug)]
struct Chunk {
    limit: Option<NonNull<u8>>,
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
            use_extra_arg: false,
            maybe_empty_object: false,
            alloc_failed: false,
            extra_arg: None,
        };

        obstack.allocate_first_chunk()?;
        Ok(obstack)
    }

    fn allocate_first_chunk(&mut self) -> Result<(), &'static str> {
        let layout = Layout::from_size_align(self.chunk_size, 1).unwrap();
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err("Allocation failed");
        }

        let chunk_ptr = ptr as *mut Chunk;
        unsafe {
            (*chunk_ptr).limit = NonNull::new(ptr.add(self.chunk_size));
            (*chunk_ptr).prev = None;
        }

        let chunk = NonNull::new(chunk_ptr).unwrap();
        self.chunk = Some(chunk);

        let object_base = self.align_ptr(unsafe { chunk.as_ref().contents.as_ptr() })?;
        self.object_base = Some(object_base);
        self.next_free = Some(object_base);
        self.chunk_limit = unsafe { chunk.as_ref().limit };
        self.maybe_empty_object = false;
        self.alloc_failed = false;

        Ok(())
    }

    fn align_ptr(&self, ptr: *const u8) -> Result<NonNull<u8>, &'static str> {
        let aligned_ptr = ((ptr as usize + self.alignment_mask) & !self.alignment_mask) as *mut u8;
        NonNull::new(aligned_ptr).ok_or("Alignment failed")
    }

    pub fn alloc(&mut self, size: usize) -> Result<NonNull<u8>, &'static str> {
        if self.next_free.is_none() || self.chunk_limit.is_none() {
            return Err("Obstack not initialized");
        }

        let next_free = self.next_free.unwrap();
        let chunk_limit = self.chunk_limit.unwrap();

        if next_free.as_ptr() as usize + size > chunk_limit.as_ptr() as usize {
            self.new_chunk(size)?;
        }

        let ptr = self.next_free.unwrap();
        self.next_free = NonNull::new(unsafe { ptr.as_ptr().add(size) });
        self.maybe_empty_object = false;
        Ok(ptr)
    }

    fn new_chunk(&mut self, length: usize) -> Result<(), &'static str> {
        let old_chunk = self.chunk.ok_or("No current chunk")?;
        let obj_size = self.next_free.unwrap().as_ptr() as usize - self.object_base.unwrap().as_ptr() as usize;
        let sum1 = obj_size + length;
        let sum2 = sum1 + self.alignment_mask;
        let mut new_size = sum2 + (obj_size >> 3) + 100;

        new_size = new_size.max(sum2).max(self.chunk_size);

        let layout = Layout::from_size_align(new_size, 1).unwrap();
        let new_ptr = unsafe { alloc(layout) };
        if new_ptr.is_null() {
            return Err("Allocation failed");
        }

        let new_chunk_ptr = new_ptr as *mut Chunk;
        unsafe {
            (*new_chunk_ptr).limit = NonNull::new(new_ptr.add(new_size));
            (*new_chunk_ptr).prev = Some(old_chunk);
        }

        let new_chunk = NonNull::new(new_chunk_ptr).unwrap();
        self.chunk = Some(new_chunk);
        self.chunk_limit = unsafe { new_chunk.as_ref().limit };

        let object_base = self.align_ptr(unsafe { new_chunk.as_ref().contents.as_ptr() })?;
        
        unsafe {
            ptr::copy_nonoverlapping(
                self.object_base.unwrap().as_ptr(),
                object_base.as_ptr(),
                obj_size,
            );
        }

        self.object_base = Some(object_base);
        self.next_free = NonNull::new(unsafe { object_base.as_ptr().add(obj_size) });
        self.maybe_empty_object = false;

        Ok(())
    }

    pub fn free(&mut self, obj: NonNull<u8>) -> Result<(), &'static str> {
        let mut current_chunk = self.chunk;
        while let Some(chunk) = current_chunk {
            let chunk_ptr = chunk.as_ptr() as *mut u8;
            let chunk_end = unsafe { chunk.as_ref().limit.unwrap().as_ptr() };

            if chunk_ptr <= obj.as_ptr() && obj.as_ptr() < chunk_end {
                self.next_free = Some(obj);
                self.object_base = Some(obj);
                self.chunk_limit = unsafe { chunk.as_ref().limit };
                self.chunk = Some(chunk);
                self.maybe_empty_object = true;
                return Ok(());
            }

            let prev = unsafe { chunk.as_ref().prev };
            self.deallocate_chunk(chunk);
            current_chunk = prev;
            self.maybe_empty_object = true;
        }

        if !obj.as_ptr().is_null() {
            return Err("Object not found in any chunk");
        }

        Ok(())
    }

    fn deallocate_chunk(&mut self, chunk: NonNull<Chunk>) {
        let layout = Layout::from_size_align(
            unsafe { chunk.as_ref().limit.unwrap().as_ptr() as usize - chunk.as_ptr() as usize },
            1,
        ).unwrap();
        unsafe { dealloc(chunk.as_ptr() as *mut u8, layout) };
    }

    pub fn memory_used(&self) -> usize {
        let mut total = 0;
        let mut current_chunk = self.chunk;
        while let Some(chunk) = current_chunk {
            total += unsafe { chunk.as_ref().limit.unwrap().as_ptr() as usize - chunk.as_ptr() as usize };
            current_chunk = unsafe { chunk.as_ref().prev };
        }
        total
    }
}

impl Drop for Obstack {
    fn drop(&mut self) {
        let mut current_chunk = self.chunk;
        while let Some(chunk) = current_chunk {
            let prev = unsafe { chunk.as_ref().prev };
            self.deallocate_chunk(chunk);
            current_chunk = prev;
        }
    }
}