use std::alloc::{alloc, dealloc, Layout};
use std::cmp::Ordering;
use std::ffi::CString;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

pub type SizeT = usize;

pub trait BstCmpFn: Fn(&[u8], &[u8]) -> Ordering {}
impl<T: Fn(&[u8], &[u8]) -> Ordering> BstCmpFn for T {}

#[derive(Debug)]
pub struct BstAllocator {
    pub alloc: Box<dyn Fn(SizeT) -> *mut u8>,
    pub free: Box<dyn Fn(*mut u8)>,
}

impl Default for BstAllocator {
    fn default() -> Self {
        BstAllocator {
            alloc: Box::new(|size| unsafe {
                let layout = Layout::from_size_align(size, std::mem::align_of::<u8>()).unwrap();
                alloc(layout)
            }),
            free: Box::new(|ptr| unsafe {
                let layout = Layout::from_size_align(1, std::mem::align_of::<u8>()).unwrap();
                dealloc(ptr, layout);
            }),
        }
    }
}

#[derive(Debug)]
pub struct BstAvlNode {
    pub links: [*mut BstAvlNode; 2],
    pub data: *mut u8,
    pub balance: i8,
}

#[derive(Debug)]
pub struct BstAvlTable {
    pub root: *mut BstAvlNode,
    pub compare: Box<dyn BstCmpFn>,
    pub param: *mut u8,
    pub alloc: BstAllocator,
    pub count: SizeT,
    pub generation: u64,
}

#[derive(Debug)]
pub struct BstRbNode {
    pub links: [*mut BstRbNode; 2],
    pub data: *mut u8,
    pub color: u8,
}

#[derive(Debug)]
pub struct BstRbTable {
    pub root: *mut BstRbNode,
    pub compare: Box<dyn BstCmpFn>,
    pub param: *mut u8,
    pub alloc: BstAllocator,
    pub count: SizeT,
    pub generation: u64,
}

#[derive(Debug)]
pub enum BstTable {
    Avl(BstAvlTable),
    Rb(BstRbTable),
}

#[derive(Debug)]
pub struct BstType {
    pub name: CString,
    pub node_size: SizeT,
    pub init: Box<dyn Fn(&BstAllocator, Box<dyn BstCmpFn>, *mut u8) -> Result<BstTable, GslError>>,
    pub nodes: Box<dyn Fn(&BstTable) -> SizeT>,
    pub insert: Box<dyn Fn(&mut BstTable, *mut u8) -> *mut u8>,
    pub find: Box<dyn Fn(&BstTable, *const u8) -> *mut u8>,
    pub remove: Box<dyn Fn(&mut BstTable, *const u8) -> *mut u8>,
    pub empty: Box<dyn Fn(&mut BstTable) -> Result<(), GslError>>,
}

#[derive(Debug)]
pub struct BstWorkspace {
    pub type_: &'static BstType,
    pub table: BstTable,
}

impl BstWorkspace {
    pub fn new(
        type_: &'static BstType,
        allocator: Option<&BstAllocator>,
        compare: Box<dyn BstCmpFn>,
        params: *mut u8,
    ) -> Result<Self, GslError> {
        let allocator = allocator.unwrap_or(&BstAllocator::default());
        let table = (type_.init)(allocator, compare, params)?;
        Ok(Self { type_, table })
    }

    pub fn free(self) {
        self.empty();
    }

    pub fn empty(&mut self) -> Result<(), GslError> {
        (self.type_.empty)(&mut self.table)
    }

    pub fn insert(&mut self, item: *mut u8) -> *mut u8 {
        (self.type_.insert)(&mut self.table, item)
    }

    pub fn find(&self, item: *const u8) -> *mut u8 {
        (self.type_.find)(&self.table, item)
    }

    pub fn remove(&mut self, item: *const u8) -> *mut u8 {
        (self.type_.remove)(&mut self.table, item)
    }

    pub fn nodes(&self) -> SizeT {
        (self.type_.nodes)(&self.table)
    }

    pub fn node_size(&self) -> SizeT {
        self.type_.node_size
    }

    pub fn name(&self) -> &CString {
        &self.type_.name
    }
}