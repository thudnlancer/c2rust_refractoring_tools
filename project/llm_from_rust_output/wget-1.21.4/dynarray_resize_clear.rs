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

pub fn gl_dynarray_resize_clear(
    list: &mut DynArrayHeader,
    size: SizeType,
    element_size: SizeType,
) -> bool {
    let old_size = list.used;
    
    if !gl_dynarray_resize(list, size, element_size) {
        return false;
    }

    if size > old_size {
        let start = old_size * element_size;
        let end = size * element_size;
        if end > list.array.len() {
            list.array.resize(end, 0);
        }
        let slice = &mut list.array[start..end];
        slice.fill(0);
    }

    true
}

fn gl_dynarray_resize(
    list: &mut DynArrayHeader,
    size: SizeType,
    element_size: SizeType,
) -> bool {
    if size > list.allocated {
        let new_capacity = size.next_power_of_two();
        list.array.reserve(new_capacity * element_size);
        list.allocated = new_capacity;
    }
    list.used = size;
    true
}