use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

struct ObstackChunk {
    limit: *mut u8,
    prev: *mut ObstackChunk,
    contents: [u8; 0],
}

struct Obstack {
    chunk_size: usize,
    chunk: *mut ObstackChunk,
    object_base: *mut u8,
    next_free: *mut u8,
    chunk_limit: *mut u8,
    alignment_mask: usize,
    chunk_fun: Option<Box<dyn Fn(usize) -> *mut u8>>,
    free_fun: Option<Box<dyn Fn(*mut u8)>>,
    extra_arg: *mut (),
    use_extra_arg: bool,
    maybe_empty_object: bool,
    alloc_failed: bool,
}

impl Obstack {
    fn new(
        size: usize,
        alignment: usize,
        chunk_fun: Option<Box<dyn Fn(usize) -> *mut u8>>,
        free_fun: Option<Box<dyn Fn(*mut u8)>>,
    ) -> Result<Self, &'static str> {
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

        let chunk = if let Some(f) = &chunk_fun {
            f(size)
        } else {
            let layout = Layout::from_size_align(size, alignment).map_err(|_| "invalid layout")?;
            unsafe { alloc(layout) }
        };

        if chunk.is_null() {
            return Err("allocation failed");
        }

        let chunk = chunk as *mut ObstackChunk;
        let object_base = unsafe {
            let contents_ptr = (*chunk).contents.as_ptr();
            let aligned = (contents_ptr as usize + alignment - 1) & !(alignment - 1);
            aligned as *mut u8
        };

        let chunk_limit = unsafe { chunk.cast::<u8>().add(size) };

        Ok(Self {
            chunk_size: size,
            chunk,
            object_base,
            next_free: object_base,
            chunk_limit,
            alignment_mask: alignment - 1,
            chunk_fun,
            free_fun,
            extra_arg: ptr::null_mut(),
            use_extra_arg: false,
            maybe_empty_object: false,
            alloc_failed: false,
        })
    }

    fn new_with_extra(
        size: usize,
        alignment: usize,
        chunk_fun: Option<Box<dyn Fn(*mut (), usize) -> *mut u8>>,
        free_fun: Option<Box<dyn Fn(*mut (), *mut u8)>>,
        extra_arg: *mut (),
    ) -> Result<Self, &'static str> {
        let mut obstack = Self::new(size, alignment, None, None)?;
        obstack.use_extra_arg = true;
        obstack.extra_arg = extra_arg;
        Ok(obstack)
    }

    fn alloc(&mut self, length: usize) -> Result<*mut u8, &'static str> {
        let obj_size = unsafe { self.next_free.offset_from(self.object_base) as usize };
        let sum1 = obj_size + length;
        let sum2 = sum1 + self.alignment_mask;
        let mut new_size = sum2 + (obj_size >> 3) + 100;

        new_size = new_size.max(sum2).max(self.chunk_size);

        let new_chunk = if let Some(f) = &self.chunk_fun {
            f(new_size)
        } else {
            let layout = Layout::from_size_align(new_size, self.alignment_mask + 1)
                .map_err(|_| "invalid layout")?;
            unsafe { alloc(layout) }
        };

        if new_chunk.is_null() {
            return Err("allocation failed");
        }

        let new_chunk = new_chunk as *mut ObstackChunk;
        let object_base = unsafe {
            let contents_ptr = (*new_chunk).contents.as_ptr();
            let aligned = (contents_ptr as usize + self.alignment_mask) & !self.alignment_mask;
            aligned as *mut u8
        };

        unsafe {
            (*new_chunk).prev = self.chunk;
            (*new_chunk).limit = new_chunk.cast::<u8>().add(new_size);
            self.chunk_limit = (*new_chunk).limit;

            ptr::copy_nonoverlapping(
                self.object_base,
                object_base,
                obj_size,
            );

            self.chunk = new_chunk;
            self.object_base = object_base;
            self.next_free = self.object_base.add(obj_size);
            self.maybe_empty_object = false;
        }

        Ok(self.next_free)
    }

    fn free(&mut self, obj: *mut u8) {
        let mut lp = self.chunk;
        while !lp.is_null() {
            let chunk_start = lp.cast::<u8>();
            let chunk_end = unsafe { (*lp).limit };
            if !(chunk_start <= obj && obj < chunk_end) {
                let prev = unsafe { (*lp).prev };
                if let Some(f) = &self.free_fun {
                    f(lp.cast());
                } else {
                    let size = unsafe { chunk_end.offset_from(chunk_start) as usize };
                    let layout = Layout::from_size_align(size, self.alignment_mask + 1)
                        .unwrap();
                    unsafe { dealloc(lp.cast(), layout) };
                }
                lp = prev;
                self.maybe_empty_object = true;
            } else {
                break;
            }
        }

        if !lp.is_null() {
            unsafe {
                self.next_free = obj;
                self.object_base = self.next_free;
                self.chunk_limit = (*lp).limit;
                self.chunk = lp;
            }
        } else if !obj.is_null() {
            panic!("attempt to free non-allocated object");
        }
    }

    fn memory_used(&self) -> usize {
        let mut nbytes = 0;
        let mut lp = self.chunk;
        while !lp.is_null() {
            unsafe {
                nbytes += (*lp).limit.offset_from(lp.cast()) as usize;
                lp = (*lp).prev;
            }
        }
        nbytes
    }
}

impl Drop for Obstack {
    fn drop(&mut self) {
        let mut lp = self.chunk;
        while !lp.is_null() {
            let prev = unsafe { (*lp).prev };
            if let Some(f) = &self.free_fun {
                f(lp.cast());
            } else {
                let size = unsafe { (*lp).limit.offset_from(lp.cast()) as usize };
                let layout = Layout::from_size_align(size, self.alignment_mask + 1).unwrap();
                unsafe { dealloc(lp.cast(), layout) };
            }
            lp = prev;
        }
    }
}

fn print_and_abort() -> ! {
    eprintln!("memory exhausted");
    std::process::abort();
}