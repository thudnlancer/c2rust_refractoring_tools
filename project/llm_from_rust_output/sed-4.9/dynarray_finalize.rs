use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::mem;

#[derive(Debug)]
pub struct DynarrayError;

#[derive(Debug)]
pub struct DynarrayHeader {
    used: usize,
    allocated: usize,
    array: Option<NonNull<u8>>,
}

#[derive(Debug)]
pub struct DynarrayFinalizeResult {
    pub array: Option<NonNull<u8>>,
    pub length: usize,
}

impl DynarrayHeader {
    fn is_error(&self) -> bool {
        self.allocated == usize::MAX
    }

    pub fn finalize(
        mut self,
        scratch: Option<NonNull<u8>>,
        element_size: usize,
    ) -> Result<DynarrayFinalizeResult, DynarrayError> {
        if self.is_error() {
            return Err(DynarrayError);
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

        let allocation_size = used.checked_mul(element_size).ok_or(DynarrayError)?;
        let layout = Layout::array::<u8>(allocation_size).map_err(|_| DynarrayError)?;
        let heap_array = unsafe { alloc(layout) };

        if heap_array.is_null() {
            return Err(DynarrayError);
        }

        if let Some(array) = self.array {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    array.as_ptr(),
                    heap_array,
                    allocation_size,
                );
            }
        }

        if let Some(array) = self.array {
            if scratch != Some(array) {
                let layout = Layout::array::<u8>(self.allocated).unwrap();
                unsafe { dealloc(array.as_ptr(), layout) };
            }
        }

        Ok(DynarrayFinalizeResult {
            array: NonNull::new(heap_array),
            length: used,
        })
    }
}