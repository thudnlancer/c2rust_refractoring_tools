use std::cmp::Ordering;
use std::ptr;

pub type SizeT = usize;

pub trait BstCmpFunction {
    fn compare(&self, a: &[u8], b: &[u8]) -> Ordering;
}

pub struct BstAllocator {
    pub alloc: Box<dyn Fn(SizeT) -> Vec<u8>>,
    pub free: Box<dyn Fn(Vec<u8>)>,
}

pub enum BstNode {
    Avl(AvlNode),
    Rb(RbNode),
}

pub struct AvlNode {
    pub links: [Option<Box<AvlNode>>; 2],
    pub data: Vec<u8>,
    pub balance: i8,
}

pub struct RbNode {
    pub links: [Option<Box<RbNode>>; 2],
    pub data: Vec<u8>,
    pub color: u8,
}

pub struct BstTable {
    pub root: Option<BstNode>,
    pub compare: Box<dyn BstCmpFunction>,
    pub allocator: BstAllocator,
    pub count: SizeT,
    pub generation: u64,
}

pub struct BstTraverser {
    pub table: BstTable,
    pub current: Option<BstNode>,
    pub stack: Vec<BstNode>,
    pub height: SizeT,
    pub generation: u64,
}

pub struct BstType {
    pub name: String,
    pub node_size: SizeT,
    pub init: Box<dyn Fn(&BstAllocator, Box<dyn BstCmpFunction>) -> Result<BstTable, ()>>,
    pub nodes: Box<dyn Fn(&BstTable) -> SizeT>,
    pub insert: Box<dyn Fn(&mut BstTable, Vec<u8>) -> Option<Vec<u8>>>,
    pub find: Box<dyn Fn(&BstTable, &[u8]) -> Option<Vec<u8>>>,
    pub remove: Box<dyn Fn(&mut BstTable, &[u8]) -> Option<Vec<u8>>>,
    pub empty: Box<dyn Fn(&BstTable) -> bool>,
    pub trav_init: Box<dyn Fn(&BstTable) -> BstTraverser>,
    pub trav_first: Box<dyn Fn(&BstTable) -> Option<Vec<u8>>>,
    pub trav_last: Box<dyn Fn(&BstTable) -> Option<Vec<u8>>>,
    pub trav_find: Box<dyn Fn(&BstTable, &[u8]) -> Option<Vec<u8>>>,
    pub trav_insert: Box<dyn Fn(&mut BstTable, Vec<u8>) -> Option<Vec<u8>>>,
    pub trav_copy: Box<dyn Fn(&BstTraverser) -> BstTraverser>,
    pub trav_next: Box<dyn Fn(&mut BstTraverser) -> Option<Vec<u8>>>,
    pub trav_prev: Box<dyn Fn(&mut BstTraverser) -> Option<Vec<u8>>>,
    pub trav_cur: Box<dyn Fn(&BstTraverser) -> Option<Vec<u8>>>,
    pub trav_replace: Box<dyn Fn(&mut BstTraverser, Vec<u8>) -> Option<Vec<u8>>>,
}

pub struct BstWorkspace {
    pub bst_type: BstType,
    pub table: BstTable,
}

pub struct BstTrav {
    pub bst_type: BstType,
    pub trav_data: BstTraverser,
}

impl BstTrav {
    pub fn init(&mut self, workspace: &BstWorkspace) -> Result<(), ()> {
        self.bst_type = workspace.bst_type.clone();
        self.trav_data = (workspace.bst_type.trav_init)(&workspace.table);
        Ok(())
    }

    pub fn first(&mut self, workspace: &BstWorkspace) -> Option<Vec<u8>> {
        self.bst_type = workspace.bst_type.clone();
        (workspace.bst_type.trav_first)(&workspace.table)
    }

    pub fn last(&mut self, workspace: &BstWorkspace) -> Option<Vec<u8>> {
        self.bst_type = workspace.bst_type.clone();
        (workspace.bst_type.trav_last)(&workspace.table)
    }

    pub fn find(&mut self, item: &[u8], workspace: &BstWorkspace) -> Option<Vec<u8>> {
        self.bst_type = workspace.bst_type.clone();
        (workspace.bst_type.trav_find)(&workspace.table, item)
    }

    pub fn insert(&mut self, item: Vec<u8>, workspace: &mut BstWorkspace) -> Option<Vec<u8>> {
        self.bst_type = workspace.bst_type.clone();
        (workspace.bst_type.trav_insert)(&mut workspace.table, item)
    }

    pub fn copy(&mut self, src: &BstTrav) -> Option<Vec<u8>> {
        self.bst_type = src.bst_type.clone();
        (src.bst_type.trav_copy)(&src.trav_data);
        None
    }

    pub fn next(&mut self) -> Option<Vec<u8>> {
        (self.bst_type.trav_next)(&mut self.trav_data)
    }

    pub fn prev(&mut self) -> Option<Vec<u8>> {
        (self.bst_type.trav_prev)(&mut self.trav_data)
    }

    pub fn current(&self) -> Option<Vec<u8>> {
        (self.bst_type.trav_cur)(&self.trav_data)
    }

    pub fn replace(&mut self, new_item: Vec<u8>) -> Option<Vec<u8>> {
        (self.bst_type.trav_replace)(&mut self.trav_data, new_item)
    }
}