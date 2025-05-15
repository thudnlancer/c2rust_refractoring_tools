use std::alloc::{self, Layout};
use std::mem;
use std::ptr;

/// Represents a dynamic array header similar to the C version
#[derive(Debug)]
struct DynarrayHeader<T> {
    array: Option<ptr::NonNull<T>>,
    allocated: usize,
    used: usize,
}

impl<T> DynarrayHeader<T> {
    fn new() -> Self {
        DynarrayHeader {
            array: None,
            allocated: 0,
            used: 0,
        }
    }
}

/// Increases the size of a dynamic array in preparation for an emplace operation.
/// Returns Ok(()) on success, Err(()) on failure.
fn dynarray_emplace_enlarge<T>(
    list: &mut DynarrayHeader<T>,
    scratch: Option<&mut [T]>,
) -> Result<(), ()> {
    let element_size = mem::size_of::<T>();
    let new_allocated = if list.allocated == 0 {
        // No scratch buffer provided. Choose a reasonable default size.
        if element_size < 4 {
            16
        } else if element_size < 8 {
            8
        } else {
            4
        }
    } else {
        // Increase the allocated size, using an exponential growth policy.
        let new = list.allocated + list.allocated / 2 + 1;
        if new <= list.allocated {
            // Overflow
            return Err(());
        }
        new
    };

    let new_size = match new_allocated.checked_mul(element_size) {
        Some(size) => size,
        None => return Err(()),
    };

    let new_layout = match Layout::array::<T>(new_allocated) {
        Ok(layout) => layout,
        Err(_) => return Err(()),
    };

    let new_array = unsafe {
        if let Some(scratch_buf) = scratch {
            if list.array.map_or(false, |ptr| ptr.as_ptr() == scratch_buf.as_mut_ptr()) {
                // The previous array was not heap-allocated.
                let new_ptr = alloc::alloc(new_layout) as *mut T;
                if new_ptr.is_null() {
                    return Err(());
                }
                if let Some(old_ptr) = list.array {
                    ptr::copy_nonoverlapping(
                        old_ptr.as_ptr(),
                        new_ptr,
                        list.used,
                    );
                }
                ptr::NonNull::new(new_ptr)
            } else {
                // Reallocate existing heap memory
                let old_ptr = list.array.map(|p| p.as_ptr() as *mut u8);
                let old_size = list.allocated * element_size;
                let old_layout = Layout::from_size_align(old_size, new_layout.align()).unwrap();
                alloc::realloc(old_ptr.unwrap(), old_layout, new_size) as *mut T
            }
        } else {
            // No scratch buffer provided
            alloc::alloc(new_layout) as *mut T
        }
    };

    let new_array = match ptr::NonNull::new(new_array) {
        Some(ptr) => ptr,
        None => return Err(()),
    };

    list.array = Some(new_array);
    list.allocated = new_allocated;
    Ok(())
}