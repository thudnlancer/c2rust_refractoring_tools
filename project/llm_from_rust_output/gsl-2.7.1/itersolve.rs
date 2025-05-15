use std::ffi::CString;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
    Badtol = 13,
    Tol = 14,
    Undrflw = 15,
    Ovrflw = 16,
    Loss = 17,
    Round = 18,
    Badlen = 19,
    Notsqr = 20,
    Sing = 21,
    Diverge = 22,
    Unsup = 23,
    Unimpl = 24,
    Cache = 25,
    Table = 26,
    Noprog = 27,
    Noprogj = 28,
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
}

#[derive(Debug, Clone)]
pub struct GslBlock {
    pub size: usize,
    pub data: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct GslVector {
    pub size: usize,
    pub stride: usize,
    pub data: Vec<f64>,
    pub block: Option<Box<GslBlock>>,
    pub owner: i32,
}

#[derive(Debug, Clone)]
pub struct GslSpMatrixPoolNode {
    pub next: Option<Box<GslSpMatrixPoolNode>>,
    pub block_ptr: Option<Box<Vec<u8>>>,
    pub free_slot: Option<Box<Vec<u8>>>,
}

pub type GslSpMatrixPool = GslSpMatrixPoolNode;

pub type GslBstCmpFunction = fn(&[u8], &[u8], &mut [u8]) -> i32;

#[derive(Debug, Clone)]
pub struct GslBstAllocator {
    pub alloc: Option<fn(usize, &mut [u8]) -> Option<Box<Vec<u8>>>>,
    pub free: Option<fn(Box<Vec<u8>>, &mut [u8])>,
}

#[derive(Debug, Clone)]
pub struct GslBstAvlNode {
    pub avl_link: [Option<Box<GslBstAvlNode>>; 2],
    pub avl_data: Option<Box<Vec<u8>>>,
    pub avl_balance: i8,
}

#[derive(Debug, Clone)]
pub struct GslBstAvlTable {
    pub avl_root: Option<Box<GslBstAvlNode>>,
    pub avl_compare: Option<GslBstCmpFunction>,
    pub avl_param: Option<Box<Vec<u8>>>,
    pub avl_alloc: Option<Box<GslBstAllocator>>,
    pub avl_count: usize,
    pub avl_generation: u64,
}

#[derive(Debug, Clone)]
pub struct GslBstRbNode {
    pub rb_link: [Option<Box<GslBstRbNode>>; 2],
    pub rb_data: Option<Box<Vec<u8>>>,
    pub rb_color: u8,
}

#[derive(Debug, Clone)]
pub struct GslBstRbTable {
    pub rb_root: Option<Box<GslBstRbNode>>,
    pub rb_compare: Option<GslBstCmpFunction>,
    pub rb_param: Option<Box<Vec<u8>>>,
    pub rb_alloc: Option<Box<GslBstAllocator>>,
    pub rb_count: usize,
    pub rb_generation: u64,
}

#[derive(Debug, Clone)]
pub struct GslBstType {
    pub name: String,
    pub node_size: usize,
    pub init: Option<fn(&GslBstAllocator, Option<GslBstCmpFunction>, &mut [u8], &mut [u8]) -> i32>,
    pub nodes: Option<fn(&[u8]) -> usize>,
    pub insert: Option<fn(&mut [u8], &mut [u8]) -> Option<Box<Vec<u8>>>>,
    pub find: Option<fn(&[u8], &[u8]) -> Option<Box<Vec<u8>>>>,
    pub remove: Option<fn(&[u8], &mut [u8]) -> Option<Box<Vec<u8>>>>,
    pub empty: Option<fn(&mut [u8]) -> i32>,
    pub trav_init: Option<fn(&mut [u8], &[u8]) -> i32>,
    pub trav_first: Option<fn(&mut [u8], &[u8]) -> Option<Box<Vec<u8>>>>,
    pub trav_last: Option<fn(&mut [u8], &[u8]) -> Option<Box<Vec<u8>>>>,
    pub trav_find: Option<fn(&[u8], &mut [u8], &[u8]) -> Option<Box<Vec<u8>>>>,
    pub trav_insert: Option<fn(&mut [u8], &mut [u8], &mut [u8]) -> Option<Box<Vec<u8>>>>,
    pub trav_copy: Option<fn(&mut [u8], &[u8]) -> Option<Box<Vec<u8>>>>,
    pub trav_next: Option<fn(&mut [u8]) -> Option<Box<Vec<u8>>>>,
    pub trav_prev: Option<fn(&mut [u8]) -> Option<Box<Vec<u8>>>>,
    pub trav_cur: Option<fn(&[u8]) -> Option<Box<Vec<u8>>>>,
    pub trav_replace: Option<fn(&mut [u8], &mut [u8]) -> Option<Box<Vec<u8>>>>,
}

#[derive(Debug, Clone)]
pub enum GslBstTable {
    Avl(GslBstAvlTable),
    Rb(GslBstRbTable),
}

#[derive(Debug, Clone)]
pub struct GslBstWorkspace {
    pub type_: Box<GslBstType>,
    pub table: GslBstTable,
}

#[derive(Debug, Clone)]
pub enum GslSpMatrixWork {
    Void(Option<Box<Vec<u8>>>),
    Int(Option<Box<Vec<i32>>>),
    Atomic(Option<Box<Vec<f64>>>),
}

#[derive(Debug, Clone)]
pub struct GslSpMatrix {
    pub size1: usize,
    pub size2: usize,
    pub i: Vec<i32>,
    pub data: Vec<f64>,
    pub p: Vec<i32>,
    pub nzmax: usize,
    pub nz: usize,
    pub tree: Option<Box<GslBstWorkspace>>,
    pub pool: Option<Box<GslSpMatrixPool>>,
    pub node_size: usize,
    pub work: GslSpMatrixWork,
    pub sptype: i32,
    pub spflags: usize,
}

#[derive(Debug, Clone)]
pub struct GslSplinalgItersolveType {
    pub name: String,
    pub alloc: Option<fn(usize, usize) -> Option<Box<Vec<u8>>>>,
    pub iterate: Option<fn(&GslSpMatrix, &GslVector, f64, &mut GslVector, &mut [u8]) -> i32>,
    pub normr: Option<fn(&[u8]) -> f64>,
    pub free: Option<fn(Box<Vec<u8>>)>,
}

#[derive(Debug, Clone)]
pub struct GslSplinalgItersolve {
    pub type_: Box<GslSplinalgItersolveType>,
    pub normr: f64,
    pub state: Option<Box<Vec<u8>>>,
}

impl GslSplinalgItersolve {
    pub fn alloc(
        type_: &GslSplinalgItersolveType,
        n: usize,
        m: usize,
    ) -> Result<Self, GslError> {
        let mut w = Self {
            type_: Box::new(type_.clone()),
            normr: 0.0,
            state: None,
        };

        if let Some(alloc_fn) = type_.alloc {
            w.state = alloc_fn(n, m);
            if w.state.is_none() {
                return Err(GslError::Nomem);
            }
        } else {
            return Err(GslError::Nomem);
        }

        Ok(w)
    }

    pub fn free(&mut self) {
        if let Some(free_fn) = self.type_.free {
            if let Some(state) = self.state.take() {
                free_fn(state);
            }
        }
    }

    pub fn name(&self) -> &str {
        &self.type_.name
    }

    pub fn iterate(
        &mut self,
        a: &GslSpMatrix,
        b: &GslVector,
        tol: f64,
        x: &mut GslVector,
    ) -> Result<i32, GslError> {
        if let Some(iterate_fn) = self.type_.iterate {
            let status = iterate_fn(a, b, tol, x, self.state.as_mut().unwrap());
            if let Some(normr_fn) = self.type_.normr {
                self.normr = normr_fn(self.state.as_ref().unwrap());
            }
            Ok(status)
        } else {
            Err(GslError::Failure)
        }
    }

    pub fn normr(&self) -> f64 {
        self.normr
    }
}