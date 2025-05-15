use std::alloc::{alloc, realloc, Layout};
use std::mem;
use std::ptr;

struct DynArrayHeader {
    used: usize,
    allocated: usize,
    array: *mut u8,
}

fn gl_dynarray_emplace_enlarge(
    list: &mut DynArrayHeader,
    scratch: *mut u8,
    element_size: usize,
) -> bool {
    let new_allocated = if list.allocated == 0 {
        match element_size {
            0..=3 => 16,
            4..=7 => 8,
            _ => 4,
        }
    } else {
        let new = list
            .allocated
            .checked_add(list.allocated / 2)
            .and_then(|n| n.checked_add(1))
            .unwrap_or(0);
        if new <= list.allocated {
            unsafe { *libc::__errno_location() = libc::ENOMEM };
            return false;
        }
        new
    };

    let new_size = match new_allocated.checked_mul(element_size) {
        Some(size) => size,
        None => {
            unsafe { *libc::__errno_location() = libc::ENOMEM };
            return false;
        }
    };

    let new_array = if list.array == scratch {
        let layout = Layout::from_size_align(new_size, mem::align_of::<u8>()).unwrap();
        unsafe {
            let new_ptr = alloc(layout);
            if !new_ptr.is_null() && !list.array.is_null() {
                ptr::copy_nonoverlapping(
                    list.array,
                    new_ptr,
                    list.used.checked_mul(element_size).unwrap(),
                );
            }
            new_ptr
        }
    } else {
        let layout = Layout::from_size_align(new_size, mem::align_of::<u8>()).unwrap();
        unsafe { realloc(list.array, layout, new_size) }
    };

    if new_array.is_null() {
        return false;
    }

    list.array = new_array;
    list.allocated = new_allocated;
    true
}