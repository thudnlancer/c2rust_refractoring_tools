use std::mem;
use std::ptr;
use std::slice;

pub type SizeType = usize;

#[derive(Clone)]
pub struct DynArrayHeader {
    pub used: SizeType,
    pub allocated: SizeType,
    pub array: Vec<u8>,
}

impl DynArrayHeader {
    pub fn new() -> Self {
        DynArrayHeader {
            used: 0,
            allocated: 0,
            array: Vec::new(),
        }
    }

    pub fn resize(&mut self, new_size: SizeType, element_size: SizeType) -> bool {
        if new_size > self.allocated {
            let new_capacity = new_size.next_power_of_two();
            self.array.resize(new_capacity * element_size, 0);
            self.allocated = new_capacity;
        }
        self.used = new_size;
        true
    }

    pub fn resize_clear(&mut self, new_size: SizeType, element_size: SizeType) -> bool {
        let old_size = self.used;
        if !self.resize(new_size, element_size) {
            return false;
        }

        if new_size > old_size {
            let start = old_size * element_size;
            let end = new_size * element_size;
            if end > self.array.len() {
                return false;
            }
            let slice = &mut self.array[start..end];
            slice.fill(0);
        }
        true
    }
}

pub fn gl_dynarray_resize_clear(
    list: &mut DynArrayHeader,
    size: SizeType,
    element_size: SizeType,
) -> bool {
    list.resize_clear(size, element_size)
}