use std::mem;
use std::ptr;
use std::alloc::{alloc, realloc, Layout};
use std::num::NonZeroUsize;

#[derive(Debug, Clone)]
pub struct DynArrayHeader {
    pub used: usize,
    pub allocated: usize,
    pub array: *mut u8,
}

impl DynArrayHeader {
    pub fn new() -> Self {
        DynArrayHeader {
            used: 0,
            allocated: 0,
            array: ptr::null_mut(),
        }
    }
}

pub fn gl_dynarray_resize(
    list: &mut DynArrayHeader,
    size: usize,
    scratch: *mut u8,
    element_size: usize,
) -> bool {
    if size <= list.allocated {
        list.used = size;
        return true;
    }

    let new_size_bytes = match size.checked_mul(element_size) {
        Some(size) => size,
        None => {
            std::alloc::handle_alloc_error(Layout::from_size_align(size * element_size, 1).unwrap());
            return false;
        }
    };

    let new_array = if list.array == scratch {
        unsafe {
            let new_ptr = alloc(Layout::from_size_align(new_size_bytes, 1).unwrap());
            if !new_ptr.is_null() && !list.array.is_null() {
                ptr::copy_nonoverlapping(
                    list.array,
                    new_ptr,
                    list.used * element_size,
                );
            }
            new_ptr
        }
    } else {
        unsafe {
            realloc(
                list.array,
                Layout::from_size_align(list.allocated * element_size, 1).unwrap(),
                new_size_bytes,
            )
        }
    };

    if new_array.is_null() {
        return false;
    }

    list.array = new_array;
    list.allocated = size;
    list.used = size;
    true
}