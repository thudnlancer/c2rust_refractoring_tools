use std::cmp::Ordering;
use std::mem;
use std::ptr;
use std::slice;

pub type rstatus_t = i32;
pub type uint32_t = u32;
pub type uint8_t = u8;

pub struct Array<T> {
    nelem: uint32_t,
    elem: Vec<T>,
    nalloc: uint32_t,
}

impl<T> Array<T> {
    pub fn new(n: uint32_t) -> Self {
        Array {
            nelem: 0,
            elem: Vec::with_capacity(n as usize),
            nalloc: n,
        }
    }

    pub fn len(&self) -> uint32_t {
        self.nelem
    }

    pub fn init(&mut self, n: uint32_t) -> rstatus_t {
        self.elem = Vec::with_capacity(n as usize);
        self.nelem = 0;
        self.nalloc = n;
        0
    }

    pub fn deinit(&mut self) {
        self.elem.clear();
        self.elem.shrink_to_fit();
    }

    pub fn idx(&self, elem: &T) -> uint32_t {
        let base = self.elem.as_ptr();
        let elem_ptr = elem as *const T;
        let offset = unsafe { elem_ptr.offset_from(base) } as uint32_t;
        offset / mem::size_of::<T>() as uint32_t
    }

    pub fn push(&mut self) -> Option<&mut T> {
        if self.nelem == self.nalloc {
            self.nalloc *= 2;
            self.elem.reserve(self.nalloc as usize);
        }

        if self.nelem < self.nalloc {
            self.nelem += 1;
            self.elem.push(unsafe { mem::zeroed() });
            self.elem.last_mut()
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<&mut T> {
        if self.nelem > 0 {
            self.nelem -= 1;
            self.elem.pop()
        } else {
            None
        }
    }

    pub fn get(&self, idx: uint32_t) -> Option<&T> {
        self.elem.get(idx as usize)
    }

    pub fn get_mut(&mut self, idx: uint32_t) -> Option<&mut T> {
        self.elem.get_mut(idx as usize)
    }

    pub fn top(&self) -> Option<&T> {
        if self.nelem > 0 {
            self.elem.get((self.nelem - 1) as usize)
        } else {
            None
        }
    }

    pub fn swap(&mut self, other: &mut Array<T>) {
        mem::swap(&mut self.nelem, &mut other.nelem);
        mem::swap(&mut self.elem, &mut other.elem);
        mem::swap(&mut self.nalloc, &mut other.nalloc);
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.elem.sort_by(compare);
    }

    pub fn each<F>(&mut self, mut func: F, data: &mut T) -> rstatus_t
    where
        F: FnMut(&mut T, &mut T) -> rstatus_t,
    {
        for item in self.elem.iter_mut() {
            let status = func(item, data);
            if status != 0 {
                return status;
            }
        }
        0
    }
}

impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        self.deinit();
    }
}