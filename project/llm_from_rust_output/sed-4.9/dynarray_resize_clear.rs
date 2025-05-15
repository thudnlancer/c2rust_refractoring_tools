use std::mem;
use std::ptr;
use std::slice;

pub type SizeT = usize;

#[derive(Clone)]
pub struct DynarrayHeader {
    pub used: SizeT,
    pub allocated: SizeT,
    pub array: Vec<u8>,
}

impl DynarrayHeader {
    pub fn new() -> Self {
        DynarrayHeader {
            used: 0,
            allocated: 0,
            array: Vec::new(),
        }
    }

    pub fn resize(&mut self, new_size: SizeT, element_size: SizeT) -> bool {
        if new_size > self.allocated {
            let new_alloc = new_size.next_power_of_two();
            self.array.resize(new_alloc * element_size, 0);
            self.allocated = new_alloc;
        }
        self.used = new_size;
        true
    }

    pub fn resize_clear(&mut self, new_size: SizeT, element_size: SizeT) -> bool {
        let old_size = self.used;
        if !self.resize(new_size, element_size) {
            return false;
        }
        if new_size > old_size {
            let start = old_size * element_size;
            let end = new_size * element_size;
            let slice = &mut self.array[start..end];
            ptr::write_bytes(slice.as_mut_ptr(), 0, slice.len());
        }
        true
    }

    pub fn as_slice<T>(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(
                self.array.as_ptr() as *const T,
                self.used,
            )
        }
    }

    pub fn as_mut_slice<T>(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(
                self.array.as_mut_ptr() as *mut T,
                self.used,
            )
        }
    }
}