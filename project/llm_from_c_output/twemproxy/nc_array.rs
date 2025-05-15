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

use std::cmp::Ordering;
use std::mem;
use std::ptr;
use std::slice;

type ArrayCompare<T> = fn(&T, &T) -> Ordering;
type ArrayEach<T, D> = fn(&mut T, &mut D) -> Result<(), ()>;

struct Array<T> {
    nelem: u32,    // # elements
    elem: Vec<T>,  // elements
    nalloc: u32,   // # allocated elements
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Array {
            nelem: 0,
            elem: Vec::new(),
            nalloc: 0,
        }
    }
}

impl<T> Array<T> {
    fn new() -> Self {
        Self::default()
    }

    fn with_capacity(n: u32) -> Self {
        Array {
            nelem: 0,
            elem: Vec::with_capacity(n as usize),
            nalloc: n,
        }
    }

    fn n(&self) -> u32 {
        self.nelem
    }

    fn idx(&self, elem: &T) -> Option<u32> {
        let base = self.elem.as_ptr();
        let elem_ptr = elem as *const T;
        
        if elem_ptr < base || elem_ptr >= unsafe { base.add(self.nelem as usize) } {
            return None;
        }

        let offset = unsafe { elem_ptr.offset_from(base) } as u32;
        Some(offset)
    }

    fn push(&mut self) -> Option<&mut T> {
        if self.nelem == self.nalloc {
            let new_cap = if self.nalloc == 0 { 1 } else { self.nalloc * 2 };
            self.elem.reserve_exact(new_cap as usize);
            self.nalloc = new_cap;
        }

        if self.nelem < self.nalloc {
            self.nelem += 1;
            // Safety: we just checked bounds and increased nelem
            unsafe {
                let elem = self.elem.get_unchecked_mut(self.nelem as usize - 1);
                ptr::write(elem, mem::zeroed());
                Some(elem)
            }
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<&mut T> {
        if self.nelem == 0 {
            None
        } else {
            self.nelem -= 1;
            // Safety: we just checked bounds and decreased nelem
            unsafe { Some(self.elem.get_unchecked_mut(self.nelem as usize)) }
        }
    }

    fn get(&self, idx: u32) -> Option<&T> {
        if idx < self.nelem {
            self.elem.get(idx as usize)
        } else {
            None
        }
    }

    fn get_mut(&mut self, idx: u32) -> Option<&mut T> {
        if idx < self.nelem {
            self.elem.get_mut(idx as usize)
        } else {
            None
        }
    }

    fn top(&self) -> Option<&T> {
        if self.nelem == 0 {
            None
        } else {
            self.get(self.nelem - 1)
        }
    }

    fn top_mut(&mut self) -> Option<&mut T> {
        if self.nelem == 0 {
            None
        } else {
            self.get_mut(self.nelem - 1)
        }
    }

    fn swap(&mut self, other: &mut Self) {
        mem::swap(&mut self.nelem, &mut other.nelem);
        mem::swap(&mut self.elem, &mut other.elem);
        mem::swap(&mut self.nalloc, &mut other.nalloc);
    }

    fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.elem[..self.nelem as usize].sort_by(compare);
    }

    fn each<D, F>(&mut self, mut func: F, data: &mut D) -> Result<(), ()>
    where
        F: FnMut(&mut T, &mut D) -> Result<(), ()>,
    {
        for item in self.elem.iter_mut().take(self.nelem as usize) {
            func(item, data)?;
        }
        Ok(())
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        self.elem.clear();
        self.nelem = 0;
        self.nalloc = 0;
    }
}

fn array_create<T>(n: u32) -> Option<Array<T>> {
    if n == 0 {
        return None;
    }
    Some(Array::with_capacity(n))
}

fn array_init<T>(a: &mut Array<T>, n: u32) -> Result<(), ()> {
    if n == 0 {
        return Err(());
    }
    *a = Array::with_capacity(n);
    Ok(())
}

fn array_deinit<T>(a: &mut Array<T>) {
    a.elem.clear();
    a.nelem = 0;
    a.nalloc = 0;
}