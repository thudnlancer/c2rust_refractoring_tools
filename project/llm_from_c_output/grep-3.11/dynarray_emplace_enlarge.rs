use std::mem;
use std::ptr;
use std::alloc::{self, Layout};
use std::num::NonZeroUsize;

#[derive(Debug)]
struct DynarrayHeader {
    array: Option<ptr::NonNull<u8>>,
    allocated: usize,
    used: usize,
}

fn dynarray_emplace_enlarge(
    list: &mut DynarrayHeader,
    scratch: Option<ptr::NonNull<u8>>,
    element_size: usize,
) -> Result<(), alloc::LayoutError> {
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
        let new_allocated = list.allocated + list.allocated / 2 + 1;
        if new_allocated <= list.allocated {
            // Overflow.
            return Err(alloc::LayoutError);
        }
        new_allocated
    };

    let new_size = new_allocated.checked_mul(element_size).ok_or(alloc::LayoutError)?;
    let layout = Layout::from_size_align(new_size, mem::align_of::<u8>())?;

    let new_array = if list.array == scratch {
        // The previous array was not heap-allocated.
        let new_ptr = unsafe { alloc::alloc(layout) };
        if new_ptr.is_null() {
            return Err(alloc::LayoutError);
        }
        let new_array = ptr::NonNull::new(new_ptr).ok_or(alloc::LayoutError)?;
        
        if let Some(old_array) = list.array {
            unsafe {
                ptr::copy_nonoverlapping(
                    old_array.as_ptr(),
                    new_ptr,
                    list.used * element_size,
                );
            }
        }
        new_array
    } else {
        let old_ptr = list.array.map_or(ptr::null_mut(), |p| p.as_ptr());
        let new_ptr = unsafe { alloc::realloc(old_ptr, layout, new_size) };
        ptr::NonNull::new(new_ptr).ok_or(alloc::LayoutError)?
    };

    list.array = Some(new_array);
    list.allocated = new_allocated;
    Ok(())
}