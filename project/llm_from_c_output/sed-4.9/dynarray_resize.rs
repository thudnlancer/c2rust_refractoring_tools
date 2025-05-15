use std::mem;
use std::ptr;
use std::alloc::{self, Layout};
use std::num::TryFromIntError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct DynarrayHeader {
    array: Option<ptr::NonNull<u8>>,
    allocated: usize,
    used: usize,
}

#[derive(Debug)]
enum DynarrayError {
    SizeOverflow,
    AllocationFailed,
}

impl Error for DynarrayError {}
impl fmt::Display for DynarrayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DynarrayError::SizeOverflow => write!(f, "Size overflow"),
            DynarrayError::AllocationFailed => write!(f, "Allocation failed"),
        }
    }
}

impl From<TryFromIntError> for DynarrayError {
    fn from(_: TryFromIntError) -> Self {
        DynarrayError::SizeOverflow
    }
}

fn dynarray_resize(
    list: &mut DynarrayHeader,
    size: usize,
    scratch: Option<ptr::NonNull<u8>>,
    element_size: usize,
) -> Result<(), DynarrayError> {
    if size <= list.allocated {
        list.used = size;
        return Ok(());
    }

    let new_size_bytes = size.checked_mul(element_size)
        .ok_or(DynarrayError::SizeOverflow)?;

    let new_array = if list.array == scratch {
        let new_layout = Layout::from_size_align(new_size_bytes, mem::align_of::<u8>())?;
        let new_ptr = unsafe { alloc::alloc(new_layout) };
        
        if new_ptr.is_null() {
            return Err(DynarrayError::AllocationFailed);
        }

        if let Some(old_ptr) = list.array {
            let old_size = list.used * element_size;
            unsafe {
                ptr::copy_nonoverlapping(
                    old_ptr.as_ptr(),
                    new_ptr,
                    old_size,
                );
            }
        }

        ptr::NonNull::new(new_ptr)
    } else {
        let old_layout = Layout::from_size_align(list.allocated * element_size, mem::align_of::<u8>())?;
        let new_ptr = unsafe {
            alloc::realloc(
                list.array.unwrap().as_ptr(),
                old_layout,
                new_size_bytes,
            )
        };
        
        if new_ptr.is_null() {
            return Err(DynarrayError::AllocationFailed);
        }

        ptr::NonNull::new(new_ptr)
    };

    list.array = new_array;
    list.allocated = size;
    list.used = size;
    Ok(())
}