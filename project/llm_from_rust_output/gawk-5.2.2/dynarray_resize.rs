use std::mem;
use std::ptr;
use std::os::raw::{c_void, c_int, c_ulong};

pub type size_t = c_ulong;

#[derive(Clone, Copy)]
pub struct DynarrayHeader {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut c_void,
}

pub fn libc_dynarray_resize(
    list: &mut DynarrayHeader,
    size: size_t,
    scratch: *mut c_void,
    element_size: size_t,
) -> bool {
    if size <= list.allocated {
        list.used = size;
        return true;
    }

    let new_size_bytes = match size.checked_mul(element_size) {
        Some(v) => v,
        None => {
            unsafe { *libc::__errno_location() = libc::ENOMEM };
            return false;
        }
    };

    let new_array = if list.array == scratch {
        let new_ptr = unsafe { libc::malloc(new_size_bytes) };
        if !new_ptr.is_null() && !list.array.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(
                    list.array.cast::<u8>(),
                    new_ptr.cast::<u8>(),
                    list.used * element_size,
                );
            }
        }
        new_ptr
    } else {
        unsafe { libc::realloc(list.array, new_size_bytes) }
    };

    if new_array.is_null() {
        return false;
    }

    list.array = new_array;
    list.allocated = size;
    list.used = size;
    true
}