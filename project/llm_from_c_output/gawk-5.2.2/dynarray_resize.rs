use std::mem;
use std::ptr;
use std::alloc::{self, Layout};
use std::num::TryFromIntError;

struct DynarrayHeader {
    array: *mut u8,
    allocated: usize,
    used: usize,
}

fn dynarray_resize(
    list: &mut DynarrayHeader,
    size: usize,
    scratch: *mut u8,
    element_size: usize,
) -> Result<(), TryFromIntError> {
    // The existing allocation provides sufficient room.
    if size <= list.allocated {
        list.used = size;
        return Ok(());
    }

    // Otherwise, use size as the new allocation size. The caller is
    // expected to provide the final size of the array, so there is no
    // over-allocation here.

    let new_size_bytes = size.checked_mul(element_size).ok_or_else(|| {
        std::num::TryFromIntError(())
    })?;

    let new_array = if list.array == scratch {
        // The previous array was not heap-allocated.
        let layout = Layout::from_size_align(new_size_bytes, mem::align_of::<u8>())
            .map_err(|_| std::num::TryFromIntError(()))?;
        let new_array = unsafe { alloc::alloc(layout) };
        if !new_array.is_null() && !list.array.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(
                    list.array,
                    new_array,
                    list.used * element_size,
                );
            }
        }
        new_array
    } else {
        let layout = Layout::from_size_align(list.allocated * element_size, mem::align_of::<u8>())
            .map_err(|_| std::num::TryFromIntError(()))?;
        unsafe { alloc::realloc(list.array, layout, new_size_bytes) }
    };

    if new_array.is_null() {
        return Err(std::num::TryFromIntError(()));
    }

    list.array = new_array;
    list.allocated = size;
    list.used = size;
    Ok(())
}