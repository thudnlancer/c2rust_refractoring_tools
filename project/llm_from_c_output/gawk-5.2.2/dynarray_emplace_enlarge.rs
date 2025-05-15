use std::mem;
use std::ptr;
use std::alloc::{self, Layout};
use std::num::TryFromIntError;

struct DynarrayHeader<T> {
    array: Option<Box<[T]>>,
    allocated: usize,
    used: usize,
}

fn dynarray_emplace_enlarge<T>(
    list: &mut DynarrayHeader<T>,
    scratch: Option<&[T]>,
) -> Result<(), TryFromIntError> 
where
    T: Clone,
{
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

    let new_array = if let Some(scratch_buf) = scratch {
        if list.array.is_none() || ptr::eq(list.array.as_ref().unwrap().as_ptr(), scratch_buf.as_ptr()) {
            let mut new_vec = Vec::with_capacity(new_allocated);
            if let Some(old_array) = &list.array {
                new_vec.extend_from_slice(&old_array[..list.used]);
            }
            Some(new_vec.into_boxed_slice())
        } else {
            let mut new_vec = Vec::with_capacity(new_allocated);
            new_vec.extend_from_slice(&list.array.as_ref().unwrap()[..list.used]);
            Some(new_vec.into_boxed_slice())
        }
    } else {
        let mut new_vec = Vec::with_capacity(new_allocated);
        if let Some(old_array) = &list.array {
            new_vec.extend_from_slice(&old_array[..list.used]);
        }
        Some(new_vec.into_boxed_slice())
    };

    list.array = new_array;
    list.allocated = new_allocated;
    Ok(())
}