use std::mem;
use std::ptr;
use std::alloc::{alloc, realloc, Layout};
use std::os::raw::c_void;

pub type size_t = usize;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynarrayHeader {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut c_void,
}

fn gl_dynarray_emplace_enlarge(
    list: &mut DynarrayHeader,
    scratch: *mut c_void,
    element_size: size_t,
) -> bool {
    let new_allocated = if list.allocated == 0 {
        if element_size < 4 {
            16
        } else if element_size < 8 {
            8
        } else {
            4
        }
    } else {
        let new = list.allocated + (list.allocated / 2) + 1;
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
                    list.array as *const u8,
                    ptr,
                    list.used * element_size,
                );
            }
            ptr as *mut c_void
        }
    } else {
        unsafe {
            let layout = Layout::from_size_align(new_size, mem::align_of::<u8>()).unwrap();
            realloc(list.array, layout, new_size) as *mut c_void
        }
    };

    if new_array.is_null() {
        return false;
    }

    list.array = new_array;
    list.allocated = new_allocated;
    true
}