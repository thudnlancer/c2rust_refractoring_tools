use std::mem;
use std::ptr;
use std::alloc::{alloc, realloc, Layout};

pub type size_t = usize;

#[derive(Copy, Clone)]
pub struct DynarrayHeader {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut u8,
}

pub fn gl_dynarray_resize(
    list: &mut DynarrayHeader,
    size: size_t,
    scratch: *mut u8,
    element_size: size_t,
) -> bool {
    if size <= list.allocated {
        list.used = size;
        return true;
    }

    let new_size_bytes = match size.checked_mul(element_size) {
        Some(bytes) => bytes,
        None => {
            std::alloc::handle_alloc_error(Layout::from_size_align(size * element_size, 1).unwrap());
        }
    };

    let new_array = if list.array == scratch {
        unsafe { alloc(Layout::from_size_align(new_size_bytes, 1).unwrap()) }
    } else {
        unsafe { realloc(list.array, Layout::from_size_align(list.allocated * element_size, 1).unwrap(), new_size_bytes) }
    };

    if new_array.is_null() {
        return false;
    }

    if list.array == scratch && !list.array.is_null() {
        unsafe {
            ptr::copy_nonoverlapping(list.array, new_array, list.used * element_size);
        }
    }

    list.array = new_array;
    list.allocated = size;
    list.used = size;
    true
}