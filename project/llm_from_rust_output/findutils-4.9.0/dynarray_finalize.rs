use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::mem;

pub type SizeT = usize;

#[derive(Debug)]
pub enum DynarrayError {
    AllocationFailed,
    InvalidState,
}

#[derive(Debug)]
pub struct DynarrayHeader {
    pub used: SizeT,
    pub allocated: SizeT,
    pub array: Option<NonNull<u8>>,
}

#[derive(Debug)]
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

    pub fn finalize(
        mut self,
        scratch: Option<NonNull<u8>>,
        element_size: SizeT,
    ) -> Result<DynarrayFinalizeResult, DynarrayError> {
        if self.is_error() {
            return Err(DynarrayError::InvalidState);
        }

        let used = self.used;
        if used == 0 {
            if let Some(array) = self.array {
                if scratch != Some(array) {
                    let layout = Layout::array::<u8>(self.allocated).unwrap();
                    unsafe { dealloc(array.as_ptr(), layout) };
                }
            }

            return Ok(DynarrayFinalizeResult {
                array: None,
                length: 0,
            });
        }

        let allocation_size = used.checked_mul(element_size).ok_or(DynarrayError::InvalidState)?;
        let layout = Layout::array::<u8>(allocation_size).map_err(|_| DynarrayError::InvalidState)?;
        
        let heap_array = unsafe { alloc(layout) };
        if heap_array.is_null() {
            return Err(DynarrayError::AllocationFailed);
        }

        let heap_array = NonNull::new(heap_array).unwrap();

        if let Some(array) = self.array {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    array.as_ptr(),
                    heap_array.as_ptr(),
                    allocation_size,
                );
            }

            if scratch != Some(array) {
                let old_layout = Layout::array::<u8>(self.allocated).unwrap();
                unsafe { dealloc(array.as_ptr(), old_layout) };
            }
        }

        Ok(DynarrayFinalizeResult {
            array: Some(heap_array),
            length: used,
        })
    }
}