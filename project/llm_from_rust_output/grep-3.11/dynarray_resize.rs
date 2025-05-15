use std::mem;
use std::ptr;
use std::alloc::{alloc, realloc, Layout};

#[derive(Clone, Copy)]
pub struct DynArrayHeader {
    pub used: usize,
    pub allocated: usize,
    pub array: *mut u8,
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
            std::ptr::write_volatile(std::ptr::null_mut(), 12); // ENOMEM
            return false;
        }
    };

    let new_array = if list.array == scratch {
        unsafe {
            let layout = Layout::from_size_align(new_size_bytes, mem::align_of::<u8>()).unwrap();
            let new_ptr = alloc(layout);
            if !new_ptr.is_null() && !list.array.is_null() {
                ptr::copy_nonoverlapping(
                    list.array,
                    new_ptr,
                    list.used.wrapping_mul(element_size),
                );
            }
            new_ptr
        }
    } else {
        unsafe {
            let layout = Layout::from_size_align(list.allocated * element_size, mem::align_of::<u8>())
                .unwrap();
            realloc(list.array, layout, new_size_bytes)
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