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

fn dynarray_resize(
    list: &mut DynarrayHeader,
    size: usize,
    scratch: Option<ptr::NonNull<u8>>,
    element_size: NonZeroUsize,
) -> Result<(), alloc::LayoutError> {
    // The existing allocation provides sufficient room.
    if size <= list.allocated {
        list.used = size;
        return Ok(());
    }

    // Otherwise, use size as the new allocation size. The caller is
    // expected to provide the final size of the array, so there is no
    // over-allocation here.

    let new_size_bytes = size.checked_mul(element_size.get())
        .ok_or(alloc::LayoutError)?;

    let new_layout = Layout::from_size_align(new_size_bytes, mem::align_of::<u8>())?;

    let new_array = match (list.array, scratch) {
        (Some(list_ptr), Some(scratch_ptr)) if list_ptr == scratch_ptr => {
            // The previous array was not heap-allocated.
            let new_ptr = unsafe { alloc::alloc(new_layout) };
            if new_ptr.is_null() {
                return Err(alloc::LayoutError);
            }
            let new_ptr = ptr::NonNull::new(new_ptr).unwrap();
            
            if let Some(old_ptr) = list.array {
                unsafe {
                    ptr::copy_nonoverlapping(
                        old_ptr.as_ptr(),
                        new_ptr.as_ptr(),
                        list.used * element_size.get()
                    );
                }
            }
            new_ptr
        }
        (Some(old_ptr), _) => {
            let new_ptr = unsafe { alloc::realloc(old_ptr.as_ptr(), new_layout, new_size_bytes) };
            if new_ptr.is_null() {
                return Err(alloc::LayoutError);
            }
            ptr::NonNull::new(new_ptr).unwrap()
        }
        (None, _) => {
            let new_ptr = unsafe { alloc::alloc(new_layout) };
            if new_ptr.is_null() {
                return Err(alloc::LayoutError);
            }
            ptr::NonNull::new(new_ptr).unwrap()
        }
    };

    list.array = Some(new_array);
    list.allocated = size;
    list.used = size;
    Ok(())
}