use std::mem;
use std::ptr;
use std::os::raw::{c_char, c_int, c_ulong, c_uchar, c_ushort, c_uint, c_long};
use std::sync::atomic::{AtomicU32, Ordering};
use std::collections::LinkedList;

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint16_t = c_ushort;
type uint32_t = c_uint;
type int64_t = c_long;
type pid_t = c_int;

#[derive(Debug, Clone)]
struct Mbuf {
    magic: u32,
    pos: usize,
    last: usize,
    start: usize,
    end: usize,
    data: Vec<u8>,
}

impl Mbuf {
    fn new(chunk_size: size_t, offset: size_t) -> Option<Self> {
        let mut data = vec![0; chunk_size as usize];
        let mbuf = Self {
            magic: 0xdeadbeef,
            pos: offset as usize,
            last: offset as usize,
            start: offset as usize,
            end: chunk_size as usize,
            data,
        };
        Some(mbuf)
    }

    fn length(&self) -> uint32_t {
        (self.last - self.pos) as uint32_t
    }

    fn size(&self) -> uint32_t {
        (self.end - self.last) as uint32_t
    }

    fn rewind(&mut self) {
        self.pos = self.start;
        self.last = self.start;
    }

    fn copy(&mut self, pos: &[u8], n: size_t) {
        if n == 0 {
            return;
        }
        let dst = &mut self.data[self.last..self.last + n as usize];
        dst.copy_from_slice(&pos[..n as usize]);
        self.last += n as usize;
    }
}

struct MbufPool {
    chunk_size: size_t,
    offset: size_t,
    free_list: LinkedList<Mbuf>,
    free_count: AtomicU32,
}

impl MbufPool {
    fn new(chunk_size: size_t) -> Self {
        let offset = chunk_size - mem::size_of::<Mbuf>() as size_t;
        Self {
            chunk_size,
            offset,
            free_list: LinkedList::new(),
            free_count: AtomicU32::new(0),
        }
    }

    fn get(&mut self) -> Option<Mbuf> {
        if let Some(mut mbuf) = self.free_list.pop_front() {
            self.free_count.fetch_sub(1, Ordering::Relaxed);
            mbuf.rewind();
            Some(mbuf)
        } else {
            Mbuf::new(self.chunk_size, self.offset)
        }
    }

    fn put(&mut self, mut mbuf: Mbuf) {
        mbuf.rewind();
        self.free_list.push_back(mbuf);
        self.free_count.fetch_add(1, Ordering::Relaxed);
    }

    fn init(&mut self, nci: &Instance) {
        self.chunk_size = nci.mbuf_chunk_size;
        self.offset = self.chunk_size - mem::size_of::<Mbuf>() as size_t;
        self.free_list.clear();
        self.free_count.store(0, Ordering::Relaxed);
    }

    fn deinit(&mut self) {
        self.free_list.clear();
        self.free_count.store(0, Ordering::Relaxed);
    }
}

#[derive(Debug)]
struct Instance {
    mbuf_chunk_size: size_t,
    // Other fields omitted for brevity
}

// Global mbuf pool
static mut MBUF_POOL: Option<MbufPool> = None;

fn mbuf_init(nci: &Instance) {
    unsafe {
        MBUF_POOL = Some(MbufPool::new(nci.mbuf_chunk_size));
    }
}

fn mbuf_deinit() {
    unsafe {
        if let Some(pool) = MBUF_POOL.take() {
            // Pool will be dropped automatically
        }
    }
}

fn mbuf_get() -> Option<Mbuf> {
    unsafe {
        MBUF_POOL.as_mut().and_then(|pool| pool.get())
    }
}

fn mbuf_put(mbuf: Mbuf) {
    unsafe {
        if let Some(pool) = MBUF_POOL.as_mut() {
            pool.put(mbuf);
        }
    }
}

fn mbuf_rewind(mbuf: &mut Mbuf) {
    mbuf.rewind();
}

fn mbuf_length(mbuf: &Mbuf) -> uint32_t {
    mbuf.length()
}

fn mbuf_size(mbuf: &Mbuf) -> uint32_t {
    mbuf.size()
}

fn mbuf_data_size() -> size_t {
    unsafe {
        MBUF_POOL.as_ref().map(|pool| pool.offset).unwrap_or(0)
    }
}

fn mbuf_copy(mbuf: &mut Mbuf, pos: &[u8], n: size_t) {
    mbuf.copy(pos, n);
}

// Other functions would be similarly converted to safe Rust