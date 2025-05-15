use std::ptr;
use std::mem;
use std::alloc::{alloc, dealloc, Layout};

type SizeT = usize;

#[derive(Clone)]
pub struct DynarrayHeader {
    pub used: SizeT,
    pub allocated: SizeT,
    pub array: Option<Box<[u8]>>,
}

#[derive(Clone)]
pub struct DynarrayFinalizeResult {
    pub array: Option<Box<[u8]>>,
    pub length: SizeT,
}

impl DynarrayHeader {
    fn is_error(&self) -> bool {
        self.allocated == usize::MAX
    }
}

pub fn dynarray_finalize(
    mut list: DynarrayHeader,
    scratch: Option<Box<[u8]>>,
    element_size: SizeT,
) -> Option<DynarrayFinalizeResult> {
    if list.is_error() {
        return None;
    }

    let used = list.used;
    if used == 0 {
        if let Some(arr) = list.array {
            if !ptr::eq(arr.as_ptr(), scratch.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null())) {
                // Box will be dropped here automatically
            }
        }
        return Some(DynarrayFinalizeResult {
            array: None,
            length: 0,
        });
    }

    let allocation_size = used.checked_mul(element_size)?;
    let layout = Layout::from_size_align(allocation_size, 1).ok()?;

    unsafe {
        let heap_ptr = alloc(layout);
        if heap_ptr.is_null() {
            return None;
        }

        if let Some(ref arr) = list.array {
            ptr::copy_nonoverlapping(arr.as_ptr(), heap_ptr, allocation_size);
        }

        if let Some(arr) = list.array {
            if !ptr::eq(arr.as_ptr(), scratch.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null())) {
                // Box will be dropped here automatically
            }
        }

        let heap_slice = Box::from_raw(std::slice::from_raw_parts_mut(heap_ptr, allocation_size));
        
        Some(DynarrayFinalizeResult {
            array: Some(heap_slice),
            length: used,
        })
    }
}