use std::mem;
use std::ptr;
use std::alloc::{self, Layout};
use std::num::NonZeroUsize;

#[derive(Debug)]
struct DynarrayHeader<T> {
    array: Option<Box<[T]>>,
    allocated: usize,
    used: usize,
}

fn dynarray_emplace_enlarge<T>(
    list: &mut DynarrayHeader<T>,
    scratch: Option<&mut [T]>,
) -> Result<(), alloc::LayoutError> {
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
        let new_allocated = list.allocated + list.allocated / 2 + 1;
        if new_allocated <= list.allocated {
            // Overflow.
            return Err(alloc::LayoutError);
        }
        new_allocated
    };

    let new_size = match NonZeroUsize::new(new_allocated.checked_mul(element_size)?) {
        Some(size) => size,
        None => return Err(alloc::LayoutError),
    };

    let new_layout = Layout::from_size_align(new_size.get(), mem::align_of::<T>())?;

    let new_array = if list.array.is_none() && scratch.is_some() {
        // The previous array was not heap-allocated.
        let new_array = unsafe {
            let ptr = alloc::alloc(new_layout) as *mut T;
            if ptr.is_null() {
                return Err(alloc::LayoutError);
            }
            Box::from_raw(ptr::slice_from_raw_parts_mut(ptr, new_allocated))
        };

        if let Some(scratch_slice) = scratch {
            let copy_len = list.used.min(scratch_slice.len());
            new_array[..copy_len].copy_from_slice(&scratch_slice[..copy_len]);
        }

        Some(new_array)
    } else {
        let old_len = list.allocated;
        let mut old_array = list.array.take().unwrap_or_else(|| {
            Box::from_raw(ptr::slice_from_raw_parts_mut(ptr::null_mut(), 0))
        });

        unsafe {
            let ptr = alloc::realloc(
                old_array.as_mut_ptr() as *mut u8,
                Layout::array::<T>(old_len)?,
                new_size.get(),
            ) as *mut T;

            if ptr.is_null() {
                return Err(alloc::LayoutError);
            }

            Some(Box::from_raw(ptr::slice_from_raw_parts_mut(ptr, new_allocated)))
        }
    };

    list.array = new_array;
    list.allocated = new_allocated;
    Ok(())
}