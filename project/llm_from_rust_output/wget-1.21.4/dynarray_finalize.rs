use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::mem;

pub type SizeT = usize;

#[derive(Clone)]
pub struct DynarrayHeader {
    pub used: SizeT,
    pub allocated: SizeT,
    pub array: Option<NonNull<u8>>,
}

#[derive(Clone)]
pub struct DynarrayFinalizeResult {
    pub array: Option<NonNull<u8>>,
    pub length: SizeT,
}

impl DynarrayHeader {
    fn is_error(&self) -> bool {
        self.allocated == Self::error_marker()
    }

    fn error_marker() -> SizeT {
        (-1isize) as SizeT
    }
}

pub fn gl_dynarray_finalize(
    mut list: DynarrayHeader,
    scratch: Option<NonNull<u8>>,
    element_size: SizeT,
) -> Result<DynarrayFinalizeResult, ()> {
    if list.is_error() {
        return Err(());
    }

    let used = list.used;
    if used == 0 {
        if let Some(array) = list.array {
            if scratch != Some(array) {
                let layout = Layout::array::<u8>(list.allocated).unwrap();
                unsafe { dealloc(array.as_ptr(), layout) };
            }
        }
        return Ok(DynarrayFinalizeResult {
            array: None,
            length: 0,
        });
    }

    let allocation_size = used.checked_mul(element_size).ok_or(())?;
    let layout = Layout::array::<u8>(allocation_size).map_err(|_| ())?;
    let heap_array = unsafe { alloc(layout) };
    if heap_array.is_null() {
        return Err(());
    }

    let heap_array = NonNull::new(heap_array).ok_or(())?;
    
    if let Some(array) = list.array {
        unsafe {
            std::ptr::copy_nonoverlapping(
                array.as_ptr(),
                heap_array.as_ptr(),
                allocation_size,
            );
        }
        
        if scratch != Some(array) {
            let old_layout = Layout::array::<u8>(list.allocated).unwrap();
            unsafe { dealloc(array.as_ptr(), old_layout) };
        }
    }

    Ok(DynarrayFinalizeResult {
        array: Some(heap_array),
        length: used,
    })
}