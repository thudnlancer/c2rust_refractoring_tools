use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::{null_mut, NonNull};

#[derive(Debug)]
struct MemoryBlock {
    size: usize,
    prev: Option<NonNull<MemoryBlock>>,
    next: Option<NonNull<MemoryBlock>>,
}

struct MemoryManager {
    mem_ptr: Option<NonNull<MemoryBlock>>,
    mem_count: i32,
    mem_cpeak: i32,
    mem_total: usize,
    mem_tpeak: usize,
    mem_limit: usize,
}

impl MemoryManager {
    fn new() -> Self {
        MemoryManager {
            mem_ptr: None,
            mem_count: 0,
            mem_cpeak: 0,
            mem_total: 0,
            mem_tpeak: 0,
            mem_limit: usize::MAX,
        }
    }

    fn allocate(&mut self, size: usize) -> Result<NonNull<u8>, &'static str> {
        if size == 0 {
            return Err("Cannot allocate zero-sized block");
        }

        let total_size = size + std::mem::size_of::<MemoryBlock>();
        if total_size > self.mem_limit.saturating_sub(self.mem_total) {
            return Err("Memory allocation limit exceeded");
        }

        if self.mem_count == i32::MAX {
            return Err("Too many memory blocks allocated");
        }

        let layout = Layout::from_size_align(total_size, 16).map_err(|_| "Invalid layout")?;
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err("No memory available");
        }

        let block_ptr = ptr as *mut MemoryBlock;
        unsafe {
            (*block_ptr).size = total_size;
            (*block_ptr).prev = None;
            (*block_ptr).next = self.mem_ptr;
        }

        let non_null_block = NonNull::new(block_ptr).unwrap();
        if let Some(mut next) = self.mem_ptr {
            unsafe { next.as_mut().prev = Some(non_null_block) };
        }

        self.mem_ptr = Some(non_null_block);
        self.mem_count += 1;
        self.mem_total += total_size;
        self.mem_cpeak = self.mem_cpeak.max(self.mem_count);
        self.mem_tpeak = self.mem_tpeak.max(self.mem_total);

        let data_ptr = unsafe { ptr.add(std::mem::size_of::<MemoryBlock>()) };
        Ok(NonNull::new(data_ptr).unwrap())
    }

    fn reallocate(
        &mut self,
        ptr: NonNull<u8>,
        new_size: usize,
    ) -> Result<NonNull<u8>, &'static str> {
        if new_size == 0 {
            self.deallocate(ptr);
            return Ok(NonNull::dangling());
        }

        let block_ptr = unsafe {
            ptr.as_ptr()
                .sub(std::mem::size_of::<MemoryBlock>()) as *mut MemoryBlock
        };

        let old_size = unsafe { (*block_ptr).size };
        let total_new_size = new_size + std::mem::size_of::<MemoryBlock>();

        if total_new_size > self.mem_limit.saturating_sub(self.mem_total - old_size) {
            return Err("Memory allocation limit exceeded");
        }

        let layout = Layout::from_size_align(total_new_size, 16).map_err(|_| "Invalid layout")?;
        let new_ptr = unsafe { realloc(block_ptr as *mut u8, layout, total_new_size) };
        if new_ptr.is_null() {
            return Err("No memory available");
        }

        let new_block_ptr = new_ptr as *mut MemoryBlock;
        unsafe {
            (*new_block_ptr).size = total_new_size;
        }

        self.mem_total = self.mem_total - old_size + total_new_size;
        self.mem_tpeak = self.mem_tpeak.max(self.mem_total);

        let data_ptr = unsafe { new_ptr.add(std::mem::size_of::<MemoryBlock>()) };
        Ok(NonNull::new(data_ptr).unwrap())
    }

    fn deallocate(&mut self, ptr: NonNull<u8>) {
        let block_ptr = unsafe {
            ptr.as_ptr()
                .sub(std::mem::size_of::<MemoryBlock>()) as *mut MemoryBlock
        };

        let block_size = unsafe { (*block_ptr).size };
        let prev = unsafe { (*block_ptr).prev };
        let next = unsafe { (*block_ptr).next };

        match (prev, next) {
            (Some(mut prev_ptr), Some(mut next_ptr)) => {
                unsafe {
                    prev_ptr.as_mut().next = Some(next_ptr);
                    next_ptr.as_mut().prev = Some(prev_ptr);
                }
            }
            (Some(mut prev_ptr), None) => {
                unsafe {
                    prev_ptr.as_mut().next = None;
                }
            }
            (None, Some(mut next_ptr)) => {
                unsafe {
                    next_ptr.as_mut().prev = None;
                }
                self.mem_ptr = Some(next_ptr);
            }
            (None, None) => {
                self.mem_ptr = None;
            }
        }

        self.mem_count -= 1;
        self.mem_total -= block_size;

        let layout = Layout::from_size_align(block_size, 16).unwrap();
        unsafe { dealloc(block_ptr as *mut u8, layout) };
    }
}

pub struct GlpAllocator {
    manager: std::sync::Mutex<MemoryManager>,
}

impl GlpAllocator {
    pub fn new() -> Self {
        GlpAllocator {
            manager: std::sync::Mutex::new(MemoryManager::new()),
        }
    }

    pub fn alloc(&self, n: i32, size: i32) -> Result<NonNull<u8>, &'static str> {
        if n < 1 || size < 1 {
            return Err("Invalid parameters");
        }

        let total_size = (n as usize).checked_mul(size as usize).ok_or("Block too large")?;
        self.manager.lock().unwrap().allocate(total_size)
    }

    pub fn realloc(
        &self,
        ptr: NonNull<u8>,
        n: i32,
        size: i32,
    ) -> Result<NonNull<u8>, &'static str> {
        if n < 1 || size < 1 {
            return Err("Invalid parameters");
        }

        let total_size = (n as usize).checked_mul(size as usize).ok_or("Block too large")?;
        self.manager.lock().unwrap().reallocate(ptr, total_size)
    }

    pub fn free(&self, ptr: NonNull<u8>) {
        self.manager.lock().unwrap().deallocate(ptr);
    }

    pub fn set_mem_limit(&self, limit: i32) -> Result<(), &'static str> {
        if limit < 1 {
            return Err("Invalid limit");
        }

        let mut manager = self.manager.lock().unwrap();
        manager.mem_limit = if limit as usize <= usize::MAX >> 20 {
            (limit as usize) << 20
        } else {
            usize::MAX
        };
        Ok(())
    }

    pub fn mem_usage(
        &self,
    ) -> (
        Option<i32>,
        Option<i32>,
        Option<usize>,
        Option<usize>,
    ) {
        let manager = self.manager.lock().unwrap();
        (
            Some(manager.mem_count),
            Some(manager.mem_cpeak),
            Some(manager.mem_total),
            Some(manager.mem_tpeak),
        )
    }
}