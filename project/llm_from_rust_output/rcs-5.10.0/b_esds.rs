use std::mem;
use std::ptr;

#[derive(Debug, Clone)]
pub struct Obstack {
    chunk_size: usize,
    chunk: Option<Box<ObstackChunk>>,
    object_base: Option<Vec<u8>>,
    next_free: usize,
    chunk_limit: usize,
    temp: TempUnion,
    alignment_mask: usize,
    chunkfun: ChunkFun,
    freefun: FreeFun,
    extra_arg: Option<Box<dyn std::any::Any>>,
    use_extra_arg: bool,
    maybe_empty_object: bool,
    alloc_failed: bool,
}

#[derive(Debug, Clone)]
pub enum FreeFun {
    Plain(Box<dyn Fn(Box<dyn std::any::Any>)>),
    Extra(Box<dyn Fn(Box<dyn std::any::Any>, Box<dyn std::any::Any>)>),
}

#[derive(Debug, Clone)]
pub enum ChunkFun {
    Plain(Box<dyn Fn(usize) -> Box<dyn std::any::Any>>),
    Extra(Box<dyn Fn(Box<dyn std::any::Any>, usize) -> Box<dyn std::any::Any>>),
}

#[derive(Debug, Clone)]
pub enum TempUnion {
    Int(usize),
    Ptr(Box<dyn std::any::Any>),
}

#[derive(Debug, Clone)]
pub struct ObstackChunk {
    limit: usize,
    prev: Option<Box<ObstackChunk>>,
    contents: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct WLink {
    pub entry: Box<dyn std::any::Any>,
    pub next: Option<Box<WLink>>,
}

#[derive(Debug, Clone)]
pub struct Divvy {
    pub name: String,
    pub space: Obstack,
    pub first: Option<Box<dyn std::any::Any>>,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub entry: Box<dyn std::any::Any>,
    pub next: Option<Box<Link>>,
}

impl Divvy {
    pub fn extend(&mut self, tp: &mut Link, x: Box<dyn std::any::Any>) -> &mut Link {
        let mut pair = Link {
            entry: x,
            next: None,
        };
        
        let boxed_pair = Box::new(pair);
        let raw_pair = Box::into_raw(boxed_pair);
        
        unsafe {
            (*raw_pair).next = None;
            tp.next = Some(Box::from_raw(raw_pair));
            &mut *raw_pair
        }
    }

    pub fn wextend(&mut self, tp: &mut WLink, x: Box<dyn std::any::Any>) -> &mut WLink {
        let mut pair = WLink {
            entry: x,
            next: None,
        };
        
        let boxed_pair = Box::new(pair);
        let raw_pair = Box::into_raw(boxed_pair);
        
        unsafe {
            (*raw_pair).next = None;
            tp.next = Some(Box::from_raw(raw_pair));
            &mut *raw_pair
        }
    }

    pub fn prepend(&mut self, x: Box<dyn std::any::Any>, ls: Option<Box<Link>>) -> Box<Link> {
        Box::new(Link {
            entry: x,
            next: ls,
        })
    }

    pub fn wprepend(&mut self, x: Box<dyn std::any::Any>, ls: Option<Box<WLink>>) -> Box<WLink> {
        Box::new(WLink {
            entry: x,
            next: ls,
        })
    }
}

fn alloc(_divvy: &mut Divvy, size: usize) -> Box<dyn std::any::Any> {
    let buffer = vec![0u8; size];
    Box::new(buffer)
}