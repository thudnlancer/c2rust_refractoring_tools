use std::cmp::Ordering;
use std::ffi::CStr;
use std::ptr;

pub type BstCmpFunction<T> = fn(&T, &T) -> Ordering;

pub struct BstAllocator {
    malloc: fn(usize) -> *mut u8,
    free: fn(*mut u8),
}

impl BstAllocator {
    pub const DEFAULT: BstAllocator = BstAllocator {
        malloc: |size| {
            let layout = std::alloc::Layout::from_size_align(size, std::mem::align_of::<u8>()).unwrap();
            unsafe { std::alloc::alloc(layout) }
        },
        free: |block| {
            if !block.is_null() {
                let layout = std::alloc::Layout::new::<u8>();
                unsafe { std::alloc::dealloc(block, layout) }
            }
        },
    };
}

pub trait BstType<T> {
    fn init(
        &self,
        allocator: &BstAllocator,
        compare: BstCmpFunction<T>,
        params: *mut (),
        table: *mut (),
    ) -> Result<(), &'static str>;
    fn empty(&self, table: *mut ()) -> Result<(), &'static str>;
    fn insert(&self, item: T, table: *mut ()) -> Result<*mut T, &'static str>;
    fn find(&self, item: &T, table: *const ()) -> Option<&T>;
    fn remove(&self, item: &T, table: *mut ()) -> Option<T>;
    fn nodes(&self, table: *const ()) -> usize;
    fn node_size(&self) -> usize;
    fn name(&self) -> &'static str;
}

pub struct BstWorkspace<T> {
    bst_type: Box<dyn BstType<T>>,
    table: *mut (),
}

impl<T> BstWorkspace<T> {
    pub fn new(
        bst_type: Box<dyn BstType<T>>,
        allocator: Option<&BstAllocator>,
        compare: BstCmpFunction<T>,
        params: *mut (),
    ) -> Result<Self, &'static str> {
        let mut table = ptr::null_mut();
        let allocator = allocator.unwrap_or(&BstAllocator::DEFAULT);
        
        bst_type.init(allocator, compare, params, &mut table as *mut _ as *mut ())
            .map_err(|_| "failed to initialize bst")?;
            
        Ok(Self {
            bst_type,
            table,
        })
    }

    pub fn empty(&self) -> Result<(), &'static str> {
        self.bst_type.empty(self.table)
    }

    pub fn insert(&mut self, item: T) -> Result<&mut T, &'static str> {
        let ptr = self.bst_type.insert(item, self.table)?;
        Ok(unsafe { &mut *ptr })
    }

    pub fn find(&self, item: &T) -> Option<&T> {
        self.bst_type.find(item, self.table)
    }

    pub fn remove(&mut self, item: &T) -> Option<T> {
        self.bst_type.remove(item, self.table)
    }

    pub fn nodes(&self) -> usize {
        self.bst_type.nodes(self.table)
    }

    pub fn node_size(&self) -> usize {
        self.bst_type.node_size()
    }

    pub fn name(&self) -> &'static str {
        self.bst_type.name()
    }
}

impl<T> Drop for BstWorkspace<T> {
    fn drop(&mut self) {
        let _ = self.empty();
        if !self.table.is_null() {
            let layout = std::alloc::Layout::new::<()>();
            unsafe { std::alloc::dealloc(self.table as *mut u8, layout) };
        }
    }
}