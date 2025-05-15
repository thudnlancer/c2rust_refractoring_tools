use std::mem;
use std::ptr;
use std::alloc::{self, Layout};
use std::num::TryFromIntError;

#[derive(Debug)]
struct DynarrayHeader<T> {
    array: Option<Box<[T]>>,
    allocated: usize,
    used: usize,
}

fn dynarray_emplace_enlarge<T>(
    list: &mut DynarrayHeader<T>,
    scratch: Option<&mut [T]>,
) -> Result<(), TryFromIntError> {
    let element_size = mem::size_of::<T>();
    let new_allocated = if list.allocated == 0 {
        if element_size < 4 {
            16
        } else if element_size < 8 {
            8
        } else {
            4
        }
    } else {
        let new = list.allocated + list.allocated / 2 + 1;
        if new <= list.allocated {
            return Err(TryFromIntError(()));
        }
        new
    };

    let new_size = new_allocated.checked_mul(element_size).ok_or(TryFromIntError(()))?;

    let new_array = if let Some(array) = &mut list.array {
        let mut new_vec = Vec::with_capacity(new_allocated);
        new_vec.extend_from_slice(&array[..list.used]);
        Some(new_vec.into_boxed_slice())
    } else {
        let layout = Layout::array::<T>(new_allocated).map_err(|_| TryFromIntError(()))?;
        unsafe {
            let ptr = alloc::alloc(layout) as *mut T;
            if ptr.is_null() {
                return Err(TryFromIntError(()));
            }
            if let Some(scratch_slice) = scratch {
                ptr::copy_nonoverlapping(
                    scratch_slice.as_ptr(),
                    ptr,
                    list.used.min(scratch_slice.len()),
                );
            }
            Some(Box::from_raw(ptr::slice_from_raw_parts_mut(ptr, new_allocated)))
        }
    };

    list.array = new_array;
    list.allocated = new_allocated;
    Ok(())
}