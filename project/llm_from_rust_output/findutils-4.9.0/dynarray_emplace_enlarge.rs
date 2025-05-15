use std::alloc::{alloc, realloc, Layout};
use std::mem;
use std::ptr;

#[derive(Debug)]
pub struct DynArrayHeader {
    pub used: usize,
    pub allocated: usize,
    pub array: *mut u8,
}

impl DynArrayHeader {
    pub fn new() -> Self {
        Self {
            used: 0,
            allocated: 0,
            array: ptr::null_mut(),
        }
    }
}

pub fn gl_dynarray_emplace_enlarge(
    list: &mut DynArrayHeader,
    scratch: *mut u8,
    element_size: usize,
) -> bool {
    let new_allocated = if list.allocated == 0 {
        match element_size {
            size if size < 4 => 16,
            size if size < 8 => 8,
            _ => 4,
        }
    } else {
        let new = list
            .allocated
            .wrapping_add(list.allocated / 2)
            .wrapping_add(1);
        if new <= list.allocated {
            return false; // ENOMEM
        }
        new
    };

    let new_size = match new_allocated.checked_mul(element_size) {
        Some(size) => size,
        None => return false, // overflow
    };

    let new_array = if list.array == scratch {
        let layout = match Layout::from_size_align(new_size, mem::align_of::<u8>()) {
            Ok(l) => l,
            Err(_) => return false,
        };
        unsafe { alloc(layout) }
    } else {
        let layout = match Layout::from_size_align(new_size, mem::align_of::<u8>()) {
            Ok(l) => l,
            Err(_) => return false,
        };
        unsafe { realloc(list.array, layout, new_size) }
    };

    if new_array.is_null() {
        return false;
    }

    if list.array == scratch && !list.array.is_null() {
        unsafe {
            ptr::copy_nonoverlapping(
                list.array,
                new_array,
                list.used.wrapping_mul(element_size),
            );
        }
    }

    list.array = new_array;
    list.allocated = new_allocated;
    true
}