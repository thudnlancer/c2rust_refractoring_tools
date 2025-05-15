/*
 * twemproxy - A fast and lightweight proxy for memcached protocol.
 * Copyright (C) 2011 Twitter, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::ptr;
use std::mem;
use std::slice;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

const MBUF_MAGIC: u32 = 0xdeadbeef;
const MBUF_MIN_SIZE: usize = 512;
const MBUF_MAX_SIZE: usize = 16777216;
const MBUF_SIZE: usize = 16384;
const MBUF_HSIZE: usize = mem::size_of::<Mbuf>();

type MbufCopyCallback = fn(&mut Mbuf, &mut dyn std::any::Any);

struct Mbuf {
    magic: u32,
    pos: *mut u8,
    last: *mut u8,
    start: *mut u8,
    end: *mut u8,
    data: Vec<u8>,
}

impl Mbuf {
    fn empty(&self) -> bool {
        self.pos == self.last
    }

    fn full(&self) -> bool {
        self.last == self.end
    }

    fn length(&self) -> u32 {
        unsafe { (self.last.offset_from(self.pos)) as u32 }
    }

    fn size(&self) -> u32 {
        unsafe { (self.end.offset_from(self.last)) as u32 }
    }
}

struct MbufPool {
    free_count: AtomicU32,
    free_queue: Mutex<VecDeque<Box<Mbuf>>>,
    chunk_size: usize,
    offset: usize,
}

impl MbufPool {
    fn new(chunk_size: usize) -> Self {
        let offset = chunk_size - MBUF_HSIZE;
        MbufPool {
            free_count: AtomicU32::new(0),
            free_queue: Mutex::new(VecDeque::new()),
            chunk_size,
            offset,
        }
    }

    fn get(&self) -> Option<Box<Mbuf>> {
        if let Some(mut mbuf) = self.free_queue.lock().unwrap().pop_front() {
            self.free_count.fetch_sub(1, Ordering::SeqCst);
            mbuf.pos = mbuf.start;
            mbuf.last = mbuf.start;
            Some(mbuf)
        } else {
            let mut data = vec![0u8; self.chunk_size];
            let buf_ptr = data.as_mut_ptr();
            let mbuf_ptr = unsafe { buf_ptr.add(self.offset) as *mut Mbuf };
            
            let mbuf = unsafe {
                ptr::write(mbuf_ptr, Mbuf {
                    magic: MBUF_MAGIC,
                    pos: ptr::null_mut(),
                    last: ptr::null_mut(),
                    start: buf_ptr,
                    end: buf_ptr.add(self.offset),
                    data: data,
                });
                &mut *mbuf_ptr
            };

            mbuf.pos = mbuf.start;
            mbuf.last = mbuf.start;
            
            Some(unsafe { Box::from_raw(mbuf_ptr) })
        }
    }

    fn put(&self, mut mbuf: Box<Mbuf>) {
        mbuf.pos = mbuf.start;
        mbuf.last = mbuf.start;
        self.free_count.fetch_add(1, Ordering::SeqCst);
        self.free_queue.lock().unwrap().push_front(mbuf);
    }

    fn data_size(&self) -> usize {
        self.offset
    }
}

struct MbufChain {
    pool: MbufPool,
    chain: Mutex<VecDeque<Box<Mbuf>>>,
}

impl MbufChain {
    fn new(pool: MbufPool) -> Self {
        MbufChain {
            pool,
            chain: Mutex::new(VecDeque::new()),
        }
    }

    fn insert(&self, mbuf: Box<Mbuf>) {
        self.chain.lock().unwrap().push_back(mbuf);
    }

    fn remove(&self, mbuf: &Box<Mbuf>) -> Option<Box<Mbuf>> {
        let mut chain = self.chain.lock().unwrap();
        if let Some(pos) = chain.iter().position(|m| ptr::eq(m.as_ref(), mbuf.as_ref())) {
            chain.remove(pos)
        } else {
            None
        }
    }

    fn copy(&self, mbuf: &mut Mbuf, pos: &[u8]) {
        if pos.is_empty() {
            return;
        }
        
        let available = unsafe { mbuf.end.offset_from(mbuf.last) as usize };
        assert!(available >= pos.len(), "Not enough space in mbuf");
        
        unsafe {
            ptr::copy_nonoverlapping(pos.as_ptr(), mbuf.last, pos.len());
            mbuf.last = mbuf.last.add(pos.len());
        }
    }

    fn split(&self, pos: usize, cb: Option<MbufCopyCallback>, cbarg: Option<&mut dyn std::any::Any>) -> Option<Box<Mbuf>> {
        let mut chain = self.chain.lock().unwrap();
        let mbuf = chain.back_mut()?;
        
        let data_pos = unsafe { mbuf.start.add(pos) };
        assert!(data_pos >= mbuf.pos && data_pos <= mbuf.last, "Invalid split position");

        let mut nbuf = self.pool.get()?;
        
        if let Some(callback) = cb {
            callback(&mut *nbuf, cbarg.unwrap());
        }

        let size = unsafe { mbuf.last.offset_from(data_pos) as usize };
        let data = unsafe { slice::from_raw_parts(data_pos, size) };
        self.copy(&mut *nbuf, data);

        unsafe {
            mbuf.last = data_pos;
        }

        Some(nbuf)
    }
}

fn mbuf_init(chunk_size: usize) -> MbufPool {
    MbufPool::new(chunk_size)
}

fn mbuf_deinit(pool: MbufPool) {
    let mut queue = pool.free_queue.lock().unwrap();
    while let Some(mbuf) = queue.pop_front() {
        pool.free_count.fetch_sub(1, Ordering::SeqCst);
    }
    assert_eq!(pool.free_count.load(Ordering::SeqCst), 0);
}