use std::alloc::{alloc, realloc, Layout};
use std::mem;
use std::ptr;

pub type size_t = usize;

#[derive(Copy, Clone)]
pub struct DynarrayHeader {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut u8,
}

pub fn dynarray_emplace_enlarge(
    list: &mut DynarrayHeader,
    scratch: *mut u8,
    element_size: size_t,
) -> bool {
    let new_allocated = if list.allocated == 0 {
        match element_size {
            x if x < 4 => 16,
            x if x < 8 => 8,
            _ => 4,
        }
    } else {
        let new = list
            .allocated
            .wrapping_add(list.allocated / 2)
            .wrapping_add(1);
        if new <= list.allocated {
            unsafe {
                *libc::__errno_location() = libc::ENOMEM;
            }
            return false;
        }
        new
    };

    let new_size = match new_allocated.checked_mul(element_size) {
        Some(size) => size,
        None => {
            unsafe {
                *libc::__errno_location() = libc::ENOMEM;
            }
            return false;
        }
    };

    let new_array = if list.array == scratch {
        unsafe {
            let layout = Layout::from_size_align(new_size, mem::align_of::<u8>()).unwrap();
            let ptr = alloc(layout);
            if !ptr.is_null() && !list.array.is_null() {
                ptr::copy_nonoverlapping(
                    list.array,
                    ptr,
                    list.used.wrapping_mul(element_size),
                );
            }
            ptr
        }
    } else {
        unsafe {
            let layout = Layout::from_size_align(new_size, mem::align_of::<u8>()).unwrap();
            realloc(list.array, layout, new_size)
        }
    };

    if new_array.is_null() {
        return false;
    }

    list.array = new_array;
    list.allocated = new_allocated;
    true
}